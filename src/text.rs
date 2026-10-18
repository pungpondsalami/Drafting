// text.rs - ข้อความ (TEXT / MTEXT): รูปทรงโดยประมาณสำหรับ select/snap + แปลงจาก/ไป DXF
//
// engine ไม่รู้จักฟอนต์ จึงประมาณกล่องข้อความจากจำนวนตัวอักษร
// (ตัววาดจริงใช้ Pango ใน renderer.rs ซึ่งอาจต่างจากกล่องนี้นิดหน่อย)

use crate::engine::{ElementData, Point};
use crate::spatial::BoundingBox;
use acadrust::entities::{AttachmentPoint, MText, Text, TextHorizontalAlignment, TextVerticalAlignment};
use acadrust::{EntityType, Vector3};

/// ความสูงตัวอักษรใน CAD = ความสูงตัวพิมพ์ใหญ่ → ขนาด em ของฟอนต์ใหญ่กว่านั้น
pub const EM_PER_HEIGHT: f64 = 1.0 / 0.72;
/// ความสูงหนึ่งบรรทัด (หน่วย em)
pub const LINE_EM: f64 = 1.2;
/// ส่วนที่ต่ำกว่า baseline (หน่วย em)
pub const DESCENT_EM: f64 = 0.25;
/// ความกว้างเฉลี่ยต่อตัวอักษร (หน่วย em)
pub const CHAR_EM: f64 = 0.55;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VAlign {
    Top,
    Middle,
    Bottom,
    Baseline,
}

/// (กว้าง, สูง) โดยประมาณของกล่องข้อความ หน่วยโลก; width > 0 = ตัดบรรทัดที่ความกว้างนี้
pub fn estimate_size(text: &str, height: f64, width: f64) -> (f64, f64) {
    let em = height * EM_PER_HEIGHT;
    let mut lines = 0usize;
    let mut max_chars = 0usize;
    for line in text.split('\n') {
        let n = line.chars().count();
        max_chars = max_chars.max(n);
        if width > 0.0 {
            let per_line = (width / (CHAR_EM * em)).floor().max(1.0) as usize;
            lines += n.div_ceil(per_line).max(1);
        } else {
            lines += 1;
        }
    }
    let w = if width > 0.0 { width } else { max_chars as f64 * CHAR_EM * em };
    (w, lines.max(1) as f64 * LINE_EM * em)
}

/// กล่องในพิกัดท้องถิ่น (จุดยึดอยู่ที่ 0,0 แกน y ชี้ขึ้น): (x0, y0, x1, y1)
pub fn local_rect(
    text: &str,
    height: f64,
    halign: HAlign,
    valign: VAlign,
    width: f64,
) -> (f64, f64, f64, f64) {
    let (w, h) = estimate_size(text, height, width);
    let em = height * EM_PER_HEIGHT;
    let x0 = match halign {
        HAlign::Left => 0.0,
        HAlign::Center => -w / 2.0,
        HAlign::Right => -w,
    };
    let y_top = match valign {
        VAlign::Top => 0.0,
        VAlign::Middle => h / 2.0,
        VAlign::Bottom => h,
        VAlign::Baseline => (LINE_EM - DESCENT_EM) * em,
    };
    (x0, y_top - h, x0 + w, y_top)
}

fn to_world(pos: Point, rot: f64, lx: f64, ly: f64) -> Point {
    let (s, c) = rot.sin_cos();
    Point { x: pos.x + lx * c - ly * s, y: pos.y + lx * s + ly * c }
}

fn to_local(pos: Point, rot: f64, p: Point) -> (f64, f64) {
    let (s, c) = rot.sin_cos();
    let (dx, dy) = (p.x - pos.x, p.y - pos.y);
    (dx * c + dy * s, -dx * s + dy * c)
}

/// มุมทั้งสี่ของกล่อง (หมุนแล้ว) ในพิกัดโลก ไล่ตามเข็ม/ทวนเข็มต่อเนื่องกัน
#[allow(clippy::too_many_arguments)]
pub fn corners(
    pos: Point,
    height: f64,
    rotation: f64,
    text: &str,
    halign: HAlign,
    valign: VAlign,
    width: f64,
) -> [Point; 4] {
    let (x0, y0, x1, y1) = local_rect(text, height, halign, valign, width);
    [
        to_world(pos, rotation, x0, y0),
        to_world(pos, rotation, x1, y0),
        to_world(pos, rotation, x1, y1),
        to_world(pos, rotation, x0, y1),
    ]
}

