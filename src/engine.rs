// engine.rs - เป็นเครื่องยนต์สภาพต้นไม้อ่ะค่ะ

use crate::spatial::BoundingBox;
use crate::spatial::SpatialIndex;
use std::path::PathBuf;
use std::f64::consts::{FRAC_PI_2, TAU};
use crate::text::{self, HAlign, VAlign};
use crate::hatch::{self, HatchSpec};

pub const DEFAULT_DIMENSION_OFFSET: f64 = 30.0;

/// ช่วงซูม (พิกเซลต่อ 1 หน่วยของแบบ = mm)
pub const MIN_SCALE: f64 = 0.0002;
pub const MAX_SCALE: f64 = 5000.0;
/// Cairo รับพิกัดหน้าจอได้ราว ±8 ล้านพิกเซล: เผื่อไว้ครึ่งหนึ่งสำหรับปลายเส้นที่อยู่นอกจอ
const SAFE_SCREEN_PX: f64 = 4.0e6;

#[derive(Debug, Clone)]
pub struct DimensionGeometry {
    pub start: Point,
    pub end: Point,
    pub dim_from: Point,
    pub dim_to: Point,
}

pub fn linear_dimension_geometry(
    start: Point,
    end: Point,
    offset: f64,
) -> Option<DimensionGeometry> {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let dist = (dx * dx + dy * dy).sqrt();

    if dist < 0.001 {
        return None;
    }

    // Perpendicular direction
    let px = -dy / dist;
    let py = dx / dist;

    Some(DimensionGeometry {
        start,
        end,
        dim_from: Point {
            x: start.x + px * offset,
            y: start.y + py * offset,
        },
        dim_to: Point {
            x: end.x + px * offset,
            y: end.y + py * offset,
        },
    })
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ToolMode {
    #[default]
    Select,
    Line,
    Circle,
    Array,
    Move,
    Dimension,
    Arc,
    Polyline,
    Offset,
    Trim,
    Extend,
    Mirror,
    Rotate,
    Scale,
    Fillet,
    Chamfer,
    Hatch,
    Text,
    Rectangle,
    Ellipse,
    Break,
    Stretch,
    /// ลากกรอบเลือกพื้นที่พิมพ์ (กล่องพิมพ์ > Area > Window)
    PrintWindow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SnapKind {
    Endpoint,
    Midpoint,
    Center,
    Quadrant,
    Intersection,
}

impl SnapKind {
    pub fn label(self) -> &'static str {
        match self {
            SnapKind::Endpoint => "ENDPOINT",
            SnapKind::Midpoint => "MIDPOINT",
            SnapKind::Center => "CENTER",
            SnapKind::Quadrant => "QUADRANT",
            SnapKind::Intersection => "INTERSECTION",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SnapSettings {
    pub endpoint: bool,
    pub midpoint: bool,
    pub center: bool,
    pub quadrant: bool,
    pub intersection: bool,
}

impl Default for SnapSettings {
    fn default() -> Self {
        Self {
            endpoint: true,
            midpoint: true,
            center: true,
            quadrant: true,
            intersection: true,
        }
    }
}

impl SnapSettings {
    pub fn allows(&self, kind: SnapKind) -> bool {
        match kind {
            SnapKind::Endpoint => self.endpoint,
            SnapKind::Midpoint => self.midpoint,
            SnapKind::Center => self.center,
            SnapKind::Quadrant => self.quadrant,
            SnapKind::Intersection => self.intersection,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ArraySettings {
    pub rows: i32,
    pub cols: i32,
    pub spacing_x: f64,      // ค่าเป้าหมายจาก UI
    pub spacing_y: f64,      // ค่าเป้าหมายจาก UI
    pub anim_spacing_x: f64, // ค่าที่ใช้ปั่น Animation (ค่อยๆ วิ่งตาม spacing_x)
    pub anim_spacing_y: f64, // ค่าที่ใช้ปั่น Animation (ค่อยๆ วิ่งตาม spacing_y)
    pub anim_scale: f64,
    pub target_anim_scale: f64,
}

impl Default for ArraySettings {
    fn default() -> Self {
        Self {
            rows: 2,
            cols: 2,
            spacing_x: 100.0,
            spacing_y: 100.0,
            anim_spacing_x: 0.0,
            anim_spacing_y: 0.0,
            anim_scale: 0.0,
            target_anim_scale: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DrawingElement {
    pub id: u64,
    pub is_selected: bool,
    pub layer: usize,
    pub color: Option<Rgb>,
    pub style: crate::linestyle::ElementStyle,
    pub data: ElementData,
}

/// วัตถุที่ copy ไว้ (ข้ามแท็บได้): เก็บ layer ทั้งก้อนเพื่อสร้างให้ถ้าแบบปลายทางยังไม่มี
#[derive(Debug, Clone)]
pub struct ClipItem {
    pub layer: Layer,
    pub color: Option<Rgb>,
    pub style: crate::linestyle::ElementStyle,
    pub data: ElementData,
}

/// กรอบรวม (min_x, min_y, max_x, max_y) ของที่ copy ไว้
pub fn clip_bounds(items: &[ClipItem]) -> Option<(f64, f64, f64, f64)> {
    let mut r: Option<(f64, f64, f64, f64)> = None;
    for it in items {
        let el = DrawingElement {
            id: 0,
            is_selected: false,
            layer: 0,
            color: None,
            style: Default::default(),
            data: it.data.clone(),
        };
        let b = el.bounding_box();
        if !(b.min_x.is_finite() && b.max_x.is_finite() && b.min_y.is_finite() && b.max_y.is_finite()) {
            continue;
        }
        r = Some(match r {
            None => (b.min_x, b.min_y, b.max_x, b.max_y),
            Some((a, bb, c, d)) => (a.min(b.min_x), bb.min(b.min_y), c.max(b.max_x), d.max(b.max_y)),
        });
    }
    r
}

#[derive(Debug, Clone)]
pub struct Snapshot {
    history: Vec<DrawingElement>,
    layers: Vec<Layer>,
    current_layer: usize,
}

#[derive(Debug, Clone)]
pub enum ElementData {
    Line { start: Point, end: Point },
    Circle { center: Point, radius: f64 },
    Dimension { start: Point, end: Point, offset: f64 },
    /// มุมเป็นเรเดียน หมุนทวนเข็ม (CCW) จาก start ไป end และ end > start เสมอ
    Arc { center: Point, radius: f64, start_angle: f64, end_angle: f64 },
    Polyline { points: Vec<Point>, closed: bool },
    /// พื้นที่ทึบ/hatch: หลาย loop (even-odd) filled=false → วาดแต่เส้นขอบ
    Fill { loops: Vec<Vec<Point>>, filled: bool },
    /// Hatch ลายเส้น: loops (even-odd) + รูปแบบ; เส้นลายสร้างตอนวาด
    Hatch { loops: Vec<Vec<Point>>, spec: HatchSpec },
    /// ข้อความ: pos = จุดยึด, height = ความสูงตัวพิมพ์ใหญ่, rotation เรเดียนทวนเข็ม,
    /// text ขึ้นบรรทัดใหม่ด้วย '\n', width > 0 = ตัดบรรทัดที่ความกว้างนี้ (MTEXT)
    Text {
        pos: Point,
        height: f64,
        rotation: f64,
        text: String,
        halign: HAlign,
        valign: VAlign,
        width: f64,
    },
}

impl ElementData {
    pub fn translate(&mut self, dx: f64, dy: f64) {
        match self {
            ElementData::Line { start, end } | ElementData::Dimension { start, end, .. } => {
                start.x += dx;
                start.y += dy;
                end.x += dx;
                end.y += dy;
            }
            ElementData::Circle { center, .. } | ElementData::Arc { center, .. } => {
                center.x += dx;
                center.y += dy;
            }
            ElementData::Polyline { points, .. } => {
                for p in points {
                    p.x += dx;
                    p.y += dy;
                }
            }
            ElementData::Text { pos, .. } => {
                pos.x += dx;
                pos.y += dy;
            }
            ElementData::Fill { loops, .. } | ElementData::Hatch { loops, .. } => {
                for p in loops.iter_mut().flatten() {
                    p.x += dx;
                    p.y += dy;
                }
            }
        }
    }

    /// ย่อ/ขยายรอบจุด c ด้วยตัวคูณ k > 0 (ความสูงตัวอักษร รัศมี ระยะ offset และระยะห่างลายก็คูณด้วย)
    pub fn scale_about(&mut self, c: Point, k: f64) {
        if !(k.is_finite() && k > 0.0) {
            return;
        }
        let sp = |p: &mut Point| {
            p.x = c.x + (p.x - c.x) * k;
            p.y = c.y + (p.y - c.y) * k;
        };
        match self {
            ElementData::Line { start, end } => {
                sp(start);
                sp(end);
            }
            ElementData::Dimension { start, end, offset } => {
                sp(start);
                sp(end);
                *offset *= k;
            }
            ElementData::Circle { center, radius } | ElementData::Arc { center, radius, .. } => {
                sp(center);
                *radius *= k;
            }
            ElementData::Polyline { points, .. } => points.iter_mut().for_each(|p| sp(p)),
            ElementData::Text { pos, height, width, .. } => {
                sp(pos);
                *height *= k;
                *width *= k;
            }
            ElementData::Fill { loops, .. } => loops.iter_mut().flatten().for_each(|p| sp(p)),
            ElementData::Hatch { loops, spec } => {
                loops.iter_mut().flatten().for_each(|p| sp(p));
                spec.spacing *= k;
            }
        }
    }

    pub fn ref_point(&self) -> Point {
        match self {
            ElementData::Line { start, .. } | ElementData::Dimension { start, .. } => *start,
            ElementData::Circle { center, .. } | ElementData::Arc { center, .. } => *center,
            ElementData::Polyline { points, .. } => {
                points.first().copied().unwrap_or(Point { x: 0.0, y: 0.0 })
            }
            ElementData::Text { pos, .. } => *pos,
            ElementData::Fill { loops, .. } | ElementData::Hatch { loops, .. } => loops
                .iter()
                .flatten()
                .next()
                .copied()
                .unwrap_or(Point { x: 0.0, y: 0.0 }),
        }
    }
}

impl ElementData {
    /// Stretch: จุดที่อยู่ในกรอบ r = (min_x, min_y, max_x, max_y) ขยับตาม (dx, dy) จุดอื่นอยู่ที่เดิม
    /// วงกลม/ส่วนโค้ง/ข้อความ ขยับทั้งชิ้นเมื่อจุดอ้างอิง (ศูนย์กลาง/จุดวาง) อยู่ในกรอบ
    /// None = ไม่มีอะไรขยับ
    pub fn stretched(&self, r: (f64, f64, f64, f64), dx: f64, dy: f64) -> Option<ElementData> {
        let inside = |p: &Point| p.x >= r.0 && p.x <= r.2 && p.y >= r.1 && p.y <= r.3;
        let mv = |p: &Point| if inside(p) { Point { x: p.x + dx, y: p.y + dy } } else { *p };
        let mut out = self.clone();
        let mut moved = false;
        match &mut out {
            ElementData::Line { start, end } | ElementData::Dimension { start, end, .. } => {
                moved = inside(start) || inside(end);
                let (s, e) = (mv(start), mv(end));
                *start = s;
                *end = e;
            }
            ElementData::Circle { center, .. } | ElementData::Arc { center, .. } => {
                if inside(center) {
                    moved = true;
                    center.x += dx;
                    center.y += dy;
                }
            }
            ElementData::Polyline { points, .. } => {
                for p in points.iter_mut() {
                    if inside(p) {
                        moved = true;
                        p.x += dx;
                        p.y += dy;
                    }
                }
            }
            ElementData::Text { pos, .. } => {
                if inside(pos) {
                    moved = true;
                    pos.x += dx;
                    pos.y += dy;
                }
            }
            ElementData::Fill { loops, .. } | ElementData::Hatch { loops, .. } => {
                for p in loops.iter_mut().flatten() {
                    if inside(p) {
                        moved = true;
                        p.x += dx;
                        p.y += dy;
                    }
                }
            }
        }
        moved.then_some(out)
    }
}

/// สะท้อนจุด p ข้ามเส้นตรงที่ผ่าน a และ b
pub fn reflect_point(p: Point, a: Point, b: Point) -> Point {
    let (dx, dy) = (b.x - a.x, b.y - a.y);
    let l2 = dx * dx + dy * dy;
    if l2 < 1e-24 {
        return p;
    }
    let t = ((p.x - a.x) * dx + (p.y - a.y) * dy) / l2;
    let (fx, fy) = (a.x + dx * t, a.y + dy * t);
    Point { x: 2.0 * fx - p.x, y: 2.0 * fy - p.y }
}

/// ผลคำนวณ Fillet / Chamfer (ยังไม่แก้ข้อมูล)
struct CornerPlan {
    info: CornerInfo,
    i1: usize,
    i2: usize,
    t1: Point,
    t2: Point,
    extra: Option<ElementData>,
}

/// ผลของ Fillet / Chamfer ที่ใช้เล่นแอนิเมชัน "มุมยุบ"
#[derive(Debug, Clone)]
pub struct CornerInfo {
    pub ids: Vec<u64>, // เส้นสองเส้น + วัตถุใหม่ (ส่วนโค้ง/เส้นตัดมุม) ถ้ามี
    pub x: Point,      // จุดตัดของเส้นสองเส้น
    pub u1: (f64, f64),
    pub u2: (f64, f64),
    pub e1: Point, // ปลายที่เก็บไว้ของแต่ละเส้น
    pub e2: Point,
    pub ang: f64,
    pub v: f64,
    pub chamfer: bool,
    pub tdist: f64,
}

impl CornerInfo {
    /// ภาพมุมตอนยุบไปแล้ว e (0 = มุมแหลมเดิม, 1 = ผลลัพธ์จริง)
    pub fn ghost(&self, e: f64) -> Vec<ElementData> {
        let td = self.tdist * e;
        let x = self.x;
        let t1 = Point { x: x.x + self.u1.0 * td, y: x.y + self.u1.1 * td };
        let t2 = Point { x: x.x + self.u2.0 * td, y: x.y + self.u2.1 * td };
        let mut out = vec![
            ElementData::Line { start: t1, end: self.e1 },
            ElementData::Line { start: t2, end: self.e2 },
        ];
        if self.chamfer {
            if td > 1e-12 {
                out.push(ElementData::Line { start: t1, end: t2 });
            }
        } else {
            let v = self.v * e;
            if v > 1e-9 {
                let bis = (self.u1.0 + self.u2.0, self.u1.1 + self.u2.1);
                let bl = bis.0.hypot(bis.1);
                let cdist = v / (self.ang / 2.0).sin();
                let c = Point { x: x.x + bis.0 / bl * cdist, y: x.y + bis.1 / bl * cdist };
                let aa = (t1.y - c.y).atan2(t1.x - c.x);
                let ab = (t2.y - c.y).atan2(t2.x - c.x);
                let pi = std::f64::consts::PI;
                let dd = (ab - aa + pi).rem_euclid(TAU) - pi;
                let (s, span) = if dd >= 0.0 { (aa, dd) } else { (ab, -dd) };
                let s = s.rem_euclid(TAU);
                out.push(ElementData::Arc {
                    center: c,
                    radius: v,
                    start_angle: s,
                    end_angle: s + span,
                });
            }
        }
        out
    }
}

impl ElementData {
    /// สำเนาที่หมุนรอบจุด c ด้วยมุม ang เรเดียน (ทวนเข็ม)
    pub fn rotated(&self, c: Point, ang: f64) -> ElementData {
        let (s, co) = ang.sin_cos();
        let r = |p: &Point| Point {
            x: c.x + (p.x - c.x) * co - (p.y - c.y) * s,
            y: c.y + (p.x - c.x) * s + (p.y - c.y) * co,
        };
        match self {
            ElementData::Line { start, end } => ElementData::Line { start: r(start), end: r(end) },
            ElementData::Circle { center, radius } => {
                ElementData::Circle { center: r(center), radius: *radius }
            }
            ElementData::Dimension { start, end, offset } => {
                ElementData::Dimension { start: r(start), end: r(end), offset: *offset }
            }
            ElementData::Arc { center, radius, start_angle, end_angle } => {
                let span = end_angle - start_angle;
                let a0 = (start_angle + ang).rem_euclid(TAU);
                ElementData::Arc {
                    center: r(center),
                    radius: *radius,
                    start_angle: a0,
                    end_angle: a0 + span,
                }
            }
            ElementData::Polyline { points, closed } => ElementData::Polyline {
                points: points.iter().map(r).collect(),
                closed: *closed,
            },
            ElementData::Fill { loops, filled } => ElementData::Fill {
                loops: loops.iter().map(|l| l.iter().map(r).collect()).collect(),
                filled: *filled,
            },
            ElementData::Hatch { loops, spec } => ElementData::Hatch {
                loops: loops.iter().map(|l| l.iter().map(r).collect()).collect(),
                spec: HatchSpec { angle: spec.angle + ang, ..*spec },
            },
            ElementData::Text { pos, height, rotation, text, halign, valign, width } => {
                ElementData::Text {
                    pos: r(pos),
                    height: *height,
                    rotation: rotation + ang,
                    text: text.clone(),
                    halign: *halign,
                    valign: *valign,
                    width: *width,
                }
            }
        }
    }

    /// สำเนาที่ย่อ/ขยายรอบจุด c ด้วยตัวคูณ k
    pub fn scaled(&self, c: Point, k: f64) -> ElementData {
        let mut e = self.clone();
        e.scale_about(c, k);
        e
    }
}

impl ElementData {
    /// สำเนาที่สะท้อนข้ามเส้น a→b (ข้อความ: ย้ายตำแหน่ง แต่ไม่กลับด้านตัวอักษร)
    pub fn mirrored(&self, a: Point, b: Point) -> ElementData {
        let r = |p: &Point| reflect_point(*p, a, b);
        let phi = (b.y - a.y).atan2(b.x - a.x);
        match self {
            ElementData::Line { start, end } => ElementData::Line { start: r(start), end: r(end) },
            ElementData::Circle { center, radius } => {
                ElementData::Circle { center: r(center), radius: *radius }
            }
            // การสะท้อนกลับด้านซ้าย-ขวา: offset ต้องกลับเครื่องหมายเพื่อให้เส้นมิติอยู่ฝั่งที่สะท้อนถูก
            ElementData::Dimension { start, end, offset } => {
                ElementData::Dimension { start: r(start), end: r(end), offset: -*offset }
            }
            ElementData::Arc { center, radius, start_angle, end_angle } => {
                let span = end_angle - start_angle;
                let s = (2.0 * phi - end_angle).rem_euclid(TAU);
                ElementData::Arc { center: r(center), radius: *radius, start_angle: s, end_angle: s + span }
            }
            ElementData::Polyline { points, closed } => ElementData::Polyline {
                points: points.iter().map(r).collect(),
                closed: *closed,
            },
            ElementData::Fill { loops, filled } => ElementData::Fill {
                loops: loops.iter().map(|l| l.iter().map(r).collect()).collect(),
                filled: *filled,
            },
            ElementData::Hatch { loops, spec } => ElementData::Hatch {
                loops: loops.iter().map(|l| l.iter().map(r).collect()).collect(),
                // เส้นลายที่ทำมุม θ สะท้อนข้ามแกนมุม φ จะเป็นมุม 2φ − θ
                spec: HatchSpec { angle: 2.0 * phi - spec.angle, ..*spec },
            },
            ElementData::Text { pos, height, rotation, text, halign, valign, width } => {
                ElementData::Text {
                    pos: r(pos),
                    height: *height,
                    rotation: *rotation,
                    text: text.clone(),
                    halign: *halign,
                    valign: *valign,
                    width: *width,
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl From<(f64, f64)> for Point {
    fn from(p: (f64, f64)) -> Self {
        Point { x: p.0, y: p.1 }
    }
}

pub type Rgb = (u8, u8, u8);

#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub color: Rgb,
    pub visible: bool,
    pub locked: bool,
    /// ความหนาเส้นเริ่มต้นของ layer (มม.)
    pub weight: f64,
    /// None = เส้นทึบ
    pub linetype: Option<crate::linestyle::Lt>,
}

impl Layer {
    pub fn new(name: &str, color: Rgb) -> Self {
        Self {
            name: name.to_string(),
            color,
            visible: true,
            locked: false,
            weight: crate::linestyle::DEFAULT_WEIGHT,
            linetype: None,
        }
    }
}

#[derive(Debug)]
pub struct Camera {
    pub scale: f64,
    pub target_scale: f64,
    pub zoom_index: usize,
    pub offset: (f64, f64),
    pub target_offset: (f64, f64),
    /// เพดานซูมเข้าตอนนี้ (Engine ปรับตามขนาดแบบ)
    pub max_scale: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RenderCommand {
    Line {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        is_selected: bool,
        color: (f64, f64, f64),
    },
    Circle {
        cx: f64,
        cy: f64,
        radius: f64,
        is_selected: bool,
        color: (f64, f64, f64),
    },
    Dimension {
        text_x: f64,
        text_y: f64,
        text: String,
        is_selected: bool,
        // เส้นมิติหลัก (dim line)
        dim_x1: f64, dim_y1: f64,
        dim_x2: f64, dim_y2: f64,
        // extension lines (2 เส้นขั้น)
        ext1_x1: f64, ext1_y1: f64, ext1_x2: f64, ext1_y2: f64,
        ext2_x1: f64, ext2_y1: f64, ext2_x2: f64, ext2_y2: f64,
    },
    Arc {
        cx: f64,
        cy: f64,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
        is_selected: bool,
        color: (f64, f64, f64),
    },
    Polyline {
        points: Vec<(f64, f64)>,
        closed: bool,
        is_selected: bool,
        color: (f64, f64, f64),
    },
    Fill {
        loops: Vec<Vec<(f64, f64)>>,
        filled: bool,
        is_selected: bool,
        color: (f64, f64, f64),
    },
    /// ตั้งความหนา (px) และ dash (px, ว่าง = ทึบ) ให้คำสั่งวาดเส้นที่ตามมา
    Style { lw: f64, dash: Vec<f64> },
    /// พิกัดและขนาดเป็นพิกเซลหน้าจอแล้ว; em_px = ขนาด em ของฟอนต์, wrap_px > 0 = ตัดบรรทัด
    Text {
        x: f64,
        y: f64,
        em_px: f64,
        rotation: f64,
        wrap_px: f64,
        text: String,
        halign: HAlign,
        valign: VAlign,
        is_selected: bool,
        color: (f64, f64, f64),
    },
}

#[derive(Debug)]
pub struct ApplyAnimation {
    pub is_playing: bool,
    pub timer: f64,
    pub total_duration: f64, // เวลารวมทั้งหมด
    pub stagger_delay: f64,  // delay ระหว่างแต่ละตัว
    pub elements_to_reveal: Vec<u64>,
}

impl Default for ApplyAnimation {
    fn default() -> Self {
        Self {
            is_playing: false,
            timer: 0.0,
            total_duration: 0.0,
            stagger_delay: 0.03,
            elements_to_reveal: Vec::new(),
        }
    }
}

impl DrawingElement {
    pub fn bounding_box(&self) -> BoundingBox {
        match &self.data {
            ElementData::Line { start, end } => {
                BoundingBox::from_points(start.x, start.y, end.x, end.y)
            }
            ElementData::Circle { center, radius } => BoundingBox {
                min_x: center.x - radius,
                min_y: center.y - radius,
                max_x: center.x + radius,
                max_y: center.y + radius,
            },
            ElementData::Dimension { start, end, offset } => {
                if let Some(g) = linear_dimension_geometry(*start, *end, *offset) {
                    BoundingBox {
                        min_x: g.start.x.min(g.end.x).min(g.dim_from.x).min(g.dim_to.x),
                        min_y: g.start.y.min(g.end.y).min(g.dim_from.y).min(g.dim_to.y),
                        max_x: g.start.x.max(g.end.x).max(g.dim_from.x).max(g.dim_to.x),
                        max_y: g.start.y.max(g.end.y).max(g.dim_from.y).max(g.dim_to.y),
                    }
                } else {
                    BoundingBox::from_points(start.x, start.y, end.x, end.y)
                }
            }
            ElementData::Arc { center, radius, start_angle, end_angle } => {
                arc_bounds(*center, *radius, *start_angle, *end_angle)
            }
            ElementData::Polyline { points, .. } => {
                let mut b = BoundingBox {
                    min_x: f64::MAX,
                    min_y: f64::MAX,
                    max_x: f64::MIN,
                    max_y: f64::MIN,
                };
                for p in points {
                    b.min_x = b.min_x.min(p.x);
                    b.min_y = b.min_y.min(p.y);
                    b.max_x = b.max_x.max(p.x);
                    b.max_y = b.max_y.max(p.y);
                }
                b
            }
            ElementData::Text { pos, height, rotation, text, halign, valign, width } => {
                text::bbox(*pos, *height, *rotation, text, *halign, *valign, *width)
            }
            ElementData::Fill { loops, .. } | ElementData::Hatch { loops, .. } => {
                let mut b = BoundingBox {
                    min_x: f64::MAX,
                    min_y: f64::MAX,
                    max_x: f64::MIN,
                    max_y: f64::MIN,
                };
                for p in loops.iter().flatten() {
                    b.min_x = b.min_x.min(p.x);
                    b.min_y = b.min_y.min(p.y);
                    b.max_x = b.max_x.max(p.x);
                    b.max_y = b.max_y.max(p.y);
                }
                if b.min_x > b.max_x {
                    b = BoundingBox { min_x: 0.0, min_y: 0.0, max_x: 0.0, max_y: 0.0 };
                }
                b
            }
        }
    }

    pub fn cloned_with_offset(&self, id: u64, dx: f64, dy: f64) -> Self {
        let mut new_el = self.clone();
        new_el.id = id;
        new_el.is_selected = false; // ตัวก๊อปปี้ไม่ต้องติดสถานะเลือก
        new_el.data.translate(dx, dy);
        new_el
    }
}

impl Camera {
    pub fn new() -> Self {
        Self {
            scale: 1.0,
            target_scale: 1.0,
            zoom_index: 0,
            offset: (0.0, 0.0),
            target_offset: (0.0, 0.0),
            max_scale: 50.0,
        }
    }

    pub fn update(&mut self, dt: f64) -> bool {
        let lerp_factor = (dt * 10.0).min(1.0);
        let mut changed = false;

        // scale กับ offset ต้องเดินพร้อมกันและหยุดพร้อมกันเสมอ
        // (ซูมลึก offset มีค่าเป็นล้านพิกเซล ถ้า scale หยุดก่อน offset จะไถลต่อจนภาพส่าย)
        let scale_ratio = (self.scale / self.target_scale).ln().abs();
        let dx = self.target_offset.0 - self.offset.0;
        let dy = self.target_offset.1 - self.offset.1;

        if scale_ratio > 0.00001 || dx.abs() > 0.01 || dy.abs() > 0.01 {
            self.scale += (self.target_scale - self.scale) * lerp_factor;
            self.offset.0 += dx * lerp_factor;
            self.offset.1 += dy * lerp_factor;
            changed = true;
        } else {
            self.scale = self.target_scale;
            self.offset = self.target_offset;
        }

        changed
    }

    pub fn screen_to_world(&self, x: f64, y: f64) -> (f64, f64) {
        let wx = (x - self.offset.0) / self.scale;
        let wy = -(y - self.offset.1) / self.scale;
        (wx, wy)
    }

    pub fn world_to_screen(&self, x: f64, y: f64) -> (f64, f64) {
        let sx = x * self.scale + self.offset.0;
        let sy = -y * self.scale + self.offset.1;
        (sx, sy)
    }

    fn screen_to_world_target(&self, x: f64, y: f64) -> (f64, f64) {
        let wx = (x - self.target_offset.0) / self.target_scale;
        let wy = -(y - self.target_offset.1) / self.target_scale;
        (wx, wy)
    }

    pub fn focus_on_area(&mut self, min_p: Point, max_p: Point, screen_size: (f64, f64)) {
        let center_world = ((min_p.x + max_p.x) / 2.0, (min_p.y + max_p.y) / 2.0);
        let world_w = (max_p.x - min_p.x).max(1.0); // กันหารศูนย์
        let world_h = (max_p.y - min_p.y).max(1.0);
        let scale_x = (screen_size.0 * 0.9) / world_w;
        let scale_y = (screen_size.1 * 0.9) / world_h;
        self.target_scale = scale_x.min(scale_y);
        self.target_offset.0 = (screen_size.0 / 2.0) - (center_world.0 * self.target_scale);
        self.target_offset.1 = (screen_size.1 / 2.0) + (center_world.1 * self.target_scale);
    }

    pub fn zoom_in_at(&mut self, screen_x: f64, screen_y: f64) {
        let (wx, wy) = self.screen_to_world_target(screen_x, screen_y);
        self.target_scale = (self.target_scale * 1.2).min(self.max_scale);
        self.target_offset.0 = screen_x - (wx * self.target_scale);
        self.target_offset.1 = screen_y - (-wy * self.target_scale);
    }

    pub fn zoom_out_at(&mut self, screen_x: f64, screen_y: f64) {
        let (wx, wy) = self.screen_to_world_target(screen_x, screen_y);
        self.target_scale = (self.target_scale / 1.2).max(MIN_SCALE);
        self.target_offset.0 = screen_x - (wx * self.target_scale);
        self.target_offset.1 = screen_y - (-wy * self.target_scale);
    }

    pub fn zoom_to_point(
        &mut self,
        world_x: f64,
        world_y: f64,
        screen_size: (f64, f64),
        factor: f64,
    ) {
        self.target_scale *= factor;
        self.target_scale = self.target_scale.clamp(MIN_SCALE, self.max_scale);
        let center_x = screen_size.0 / 2.0;
        let center_y = screen_size.1 / 2.0;
        self.target_offset = (
            center_x - (world_x * self.target_scale),
            center_y + (world_y * self.target_scale),
        );
    }
}

#[derive(Debug)]
pub struct Engine {
    pub spatial_index: SpatialIndex<u64>,
    pub history: Vec<DrawingElement>,
    pub camera: Camera,
    pub active_tool: ToolMode,
    pub last_update_time: Option<u64>,
    pub id_counter: u64,
    pub array_settings: ArraySettings,
    pub show_array_ui: bool,
    pub apply_anim: ApplyAnimation,
    pub hovered_id: Option<u64>,
    pub undo_stack: Vec<Snapshot>,
    pub redo_stack: Vec<Snapshot>,
    pub file_path: Option<PathBuf>,
    pub dirty: bool,
    dirty_before_cp: bool,
    /// สถานะก่อน checkpoint ล่าสุด (คืนให้เมื่อทิ้ง checkpoint ที่ไม่ได้แก้อะไร)
    redo_before_cp: Vec<Snapshot>,
    box_before_cp: Option<(f64, f64, f64, f64)>,
    /// กรอบ crossing ล่าสุดที่ใช้เลือก (min_x, min_y, max_x, max_y) — Stretch ใช้
    pub last_sel_box: Option<(f64, f64, f64, f64)>,
    pub layers: Vec<Layer>,
    pub current_layer: usize,
    pub skipped: usize,
    /// เพิ่มทุกครั้งที่การเลือก/สไตล์ของวัตถุอาจเปลี่ยน (UI ใช้ดูว่าต้องอัปเดตไหม)
    pub sel_rev: u64,
    sel_bbox_cache: std::cell::Cell<(u64, usize, Option<(f64, f64, f64, f64)>)>,
    /// Layout (paper space) ของแบบนี้ เก็บลง DXF; layout_rev เพิ่มทุกครั้งที่แก้ (ให้ autosave รู้)
    pub layouts: Vec<crate::layout::LayoutSpec>,
    pub layout_rev: u64,
    /// กำลังลากย้าย: index ยังเป็นของตำแหน่งเดิม; moving = id ที่ต้องวาดเสมอ
    moving: Vec<u64>,
    /// วัตถุที่ซ่อนชั่วคราวระหว่างแอนิเมชัน (window วาด ghost แทน)
    pub anim_hidden: Vec<u64>,
    index_stale: bool,
    zoom_cap_key: (usize, u64),
}

impl Engine {
    pub fn new() -> Self {
        Self {
            spatial_index: SpatialIndex::new(8),
            history: Vec::new(),
            camera: Camera::new(),
            active_tool: ToolMode::Line,
            last_update_time: None,
            id_counter: 0,
            array_settings: ArraySettings::default(),
            show_array_ui: false,
            apply_anim: ApplyAnimation::default(),
            hovered_id: None,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            file_path: None,
            dirty: false,
            dirty_before_cp: false,
            redo_before_cp: Vec::new(),
            box_before_cp: None,
            last_sel_box: None,
            layers: vec![Layer::new("0", (255, 255, 255))],
            current_layer: 0,
            skipped: 0,
            sel_rev: 0,
            sel_bbox_cache: std::cell::Cell::new((u64::MAX, 0, None)),
            layouts: Vec::new(),
            layout_rev: 0,
            moving: Vec::new(),
            anim_hidden: Vec::new(),
            index_stale: false,
            zoom_cap_key: (usize::MAX, 0),
        }
    }

    fn pick_at(&self, wx: f64, wy: f64) -> Option<u64> {
        let tol = 6.0 / self.camera.scale;
        let query_box = BoundingBox {
            min_x: wx - tol,
            min_y: wy - tol,
            max_x: wx + tol,
            max_y: wy + tol,
        };
        let mut hits = Vec::new();
        self.spatial_index.query(&query_box, &mut hits);

        let p = Point { x: wx, y: wy };
        let mut best: Option<(f64, u64)> = None;
        for id in hits.into_iter().copied() {
            let Some(i) = self.index_of(id) else { continue };
            if !self.pickable(i) {
                continue;
            }
            let mut d = self.history[i].data.distance_to(p);
            // คลิกข้างในพื้นที่ทึบเลือกได้ แต่ให้เส้นที่อยู่ใกล้ ๆ ชนะ
            if d > tol * 0.9 && self.history[i].data.fill_contains(p) {
                d = tol * 0.9;
            }
            if d > tol {
                continue;
            }
            let better = match best {
                None => true,
                Some((bd, bid)) => d < bd - 1e-9 || ((d - bd).abs() <= 1e-9 && id > bid),
            };
            if better {
                best = Some((d, id));
            }
        }
        best.map(|(_, id)| id)
    }

    pub fn select_at(&mut self, world_x: f64, world_y: f64, add_to_selection: bool) {
        self.finish_move();
        self.sel_rev += 1;
        self.last_sel_box = None;
        let top = self.pick_at(world_x, world_y);

        if !add_to_selection {
            for el in &mut self.history {
                el.is_selected = false;
            }
        }
        if let Some(i) = top.and_then(|id| self.index_of(id)) {
            self.history[i].is_selected = true;
        }
    }

    pub fn hover_at(&mut self, world_x: f64, world_y: f64) {
        self.finish_move();
        self.hovered_id = self.pick_at(world_x, world_y);
    }

    pub fn offset_pick(&mut self, wx: f64, wy: f64) -> Option<u64> {
        let id = self.pick_at(wx, wy)?;
        let i = self.index_of(id)?;
        if matches!(
            self.history[i].data,
            ElementData::Dimension { .. } | ElementData::Fill { .. } | ElementData::Hatch { .. } | ElementData::Text { .. }
        ) {
            return None;
        }
        self.deselect_all();
        self.history[i].is_selected = true;
        Some(id)
    }

    pub fn offset_preview(&self, id: u64, side: Point, dist: f64) -> Option<ElementData> {
        let i = self.index_of(id)?;
        offset_data(&self.history[i].data, side, dist).ok()
    }

    /// เส้นตรง (Line) ใต้เมาส์ สำหรับ Fillet / Chamfer
    pub fn corner_pick(&self, wx: f64, wy: f64) -> Option<u64> {
        let id = self.pick_at(wx, wy)?;
        let i = self.index_of(id)?;
        matches!(self.history[i].data, ElementData::Line { .. }).then_some(id)
    }

    /// คำนวณ Fillet (chamfer = false, v = รัศมี) หรือ Chamfer (v = ระยะตัดมุมตามแต่ละเส้น) โดยไม่แก้ข้อมูล
    /// ส่วนที่คลิก (p1, p2) คือส่วนที่เก็บไว้; v = 0 ของ Fillet = ต่อมุมให้ชนกันเฉยๆ
    fn corner_plan(
        &self,
        id1: u64,
        p1: Point,
        id2: u64,
        p2: Point,
        v: f64,
        chamfer: bool,
    ) -> Result<CornerPlan, &'static str> {
        if id1 == id2 {
            return Err("Pick two different lines");
        }
        let i1 = self.index_of(id1).ok_or("Object no longer exists")?;
        let i2 = self.index_of(id2).ok_or("Object no longer exists")?;
        let (ElementData::Line { start: a1, end: b1 }, ElementData::Line { start: a2, end: b2 }) =
            (&self.history[i1].data, &self.history[i2].data)
        else {
            return Err("Fillet / Chamfer works on two straight lines");
        };
        let (a1, b1, a2, b2) = (*a1, *b1, *a2, *b2);
        let (d1x, d1y) = (b1.x - a1.x, b1.y - a1.y);
        let (d2x, d2y) = (b2.x - a2.x, b2.y - a2.y);
        let (l1, l2) = (d1x.hypot(d1y), d2x.hypot(d2y));
        let den = d1x * d2y - d1y * d2x;
        if l1 < 1e-12 || l2 < 1e-12 || den.abs() < 1e-9 * l1 * l2 {
            return Err("Lines are parallel");
        }
        let t = ((a2.x - a1.x) * d2y - (a2.y - a1.y) * d2x) / den;
        let x = Point { x: a1.x + d1x * t, y: a1.y + d1y * t };
        // ทิศจากจุดตัดไปทางด้านที่คลิก
        let dir = |dx: f64, dy: f64, l: f64, p: Point| {
            let s = (p.x - x.x) * dx + (p.y - x.y) * dy;
            let sg = if s >= 0.0 { 1.0 } else { -1.0 };
            (dx / l * sg, dy / l * sg)
        };
        let u1 = dir(d1x, d1y, l1, p1);
        let u2 = dir(d2x, d2y, l2, p2);
        // ปลายที่เก็บไว้ (ไกลจุดตัดสุดไปทางด้านที่คลิก) และระยะจากจุดตัด
        let far = |a: Point, b: Point, u: (f64, f64)| {
            let sa = (a.x - x.x) * u.0 + (a.y - x.y) * u.1;
            let sb = (b.x - x.x) * u.0 + (b.y - x.y) * u.1;
            if sa >= sb { (a, sa) } else { (b, sb) }
        };
        let (e1, s1) = far(a1, b1, u1);
        let (e2, s2) = far(a2, b2, u2);
        let ang = (u1.0 * u2.0 + u1.1 * u2.1).clamp(-1.0, 1.0).acos(); // มุมระหว่างสองแขน
        if ang < 1e-6 || std::f64::consts::PI - ang < 1e-6 {
            return Err("Lines are parallel");
        }
        let v = if v.is_finite() { v.max(0.0) } else { 0.0 };
        let tdist = if chamfer {
            v
        } else if v <= 1e-12 {
            0.0
        } else {
            v / (ang / 2.0).tan()
        };
        if tdist > 1e-12 && (tdist > s1 - 1e-6 || tdist > s2 - 1e-6) {
            return Err("Lines are too short for this size");
        }
        let t1 = Point { x: x.x + u1.0 * tdist, y: x.y + u1.1 * tdist };
        let t2 = Point { x: x.x + u2.0 * tdist, y: x.y + u2.1 * tdist };
        let extra = if !chamfer && v > 1e-12 {
            let bis = (u1.0 + u2.0, u1.1 + u2.1);
            let bl = bis.0.hypot(bis.1);
            let cdist = v / (ang / 2.0).sin();
            let c = Point { x: x.x + bis.0 / bl * cdist, y: x.y + bis.1 / bl * cdist };
            let aa = (t1.y - c.y).atan2(t1.x - c.x);
            let ab = (t2.y - c.y).atan2(t2.x - c.x);
            let dd = (ab - aa + std::f64::consts::PI).rem_euclid(TAU) - std::f64::consts::PI;
            let (s, span) = if dd >= 0.0 { (aa, dd) } else { (ab, -dd) };
            let s = s.rem_euclid(TAU);
            Some(ElementData::Arc { center: c, radius: v, start_angle: s, end_angle: s + span })
        } else if chamfer && tdist > 1e-12 {
            Some(ElementData::Line { start: t1, end: t2 })
        } else {
            None
        };
        Ok(CornerPlan {
            info: CornerInfo { ids: vec![id1, id2], x, u1, u2, e1, e2, ang, v, chamfer, tdist },
            i1,
            i2,
            t1,
            t2,
            extra,
        })
    }

    /// พรีวิว: คืนข้อมูลสำหรับวาด ghost (ไม่แก้แบบ)
    pub fn corner_preview(
        &self,
        id1: u64,
        p1: Point,
        id2: u64,
        p2: Point,
        v: f64,
        chamfer: bool,
    ) -> Result<CornerInfo, &'static str> {
        self.corner_plan(id1, p1, id2, p2, v, chamfer).map(|p| p.info)
    }

    /// ทำ Fillet / Chamfer จริง
    pub fn corner_apply(
        &mut self,
        id1: u64,
        p1: Point,
        id2: u64,
        p2: Point,
        v: f64,
        chamfer: bool,
    ) -> Result<CornerInfo, &'static str> {
        let CornerPlan { mut info, i1, i2, t1, t2, extra } =
            self.corner_plan(id1, p1, id2, p2, v, chamfer)?;
        let (layer, color, style) =
            (self.history[i1].layer, self.history[i1].color, self.history[i1].style.clone());
        self.checkpoint();
        self.history[i1].data = ElementData::Line { start: t1, end: info.e1 };
        self.history[i2].data = ElementData::Line { start: t2, end: info.e2 };
        if let Some(data) = extra {
            let id = self.get_next_id();
            info.ids.push(id);
            self.add_element(DrawingElement { id, is_selected: false, layer, color, style, data });
        }
        self.rebuild_index();
        Ok(info)
    }

    pub fn offset_apply(&mut self, id: u64, side: Point, dist: f64) -> Result<(), &'static str> {
        self.offset_apply_new(id, side, dist).map(|_| ())
    }

    /// เหมือน offset_apply แต่คืน id ของวัตถุที่สร้างใหม่
    pub fn offset_apply_new(
        &mut self,
        id: u64,
        side: Point,
        dist: f64,
    ) -> Result<u64, &'static str> {
        let i = self.index_of(id).ok_or("Object no longer exists")?;
        let data = offset_data(&self.history[i].data, side, dist)?;
        let (layer, color, style) =
            (self.history[i].layer, self.history[i].color, self.history[i].style.clone());
        self.checkpoint();
        let id = self.get_next_id();
        self.add_element(DrawingElement { id, is_selected: false, layer, color, style, data });
        Ok(id)
    }

    fn cutters_in(&self, skip_id: u64, area: &BoundingBox) -> Vec<Prim> {
        let mut hits = Vec::new();
        self.spatial_index.query(area, &mut hits);
        hits.into_iter()
            .copied()
            .filter(|&id| id != skip_id)
            .filter_map(|id| self.index_of(id))
            .filter(|&i| self.layer_visible(i))
            .flat_map(|i| prims(&self.history[i].data))
            .collect()
    }

    fn trim_cutters(&self, id: u64, i: usize) -> Vec<Prim> {
        let b = self.history[i].bounding_box();
        let pad = 1e-6 * (1.0 + (b.max_x - b.min_x).max(b.max_y - b.min_y));
        let area = BoundingBox {
            min_x: b.min_x - pad,
            min_y: b.min_y - pad,
            max_x: b.max_x + pad,
            max_y: b.max_y + pad,
        };
        self.cutters_in(id, &area)
    }

    fn extend_cutters(&self, id: u64, i: usize) -> (Vec<Prim>, f64) {
        let diag = self
            .get_elements_bounds()
            .map(|(a, b)| (b.x - a.x).hypot(b.y - a.y))
            .unwrap_or(0.0);
        let reach = diag + 1.0;
        let cutters = match extend_area(&self.history[i].data, reach) {
            Some(area) => self.cutters_in(id, &area),
            None => Vec::new(),
        };
        (cutters, reach)
    }

    pub fn trim_at(&mut self, wx: f64, wy: f64) -> Result<(), &'static str> {
        let id = self.pick_at(wx, wy).ok_or("Click on an object to trim")?;
        let i = self.index_of(id).ok_or("Object no longer exists")?;
        let cutters = self.trim_cutters(id, i);
        let (pieces, _) = trim_data(&self.history[i].data, Point { x: wx, y: wy }, &cutters)?;
        let (layer, color, style) =
            (self.history[i].layer, self.history[i].color, self.history[i].style.clone());
        self.checkpoint();
        self.history.remove(i);
        for data in pieces {
            let nid = self.get_next_id();
            self.history.push(DrawingElement {
                id: nid,
                is_selected: false,
                layer,
                color,
                style: style.clone(),
                data,
            });
        }
        self.rebuild_index();
        Ok(())
    }

    pub fn extend_at(&mut self, wx: f64, wy: f64) -> Result<(), &'static str> {
        let id = self.pick_at(wx, wy).ok_or("Click on an object to extend")?;
        let i = self.index_of(id).ok_or("Object no longer exists")?;
        let (cutters, reach) = self.extend_cutters(id, i);
        let (new, _) = extend_data(&self.history[i].data, Point { x: wx, y: wy }, &cutters, reach)?;
        self.checkpoint();
        self.history[i].data = new;
        self.rebuild_index();
        Ok(())
    }

    pub fn trim_preview(&self, wx: f64, wy: f64) -> Option<ElementData> {
        let id = self.pick_at(wx, wy)?;
        let i = self.index_of(id)?;
        let cutters = self.trim_cutters(id, i);
        trim_data(&self.history[i].data, Point { x: wx, y: wy }, &cutters).ok().map(|(_, r)| r)
    }

    pub fn extend_preview(&self, wx: f64, wy: f64) -> Option<ElementData> {
        let id = self.pick_at(wx, wy)?;
        let i = self.index_of(id)?;
        let (cutters, reach) = self.extend_cutters(id, i);
        extend_data(&self.history[i].data, Point { x: wx, y: wy }, &cutters, reach)
            .ok()
            .map(|(_, a)| a)
    }

    /// แตก Polyline ที่เลือกเป็นเส้นตรงทีละช่วง (ผลลัพธ์ยังถูกเลือกอยู่) คืนจำนวน polyline ที่แตก
    pub fn explode_selected(&mut self) -> usize {
        let ids: std::collections::HashSet<u64> = self
            .history
            .iter()
            .filter(|e| e.is_selected && matches!(e.data, ElementData::Polyline { .. }))
            .map(|e| e.id)
            .collect();
        if ids.is_empty() {
            return 0;
        }
        self.checkpoint();
        let mut news = Vec::new();
        for el in self.history.iter().filter(|e| ids.contains(&e.id)) {
            if let ElementData::Polyline { points, closed } = &el.data {
                let n = points.len();
                let segs = if *closed { n } else { n.saturating_sub(1) };
                for i in 0..segs {
                    let (a, b) = (points[i], points[(i + 1) % n]);
                    if (a.x - b.x).hypot(a.y - b.y) > 1e-12 {
                        news.push((el.layer, el.color, el.style.clone(), ElementData::Line { start: a, end: b }));
                    }
                }
            }
        }
        self.history.retain(|e| !ids.contains(&e.id));
        for (layer, color, style, data) in news {
            let id = self.get_next_id();
            self.history.push(DrawingElement { id, is_selected: true, layer, color, style, data });
        }
        self.rebuild_index();
        self.hovered_id = None;
        ids.len()
    }

    /// วัตถุที่ Break ได้ใต้จุดนี้ (Line / Circle / Arc / Polyline)
    pub fn break_pick(&self, wx: f64, wy: f64) -> Option<u64> {
        let id = self.pick_at(wx, wy)?;
        let i = self.index_of(id)?;
        matches!(
            self.history[i].data,
            ElementData::Line { .. }
                | ElementData::Circle { .. }
                | ElementData::Arc { .. }
                | ElementData::Polyline { .. }
        )
        .then_some(id)
    }

    /// Break: ตัดส่วนระหว่างจุด p1 → p2 ออกจากวัตถุ (Circle/Polyline ปิด ตัดตามทิศ p1 ไป p2)
    pub fn break_apply(&mut self, id: u64, p1: Point, p2: Point) -> Result<(), &'static str> {
        let i = self.index_of(id).ok_or("Object no longer exists")?;
        let pieces = break_data(&self.history[i].data, p1, p2)?;
        let (layer, color, style) =
            (self.history[i].layer, self.history[i].color, self.history[i].style.clone());
        self.checkpoint();
        self.history.remove(i);
        for data in pieces {
            let nid = self.get_next_id();
            self.history.push(DrawingElement {
                id: nid,
                is_selected: false,
                layer,
                color,
                style: style.clone(),
                data,
            });
        }
        self.hovered_id = None;
        self.rebuild_index();
        Ok(())
    }

    /// Join: เชื่อม Line / Polyline เปิด ที่เลือกไว้และปลายชนกัน เป็นเส้นเดียว (วนกลับมาชนตัวเอง = ปิด)
    /// คืนจำนวนกลุ่มที่เชื่อมได้
    pub fn join_selected(&mut self) -> usize {
        let diag = self
            .get_elements_bounds()
            .map(|(a, b)| (b.x - a.x).hypot(b.y - a.y))
            .unwrap_or(0.0);
        let eps = 1e-6 * (1.0 + diag);
        let near = |a: Point, b: Point| (a.x - b.x).hypot(a.y - b.y) <= eps;

        // (จุด, id ของวัตถุต้นทาง, ตำแหน่งใน history ของชิ้นแรก)
        let mut chains: Vec<(Vec<Point>, Vec<u64>, usize)> = Vec::new();
        for (i, e) in self.history.iter().enumerate().filter(|(_, e)| e.is_selected) {
            match &e.data {
                ElementData::Line { start, end } => chains.push((vec![*start, *end], vec![e.id], i)),
                ElementData::Polyline { points, closed: false } if points.len() >= 2 => {
                    chains.push((points.clone(), vec![e.id], i))
                }
                _ => {}
            }
        }

        let mut merged = true;
        while merged {
            merged = false;
            'scan: for i in 0..chains.len() {
                for j in (i + 1)..chains.len() {
                    let (a_first, a_last) = (chains[i].0[0], chains[i].0[chains[i].0.len() - 1]);
                    let (b_first, b_last) = (chains[j].0[0], chains[j].0[chains[j].0.len() - 1]);
                    let case = if near(a_last, b_first) {
                        1
                    } else if near(a_last, b_last) {
                        2
                    } else if near(a_first, b_last) {
                        3
                    } else if near(a_first, b_first) {
                        4
                    } else {
                        0
                    };
                    if case == 0 {
                        continue;
                    }
                    let (mut bp, bids, bi) = chains.remove(j);
                    let a = &mut chains[i];
                    match case {
                        1 => a.0.extend(bp.into_iter().skip(1)),
                        2 => {
                            bp.reverse();
                            a.0.extend(bp.into_iter().skip(1));
                        }
                        3 => {
                            bp.extend(a.0.drain(..).skip(1));
                            a.0 = bp;
                        }
                        _ => {
                            bp.reverse();
                            bp.extend(a.0.drain(..).skip(1));
                            a.0 = bp;
                        }
                    }
                    a.1.extend(bids);
                    a.2 = a.2.min(bi);
                    merged = true;
                    break 'scan;
                }
            }
        }

        // (layer, color, style, data, ids ที่ถูกแทน)
        let mut results: Vec<(usize, Option<Rgb>, crate::linestyle::ElementStyle, ElementData, Vec<u64>)> =
            Vec::new();
        for (mut pts, ids, first) in chains.into_iter().filter(|c| c.1.len() >= 2) {
            let closed = pts.len() >= 4 && near(pts[0], pts[pts.len() - 1]);
            if closed {
                pts.pop();
            }
            // ทิ้งจุดกลางที่อยู่ในแนวเดียวกัน (เส้นตรงสองท่อนต่อกัน → เส้นตรงเส้นเดียว)
            let mut k = 1;
            while k + 1 < pts.len() {
                let (a, b, c) = (pts[k - 1], pts[k], pts[k + 1]);
                let (v1, v2) = ((b.x - a.x, b.y - a.y), (c.x - b.x, c.y - b.y));
                let (l1, l2) = (v1.0.hypot(v1.1), v2.0.hypot(v2.1));
                let cross = v1.0 * v2.1 - v1.1 * v2.0;
                let dot = v1.0 * v2.0 + v1.1 * v2.1;
                if l1 > 0.0 && l2 > 0.0 && cross.abs() <= 1e-9 * l1 * l2 && dot > 0.0 {
                    pts.remove(k);
                } else {
                    k += 1;
                }
            }
            let data = if !closed && pts.len() == 2 {
                ElementData::Line { start: pts[0], end: pts[1] }
            } else {
                ElementData::Polyline { points: pts, closed }
            };
            let e = &self.history[first];
            results.push((e.layer, e.color, e.style.clone(), data, ids));
        }
        if results.is_empty() {
            return 0;
        }

        self.checkpoint();
        let gone: std::collections::HashSet<u64> =
            results.iter().flat_map(|r| r.4.iter().copied()).collect();
        self.history.retain(|e| !gone.contains(&e.id));
        let n = results.len();
        for (layer, color, style, data, _) in results {
            let id = self.get_next_id();
            self.history.push(DrawingElement { id, is_selected: true, layer, color, style, data });
        }
        self.hovered_id = None;
        self.rebuild_index();
        n
    }

    /// Stretch: วัตถุที่เลือกไว้ จุดที่อยู่ในกรอบ crossing ล่าสุดขยับไป (dx, dy) จุดอื่นอยู่ที่เดิม
    /// คืนจำนวนวัตถุที่เปลี่ยน
    pub fn stretch_selected(&mut self, dx: f64, dy: f64) -> usize {
        let Some(r) = self.last_sel_box else { return 0 };
        if dx.abs() < 1e-12 && dy.abs() < 1e-12 {
            return 0;
        }
        let changes: Vec<(usize, ElementData)> = self
            .history
            .iter()
            .enumerate()
            .filter(|(_, e)| e.is_selected)
            .filter_map(|(i, e)| e.data.stretched(r, dx, dy).map(|d| (i, d)))
            .collect();
        if changes.is_empty() {
            return 0;
        }
        self.checkpoint();
        let n = changes.len();
        for (i, d) in changes {
            self.history[i].data = d;
        }
        self.last_sel_box = None;
        self.rebuild_index();
        n
    }

    /// ภาพเงาของ Stretch (ว่างถ้ามีเกิน limit ชิ้นเพื่อไม่ให้หน่วง)
    pub fn stretch_preview(&self, dx: f64, dy: f64, limit: usize) -> Vec<ElementData> {
        let Some(r) = self.last_sel_box else { return Vec::new() };
        let mut out = Vec::new();
        for e in self.history.iter().filter(|e| e.is_selected) {
            if let Some(d) = e.data.stretched(r, dx, dy) {
                out.push(d);
                if out.len() > limit {
                    return Vec::new();
                }
            }
        }
        out
    }

    pub fn delete_selected(&mut self) -> usize {
        let n = self.history.iter().filter(|e| e.is_selected).count();
        if n == 0 {
            return 0;
        }
        self.checkpoint();
        self.history.retain(|e| !e.is_selected);
        self.hovered_id = None;
        self.rebuild_index();
        n
    }

    pub fn select_in_box(
        &mut self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        crossing: bool,
        add_to_selection: bool,
    ) {
        self.sel_rev += 1;
        self.last_sel_box = crossing.then(|| (x1.min(x2), y1.min(y2), x1.max(x2), y1.max(y2)));
        let sel_box = BoundingBox::from_points(x1, y1, x2, y2);

        if !add_to_selection {
            for el in &mut self.history {
                el.is_selected = false;
            }
        }

        let mut hits = Vec::new();
        self.spatial_index.query(&sel_box, &mut hits);
        let hit_ids: Vec<u64> = hits.into_iter().copied().collect();

        for id in hit_ids {
            let Some(i) = self.index_of(id) else { continue };
            if !self.pickable(i) {
                continue;
            }
            let el = &mut self.history[i];
            let hit = if crossing {
                el.data.touches_rect(&sel_box)
            } else {
                let bb = el.bounding_box();
                bb.min_x >= sel_box.min_x
                    && bb.max_x <= sel_box.max_x
                    && bb.min_y >= sel_box.min_y
                    && bb.max_y <= sel_box.max_y
            };
            if hit {
                el.is_selected = true;
            }
        }
    }

    pub fn deselect_all(&mut self) {
        self.sel_rev += 1;
        self.last_sel_box = None;
        for el in &mut self.history {
            el.is_selected = false;
        }
    }

    const MAX_UNDO: usize = 100;
    /// ผลรวมจำนวนวัตถุในทุก snapshot ของ Undo
    const MAX_UNDO_ELEMS: usize = 1_000_000;

    fn index_of(&self, id: u64) -> Option<usize> {
        self.history.binary_search_by_key(&id, |e| e.id).ok()
    }

    fn rebuild_index(&mut self) {
        let mut idx = SpatialIndex::new(8);
        for el in &self.history {
            idx.insert(el.id, el.bounding_box());
        }
        self.spatial_index = idx;
        self.index_stale = false;
        self.moving.clear();
    }

    fn snapshot(&self) -> Snapshot {
        Snapshot {
            history: self.history.clone(),
            layers: self.layers.clone(),
            current_layer: self.current_layer,
        }
    }

    fn restore(&mut self, s: Snapshot) {
        self.history = s.history;
        self.layers = s.layers;
        self.current_layer = s.current_layer;
        self.sel_rev += 1;
        self.last_sel_box = None;
        self.hovered_id = None;
        self.rebuild_index();
    }

    pub fn checkpoint(&mut self) {
        self.finish_move();
        self.dirty_before_cp = self.dirty;
        // แบบกำลังจะเปลี่ยน กรอบ crossing เดิมใช้ไม่ได้แล้ว (Stretch ต้องเลือกใหม่)
        self.box_before_cp = self.last_sel_box.take();
        self.dirty = true;
        self.sel_rev += 1;
        let snap = self.snapshot();
        self.undo_stack.push(snap);
        if self.undo_stack.len() > Self::MAX_UNDO {
            self.undo_stack.remove(0);
        }
        let mut total: usize = self.undo_stack.iter().map(|s| s.history.len()).sum();
        while self.undo_stack.len() > 1 && total > Self::MAX_UNDO_ELEMS {
            total -= self.undo_stack[0].history.len();
            self.undo_stack.remove(0);
        }
        self.redo_before_cp = std::mem::take(&mut self.redo_stack);
    }

    pub fn undo_last_action(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            self.dirty = true;
            let cur = self.snapshot();
            self.redo_stack.push(cur);
            self.restore(prev);
        }
    }

    pub fn redo_last_action(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.dirty = true;
            let cur = self.snapshot();
            self.undo_stack.push(cur);
            self.restore(next);
        }
    }

    pub fn move_selected(&mut self, dx: f64, dy: f64) {
        self.sel_rev += 1;
        if !self.index_stale {
            self.moving = self.history.iter().filter(|e| e.is_selected).map(|e| e.id).collect();
            self.index_stale = true;
        }
        for el in self.history.iter_mut().filter(|e| e.is_selected) {
            el.data.translate(dx, dy);
        }
    }

    /// จบการลากย้าย: สร้าง spatial index ให้ตรงตำแหน่งใหม่
    pub fn finish_move(&mut self) {
        if self.index_stale {
            self.index_stale = false;
            self.moving.clear();
            self.rebuild_index();
        }
    }

    pub fn move_and_snap_selected(&mut self, dx: f64, dy: f64, snap_enabled: bool) {
        self.move_selected(dx, dy);
        if !snap_enabled {
            return;
        }

        let ref_point = self
            .history
            .iter()
            .find(|e| e.is_selected)
            .map(|e| {
                let p = e.data.ref_point();
                (p.x, p.y)
            });

        if let Some((rx, ry)) = ref_point {
            let (snx, sny) = self.get_snapped_pos(rx, ry, snap_enabled);
            let snap_dx = snx - rx;
            let snap_dy = sny - ry;
            if snap_dx.abs() > 0.001 || snap_dy.abs() > 0.001 {
                self.move_selected(snap_dx, snap_dy);
            }
        }
    }

    /// ข้อมูลของวัตถุตาม id (ข้ามตัวที่ไม่มีแล้ว)
    pub fn data_by_ids(&self, ids: &[u64]) -> Vec<&ElementData> {
        ids.iter()
            .filter_map(|&id| self.index_of(id))
            .map(|i| &self.history[i].data)
            .collect()
    }

    /// เหมือน mirror_selected แต่ระบุวัตถุด้วย id (animation เลือกไว้ตอนคลิก)
    pub fn mirror_ids(&mut self, ids: &[u64], a: Point, b: Point, keep: bool) -> usize {
        if (b.x - a.x).hypot(b.y - a.y) < 1e-9 {
            return 0;
        }
        let sel: Vec<usize> = ids.iter().filter_map(|&id| self.index_of(id)).collect();
        if sel.is_empty() {
            return 0;
        }
        self.checkpoint();
        if keep {
            let copies: Vec<DrawingElement> = sel
                .iter()
                .map(|&i| {
                    let e = &self.history[i];
                    DrawingElement {
                        id: 0,
                        is_selected: false,
                        layer: e.layer,
                        color: e.color,
                        style: e.style.clone(),
                        data: e.data.mirrored(a, b),
                    }
                })
                .collect();
            for mut c in copies {
                c.id = self.get_next_id();
                self.add_element(c);
            }
        } else {
            for &i in &sel {
                let m = self.history[i].data.mirrored(a, b);
                self.history[i].data = m;
            }
            self.rebuild_index();
        }
        sel.len()
    }

    /// แปลงวัตถุที่เลือกด้วย f; keep = เก็บต้นฉบับไว้ (สร้างสำเนา) คืนจำนวนชิ้นที่ทำ
    fn transform_selected_with(
        &mut self,
        keep: bool,
        f: &dyn Fn(&ElementData) -> ElementData,
    ) -> usize {
        let ids: Vec<u64> =
            self.history.iter().filter(|e| e.is_selected).map(|e| e.id).collect();
        self.transform_ids_with(&ids, keep, f).len()
    }

    /// เหมือน transform_selected_with แต่ระบุวัตถุด้วย id (แอนิเมชันเลือกไว้ตอนคลิก)
    fn transform_ids_with(
        &mut self,
        ids: &[u64],
        keep: bool,
        f: &dyn Fn(&ElementData) -> ElementData,
    ) -> Vec<u64> {
        let sel: Vec<usize> = ids.iter().filter_map(|&id| self.index_of(id)).collect();
        if sel.is_empty() {
            return Vec::new();
        }
        let mut out: Vec<u64> = Vec::new();
        self.checkpoint();
        if keep {
            let copies: Vec<DrawingElement> = sel
                .iter()
                .map(|&i| {
                    let e = &self.history[i];
                    DrawingElement {
                        id: 0,
                        is_selected: false,
                        layer: e.layer,
                        color: e.color,
                        style: e.style.clone(),
                        data: f(&e.data),
                    }
                })
                .collect();
            for mut c in copies {
                c.id = self.get_next_id();
                out.push(c.id);
                self.add_element(c);
            }
        } else {
            for &i in &sel {
                let m = f(&self.history[i].data);
                self.history[i].data = m;
                out.push(self.history[i].id);
            }
            self.rebuild_index();
        }
        out
    }

    /// หมุนวัตถุที่เลือกรอบจุด c ด้วยมุม ang (เรเดียน)
    pub fn rotate_selected(&mut self, c: Point, ang: f64, keep: bool) -> usize {
        if !ang.is_finite() {
            return 0;
        }
        self.transform_selected_with(keep, &|e| e.rotated(c, ang))
    }

    /// คืน id ของวัตถุผลลัพธ์ (ถ้า keep = id ของสำเนาใหม่)
    pub fn rotate_ids(&mut self, ids: &[u64], c: Point, ang: f64, keep: bool) -> Vec<u64> {
        if !ang.is_finite() {
            return Vec::new();
        }
        self.transform_ids_with(ids, keep, &|e| e.rotated(c, ang))
    }

    pub fn scale_ids(&mut self, ids: &[u64], c: Point, k: f64, keep: bool) -> Vec<u64> {
        if !(k.is_finite() && k > 0.0) {
            return Vec::new();
        }
        self.transform_ids_with(ids, keep, &|e| e.scaled(c, k))
    }

    /// ย่อ/ขยายวัตถุที่เลือกรอบจุด c ด้วยตัวคูณ k > 0
    pub fn scale_selected(&mut self, c: Point, k: f64, keep: bool) -> usize {
        if !(k.is_finite() && k > 0.0) {
            return 0;
        }
        self.transform_selected_with(keep, &|e| e.scaled(c, k))
    }

    fn preview_with(&self, limit: usize, f: &dyn Fn(&ElementData) -> ElementData) -> Vec<ElementData> {
        let sel: Vec<&DrawingElement> =
            self.history.iter().filter(|e| e.is_selected).take(limit + 1).collect();
        if sel.len() > limit {
            return Vec::new();
        }
        sel.iter().map(|e| f(&e.data)).collect()
    }

    pub fn rotate_preview(&self, c: Point, ang: f64, limit: usize) -> Vec<ElementData> {
        self.preview_with(limit, &|e| e.rotated(c, ang))
    }

    pub fn scale_preview(&self, c: Point, k: f64, limit: usize) -> Vec<ElementData> {
        self.preview_with(limit, &|e| e.scaled(c, k))
    }

    /// กรอบรวมของวัตถุที่เลือก (แคชตาม sel_rev ไม่คำนวณซ้ำทุกเฟรม)
    pub fn selection_bbox(&self) -> Option<(Point, Point)> {
        let c = self.sel_bbox_cache.get();
        if c.0 == self.sel_rev && c.1 == self.history.len() {
            return c.2.map(|(a, b, x, y)| (Point { x: a, y: b }, Point { x, y }));
        }
        let mut r: Option<(f64, f64, f64, f64)> = None;
        for e in self.history.iter().filter(|e| e.is_selected) {
            let b = e.bounding_box();
            if !(b.min_x.is_finite() && b.max_x.is_finite() && b.min_y.is_finite() && b.max_y.is_finite()) {
                continue;
            }
            r = Some(match r {
                None => (b.min_x, b.min_y, b.max_x, b.max_y),
                Some((a, bb, c2, d)) => (a.min(b.min_x), bb.min(b.min_y), c2.max(b.max_x), d.max(b.max_y)),
            });
        }
        self.sel_bbox_cache.set((self.sel_rev, self.history.len(), r));
        r.map(|(a, b, x, y)| (Point { x: a, y: b }, Point { x, y }))
    }

    /// ภาพเงาของวัตถุที่เลือกหลังสะท้อน (ว่างถ้าเลือกมากกว่า limit ชิ้น เพื่อไม่ให้หน่วง)
    pub fn mirror_preview(&self, a: Point, b: Point, limit: usize) -> Vec<ElementData> {
        let sel: Vec<&DrawingElement> =
            self.history.iter().filter(|e| e.is_selected).take(limit + 1).collect();
        if sel.len() > limit {
            return Vec::new();
        }
        sel.iter().map(|e| e.data.mirrored(a, b)).collect()
    }

    /// สะท้อนวัตถุที่เลือกข้ามเส้น a→b; keep = เก็บต้นฉบับไว้ (สร้างสำเนา) คืนจำนวนชิ้นที่ทำ
    pub fn mirror_selected(&mut self, a: Point, b: Point, keep: bool) -> usize {
        if (b.x - a.x).hypot(b.y - a.y) < 1e-9 {
            return 0;
        }
        let sel: Vec<usize> = (0..self.history.len()).filter(|&i| self.history[i].is_selected).collect();
        if sel.is_empty() {
            return 0;
        }
        self.checkpoint();
        if keep {
            let copies: Vec<DrawingElement> = sel
                .iter()
                .map(|&i| {
                    let e = &self.history[i];
                    DrawingElement {
                        id: 0,
                        is_selected: false,
                        layer: e.layer,
                        color: e.color,
                        style: e.style.clone(),
                        data: e.data.mirrored(a, b),
                    }
                })
                .collect();
            for mut c in copies {
                c.id = self.get_next_id();
                self.add_element(c);
            }
        } else {
            for &i in &sel {
                let m = self.history[i].data.mirrored(a, b);
                self.history[i].data = m;
            }
            self.rebuild_index();
        }
        sel.len()
    }

    pub fn commit_array(&mut self) {
        self.apply_array_grid(
            self.array_settings.rows,
            self.array_settings.cols,
            self.array_settings.spacing_x,
            self.array_settings.spacing_y,
        );
        self.show_array_ui = false; // ปิด UI เมื่อเสร็จสิ้น
    }

    pub fn cancel_array_preview(&mut self) {
        self.array_settings.target_anim_scale = 0.0;
    }

    pub fn get_array_preview_grid(
        &self,
        rows: i32,
        cols: i32,
        _dx: f64,
        _dy: f64,
    ) -> Vec<RenderCommand> {
        let mut previews = Vec::new();
        // ใช้ค่า anim จาก engine แทนเพื่อความนุ่มนวล
        let anim_dx = self.array_settings.anim_spacing_x;
        let anim_dy = self.array_settings.anim_spacing_y;

        let selected_elements: Vec<&DrawingElement> =
            self.history.iter().filter(|e| e.is_selected).collect();

        if selected_elements.is_empty() {
            return previews;
        }

        for r in 0..rows {
            for c in 0..cols {
                if r == 0 && c == 0 {
                    continue;
                }

                let offset_x = c as f64 * anim_dx;
                let offset_y = r as f64 * anim_dy;

                for el in &selected_elements {
                    match &el.data {
                        ElementData::Line { start, end } => {
                            let mid_x = (start.x + end.x) / 2.0;
                            let mid_y = (start.y + end.y) / 2.0;
                            let s = self.array_settings.anim_scale;
                            let new_start_x = mid_x + (start.x - mid_x) * s + offset_x;
                            let new_start_y = mid_y + (start.y - mid_y) * s + offset_y;
                            let new_end_x = mid_x + (end.x - mid_x) * s + offset_x;
                            let new_end_y = mid_y + (end.y - mid_y) * s + offset_y;
                            let s_screen = self.camera.world_to_screen(new_start_x, new_start_y);
                            let e_screen = self.camera.world_to_screen(new_end_x, new_end_y);
                            previews.push(RenderCommand::Line {
                                x1: s_screen.0,
                                y1: s_screen.1,
                                x2: e_screen.0,
                                y2: e_screen.1,
                                is_selected: true,
                                color: (1.0, 1.0, 1.0),
                            });
                        }
                        ElementData::Circle { center, radius } => {
                            let c = self
                                .camera
                                .world_to_screen(center.x + offset_x, center.y + offset_y);
                            let r_screen =
                                radius * self.camera.scale * self.array_settings.anim_scale;
                            previews.push(RenderCommand::Circle {
                                cx: c.0,
                                cy: c.1,
                                radius: r_screen,
                                is_selected: true,
                                color: (1.0, 1.0, 1.0),
                            });
                        }
                        ElementData::Arc { center, radius, start_angle, end_angle } => {
                            let c = self
                                .camera
                                .world_to_screen(center.x + offset_x, center.y + offset_y);
                            previews.push(RenderCommand::Arc {
                                cx: c.0,
                                cy: c.1,
                                radius: radius * self.camera.scale * self.array_settings.anim_scale,
                                start_angle: *start_angle,
                                end_angle: *end_angle,
                                is_selected: true,
                                color: (1.0, 1.0, 1.0),
                            });
                        }
                        ElementData::Polyline { points, closed } => {
                            let pts = points
                                .iter()
                                .map(|p| {
                                    self.camera.world_to_screen(p.x + offset_x, p.y + offset_y)
                                })
                                .collect();
                            previews.push(RenderCommand::Polyline {
                                points: pts,
                                closed: *closed,
                                is_selected: true,
                                color: (1.0, 1.0, 1.0),
                            });
                        }
                        ElementData::Fill { loops, filled } => {
                            let loops = loops
                                .iter()
                                .map(|l| {
                                    l.iter()
                                        .map(|p| {
                                            self.camera
                                                .world_to_screen(p.x + offset_x, p.y + offset_y)
                                        })
                                        .collect()
                                })
                                .collect();
                            previews.push(RenderCommand::Fill {
                                loops,
                                filled: *filled,
                                is_selected: true,
                                color: (1.0, 1.0, 1.0),
                            });
                        }
                        ElementData::Dimension { .. } | ElementData::Hatch { .. } => {
                            // Skip dimension in array preview for now
                        }
                        ElementData::Text { pos, height, rotation, text, halign, valign, width } => {
                            let p = self.camera.world_to_screen(pos.x + offset_x, pos.y + offset_y);
                            previews.push(RenderCommand::Text {
                                x: p.0,
                                y: p.1,
                                em_px: height * text::EM_PER_HEIGHT * self.camera.scale,
                                rotation: *rotation,
                                wrap_px: width * self.camera.scale,
                                text: text.clone(),
                                halign: *halign,
                                valign: *valign,
                                is_selected: true,
                                color: (1.0, 1.0, 1.0),
                            });
                        }
                    }
                }
            }
        }
        previews
    }

    pub fn apply_array_grid(&mut self, rows: i32, cols: i32, dx: f64, dy: f64) {
        self.checkpoint();
        let mut new_elements = Vec::new();
        let selected_ids: Vec<u64> = self
            .history
            .iter()
            .filter(|e| e.is_selected)
            .map(|e| e.id)
            .collect();

        for r in 0..rows {
            for c in 0..cols {
                if r == 0 && c == 0 {
                    continue;
                }
                let offset_x = c as f64 * dx;
                let offset_y = r as f64 * dy;
                for id in &selected_ids {
                    if let Some(el) = self.history.iter().find(|e| e.id == *id).cloned() {
                        let new_id = self.get_next_id();
                        let new_el = el.cloned_with_offset(new_id, offset_x, offset_y);
                        new_elements.push(new_el);
                    }
                }
            }
        }

        let reveal_order: Vec<u64> = new_elements.iter().map(|e| e.id).collect();
        let total = reveal_order.len() as i32;

        for el in new_elements {
            self.add_element(el);
        }
        let total_duration = 0.8_f64;
        let stagger_delay = (total_duration / total as f64).min(0.05);
        self.apply_anim = ApplyAnimation {
            is_playing: true,
            timer: 0.0,
            total_duration: total_duration + 0.2,
            stagger_delay,
            elements_to_reveal: reveal_order,
        };
    }

    /// Copy: ทำสำเนาวัตถุที่เลือก (ไม่แตะแบบ)
    pub fn copy_selected(&self) -> Vec<ClipItem> {
        self.history
            .iter()
            .filter(|e| e.is_selected)
            .map(|e| ClipItem {
                layer: self.layers[e.layer].clone(),
                color: e.color,
                style: e.style.clone(),
                data: e.data.clone(),
            })
            .collect()
    }

    /// Paste: วางสำเนาเลื่อน (dx, dy) แล้วเลือกชิ้นที่วาง; layer ชื่อเดียวกันใช้ของเดิม ไม่มีก็สร้าง
    pub fn paste_items(&mut self, items: &[ClipItem], dx: f64, dy: f64) -> usize {
        if items.is_empty() {
            return 0;
        }
        self.checkpoint();
        for el in &mut self.history {
            el.is_selected = false;
        }
        for it in items {
            let li = match self.layers.iter().position(|l| l.name.eq_ignore_ascii_case(&it.layer.name)) {
                Some(i) => i,
                None => {
                    self.layers.push(it.layer.clone());
                    self.layers.len() - 1
                }
            };
            let mut data = it.data.clone();
            data.translate(dx, dy);
            let id = self.get_next_id();
            self.add_element(DrawingElement {
                id,
                is_selected: true,
                layer: li,
                color: it.color,
                style: it.style.clone(),
                data,
            });
        }
        self.sel_rev += 1;
        items.len()
    }

    pub fn get_next_id(&mut self) -> u64 {
        self.id_counter += 1;
        self.id_counter
    }

    pub fn add_element(&mut self, element: DrawingElement) {
        let bounds = element.bounding_box();
        self.spatial_index.insert(element.id, bounds);
        self.history.push(element);
    }

    pub fn push_new(&mut self, data: ElementData) -> u64 {
        let id = self.get_next_id();
        let layer = self.current_layer;
        self.add_element(DrawingElement {
            id,
            is_selected: false,
            layer,
            color: None,
            style: Default::default(),
            data,
        });
        id
    }

    // ---------------- Hatch / Text ----------------

    /// หาพื้นที่ปิดที่ครอบจุด (wx, wy) จากเส้น วงกลม ส่วนโค้ง polyline ของวัตถุที่มองเห็น
    pub fn hatch_region(&self, wx: f64, wy: f64) -> Result<hatch::Region, &'static str> {
        let Some(root) = self.spatial_index.get_root_bounds() else {
            return Err("Nothing to hatch");
        };
        let diag = (root.max_x - root.min_x).hypot(root.max_y - root.min_y);
        let eps = (diag * 1e-9).max(1e-7);

        // เส้นของแต่ละวัตถุ (แคช) + index ย่อยสำหรับวัตถุที่มีหลายเส้น (polyline ยาว ๆ)
        let mut cache: std::collections::HashMap<
            u64,
            (Vec<hatch::Seg>, Option<SpatialIndex<usize>>),
        > = std::collections::HashMap::new();
        let mut fetch = |bb: &BoundingBox| -> Vec<hatch::Seg> {
            let mut hits = Vec::new();
            self.spatial_index.query(bb, &mut hits);
            let ids: Vec<u64> = hits.into_iter().copied().collect();
            let mut out = Vec::new();
            for id in ids {
                let ent = cache.entry(id).or_insert_with(|| {
                    let mut v = Vec::new();
                    if let Some(i) = self.index_of(id) {
                        if self.layer_visible(i) {
                            hatch::element_segs(id, &self.history[i].data, &mut v);
                        }
                    }
                    let tree = if v.len() > 64 {
                        let mut t = SpatialIndex::new(8);
                        for (k, s) in v.iter().enumerate() {
                            t.insert(k, s.bbox());
                        }
                        Some(t)
                    } else {
                        None
                    };
                    (v, tree)
                });
                match &ent.1 {
                    Some(t) => {
                        let mut near = Vec::new();
                        t.query(bb, &mut near);
                        for k in near {
                            let s = ent.0[*k];
                            if s.bbox().intersects(bb) {
                                out.push(s);
                            }
                        }
                    }
                    None => out.extend(ent.0.iter().filter(|s| s.bbox().intersects(bb)).copied()),
                }
            }
            out
        };
        let mut region = hatch::find_region(&mut fetch, Point { x: wx, y: wy }, &root, eps)?;

        // เกาะที่เป็นวงกลม / polyline ปิด อยู่ข้างในทั้งหมดแต่รังสีไม่โดน
        let mut ob = BoundingBox {
            min_x: f64::MAX,
            min_y: f64::MAX,
            max_x: f64::MIN,
            max_y: f64::MIN,
        };
        for p in &region.outer {
            ob.min_x = ob.min_x.min(p.x);
            ob.min_y = ob.min_y.min(p.y);
            ob.max_x = ob.max_x.max(p.x);
            ob.max_y = ob.max_y.max(p.y);
        }
        let mut hits = Vec::new();
        self.spatial_index.query(&ob, &mut hits);
        let mut ids: Vec<u64> = hits.into_iter().copied().collect();
        ids.sort_unstable();
        let mut added = 0;
        for id in ids {
            if added >= 500 {
                break;
            }
            if region.used.contains(&id) {
                continue;
            }
            let Some(i) = self.index_of(id) else { continue };
            if !self.layer_visible(i) {
                continue;
            }
            let pts: Vec<Point> = match &self.history[i].data {
                ElementData::Circle { center, radius } if *radius > eps => {
                    hatch::circle_points(*center, *radius)
                }
                ElementData::Polyline { points, closed: true } if points.len() >= 3 => points.clone(),
                _ => continue,
            };
            if pts.iter().all(|p| hatch::point_in_poly(*p, &region.outer)) {
                region.holes.push(pts);
                added += 1;
            }
        }
        Ok(region)
    }

    /// สร้าง hatch (solid = Fill ทึบ) ในพื้นที่ปิดที่ครอบจุดนี้
    pub fn hatch_at(
        &mut self,
        wx: f64,
        wy: f64,
        spec: HatchSpec,
        solid: bool,
    ) -> Result<u64, &'static str> {
        if !solid && !(spec.spacing > 0.0 && spec.spacing.is_finite() && spec.angle.is_finite()) {
            return Err("Hatch spacing must be greater than 0");
        }
        self.finish_move();
        let region = self.hatch_region(wx, wy)?;
        let mut loops = vec![region.outer];
        loops.extend(region.holes);
        self.checkpoint();
        let data = if solid {
            ElementData::Fill { loops, filled: true }
        } else {
            ElementData::Hatch { loops, spec }
        };
        Ok(self.push_new(data))
    }

    /// แก้รูปแบบ hatch (คืน false ถ้าไม่ใช่ hatch)
    pub fn set_hatch_spec(&mut self, id: u64, new: HatchSpec) -> bool {
        let Some(i) = self.index_of(id) else { return false };
        match &self.history[i].data {
            ElementData::Hatch { spec, .. } if *spec == new => return true,
            ElementData::Hatch { .. } => {}
            _ => return false,
        }
        if !(new.spacing > 0.0 && new.spacing.is_finite() && new.angle.is_finite()) {
            return false;
        }
        self.checkpoint();
        if let ElementData::Hatch { spec, .. } = &mut self.history[i].data {
            *spec = new;
        }
        true
    }

    /// เพิ่มข้อความ (หลายบรรทัด → ยึดมุมซ้ายบน, บรรทัดเดียว → ยึดเส้นฐานซ้าย)
    pub fn add_text(&mut self, pos: Point, text: String, height: f64, rotation: f64) -> Option<u64> {
        if text.trim().is_empty() || !(height > 0.0 && height.is_finite()) || !rotation.is_finite() {
            return None;
        }
        let valign = if text.contains('\n') { VAlign::Top } else { VAlign::Baseline };
        self.checkpoint();
        Some(self.push_new(ElementData::Text {
            pos,
            height,
            rotation,
            text,
            halign: HAlign::Left,
            valign,
            width: 0.0,
        }))
    }

    /// แก้ข้อความ ความสูง และมุมของ Text (คืน false ถ้าไม่ใช่ Text หรือค่าไม่ถูกต้อง)
    pub fn set_text(&mut self, id: u64, new_text: String, new_height: f64, new_rot: f64) -> bool {
        let Some(i) = self.index_of(id) else { return false };
        if !matches!(self.history[i].data, ElementData::Text { .. })
            || new_text.trim().is_empty()
            || !(new_height > 0.0 && new_height.is_finite())
            || !new_rot.is_finite()
        {
            return false;
        }
        self.checkpoint();
        if let ElementData::Text { text, height, rotation, .. } = &mut self.history[i].data {
            *text = new_text;
            *height = new_height;
            *rotation = new_rot;
        }
        self.rebuild_index();
        true
    }

    /// วัตถุที่แก้ไขได้ (Text / Hatch) ใต้จุดนี้ สำหรับดับเบิลคลิก
    pub fn edit_target(&mut self, wx: f64, wy: f64) -> Option<(u64, ElementData)> {
        self.hover_at(wx, wy);
        let id = self.hovered_id?;
        let i = self.index_of(id)?;
        match &self.history[i].data {
            d @ (ElementData::Text { .. } | ElementData::Hatch { .. }) => Some((id, d.clone())),
            _ => None,
        }
    }

    /// ทิ้ง checkpoint ล่าสุด (ใช้เมื่อขึ้นต้นการย้ายไปแล้วแต่ไม่ได้ขยับ)
    pub fn discard_last_checkpoint(&mut self) {
        if self.undo_stack.pop().is_some() {
            self.dirty = self.dirty_before_cp;
            self.redo_stack = std::mem::take(&mut self.redo_before_cp);
            self.last_sel_box = self.box_before_cp.take();
        }
    }

    pub fn push_imported(
        &mut self,
        data: ElementData,
        layer: usize,
        color: Option<Rgb>,
        style: crate::linestyle::ElementStyle,
    ) {
        let id = self.get_next_id();
        self.add_element(DrawingElement { id, is_selected: false, layer, color, style, data });
    }

    fn layer_visible(&self, i: usize) -> bool {
        self.layers[self.history[i].layer].visible
    }

    fn pickable(&self, i: usize) -> bool {
        let l = &self.layers[self.history[i].layer];
        l.visible && !l.locked
    }

    fn pen_color(&self, el: &DrawingElement) -> (f64, f64, f64) {
        let c = el.color.unwrap_or(self.layers[el.layer].color);
        (c.0 as f64 / 255.0, c.1 as f64 / 255.0, c.2 as f64 / 255.0)
    }

    /// (ความหนา px, dash px) ที่ใช้วาดวัตถุนี้ ณ ระดับซูมปัจจุบัน
    fn pen_style(&self, el: &DrawingElement) -> (f64, Vec<f64>) {
        let l = &self.layers[el.layer];
        crate::linestyle::resolve(l.weight, &l.linetype, &el.style, self.camera.scale)
    }

    pub fn set_current_layer(&mut self, i: usize) {
        if i < self.layers.len() {
            self.current_layer = i;
        }
    }

    fn drop_selection_on_layer(&mut self, layer: usize) {
        self.sel_rev += 1;
        for e in self.history.iter_mut().filter(|e| e.layer == layer) {
            e.is_selected = false;
        }
        self.hovered_id = None;
    }

    pub fn set_layer_visible(&mut self, i: usize, v: bool) {
        let Some(l) = self.layers.get_mut(i) else { return };
        l.visible = v;
        self.dirty = true;
        if !v {
            self.drop_selection_on_layer(i);
        }
    }

    pub fn set_layer_locked(&mut self, i: usize, v: bool) {
        let Some(l) = self.layers.get_mut(i) else { return };
        l.locked = v;
        self.dirty = true;
        if v {
            self.drop_selection_on_layer(i);
        }
    }

    fn find_layer(&self, name: &str) -> Option<usize> {
        self.layers.iter().position(|l| l.name.eq_ignore_ascii_case(name))
    }

    pub fn add_layer(&mut self) -> usize {
        let mut n = self.layers.len();
        let name = loop {
            let c = format!("Layer{}", n);
            if self.find_layer(&c).is_none() { break c; }
            n += 1;
        };
        self.checkpoint();                       // <- เพิ่ม
        self.layers.push(Layer::new(&name, (255, 255, 255)));
        self.layers.len() - 1
    }

    pub fn delete_layer(&mut self, i: usize) -> Result<usize, &'static str> {
        if i == 0 {
            return Err("Layer 0 can't be deleted");
        }
        if i >= self.layers.len() {
            return Err("Layer no longer exists");
        }
        self.checkpoint();
        let mut moved = 0;
        for e in self.history.iter_mut() {
            if e.layer == i {
                e.layer = 0;
                moved += 1;
            } else if e.layer > i {
                e.layer -= 1;
            }
        }
        self.layers.remove(i);
        self.current_layer = if self.current_layer == i {
            0
        } else if self.current_layer > i {
            self.current_layer - 1
        } else {
            self.current_layer
        };
        self.hovered_id = None;
        Ok(moved)
    }

    pub fn rename_layer(&mut self, i: usize, new: &str) -> Result<(), &'static str> {
        if i == 0 {
            return Err("Layer 0 can't be renamed");
        }
        if i >= self.layers.len() {
            return Err("Layer no longer exists");
        }
        let new = new.trim();
        if new.is_empty() {
            return Err("Name can't be empty");
        }
        if new.chars().any(|c| "<>/\\\":;?*|=`".contains(c)) {
            return Err("Name contains characters DXF doesn't allow");
        }
        match self.find_layer(new) {
            Some(j) if j == i => return Ok(()),
            Some(_) => return Err("A layer with this name already exists"),
            None => {}
        }
        self.checkpoint();
        self.layers[i].name = new.to_string();
        self.dirty = true;
        Ok(())
    }

    pub fn set_layer_color(&mut self, i: usize, c: Rgb) {
        if self.layers.get(i).map_or(true, |l| l.color == c) {
            return;
        }
        self.checkpoint();
        self.layers[i].color = c;
    }

    pub fn set_layer_weight(&mut self, i: usize, mm: f64) {
        if self.layers.get(i).map_or(true, |l| (l.weight - mm).abs() < 1e-9) {
            return;
        }
        self.checkpoint();
        self.layers[i].weight = mm;
    }

    pub fn set_layer_linetype(&mut self, i: usize, lt: Option<crate::linestyle::Lt>) {
        if self.layers.get(i).map_or(true, |l| l.linetype == lt) {
            return;
        }
        self.checkpoint();
        self.layers[i].linetype = lt;
    }

    /// ชนิดเส้นที่เลือกได้ (ไม่รวมเส้นทึบ): ของสำเร็จรูป + ที่มากับไฟล์
    pub fn linetype_choices(&self) -> Vec<crate::linestyle::Lt> {
        let mut out = crate::linestyle::presets();
        let mut seen: std::collections::HashSet<String> =
            out.iter().map(|l| l.name.to_lowercase()).collect();
        let from_layers = self.layers.iter().filter_map(|l| l.linetype.as_ref());
        let from_elems = self.history.iter().filter_map(|e| e.style.linetype.as_ref());
        for lt in from_layers.chain(from_elems) {
            if !lt.pattern.is_empty() && seen.insert(lt.name.to_lowercase()) {
                out.push(lt.clone());
            }
        }
        out
    }

    pub fn selection_style(&self) -> crate::linestyle::SelStyle {
        let mut s = crate::linestyle::SelStyle::default();
        for e in self.history.iter().filter(|e| e.is_selected) {
            if s.count == 0 {
                s.weight = e.style.weight;
                s.linetype = e.style.linetype.clone();
            } else {
                if s.weight != e.style.weight {
                    s.weight_mixed = true;
                }
                if s.linetype != e.style.linetype {
                    s.lt_mixed = true;
                }
            }
            s.count += 1;
        }
        s
    }

    /// None = ตาม layer
    pub fn set_selected_weight(&mut self, w: Option<f64>) {
        if !self.history.iter().any(|e| e.is_selected && e.style.weight != w) {
            return;
        }
        self.checkpoint();
        for e in self.history.iter_mut().filter(|e| e.is_selected) {
            e.style.weight = w;
        }
    }

    /// None = ตาม layer, Some(continuous()) = บังคับเส้นทึบ
    pub fn set_selected_linetype(&mut self, lt: Option<crate::linestyle::Lt>) {
        if !self.history.iter().any(|e| e.is_selected && e.style.linetype != lt) {
            return;
        }
        self.checkpoint();
        for e in self.history.iter_mut().filter(|e| e.is_selected) {
            e.style.linetype = lt.clone();
        }
    }

    pub fn move_selected_to_layer(&mut self) -> usize {
        let target = self.current_layer;
        let n = self
            .history
            .iter()
            .filter(|e| e.is_selected && e.layer != target)
            .count();
        if n == 0 {
            return 0;
        }
        self.checkpoint();
        for e in self.history.iter_mut().filter(|e| e.is_selected) {
            e.layer = target;
        }
        let l = &self.layers[target];
        if !l.visible || l.locked {
            self.drop_selection_on_layer(target);
        }
        n
    }

    pub fn get_visible_elements(&self, viewport_bounds: BoundingBox) -> Vec<&DrawingElement> {
        let mut hits = Vec::new();
        // เล็กกว่า 0.5 พิกเซลทั้งสองแกน = มองไม่เห็น ไม่ต้องสร้างคำสั่งวาด
        let min_extent = 0.5 / self.camera.scale.max(1e-9);
        self.spatial_index.query_min(&viewport_bounds, min_extent, &mut hits);
        let mut ids: Vec<u64> = hits.into_iter().copied().collect();
        // กำลังลากย้าย: index ยังเป็นตำแหน่งเดิม จึงเติมชิ้นที่ย้ายและอยู่ในจอตอนนี้เข้าไปเอง
        for &id in &self.moving {
            if let Some(i) = self.index_of(id) {
                let b = self.history[i].bounding_box();
                if b.max_x >= viewport_bounds.min_x
                    && b.min_x <= viewport_bounds.max_x
                    && b.max_y >= viewport_bounds.min_y
                    && b.min_y <= viewport_bounds.max_y
                {
                    ids.push(id);
                }
            }
        }
        ids.sort_unstable();
        ids.dedup();
        ids.into_iter()
            .filter_map(|id| self.index_of(id).map(|i| &self.history[i]))
            .filter(|e| self.layers[e.layer].visible)
            .filter(|e| self.anim_hidden.is_empty() || !self.anim_hidden.contains(&e.id))
            .collect()
    }

    /// เลขที่เปลี่ยนเมื่อสิ่งที่มีผลต่อคำสั่งวาดเปลี่ยน (ไม่รวมกล้อง) ใช้ข้ามการสร้างคำสั่งวาดซ้ำ
    pub fn scene_rev(&self) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        self.sel_rev.hash(&mut h);
        self.history.len().hash(&mut h);
        self.id_counter.hash(&mut h);
        self.anim_hidden.hash(&mut h);
        if self.apply_anim.is_playing {
            self.apply_anim.timer.to_bits().hash(&mut h);
        }
        for l in &self.layers {
            l.visible.hash(&mut h);
            l.color.hash(&mut h);
            l.weight.to_bits().hash(&mut h);
            if let Some(t) = &l.linetype {
                t.name.hash(&mut h);
                for p in &t.pattern {
                    p.to_bits().hash(&mut h);
                }
            }
        }
        h.finish()
    }

    pub fn get_all_elements(&self) -> Vec<&DrawingElement> {
        self.history.iter().collect()
    }

    /// ซูมเข้าได้ลึกสุดเท่าไร: แบบใหญ่ → เพดานต่ำลง (แต่ไม่ต่ำกว่า 50 เท่าของเดิม)
    fn zoom_limit(&self) -> f64 {
        let diag = self
            .fit_bounds()
            .map(|(a, b)| (b.x - a.x).hypot(b.y - a.y))
            .unwrap_or(0.0);
        if !(diag > 0.0) {
            return MAX_SCALE;
        }
        (SAFE_SCREEN_PX / diag).clamp(50.0, MAX_SCALE)
    }

    pub fn step(&mut self, current_time: u64) -> bool {
        let dt = if let Some(last) = self.last_update_time {
            (current_time - last) as f64 / 1_000_000.0
        } else {
            self.last_update_time = Some(current_time);
            return false;
        };

        self.last_update_time = Some(current_time);
        let dt = dt.min(0.1);
        let key = (self.history.len(), self.id_counter);
        if key != self.zoom_cap_key {
            self.zoom_cap_key = key;
            self.camera.max_scale = self.zoom_limit();
        }

        let camera_changed = self.camera.update(dt);
        let mut array_changed = false;
        let lerp_factor = (dt * 15.0).min(1.0);

        if self.apply_anim.is_playing {
            self.apply_anim.timer += dt;
            if self.apply_anim.timer >= self.apply_anim.total_duration {
                self.apply_anim.is_playing = false;
            }
            array_changed = true;
        }

        let dx = self.array_settings.spacing_x - self.array_settings.anim_spacing_x;
        if dx.abs() > 0.01 {
            self.array_settings.anim_spacing_x += dx * lerp_factor;
            array_changed = true;
        }

        let dy = self.array_settings.spacing_y - self.array_settings.anim_spacing_y;
        if dy.abs() > 0.01 {
            self.array_settings.anim_spacing_y += dy * lerp_factor;
            array_changed = true;
        }

        let ds = self.array_settings.target_anim_scale - self.array_settings.anim_scale;
        if ds.abs() > 0.001 {
            self.array_settings.anim_scale += ds * lerp_factor;
            array_changed = true;
        } else {
            self.array_settings.anim_scale = self.array_settings.target_anim_scale;
        }

        camera_changed || array_changed
    }

    pub fn get_elements_bounds(&self) -> Option<(Point, Point)> {
        self.spatial_index.get_root_bounds().map(|b| {
            (
                Point {
                    x: b.min_x,
                    y: b.min_y,
                },
                Point {
                    x: b.max_x,
                    y: b.max_y,
                },
            )
        })
    }

    /// ขอบเขตของงานหลัก: ตัดวัตถุที่หลุดไกลสุดขอบ (0.2% ต่อด้าน) ออก
    pub fn fit_bounds(&self) -> Option<(Point, Point)> {
        let n = self.history.len();
        if n < 100 {
            return self.get_elements_bounds();
        }
        let bbs: Vec<_> = self.history.iter().map(|e| e.bounding_box()).collect();
        let mut xs: Vec<f64> = bbs.iter().map(|b| (b.min_x + b.max_x) * 0.5).collect();
        let mut ys: Vec<f64> = bbs.iter().map(|b| (b.min_y + b.max_y) * 0.5).collect();
        let cmp = |a: &f64, b: &f64| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal);
        xs.sort_by(cmp);
        ys.sort_by(cmp);
        let lo = (n as f64 * 0.002) as usize;
        let hi = n - 1 - lo;
        let (x0, x1, y0, y1) = (xs[lo], xs[hi], ys[lo], ys[hi]);
        let mut r: Option<(f64, f64, f64, f64)> = None;
        for b in &bbs {
            let (cx, cy) = ((b.min_x + b.max_x) * 0.5, (b.min_y + b.max_y) * 0.5);
            if cx < x0 || cx > x1 || cy < y0 || cy > y1 {
                continue;
            }
            r = Some(match r {
                None => (b.min_x, b.min_y, b.max_x, b.max_y),
                Some((a, bb, c, d)) => (a.min(b.min_x), bb.min(b.min_y), c.max(b.max_x), d.max(b.max_y)),
            });
        }
        match r {
            Some((a, b, c, d)) if (c - a).is_finite() && (d - b).is_finite() => {
                let trimmed = (Point { x: a, y: b }, Point { x: c, y: d });
                match self.get_elements_bounds() {
                    // ตัดทิ้งก็ต่อเมื่อมีของหลุดไกลจริง (ขอบเขตเต็มกว้างกว่างานหลักเกิน 3 เท่า)
                    Some(full)
                        if (full.1.x - full.0.x).hypot(full.1.y - full.0.y)
                            <= 3.0 * (c - a).hypot(d - b) =>
                    {
                        Some(full)
                    }
                    _ => Some(trimmed),
                }
            }
            _ => self.get_elements_bounds(),
        }
    }

    pub fn zoom_to_fit(&mut self, screen_w: f64, screen_h: f64) {
        if let Some((min_p, max_p)) = self.fit_bounds() {
            self.camera
                .focus_on_area(min_p, max_p, (screen_w, screen_h));
        }
    }

    /// Zoom Extents จริง: ทุกชิ้น รวมที่หลุดไกล
    pub fn zoom_to_extents(&mut self, screen_w: f64, screen_h: f64) {
        if let Some((min_p, max_p)) = self.get_elements_bounds() {
            self.camera
                .focus_on_area(min_p, max_p, (screen_w, screen_h));
        }
    }

    pub fn get_snapped_pos(&self, wx: f64, wy: f64, is_enabled: bool) -> (f64, f64) {
        if !is_enabled {
            return (wx, wy);
        }

        let scale = self.camera.scale;
        let grid_base = 100.0;
        let lod = scale.log10().floor();
        let step_major = grid_base / 10.0f64.powf(lod);
        let step_minor = step_major / 10.0;

        // snap ที่ช่องย่อยทุกช่อง (ช่องย่อยเป็นพหุคูณของกัน เส้นหลักจึงถูก snap ได้ด้วย)
        // ซูมออกจนช่องย่อยถี่เกิน 8 px ค่อยถอยไป snap ที่ช่องหลัก
        let step = if step_minor * scale >= 8.0 { step_minor } else { step_major };
        ((wx / step).round() * step, (wy / step).round() * step)
    }

    pub fn find_object_snap(
        &self,
        wx: f64,
        wy: f64,
        settings: &SnapSettings,
    ) -> Option<(Point, SnapKind)> {
        self.find_object_snap_ex(wx, wy, settings, false)
    }

    /// exclude_selected = ไม่จับวัตถุที่เลือกอยู่ (ใช้ตอน Move เพราะมันกำลังเคลื่อนตามเมาส์)
    pub fn find_object_snap_ex(
        &self,
        wx: f64,
        wy: f64,
        settings: &SnapSettings,
        exclude_selected: bool,
    ) -> Option<(Point, SnapKind)> {
        let tol = 12.0 / self.camera.scale;
        let query_box = BoundingBox {
            min_x: wx - tol,
            min_y: wy - tol,
            max_x: wx + tol,
            max_y: wy + tol,
        };
        let mut hits = Vec::new();
        self.spatial_index.query(&query_box, &mut hits);

        let elems: Vec<&ElementData> = hits
            .into_iter()
            .copied()
            .filter_map(|id| self.index_of(id))
            .filter(|&i| self.layer_visible(i))
            .filter(|&i| !(exclude_selected && self.history[i].is_selected))
            .map(|i| &self.history[i].data)
            .collect();

        let mut best: Option<(f64, Point, SnapKind)> = None;
        let mut consider = |p: Point, kind: SnapKind| {
            if !settings.allows(kind) {
                return;
            }
            let d = ((p.x - wx).powi(2) + (p.y - wy).powi(2)).sqrt();
            if d > tol {
                return;
            }
            // จุดสำคัญ (endpoint/center/intersection) ได้แต้มต่อเล็กน้อย
            let bonus = match kind {
                SnapKind::Endpoint | SnapKind::Center | SnapKind::Intersection => tol * 0.25,
                _ => 0.0,
            };
            let score = d - bonus;
            if best.map_or(true, |(s, _, _)| score < s) {
                best = Some((score, p, kind));
            }
        };

        for data in &elems {
            match data {
                ElementData::Line { start, end } => {
                    consider(*start, SnapKind::Endpoint);
                    consider(*end, SnapKind::Endpoint);
                    consider(
                        Point { x: (start.x + end.x) / 2.0, y: (start.y + end.y) / 2.0 },
                        SnapKind::Midpoint,
                    );
                }
                ElementData::Circle { center, radius } => {
                    consider(*center, SnapKind::Center);
                    consider(Point { x: center.x + radius, y: center.y }, SnapKind::Quadrant);
                    consider(Point { x: center.x - radius, y: center.y }, SnapKind::Quadrant);
                    consider(Point { x: center.x, y: center.y + radius }, SnapKind::Quadrant);
                    consider(Point { x: center.x, y: center.y - radius }, SnapKind::Quadrant);
                }
                ElementData::Dimension { start, end, .. } => {
                    consider(*start, SnapKind::Endpoint);
                    consider(*end, SnapKind::Endpoint);
                }
                ElementData::Arc { center, radius, start_angle, end_angle } => {
                    consider(*center, SnapKind::Center);
                    consider(arc_point(*center, *radius, *start_angle), SnapKind::Endpoint);
                    consider(arc_point(*center, *radius, *end_angle), SnapKind::Endpoint);
                    consider(
                        arc_point(*center, *radius, (*start_angle + *end_angle) / 2.0),
                        SnapKind::Midpoint,
                    );
                    for k in 0..4 {
                        let a = k as f64 * FRAC_PI_2;
                        if angle_in_range(a, *start_angle, *end_angle) {
                            consider(arc_point(*center, *radius, a), SnapKind::Quadrant);
                        }
                    }
                }
                ElementData::Text { pos, .. } => consider(*pos, SnapKind::Endpoint),
                ElementData::Hatch { .. } => {}
                ElementData::Fill { loops, .. } => {
                    for p in loops.iter().flatten().take(256) {
                        consider(*p, SnapKind::Endpoint);
                    }
                }
                ElementData::Polyline { points, closed } => {
                    for p in points {
                        consider(*p, SnapKind::Endpoint);
                    }
                    let n = points.len();
                    let segs = if *closed { n } else { n.saturating_sub(1) };
                    for i in 0..segs {
                        let (a, b) = (points[i], points[(i + 1) % n]);
                        consider(
                            Point { x: (a.x + b.x) / 2.0, y: (a.y + b.y) / 2.0 },
                            SnapKind::Midpoint,
                        );
                    }
                }
            }
        }

        if settings.intersection {
            let n = elems.len().min(32);
            let lists: Vec<Vec<Prim>> = elems[..n].iter().map(|d| prims(d)).collect();
            for i in 0..n {
                for j in (i + 1)..n {
                    for a in &lists[i] {
                        for b in &lists[j] {
                            for p in intersect_prims(a, b) {
                                consider(p, SnapKind::Intersection);
                            }
                        }
                    }
                }
            }
        }

        best.map(|(_, p, k)| (p, k))
    }

    pub fn get_input_snap(
        &self,
        wx: f64,
        wy: f64,
        enabled: bool,
        settings: &SnapSettings,
    ) -> (f64, f64, Option<SnapKind>) {
        self.get_input_snap_ex(wx, wy, enabled, settings, false)
    }

    pub fn get_input_snap_ex(
        &self,
        wx: f64,
        wy: f64,
        enabled: bool,
        settings: &SnapSettings,
        exclude_selected: bool,
    ) -> (f64, f64, Option<SnapKind>) {
        if !enabled {
            return (wx, wy, None);
        }
        if let Some((p, kind)) = self.find_object_snap_ex(wx, wy, settings, exclude_selected) {
            return (p.x, p.y, Some(kind));
        }
        let (x, y) = self.get_snapped_pos(wx, wy, true);
        (x, y, None)
    }

    pub fn update_auto_pan(&mut self, x: f64, y: f64, width: f64, height: f64) {
        let margin = 40.0;
        let mut dx = 0.0;
        let mut dy = 0.0;

        if x < margin {
            dx = 10.0;
        } else if x > width - margin {
            dx = -10.0;
        }
        if y < margin {
            dy = 10.0;
        } else if y > height - margin {
            dy = -10.0;
        }

        self.camera.target_offset.0 += dx;
        self.camera.target_offset.1 += dy;
    }

    pub fn get_render_commands(&self, screen_w: f64, screen_h: f64) -> Vec<RenderCommand> {
        self.get_render_commands_in(0.0, 0.0, screen_w, screen_h)
    }

    pub fn get_render_commands_in(&self, x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<RenderCommand> {
        let p1 = self.camera.screen_to_world(x0, y0);
        let p2 = self.camera.screen_to_world(x1, y1);
        let viewport_bounds = BoundingBox::from_points(p1.0, p1.1, p2.0, p2.1);
        let visible_elements = self.get_visible_elements(viewport_bounds);
        let mut commands = Vec::with_capacity(visible_elements.len());
        let mut last_style: (f64, Vec<f64>) = (2.0, Vec::new()); // เท่าค่าเริ่มต้นของตัววาด
        for el in visible_elements {
            let mut draw_scale = 1.0f64;
            if self.apply_anim.is_playing {
                if let Some(idx) = self
                    .apply_anim
                    .elements_to_reveal
                    .iter()
                    .position(|&id| id == el.id)
                {
                    let start_time = idx as f64 * self.apply_anim.stagger_delay;
                    let progress = ((self.apply_anim.timer - start_time) / 0.2).clamp(0.0, 1.0);

                    if progress <= 0.0 {
                        continue;
                    }

                    draw_scale = if progress < 0.6 {
                        progress / 0.6 * 1.2
                    } else {
                        1.2 - (progress - 0.6) / 0.4 * 0.2
                    };
                }
            }

            let col = self.pen_color(el);
            if !matches!(el.data, ElementData::Text { .. } | ElementData::Dimension { .. }) {
                let st = self.pen_style(el);
                if st != last_style {
                    commands.push(RenderCommand::Style { lw: st.0, dash: st.1.clone() });
                    last_style = st;
                }
            }

            match &el.data {
                // --- Line ---
                ElementData::Line { start, end } => {
                    let mid_x = (start.x + end.x) / 2.0;
                    let mid_y = (start.y + end.y) / 2.0;
                    let new_start_x = mid_x + (start.x - mid_x) * draw_scale;
                    let new_start_y = mid_y + (start.y - mid_y) * draw_scale;
                    let new_end_x = mid_x + (end.x - mid_x) * draw_scale;
                    let new_end_y = mid_y + (end.y - mid_y) * draw_scale;
                    let s = self.camera.world_to_screen(new_start_x, new_start_y);
                    let e = self.camera.world_to_screen(new_end_x, new_end_y);
                    // ปลายอยู่ไกลเกินที่ Cairo รับได้ (~8 ล้าน px) → ตัดให้เหลือส่วนในกรอบก่อน
                    let (s, e) = if s.0.abs().max(s.1.abs()).max(e.0.abs()).max(e.1.abs()) > 1.0e6 {
                        match clip_line(s, e, (x0, y0, x1, y1)) {
                            Some((a, b)) => (a, b),
                            None => continue,
                        }
                    } else {
                        (s, e)
                    };
                    commands.push(RenderCommand::Line {
                        x1: s.0,
                        y1: s.1,
                        x2: e.0,
                        y2: e.1,
                        is_selected: el.is_selected,
                        color: col,
                    });
                }
                // --- Circle ---
                ElementData::Circle { center, radius } => {
                    let c = self.camera.world_to_screen(center.x, center.y);
                    let r = radius * self.camera.scale * draw_scale;
                    commands.push(RenderCommand::Circle {
                        cx: c.0,
                        cy: c.1,
                        radius: r,
                        is_selected: el.is_selected,
                        color: col,
                    });
                }
                ElementData::Dimension { start, end, offset } => {
                    if let Some(g) = linear_dimension_geometry(*start, *end, *offset) {
                        let s  = self.camera.world_to_screen(g.start.x,    g.start.y);
                        let e  = self.camera.world_to_screen(g.end.x,      g.end.y);
                        let df = self.camera.world_to_screen(g.dim_from.x, g.dim_from.y);
                        let dt = self.camera.world_to_screen(g.dim_to.x,   g.dim_to.y);
                        let distance = ((end.x-start.x).powi(2)+(end.y-start.y).powi(2)).sqrt();

                        commands.push(RenderCommand::Dimension {
                            text_x: (df.0 + dt.0) / 2.0,
                            text_y: (df.1 + dt.1) / 2.0,
                            text: format!("{:.2}", distance),
                            is_selected: el.is_selected,
                            // dim line
                            dim_x1: df.0, dim_y1: df.1,
                            dim_x2: dt.0, dim_y2: dt.1,
                            // extension line 1: จากจุดเริ่ม → dim_from
                            ext1_x1: s.0,  ext1_y1: s.1,
                            ext1_x2: df.0, ext1_y2: df.1,
                            // extension line 2: จากจุดปลาย → dim_to
                            ext2_x1: e.0,  ext2_y1: e.1,
                            ext2_x2: dt.0, ext2_y2: dt.1,
                        });
                    }
                }
                ElementData::Arc { center, radius, start_angle, end_angle } => {
                    let c = self.camera.world_to_screen(center.x, center.y);
                    commands.push(RenderCommand::Arc {
                        cx: c.0,
                        cy: c.1,
                        radius: radius * self.camera.scale * draw_scale,
                        start_angle: *start_angle,
                        end_angle: *end_angle,
                        is_selected: el.is_selected,
                        color: col,
                    });
                }
                ElementData::Polyline { points, closed } => {
                    let bb = el.bounding_box();
                    let (mx, my) = ((bb.min_x + bb.max_x) / 2.0, (bb.min_y + bb.max_y) / 2.0);
                    let pts = points
                        .iter()
                        .map(|p| {
                            self.camera.world_to_screen(
                                mx + (p.x - mx) * draw_scale,
                                my + (p.y - my) * draw_scale,
                            )
                        })
                        .collect();
                    commands.push(RenderCommand::Polyline {
                        points: pts,
                        closed: *closed,
                        is_selected: el.is_selected,
                        color: col,
                    });
                }
                ElementData::Text { pos, height, rotation, text, halign, valign, width } => {
                    let em_px = height * text::EM_PER_HEIGHT * self.camera.scale * draw_scale;
                    if em_px < 3.0 {
                        continue; // เล็กเกินอ่าน ไม่ต้องวาด
                    }
                    let s = self.camera.world_to_screen(pos.x, pos.y);
                    commands.push(RenderCommand::Text {
                        x: s.0,
                        y: s.1,
                        em_px,
                        rotation: *rotation,
                        wrap_px: width * self.camera.scale,
                        text: text.clone(),
                        halign: *halign,
                        valign: *valign,
                        is_selected: el.is_selected,
                        color: col,
                    });
                }
                ElementData::Hatch { loops, spec } => {
                    let bb = el.bounding_box();
                    if (bb.max_x - bb.min_x) * self.camera.scale < 1.0
                        && (bb.max_y - bb.min_y) * self.camera.scale < 1.0
                    {
                        continue;
                    }
                    // เส้นลายถี่กว่า 3 px มองไม่ออก → วาดทึบแทน
                    let lines = if spec.spacing * self.camera.scale >= 3.0 {
                        hatch::pattern_segments(loops, spec, Some(&viewport_bounds))
                    } else {
                        None
                    };
                    match lines {
                        Some(lines) => {
                            let st = ((self.pen_style(el).0 * 0.5).max(1.0), Vec::new());
                            if st != last_style {
                                commands.push(RenderCommand::Style { lw: st.0, dash: st.1.clone() });
                                last_style = st;
                            }
                            for (a, b) in lines {
                                let s = self.camera.world_to_screen(a.x, a.y);
                                let e = self.camera.world_to_screen(b.x, b.y);
                                commands.push(RenderCommand::Line {
                                    x1: s.0,
                                    y1: s.1,
                                    x2: e.0,
                                    y2: e.1,
                                    is_selected: el.is_selected,
                                    color: col,
                                });
                            }
                        }
                        None => {
                            let loops = loops
                                .iter()
                                .map(|l| {
                                    l.iter().map(|p| self.camera.world_to_screen(p.x, p.y)).collect()
                                })
                                .collect();
                            commands.push(RenderCommand::Fill {
                                loops,
                                filled: true,
                                is_selected: el.is_selected,
                                color: col,
                            });
                        }
                    }
                }
                ElementData::Fill { loops, filled } => {
                    let bb = el.bounding_box();
                    if (bb.max_x - bb.min_x) * self.camera.scale < 1.0
                        && (bb.max_y - bb.min_y) * self.camera.scale < 1.0
                    {
                        continue;
                    }
                    let (mx, my) = ((bb.min_x + bb.max_x) / 2.0, (bb.min_y + bb.max_y) / 2.0);
                    let loops = loops
                        .iter()
                        .map(|l| {
                            l.iter()
                                .map(|p| {
                                    self.camera.world_to_screen(
                                        mx + (p.x - mx) * draw_scale,
                                        my + (p.y - my) * draw_scale,
                                    )
                                })
                                .collect()
                        })
                        .collect();
                    commands.push(RenderCommand::Fill {
                        loops,
                        filled: *filled,
                        is_selected: el.is_selected,
                        color: col,
                    });
                }
            }
        }
        commands
    }
    pub fn render_commands_at_target(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<RenderCommand> {
        let saved = (self.camera.scale, self.camera.offset);
        self.camera.scale = self.camera.target_scale;
        self.camera.offset = self.camera.target_offset;
        let r = self.get_render_commands_in(x0, y0, x1, y1);
        self.camera.scale = saved.0;
        self.camera.offset = saved.1;
        r
    }
}

/// ตัดเส้นให้เหลือเฉพาะส่วนในกรอบ (Liang–Barsky) None = ไม่ตัดกรอบเลย
fn clip_line(
    (x1, y1): (f64, f64),
    (x2, y2): (f64, f64),
    (rx0, ry0, rx1, ry1): (f64, f64, f64, f64),
) -> Option<((f64, f64), (f64, f64))> {
    let (dx, dy) = (x2 - x1, y2 - y1);
    let (mut t0, mut t1) = (0.0_f64, 1.0_f64);
    for (p, q) in [
        (-dx, x1 - rx0.min(rx1)),
        (dx, rx0.max(rx1) - x1),
        (-dy, y1 - ry0.min(ry1)),
        (dy, ry0.max(ry1) - y1),
    ] {
        if p == 0.0 {
            if q < 0.0 {
                return None;
            }
        } else {
            let r = q / p;
            if p < 0.0 {
                if r > t1 {
                    return None;
                }
                t0 = t0.max(r);
            } else {
                if r < t0 {
                    return None;
                }
                t1 = t1.min(r);
            }
        }
    }
    Some(((x1 + dx * t0, y1 + dy * t0), (x1 + dx * t1, y1 + dy * t1)))
}

fn seg_seg(a1: Point, a2: Point, b1: Point, b2: Point) -> Option<Point> {
    let (dx1, dy1) = (a2.x - a1.x, a2.y - a1.y);
    let (dx2, dy2) = (b2.x - b1.x, b2.y - b1.y);
    let denom = dx1 * dy2 - dy1 * dx2;
    if denom.abs() < 1e-12 {
        return None; // ขนานกัน
    }
    let (ex, ey) = (b1.x - a1.x, b1.y - a1.y);
    let t = (ex * dy2 - ey * dx2) / denom;
    let u = (ex * dy1 - ey * dx1) / denom;
    let range = -1e-9..=1.0 + 1e-9;
    if range.contains(&t) && range.contains(&u) {
        Some(Point { x: a1.x + dx1 * t, y: a1.y + dy1 * t })
    } else {
        None
    }
}

fn seg_circle(a1: Point, a2: Point, c: Point, r: f64) -> Vec<Point> {
    let (dx, dy) = (a2.x - a1.x, a2.y - a1.y);
    let (fx, fy) = (a1.x - c.x, a1.y - c.y);
    let a = dx * dx + dy * dy;
    if a < 1e-12 {
        return vec![];
    }
    let b = 2.0 * (fx * dx + fy * dy);
    let cc = fx * fx + fy * fy - r * r;
    let disc = b * b - 4.0 * a * cc;
    if disc < 0.0 {
        return vec![];
    }
    let sq = disc.sqrt();
    let range = -1e-9..=1.0 + 1e-9;
    let mut out = Vec::new();
    for t in [(-b - sq) / (2.0 * a), (-b + sq) / (2.0 * a)] {
        if range.contains(&t) {
            out.push(Point { x: a1.x + dx * t, y: a1.y + dy * t });
        }
    }
    out
}

fn circle_circle(c1: Point, r1: f64, c2: Point, r2: f64) -> Vec<Point> {
    let (dx, dy) = (c2.x - c1.x, c2.y - c1.y);
    let d = (dx * dx + dy * dy).sqrt();
    if d < 1e-9 || d > r1 + r2 || d < (r1 - r2).abs() {
        return vec![];
    }
    let a = (r1 * r1 - r2 * r2 + d * d) / (2.0 * d);
    let h = (r1 * r1 - a * a).max(0.0).sqrt();
    let (mx, my) = (c1.x + a * dx / d, c1.y + a * dy / d);
    vec![
        Point { x: mx - h * dy / d, y: my + h * dx / d },
        Point { x: mx + h * dy / d, y: my - h * dx / d },
    ]
}

fn intersect_elements(a: &ElementData, b: &ElementData) -> Vec<Point> {
    use ElementData::*;
    match (a, b) {
        (Line { start: a1, end: a2 }, Line { start: b1, end: b2 }) => {
            seg_seg(*a1, *a2, *b1, *b2).into_iter().collect()
        }
        (Line { start, end }, Circle { center, radius })
        | (Circle { center, radius }, Line { start, end }) => {
            seg_circle(*start, *end, *center, *radius)
        }
        (Circle { center: c1, radius: r1 }, Circle { center: c2, radius: r2 }) => {
            circle_circle(*c1, *r1, *c2, *r2)
        }
        _ => vec![],
    }
}

pub fn angle_in_range(a: f64, start: f64, end: f64) -> bool {
    (a - start).rem_euclid(TAU) <= (end - start) + 1e-9
}

pub fn arc_point(c: Point, r: f64, a: f64) -> Point {
    Point { x: c.x + r * a.cos(), y: c.y + r * a.sin() }
}

pub fn arc_from_three_points(p1: Point, p2: Point, p3: Point) -> Option<(Point, f64, f64, f64)> {
    let d = 2.0 * (p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y));
    if d.abs() < 1e-9 {
        return None; // สามจุดอยู่ในแนวเส้นตรง
    }
    let (s1, s2, s3) = (
        p1.x * p1.x + p1.y * p1.y,
        p2.x * p2.x + p2.y * p2.y,
        p3.x * p3.x + p3.y * p3.y,
    );
    let ux = (s1 * (p2.y - p3.y) + s2 * (p3.y - p1.y) + s3 * (p1.y - p2.y)) / d;
    let uy = (s1 * (p3.x - p2.x) + s2 * (p1.x - p3.x) + s3 * (p2.x - p1.x)) / d;
    let center = Point { x: ux, y: uy };
    let radius = (p1.x - ux).hypot(p1.y - uy);
    let ang = |p: Point| (p.y - uy).atan2(p.x - ux);

    let ccw = (p2.x - p1.x) * (p3.y - p2.y) - (p2.y - p1.y) * (p3.x - p2.x) > 0.0;
    let (start, mut end) = if ccw { (ang(p1), ang(p3)) } else { (ang(p3), ang(p1)) };
    while end <= start {
        end += TAU;
    }
    Some((center, radius, start, end))
}

fn arc_bounds(c: Point, r: f64, s: f64, e: f64) -> BoundingBox {
    let mut pts = vec![arc_point(c, r, s), arc_point(c, r, e)];
    for k in 0..4 {
        let a = k as f64 * FRAC_PI_2;
        if angle_in_range(a, s, e) {
            pts.push(arc_point(c, r, a));
        }
    }
    let mut b = BoundingBox { min_x: f64::MAX, min_y: f64::MAX, max_x: f64::MIN, max_y: f64::MIN };
    for p in pts {
        b.min_x = b.min_x.min(p.x);
        b.min_y = b.min_y.min(p.y);
        b.max_x = b.max_x.max(p.x);
        b.max_y = b.max_y.max(p.y);
    }
    b
}

enum Prim {
    Seg(Point, Point),
    Circle { c: Point, r: f64 },
    Arc { c: Point, r: f64, s: f64, e: f64 },
}

fn prims(d: &ElementData) -> Vec<Prim> {
    match d {
        ElementData::Line { start, end } => vec![Prim::Seg(*start, *end)],
        ElementData::Circle { center, radius } => vec![Prim::Circle { c: *center, r: *radius }],
        ElementData::Arc { center, radius, start_angle, end_angle } => {
            vec![Prim::Arc { c: *center, r: *radius, s: *start_angle, e: *end_angle }]
        }
        ElementData::Polyline { points, closed } => {
            let n = points.len();
            let segs = if *closed { n } else { n.saturating_sub(1) };
            (0..segs).map(|i| Prim::Seg(points[i], points[(i + 1) % n])).collect()
        }
        ElementData::Dimension { .. } | ElementData::Fill { .. } | ElementData::Hatch { .. } | ElementData::Text { .. } => vec![],
    }
}