#[allow(clippy::too_many_arguments)]
pub fn bbox(
    pos: Point,
    height: f64,
    rotation: f64,
    text: &str,
    halign: HAlign,
    valign: VAlign,
    width: f64,
) -> BoundingBox {
    let c = corners(pos, height, rotation, text, halign, valign, width);
    let mut b = BoundingBox { min_x: f64::MAX, min_y: f64::MAX, max_x: f64::MIN, max_y: f64::MIN };
    for p in c {
        b.min_x = b.min_x.min(p.x);
        b.min_y = b.min_y.min(p.y);
        b.max_x = b.max_x.max(p.x);
        b.max_y = b.max_y.max(p.y);
    }
    b
}

/// ระยะจากจุดถึงขอบกล่อง (ข้างในก็วัดถึงขอบที่ใกล้ที่สุด เหมือน Fill)
#[allow(clippy::too_many_arguments)]
pub fn edge_distance(
    p: Point,
    pos: Point,
    height: f64,
    rotation: f64,
    text: &str,
    halign: HAlign,
    valign: VAlign,
    width: f64,
) -> f64 {
    let (x0, y0, x1, y1) = local_rect(text, height, halign, valign, width);
    let (lx, ly) = to_local(pos, rotation, p);
    let dx = (x0 - lx).max(lx - x1).max(0.0);
    let dy = (y0 - ly).max(ly - y1).max(0.0);
    if dx > 0.0 || dy > 0.0 {
        dx.hypot(dy)
    } else {
        (lx - x0).min(x1 - lx).min(ly - y0).min(y1 - ly)
    }
}

#[allow(clippy::too_many_arguments)]
pub fn contains(
    p: Point,
    pos: Point,
    height: f64,
    rotation: f64,
    text: &str,
    halign: HAlign,
    valign: VAlign,
    width: f64,
) -> bool {
    let (x0, y0, x1, y1) = local_rect(text, height, halign, valign, width);
    let (lx, ly) = to_local(pos, rotation, p);
    lx >= x0 && lx <= x1 && ly >= y0 && ly <= y1
}

/// ตัวอักษรควบคุมของ DXF → ข้อความล้วน (TEXT: แค่ %%c %%d %%p ฯลฯ)
pub fn plain_from_text(s: &str) -> String {
    acadrust::entities::mtext_format::parser::parse_plain_text(s).to_plain_text()
}

/// MTEXT: ตัด format code (\f \H \C \P ...) เหลือข้อความ, \P → ขึ้นบรรทัดใหม่
pub fn plain_from_mtext(s: &str) -> String {
    acadrust::entities::mtext_format::parser::parse_mtext(s, true).to_plain_text()
}

// ---------------- DXF → engine ----------------

pub fn from_dxf_text(t: &Text) -> Option<ElementData> {
    let text = plain_from_text(&t.value);
    if text.trim().is_empty() || !(t.height > 0.0) {
        return None;
    }
    use TextHorizontalAlignment as H;
    use TextVerticalAlignment as V;
    let (halign, valign, uses_align_pt) = match (t.horizontal_alignment, t.vertical_alignment) {
        // Aligned / Fit ยืดข้อความระหว่างสองจุด — ประมาณด้วยชิดซ้ายที่จุดแรก
        (H::Aligned | H::Fit, _) => (HAlign::Left, VAlign::Baseline, false),
        (H::Middle, _) => (HAlign::Center, VAlign::Middle, true),
        (h, v) => {
            let ha = match h {
                H::Center => HAlign::Center,
                H::Right => HAlign::Right,
                _ => HAlign::Left,
            };
            let va = match v {
                V::Bottom => VAlign::Bottom,
                V::Middle => VAlign::Middle,
                V::Top => VAlign::Top,
                V::Baseline => VAlign::Baseline,
            };
            (ha, va, !(ha == HAlign::Left && va == VAlign::Baseline))
        }
    };
    let p = if uses_align_pt { t.alignment_point.unwrap_or(t.insertion_point) } else { t.insertion_point };
    Some(ElementData::Text {
        pos: Point { x: p.x, y: p.y },
        height: t.height,
        rotation: t.rotation,
        text,
        halign,
        valign,
        width: 0.0,
    })
}