fn on_arc(p: Point, c: Point, s: f64, e: f64) -> bool {
    angle_in_range((p.y - c.y).atan2(p.x - c.x), s, e)
}

fn intersect_prims(a: &Prim, b: &Prim) -> Vec<Point> {
    use Prim::*;
    match (a, b) {
        (Seg(a1, a2), Seg(b1, b2)) => seg_seg(*a1, *a2, *b1, *b2).into_iter().collect(),
        (Seg(a1, a2), Circle { c, r }) | (Circle { c, r }, Seg(a1, a2)) => {
            seg_circle(*a1, *a2, *c, *r)
        }
        (Seg(a1, a2), Arc { c, r, s, e }) | (Arc { c, r, s, e }, Seg(a1, a2)) => {
            seg_circle(*a1, *a2, *c, *r)
                .into_iter()
                .filter(|p| on_arc(*p, *c, *s, *e))
                .collect()
        }
        (Circle { c: c1, r: r1 }, Circle { c: c2, r: r2 }) => circle_circle(*c1, *r1, *c2, *r2),
        (Circle { c: c1, r: r1 }, Arc { c: c2, r: r2, s, e })
        | (Arc { c: c2, r: r2, s, e }, Circle { c: c1, r: r1 }) => {
            circle_circle(*c1, *r1, *c2, *r2)
                .into_iter()
                .filter(|p| on_arc(*p, *c2, *s, *e))
                .collect()
        }
        (Arc { c: c1, r: r1, s: s1, e: e1 }, Arc { c: c2, r: r2, s: s2, e: e2 }) => {
            circle_circle(*c1, *r1, *c2, *r2)
                .into_iter()
                .filter(|p| on_arc(*p, *c1, *s1, *e1) && on_arc(*p, *c2, *s2, *e2))
                .collect()
        }
    }
}

fn dist_point_seg(p: Point, a: Point, b: Point) -> f64 {
    let (dx, dy) = (b.x - a.x, b.y - a.y);
    let len2 = dx * dx + dy * dy;
    let t = if len2 < 1e-18 {
        0.0
    } else {
        (((p.x - a.x) * dx + (p.y - a.y) * dy) / len2).clamp(0.0, 1.0)
    };
    (p.x - (a.x + t * dx)).hypot(p.y - (a.y + t * dy))
}

fn seg_hits_rect(a: Point, b: Point, r: &BoundingBox) -> bool {
    let inside =
        |p: Point| p.x >= r.min_x && p.x <= r.max_x && p.y >= r.min_y && p.y <= r.max_y;
    if inside(a) || inside(b) {
        return true;
    }
    let c = [
        Point { x: r.min_x, y: r.min_y },
        Point { x: r.max_x, y: r.min_y },
        Point { x: r.max_x, y: r.max_y },
        Point { x: r.min_x, y: r.max_y },
    ];
    (0..4).any(|i| seg_seg(a, b, c[i], c[(i + 1) % 4]).is_some())
}

fn dimension_segments(start: Point, end: Point, offset: f64) -> Vec<(Point, Point)> {
    match linear_dimension_geometry(start, end, offset) {
        Some(g) => vec![(g.start, g.dim_from), (g.end, g.dim_to), (g.dim_from, g.dim_to)],
        None => vec![(start, end)],
    }
}