pub fn from_dxf_mtext(m: &MText) -> Option<ElementData> {
    let text = plain_from_mtext(&m.value);
    if text.trim().is_empty() || !(m.height > 0.0) {
        return None;
    }
    use AttachmentPoint as A;
    let (valign, halign) = match m.attachment_point {
        A::TopLeft => (VAlign::Top, HAlign::Left),
        A::TopCenter => (VAlign::Top, HAlign::Center),
        A::TopRight => (VAlign::Top, HAlign::Right),
        A::MiddleLeft => (VAlign::Middle, HAlign::Left),
        A::MiddleCenter => (VAlign::Middle, HAlign::Center),
        A::MiddleRight => (VAlign::Middle, HAlign::Right),
        A::BottomLeft => (VAlign::Bottom, HAlign::Left),
        A::BottomCenter => (VAlign::Bottom, HAlign::Center),
        A::BottomRight => (VAlign::Bottom, HAlign::Right),
    };
    Some(ElementData::Text {
        pos: Point { x: m.insertion_point.x, y: m.insertion_point.y },
        height: m.height,
        rotation: m.rotation,
        text,
        halign,
        valign,
        width: m.rectangle_width.max(0.0),
    })
}

// ---------------- engine → DXF ----------------

/// ใน MTEXT ต้อง escape \ { } และใช้ \P แทนการขึ้นบรรทัดใหม่
fn mtext_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 8);
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '{' => out.push_str("\\{"),
            '}' => out.push_str("\\}"),
            '\r' => {}
            '\n' => out.push_str("\\P"),
            c => out.push(c),
        }
    }
    out
}

#[allow(clippy::too_many_arguments)]
pub fn to_dxf_entity(
    pos: Point,
    height: f64,
    rotation: f64,
    text: &str,
    halign: HAlign,
    valign: VAlign,
    width: f64,
) -> EntityType {
    let at = Vector3::new(pos.x, pos.y, 0.0);
    if text.contains('\n') || width > 0.0 {
        let mut m = MText::new();
        m.value = mtext_escape(text);
        m.insertion_point = at;
        m.height = height;
        m.rotation = rotation;
        m.rectangle_width = width;
        use AttachmentPoint as A;
        // MTEXT ไม่มี baseline → ใช้แถวล่างแทน
        m.attachment_point = match (valign, halign) {
            (VAlign::Top, HAlign::Left) => A::TopLeft,
            (VAlign::Top, HAlign::Center) => A::TopCenter,
            (VAlign::Top, HAlign::Right) => A::TopRight,
            (VAlign::Middle, HAlign::Left) => A::MiddleLeft,
            (VAlign::Middle, HAlign::Center) => A::MiddleCenter,
            (VAlign::Middle, HAlign::Right) => A::MiddleRight,
            (_, HAlign::Left) => A::BottomLeft,
            (_, HAlign::Center) => A::BottomCenter,
            (_, HAlign::Right) => A::BottomRight,
        };
        return EntityType::MText(Box::new(m));
    }
    let mut t = Text::with_value(text, at);
    t.height = height;
    t.rotation = rotation;
    use TextHorizontalAlignment as H;
    use TextVerticalAlignment as V;
    if halign == HAlign::Center && valign == VAlign::Middle {
        t.horizontal_alignment = H::Middle;
        t.vertical_alignment = V::Baseline;
        t.alignment_point = Some(at);
    } else {
        t.horizontal_alignment = match halign {
            HAlign::Left => H::Left,
            HAlign::Center => H::Center,
            HAlign::Right => H::Right,
        };
        t.vertical_alignment = match valign {
            VAlign::Top => V::Top,
            VAlign::Middle => V::Middle,
            VAlign::Bottom => V::Bottom,
            VAlign::Baseline => V::Baseline,
        };
        if !(halign == HAlign::Left && valign == VAlign::Baseline) {
            t.alignment_point = Some(at);
        }
    }
    EntityType::Text(t)
}