fn offset_radius(c: Point, r: f64, side: Point, dist: f64) -> Result<f64, &'static str> {
    let dc = (side.x - c.x).hypot(side.y - c.y);
    let nr = if dc < r { r - dist } else { r + dist };
    if nr <= 1e-9 {
        Err("Distance is larger than the radius")
    } else {
        Ok(nr)
    }
}

fn line_line(p1: Point, u1: (f64, f64), p2: Point, u2: (f64, f64)) -> Option<Point> {
    let den = u1.0 * u2.1 - u1.1 * u2.0;
    if den.abs() < 1e-9 {
        return None; // ขนานกัน
    }
    let t = ((p2.x - p1.x) * u2.1 - (p2.y - p1.y) * u2.0) / den;
    Some(Point { x: p1.x + u1.0 * t, y: p1.y + u1.1 * t })
}

fn offset_polyline(
    points: &[Point],
    closed: bool,
    side: Point,
    dist: f64,
) -> Result<Vec<Point>, &'static str> {
    // ตัดจุดซ้ำติดกันออกก่อน
    let mut pts: Vec<Point> = Vec::with_capacity(points.len());
    for p in points {
        if pts.last().map_or(true, |l| (l.x - p.x).hypot(l.y - p.y) > 1e-9) {
            pts.push(*p);
        }
    }
    if closed && pts.len() > 1 {
        let (f, l) = (pts[0], pts[pts.len() - 1]);
        if (f.x - l.x).hypot(f.y - l.y) <= 1e-9 {
            pts.pop();
        }
    }
    let n = pts.len();
    if n < 2 || (closed && n < 3) {
        return Err("Polyline is too short to offset");
    }
    let segs = if closed { n } else { n - 1 };

    let dirs: Vec<(f64, f64)> = (0..segs)
        .map(|i| {
            let (a, b) = (pts[i], pts[(i + 1) % n]);
            let l = (b.x - a.x).hypot(b.y - a.y);
            ((b.x - a.x) / l, (b.y - a.y) / l)
        })
        .collect();

    // ด้านที่เลือก: ดูจากส่วนที่อยู่ใกล้เมาส์ที่สุด
    let mut best = (f64::MAX, 1.0_f64);
    for i in 0..segs {
        let (a, b) = (pts[i], pts[(i + 1) % n]);
        let d = dist_point_seg(side, a, b);
        if d < best.0 {
            let side_dot = -dirs[i].1 * (side.x - a.x) + dirs[i].0 * (side.y - a.y);
            best = (d, if side_dot >= 0.0 { 1.0 } else { -1.0 });
        }
    }
    let sign = best.1;

    let normal = |i: usize| (-dirs[i].1 * sign * dist, dirs[i].0 * sign * dist);
    let base = |i: usize| {
        let (ox, oy) = normal(i);
        Point { x: pts[i].x + ox, y: pts[i].y + oy }
    };

    let mut out = Vec::with_capacity(n);
    for k in 0..n {
        let p = if !closed && k == 0 {
            base(0)
        } else if !closed && k == n - 1 {
            let (ox, oy) = normal(n - 2);
            Point { x: pts[k].x + ox, y: pts[k].y + oy }
        } else {
            let prev = (k + n - 1) % n;
            line_line(base(prev), dirs[prev], base(k), dirs[k]).unwrap_or_else(|| {
                let (ox, oy) = normal(prev);
                Point { x: pts[k].x + ox, y: pts[k].y + oy }
            })
        };
        out.push(p);
    }

    // ถ้าส่วนไหนกลับทิศ แปลว่าระยะใหญ่เกินรูป
    for i in 0..segs {
        let (a, b) = (out[i], out[(i + 1) % n]);
        if (b.x - a.x) * dirs[i].0 + (b.y - a.y) * dirs[i].1 <= 1e-9 {
            return Err("Distance is too large for this shape");
        }
    }
    Ok(out)
}

fn offset_data(data: &ElementData, side: Point, dist: f64) -> Result<ElementData, &'static str> {
    match data {
        ElementData::Line { start, end } => {
            let len = (end.x - start.x).hypot(end.y - start.y);
            if len < 1e-9 {
                return Err("Line has zero length");
            }
            let (ux, uy) = ((end.x - start.x) / len, (end.y - start.y) / len);
            let side_dot = -uy * (side.x - start.x) + ux * (side.y - start.y);
            let s = if side_dot >= 0.0 { 1.0 } else { -1.0 };
            let (ox, oy) = (-uy * s * dist, ux * s * dist);
            Ok(ElementData::Line {
                start: Point { x: start.x + ox, y: start.y + oy },
                end: Point { x: end.x + ox, y: end.y + oy },
            })
        }
        ElementData::Circle { center, radius } => Ok(ElementData::Circle {
            center: *center,
            radius: offset_radius(*center, *radius, side, dist)?,
        }),
        ElementData::Arc { center, radius, start_angle, end_angle } => Ok(ElementData::Arc {
            center: *center,
            radius: offset_radius(*center, *radius, side, dist)?,
            start_angle: *start_angle,
            end_angle: *end_angle,
        }),
        ElementData::Polyline { points, closed } => Ok(ElementData::Polyline {
            points: offset_polyline(points, *closed, side, dist)?,
            closed: *closed,
        }),
        ElementData::Dimension { .. } => Err("Dimensions can't be offset"),
        ElementData::Fill { .. } => Err("Filled shapes can't be offset"),
        ElementData::Hatch { .. } => Err("Hatches can't be offset"),
        ElementData::Text { .. } => Err("Text can't be offset"),
    }
}

impl ElementData {
    /// ระยะจากจุดถึงเส้นจริงของชิ้นงาน (หน่วยโลก)
    pub fn distance_to(&self, p: Point) -> f64 {
        match self {
            ElementData::Line { start, end } => dist_point_seg(p, *start, *end),
            ElementData::Circle { center, radius } => {
                ((p.x - center.x).hypot(p.y - center.y) - radius).abs()
            }
            ElementData::Arc { center, radius, start_angle, end_angle } => {
                let a = (p.y - center.y).atan2(p.x - center.x);
                if angle_in_range(a, *start_angle, *end_angle) {
                    ((p.x - center.x).hypot(p.y - center.y) - radius).abs()
                } else {
                    let s = arc_point(*center, *radius, *start_angle);
                    let e = arc_point(*center, *radius, *end_angle);
                    (p.x - s.x).hypot(p.y - s.y).min((p.x - e.x).hypot(p.y - e.y))
                }
            }
            ElementData::Polyline { points, closed } => {
                let n = points.len();
                if n == 1 {
                    return (p.x - points[0].x).hypot(p.y - points[0].y);
                }
                let segs = if *closed { n } else { n.saturating_sub(1) };
                (0..segs)
                    .map(|i| dist_point_seg(p, points[i], points[(i + 1) % n]))
                    .fold(f64::MAX, f64::min)
            }
            ElementData::Dimension { start, end, offset } => dimension_segments(*start, *end, *offset)
                .into_iter()
                .map(|(a, b)| dist_point_seg(p, a, b))
                .fold(f64::MAX, f64::min),
            ElementData::Text { pos, height, rotation, text, halign, valign, width } => {
                text::edge_distance(p, *pos, *height, *rotation, text, *halign, *valign, *width)
            }
            // คลิกใกล้เส้นขอบต้องได้เส้นก่อน: hatch เลือกได้เฉพาะคลิกข้างใน (ดู fill_contains)
            ElementData::Hatch { .. } => f64::MAX,
            ElementData::Fill { loops, .. } => loops
                .iter()
                .map(|l| dist_to_loop(p, l))
                .fold(f64::MAX, f64::min),
        }
    }

    /// จุดอยู่ในพื้นที่ทึบหรือไม่ (ใช้เลือกด้วยการคลิกข้างใน)
    pub fn fill_contains(&self, p: Point) -> bool {
        match self {
            ElementData::Fill { loops, filled: true } => point_in_loops(p, loops),
            ElementData::Hatch { loops, .. } => point_in_loops(p, loops),
            ElementData::Text { pos, height, rotation, text, halign, valign, width } => {
                text::contains(p, *pos, *height, *rotation, text, *halign, *valign, *width)
            }
            _ => false,
        }
    }

    /// สำหรับ crossing select: ส่วนใดส่วนหนึ่งของเส้นอยู่ในกรอบหรือไม่
    pub fn touches_rect(&self, r: &BoundingBox) -> bool {
        match self {
            ElementData::Line { start, end } => seg_hits_rect(*start, *end, r),
            ElementData::Circle { center, radius } => {
                let nx = center.x.clamp(r.min_x, r.max_x);
                let ny = center.y.clamp(r.min_y, r.max_y);
                let dmin = (center.x - nx).hypot(center.y - ny);
                let dmax = [
                    (r.min_x, r.min_y),
                    (r.max_x, r.min_y),
                    (r.max_x, r.max_y),
                    (r.min_x, r.max_y),
                ]
                .iter()
                .map(|(x, y)| (center.x - x).hypot(center.y - y))
                .fold(0.0, f64::max);
                dmin <= *radius && *radius <= dmax
            }
            ElementData::Arc { center, radius, start_angle, end_angle } => {
                const N: usize = 128;
                let pts: Vec<Point> = (0..=N)
                    .map(|i| {
                        let a = start_angle + (end_angle - start_angle) * i as f64 / N as f64;
                        arc_point(*center, *radius, a)
                    })
                    .collect();
                pts.windows(2).any(|w| seg_hits_rect(w[0], w[1], r))
            }
            ElementData::Polyline { points, closed } => {
                let n = points.len();
                if n == 1 {
                    return seg_hits_rect(points[0], points[0], r);
                }
                let segs = if *closed { n } else { n.saturating_sub(1) };
                (0..segs).any(|i| seg_hits_rect(points[i], points[(i + 1) % n], r))
            }
            ElementData::Dimension { start, end, offset } => dimension_segments(*start, *end, *offset)
                .into_iter()
                .any(|(a, b)| seg_hits_rect(a, b, r)),
            ElementData::Text { pos, height, rotation, text, halign, valign, width } => {
                let c = text::corners(*pos, *height, *rotation, text, *halign, *valign, *width);
                let centre = Point { x: (r.min_x + r.max_x) / 2.0, y: (r.min_y + r.max_y) / 2.0 };
                (0..4).any(|i| seg_hits_rect(c[i], c[(i + 1) % 4], r))
                    || text::contains(centre, *pos, *height, *rotation, text, *halign, *valign, *width)
            }
            ElementData::Hatch { loops, .. } => {
                let edge_hit = loops.iter().any(|l| {
                    let n = l.len();
                    n >= 2 && (0..n).any(|i| seg_hits_rect(l[i], l[(i + 1) % n], r))
                });
                edge_hit
                    || point_in_loops(
                        Point { x: (r.min_x + r.max_x) / 2.0, y: (r.min_y + r.max_y) / 2.0 },
                        loops,
                    )
            }
            ElementData::Fill { loops, filled } => {
                let edge_hit = loops.iter().any(|l| {
                    let n = l.len();
                    n >= 2 && (0..n).any(|i| seg_hits_rect(l[i], l[(i + 1) % n], r))
                });
                edge_hit
                    || (*filled
                        && point_in_loops(
                            Point { x: (r.min_x + r.max_x) / 2.0, y: (r.min_y + r.max_y) / 2.0 },
                            loops,
                        ))
            }
        }
    }
}

fn dist_to_loop(p: Point, l: &[Point]) -> f64 {
    let n = l.len();
    match n {
        0 => f64::MAX,
        1 => (p.x - l[0].x).hypot(p.y - l[0].y),
        _ => (0..n)
            .map(|i| dist_point_seg(p, l[i], l[(i + 1) % n]))
            .fold(f64::MAX, f64::min),
    }
}

/// even-odd: จุดอยู่ข้างในถ้าตัดขอบเป็นจำนวนคี่
fn point_in_loops(p: Point, loops: &[Vec<Point>]) -> bool {
    let mut inside = false;
    for l in loops {
        let n = l.len();
        if n < 3 {
            continue;
        }
        let mut j = n - 1;
        for i in 0..n {
            let (a, b) = (l[i], l[j]);
            if (a.y > p.y) != (b.y > p.y)
                && p.x < (b.x - a.x) * (p.y - a.y) / (b.y - a.y) + a.x
            {
                inside = !inside;
            }
            j = i;
        }
    }
    inside
}

const TRIM_EPS: f64 = 1e-9;
fn trim_data(
    data: &ElementData,
    click: Point,
    cutters: &[Prim],
) -> Result<(Vec<ElementData>, ElementData), &'static str> {
    const NO_EDGE: &str = "No cutting edge crosses this part";
    match data {
        ElementData::Line { start, end } => {
            let (dx, dy) = (end.x - start.x, end.y - start.y);
            let len2 = dx * dx + dy * dy;
            if len2 < 1e-18 {
                return Err("Line has zero length");
            }
            let t_of = |p: Point| ((p.x - start.x) * dx + (p.y - start.y) * dy) / len2;
            let me = Prim::Seg(*start, *end);
            let mut ts: Vec<f64> = cutters
                .iter()
                .flat_map(|c| intersect_prims(&me, c))
                .map(t_of)
                .filter(|t| *t > TRIM_EPS && *t < 1.0 - TRIM_EPS)
                .collect();
            ts.sort_by(f64::total_cmp);
            let tc = t_of(click).clamp(0.0, 1.0);
            let lo = ts.iter().copied().filter(|t| *t < tc).last();
            let hi = ts.iter().copied().find(|t| *t > tc);
            if lo.is_none() && hi.is_none() {
                return Err(NO_EDGE);
            }
            let at = |t: f64| Point { x: start.x + dx * t, y: start.y + dy * t };
            let mut out = Vec::new();
            if let Some(l) = lo {
                out.push(ElementData::Line { start: *start, end: at(l) });
            }
            if let Some(h) = hi {
                out.push(ElementData::Line { start: at(h), end: *end });
            }
            let removed = ElementData::Line { start: at(lo.unwrap_or(0.0)), end: at(hi.unwrap_or(1.0)) };
            Ok((out, removed))
        }
        ElementData::Circle { center, radius } => {
            let me = Prim::Circle { c: *center, r: *radius };
            let ang = |p: Point| (p.y - center.y).atan2(p.x - center.x).rem_euclid(TAU);
            let mut angs: Vec<f64> = cutters
                .iter()
                .flat_map(|c| intersect_prims(&me, c))
                .map(ang)
                .collect();
            angs.sort_by(f64::total_cmp);
            angs.dedup_by(|a, b| (*a - *b).abs() < TRIM_EPS);
            if angs.len() < 2 {
                return Err("A circle needs two cutting edges to be trimmed");
            }
            let ac = ang(click);
            // ช่วงที่ครอบจุดคลิก = (prev, next) ส่วนนี้จะถูกตัดทิ้ง
            let prev = angs.iter().copied().filter(|a| *a <= ac).last().unwrap_or(angs[angs.len() - 1]);
            let next = angs.iter().copied().find(|a| *a > ac).unwrap_or(angs[0]);
            let mut e = prev;
            while e <= next {
                e += TAU;
            }
            let mut re = next;
            while re <= prev {
                re += TAU;
            }
            Ok((
                vec![ElementData::Arc { center: *center, radius: *radius, start_angle: next, end_angle: e }],
                ElementData::Arc { center: *center, radius: *radius, start_angle: prev, end_angle: re },
            ))
        }
        ElementData::Arc { center, radius, start_angle, end_angle } => {
            let span = end_angle - start_angle;
            let me = Prim::Arc { c: *center, r: *radius, s: *start_angle, e: *end_angle };
            let u_of = |p: Point| ((p.y - center.y).atan2(p.x - center.x) - start_angle).rem_euclid(TAU);
            let mut us: Vec<f64> = cutters
                .iter()
                .flat_map(|c| intersect_prims(&me, c))
                .map(u_of)
                .filter(|u| *u > TRIM_EPS && *u < span - TRIM_EPS)
                .collect();
            us.sort_by(f64::total_cmp);
            let mut uc = u_of(click);
            if uc > span {
                uc = if uc - span > TAU - uc { 0.0 } else { span };
            }
            let lo = us.iter().copied().filter(|u| *u < uc).last();
            let hi = us.iter().copied().find(|u| *u > uc);
            if lo.is_none() && hi.is_none() {
                return Err(NO_EDGE);
            }
            let mut out = Vec::new();
            if let Some(l) = lo {
                out.push(ElementData::Arc {
                    center: *center,
                    radius: *radius,
                    start_angle: *start_angle,
                    end_angle: start_angle + l,
                });
            }
            if let Some(h) = hi {
                out.push(ElementData::Arc {
                    center: *center,
                    radius: *radius,
                    start_angle: start_angle + h,
                    end_angle: *end_angle,
                });
            }
            let removed = ElementData::Arc {
                center: *center,
                radius: *radius,
                start_angle: start_angle + lo.unwrap_or(0.0),
                end_angle: start_angle + hi.unwrap_or(span),
            };
            Ok((out, removed))
        }
        ElementData::Polyline { points, closed } => trim_polyline(points, *closed, click, cutters),
        ElementData::Dimension { .. } => Err("Dimensions can't be trimmed"),
        ElementData::Fill { .. } => Err("Filled shapes can't be trimmed"),
        ElementData::Hatch { .. } => Err("Hatches can't be trimmed"),
        ElementData::Text { .. } => Err("Text can't be trimmed"),
    }
}

fn extend_data(
    data: &ElementData,
    click: Point,
    cutters: &[Prim],
    reach: f64,
) -> Result<(ElementData, ElementData), &'static str> {
    const NO_EDGE: &str = "No boundary found to extend to";
    match data {
        ElementData::Line { start, end } => {
            let (dx, dy) = (end.x - start.x, end.y - start.y);
            let len = dx.hypot(dy);
            if len < 1e-9 {
                return Err("Line has zero length");
            }
            let (ux, uy) = (dx / len, dy / len);
            let t = ((click.x - start.x) * dx + (click.y - start.y) * dy) / (len * len);
            let from_end = t >= 0.5;
            let (origin, dir) = if from_end { (*end, (ux, uy)) } else { (*start, (-ux, -uy)) };
            let ray = Prim::Seg(origin, Point { x: origin.x + dir.0 * reach, y: origin.y + dir.1 * reach });
            let best = cutters
                .iter()
                .flat_map(|c| intersect_prims(&ray, c))
                .map(|p| (p.x - origin.x) * dir.0 + (p.y - origin.y) * dir.1)
                .filter(|d| *d > 1e-7)
                .fold(f64::MAX, f64::min);
            if best == f64::MAX {
                return Err(NO_EDGE);
            }
            let np = Point { x: origin.x + dir.0 * best, y: origin.y + dir.1 * best };
            let added = ElementData::Line { start: origin, end: np };
            Ok((
                if from_end {
                    ElementData::Line { start: *start, end: np }
                } else {
                    ElementData::Line { start: np, end: *end }
                },
                added,
            ))
        }
        ElementData::Arc { center, radius, start_angle, end_angle } => {
            let span = end_angle - start_angle;
            let gap = TAU - span;
            if gap < TRIM_EPS {
                return Err("Arc is already a full circle");
            }
            let ang = |p: Point| (p.y - center.y).atan2(p.x - center.x);
            let uc = (ang(click) - start_angle).rem_euclid(TAU);
            let from_end = if uc <= span { uc >= span / 2.0 } else { uc - span <= TAU - uc };
            let me = Prim::Circle { c: *center, r: *radius };
            let best = cutters
                .iter()
                .flat_map(|c| intersect_prims(&me, c))
                .map(|p| {
                    let a = ang(p);
                    if from_end { (a - end_angle).rem_euclid(TAU) } else { (start_angle - a).rem_euclid(TAU) }
                })
                .filter(|d| *d > TRIM_EPS && *d < gap - TRIM_EPS)
                .fold(f64::MAX, f64::min);
            if best == f64::MAX {
                return Err(NO_EDGE);
            }
            let (c, r) = (*center, *radius);
            Ok(if from_end {
                (
                    ElementData::Arc { center: c, radius: r, start_angle: *start_angle, end_angle: end_angle + best },
                    ElementData::Arc { center: c, radius: r, start_angle: *end_angle, end_angle: end_angle + best },
                )
            } else {
                (
                    ElementData::Arc { center: c, radius: r, start_angle: start_angle - best, end_angle: *end_angle },
                    ElementData::Arc { center: c, radius: r, start_angle: start_angle - best, end_angle: *start_angle },
                )
            })
        }
        ElementData::Circle { .. } => Err("Circles can't be extended"),
        ElementData::Polyline { points, closed } => extend_polyline(points, *closed, click, cutters, reach),
        ElementData::Dimension { .. } => Err("Dimensions can't be extended"),
        ElementData::Fill { .. } => Err("Filled shapes can't be extended"),
        ElementData::Hatch { .. } => Err("Hatches can't be extended"),
        ElementData::Text { .. } => Err("Text can't be extended"),
    }
}

fn merge_opt(a: Option<BoundingBox>, b: Option<BoundingBox>) -> Option<BoundingBox> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.merge(&b)),
        (a, b) => a.or(b),
    }
}

fn extend_area(data: &ElementData, reach: f64) -> Option<BoundingBox> {
    let ray = |o: Point, from: Point| -> Option<BoundingBox> {
        let (dx, dy) = (o.x - from.x, o.y - from.y);
        let l = dx.hypot(dy);
        if l < 1e-12 {
            return None;
        }
        Some(BoundingBox::from_points(
            o.x,
            o.y,
            o.x + dx / l * reach,
            o.y + dy / l * reach,
        ))
    };
    match data {
        ElementData::Line { start, end } => merge_opt(ray(*end, *start), ray(*start, *end)),
        ElementData::Polyline { points, closed: false } if points.len() >= 2 => {
            let n = points.len();
            merge_opt(ray(points[n - 1], points[n - 2]), ray(points[0], points[1]))
        }
        ElementData::Arc { center, radius, .. } => Some(BoundingBox {
            min_x: center.x - radius,
            min_y: center.y - radius,
            max_x: center.x + radius,
            max_y: center.y + radius,
        }),
        _ => None,
    }
}

// ---------------- Polyline ----------------

fn poly_segs(n: usize, closed: bool) -> usize {
    if closed { n } else { n.saturating_sub(1) }
}

/// จุดบน polyline ที่พารามิเตอร์ s (จำนวนเต็ม = ลำดับ vertex, ทศนิยม = สัดส่วนบน segment)
fn poly_point_at(pts: &[Point], closed: bool, s: f64) -> Point {
    let n = pts.len();
    let segs = poly_segs(n, closed);
    let mut s = s;
    if closed && s >= segs as f64 {
        s -= segs as f64;
    }
    let i = (s.floor().max(0.0) as usize).min(segs - 1);
    let t = s - i as f64;
    let (a, b) = (pts[i], pts[(i + 1) % n]);
    Point { x: a.x + (b.x - a.x) * t, y: a.y + (b.y - a.y) * t }
}

/// ท่อนของ polyline จากพารามิเตอร์ sa ถึง sb (sb > sa, closed อนุญาตให้ sb เกินรอบได้)
fn poly_sub(pts: &[Point], closed: bool, sa: f64, sb: f64) -> Vec<Point> {
    let n = pts.len();
    let mut out = vec![poly_point_at(pts, closed, sa)];
    let mut m = sa.floor() as i64 + 1;
    while (m as f64) < sb - 1e-9 {
        if (m as f64) > sa + 1e-9 {
            out.push(pts[(m as usize) % n]);
        }
        m += 1;
    }
    out.push(poly_point_at(pts, closed, sb));
    out.dedup_by(|a, b| (a.x - b.x).hypot(a.y - b.y) < 1e-9);
    out
}

/// พารามิเตอร์ของจุดบน polyline ที่ใกล้จุดคลิกที่สุด
fn poly_click_param(pts: &[Point], closed: bool, click: Point) -> f64 {
    let n = pts.len();
    let mut best = (f64::MAX, 0.0);
    for i in 0..poly_segs(n, closed) {
        let (a, b) = (pts[i], pts[(i + 1) % n]);
        let (dx, dy) = (b.x - a.x, b.y - a.y);
        let len2 = dx * dx + dy * dy;
        let t = if len2 < 1e-18 {
            0.0
        } else {
            (((click.x - a.x) * dx + (click.y - a.y) * dy) / len2).clamp(0.0, 1.0)
        };
        let d = (click.x - (a.x + dx * t)).hypot(click.y - (a.y + dy * t));
        if d < best.0 {
            best = (d, i as f64 + t);
        }
    }
    best.1
}

/// พารามิเตอร์ของทุกจุดที่ cutters ตัด polyline (เรียงและตัดซ้ำแล้ว)
fn poly_cuts(pts: &[Point], closed: bool, cutters: &[Prim]) -> Vec<f64> {
    let n = pts.len();
    let mut cuts = Vec::new();
    for i in 0..poly_segs(n, closed) {
        let (a, b) = (pts[i], pts[(i + 1) % n]);
        let (dx, dy) = (b.x - a.x, b.y - a.y);
        let len2 = dx * dx + dy * dy;
        if len2 < 1e-18 {
            continue;
        }
        let me = Prim::Seg(a, b);
        for c in cutters {
            for p in intersect_prims(&me, c) {
                let t = (((p.x - a.x) * dx + (p.y - a.y) * dy) / len2).clamp(0.0, 1.0);
                cuts.push(i as f64 + t);
            }
        }
    }
    cuts.sort_by(f64::total_cmp);
    cuts.dedup_by(|a, b| (*a - *b).abs() < 1e-9);
    cuts
}

fn trim_polyline(
    pts: &[Point],
    closed: bool,
    click: Point,
    cutters: &[Prim],
) -> Result<(Vec<ElementData>, ElementData), &'static str> {
    let n = pts.len();
    if n < 2 || (closed && n < 3) {
        return Err("Polyline is too short to trim");
    }
    let segs = poly_segs(n, closed) as f64;
    let mut cuts = poly_cuts(pts, closed, cutters);
    let sc = poly_click_param(pts, closed, click);
    let mk = |v: Vec<Point>| ElementData::Polyline { points: v, closed: false };

    if !closed {
        cuts.retain(|s| *s > TRIM_EPS && *s < segs - TRIM_EPS);
        let lo = cuts.iter().copied().filter(|s| *s < sc).last();
        let hi = cuts.iter().copied().find(|s| *s > sc);
        if lo.is_none() && hi.is_none() {
            return Err("No cutting edge crosses this part");
        }
        let mut out = Vec::new();
        if let Some(l) = lo {
            let v = poly_sub(pts, false, 0.0, l);
            if v.len() >= 2 {
                out.push(mk(v));
            }
        }
        if let Some(h) = hi {
            let v = poly_sub(pts, false, h, segs);
            if v.len() >= 2 {
                out.push(mk(v));
            }
        }
        let removed = mk(poly_sub(pts, false, lo.unwrap_or(0.0), hi.unwrap_or(segs)));
        Ok((out, removed))
    } else {
        // จุดตัดที่ s≈0 กับ s≈segs คือจุดเดียวกัน
        if cuts.len() >= 2 && cuts[0] < TRIM_EPS && segs - cuts[cuts.len() - 1] < TRIM_EPS {
            cuts.pop();
        }
        if cuts.len() < 2 {
            return Err("A closed polyline needs two cutting edges to be trimmed");
        }
        let prev = cuts.iter().copied().filter(|s| *s <= sc).last().unwrap_or(cuts[cuts.len() - 1]);
        let next = cuts.iter().copied().find(|s| *s > sc).unwrap_or(cuts[0]);
        let mut e = prev;
        while e <= next {
            e += segs;
        }
        let mut re = next;
        while re <= prev {
            re += segs;
        }
        let kept = poly_sub(pts, true, next, e);
        let removed = poly_sub(pts, true, prev, re);
        Ok((vec![mk(kept)], mk(removed)))
    }
}

fn extend_polyline(
    pts: &[Point],
    closed: bool,
    click: Point,
    cutters: &[Prim],
    reach: f64,
) -> Result<(ElementData, ElementData), &'static str> {
    if closed {
        return Err("Closed polylines can't be extended");
    }
    let n = pts.len();
    if n < 2 {
        return Err("Polyline is too short to extend");
    }
    let (d0, d1) = (
        (click.x - pts[0].x).hypot(click.y - pts[0].y),
        (click.x - pts[n - 1].x).hypot(click.y - pts[n - 1].y),
    );
    let from_end = d1 <= d0;
    let (origin, prev) = if from_end { (pts[n - 1], pts[n - 2]) } else { (pts[0], pts[1]) };
    let (dx, dy) = (origin.x - prev.x, origin.y - prev.y);
    let len = dx.hypot(dy);
    if len < 1e-12 {
        return Err("End segment has zero length");
    }
    let dir = (dx / len, dy / len);
    let ray = Prim::Seg(origin, Point { x: origin.x + dir.0 * reach, y: origin.y + dir.1 * reach });
    let best = cutters
        .iter()
        .flat_map(|c| intersect_prims(&ray, c))
        .map(|p| (p.x - origin.x) * dir.0 + (p.y - origin.y) * dir.1)
        .filter(|d| *d > 1e-7)
        .fold(f64::MAX, f64::min);
    if best == f64::MAX {
        return Err("No boundary found to extend to");
    }
    let np = Point { x: origin.x + dir.0 * best, y: origin.y + dir.1 * best };
    let mut new_pts = pts.to_vec();
    if from_end {
        new_pts[n - 1] = np;
    } else {
        new_pts[0] = np;
    }
    Ok((
        ElementData::Polyline { points: new_pts, closed: false },
        ElementData::Line { start: origin, end: np },
    ))
}

/// ชิ้นที่เหลือหลังตัดช่วง p1 → p2 ออก (Break)
fn break_data(data: &ElementData, p1: Point, p2: Point) -> Result<Vec<ElementData>, &'static str> {
    const SAME: &str = "Pick two different points";
    match data {
        ElementData::Line { start, end } => {
            let (dx, dy) = (end.x - start.x, end.y - start.y);
            let len2 = dx * dx + dy * dy;
            if len2 < 1e-18 {
                return Err("Line has zero length");
            }
            let t = |p: Point| (((p.x - start.x) * dx + (p.y - start.y) * dy) / len2).clamp(0.0, 1.0);
            let (mut a, mut b) = (t(p1), t(p2));
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            if b - a < TRIM_EPS {
                return Err(SAME);
            }
            let at = |t: f64| Point { x: start.x + dx * t, y: start.y + dy * t };
            let mut out = Vec::new();
            if a > TRIM_EPS {
                out.push(ElementData::Line { start: *start, end: at(a) });
            }
            if b < 1.0 - TRIM_EPS {
                out.push(ElementData::Line { start: at(b), end: *end });
            }
            Ok(out)
        }
        ElementData::Circle { center, radius } => {
            let ang = |p: Point| (p.y - center.y).atan2(p.x - center.x).rem_euclid(TAU);
            let (a1, a2) = (ang(p1), ang(p2));
            if (a1 - a2).abs() < TRIM_EPS {
                return Err(SAME);
            }
            // ตัดทวนเข็มจาก a1 ไป a2 → เหลือส่วนโค้งจาก a2 ไป a1
            let mut end = a1;
            while end <= a2 {
                end += TAU;
            }
            Ok(vec![ElementData::Arc { center: *center, radius: *radius, start_angle: a2, end_angle: end }])
        }
        ElementData::Arc { center, radius, start_angle, end_angle } => {
            let span = end_angle - start_angle;
            let u_of = |p: Point| {
                let u = ((p.y - center.y).atan2(p.x - center.x) - start_angle).rem_euclid(TAU);
                if u > span {
                    if u - span > TAU - u { 0.0 } else { span }
                } else {
                    u
                }
            };
            let (mut a, mut b) = (u_of(p1), u_of(p2));
            if a > b {
                std::mem::swap(&mut a, &mut b);
            }
            if b - a < TRIM_EPS {
                return Err(SAME);
            }
            let mut out = Vec::new();
            if a > TRIM_EPS {
                out.push(ElementData::Arc {
                    center: *center,
                    radius: *radius,
                    start_angle: *start_angle,
                    end_angle: start_angle + a,
                });
            }
            if b < span - TRIM_EPS {
                out.push(ElementData::Arc {
                    center: *center,
                    radius: *radius,
                    start_angle: start_angle + b,
                    end_angle: *end_angle,
                });
            }
            Ok(out)
        }
        ElementData::Polyline { points, closed } => {
            let n = points.len();
            if n < 2 || (*closed && n < 3) {
                return Err("Polyline is too short to break");
            }
            let segs = poly_segs(n, *closed) as f64;
            let (mut s1, mut s2) = (
                poly_click_param(points, *closed, p1),
                poly_click_param(points, *closed, p2),
            );
            let mk = |v: Vec<Point>| ElementData::Polyline { points: v, closed: false };
            if !*closed {
                if s1 > s2 {
                    std::mem::swap(&mut s1, &mut s2);
                }
                if s2 - s1 < TRIM_EPS {
                    return Err(SAME);
                }
                let mut out = Vec::new();
                if s1 > TRIM_EPS {
                    let v = poly_sub(points, false, 0.0, s1);
                    if v.len() >= 2 {
                        out.push(mk(v));
                    }
                }
                if s2 < segs - TRIM_EPS {
                    let v = poly_sub(points, false, s2, segs);
                    if v.len() >= 2 {
                        out.push(mk(v));
                    }
                }
                Ok(out)
            } else {
                if (s1 - s2).abs() < TRIM_EPS {
                    return Err(SAME);
                }
                // ตัดช่วงจาก s1 ไป s2 ตามทิศของ polyline → เหลือจาก s2 วนไปถึง s1
                if s2 < s1 {
                    s2 += segs;
                }
                let v = poly_sub(points, true, s2, s1 + segs);
                if v.len() < 2 {
                    return Err(SAME);
                }
                Ok(vec![mk(v)])
            }
        }
        _ => Err("Break works on lines, circles, arcs and polylines"),
    }
}

// ---------------- Diagnostics ----------------

fn prim_distance(p: Point, prim: &Prim) -> f64 {
    match prim {
        Prim::Seg(a, b) => dist_point_seg(p, *a, *b),
        Prim::Circle { c, r } => ((p.x - c.x).hypot(p.y - c.y) - r).abs(),
        Prim::Arc { c, r, s, e } => {
            let a = (p.y - c.y).atan2(p.x - c.x);
            if angle_in_range(a, *s, *e) {
                ((p.x - c.x).hypot(p.y - c.y) - r).abs()
            } else {
                let (ps, pe) = (arc_point(*c, *r, *s), arc_point(*c, *r, *e));
                (p.x - ps.x).hypot(p.y - ps.y).min((p.x - pe.x).hypot(p.y - pe.y))
            }
        }
    }
}

fn sample_target(data: &ElementData) -> Vec<Point> {
    match data {
        ElementData::Line { start, end } => (0..=20)
            .map(|i| {
                let t = i as f64 / 20.0;
                Point { x: start.x + (end.x - start.x) * t, y: start.y + (end.y - start.y) * t }
            })
            .collect(),
        ElementData::Circle { center, radius } => (0..128)
            .map(|i| arc_point(*center, *radius, TAU * i as f64 / 128.0))
            .collect(),
        ElementData::Arc { center, radius, start_angle, end_angle } => (0..=64)
            .map(|i| arc_point(*center, *radius, start_angle + (end_angle - start_angle) * i as f64 / 64.0))
            .collect(),
        ElementData::Polyline { points, .. } => points.clone(),
        ElementData::Dimension { .. } | ElementData::Fill { .. } | ElementData::Hatch { .. } | ElementData::Text { .. } => vec![],
    }
}

/// จุดบนวัตถุที่อยู่ใกล้เส้นตัดอื่นที่สุด คืน (ระยะ, จุดนั้น) — ใช้ดีบักตอน "No cutting edge"
fn closest_approach(data: &ElementData, cutters: &[Prim]) -> Option<(f64, Point)> {
    let mut best: Option<(f64, Point)> = None;
    for s in sample_target(data) {
        for c in cutters {
            let d = prim_distance(s, c);
            if best.map_or(true, |(b, _)| d < b) {
                best = Some((d, s));
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_zoom_smoothness() {
        let mut engine = Engine::new();
        engine.camera.target_scale = 2.0;

        println!("\n--- Terminal Independent Test ---");
        let mut simulated_time = 0u64;

        for i in 0..10 {
            simulated_time += 100_000; // จำลอง 0.1 วินาที
            engine.step(simulated_time);
            println!("Frame {}: Scale = {:.4}", i, engine.camera.scale);
        }

        // ตรวจสอบว่ามันขยับจริงไหม
        assert!(engine.camera.scale > 1.0);
        println!("--- Test Passed! Engine is UI-Independent ---\n");
    }
}
