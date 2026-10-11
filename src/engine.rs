// engine.rs - เป็นเครื่องยนต์สภาพต้นไม้อ่ะค่ะ

use crate::spatial::BoundingBox;
use crate::spatial::SpatialIndex;
use std::path::PathBuf;
use std::f64::consts::{FRAC_PI_2, TAU};

pub const DEFAULT_DIMENSION_OFFSET: f64 = 30.0;

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
    pub id: u64,           // ต้องมี ID ประจำตัว
    pub is_selected: bool, // ไว้เช็คว่ากำลังถูกเลือกอยู่ไหม
    pub data: ElementData, // เก็บข้อมูลเฉพาะ
}

#[derive(Debug, Clone)]
pub enum ElementData {
    Line { start: Point, end: Point },
    Circle { center: Point, radius: f64 },
    Dimension { start: Point, end: Point, offset: f64 },
    /// มุมเป็นเรเดียน หมุนทวนเข็ม (CCW) จาก start ไป end และ end > start เสมอ
    Arc { center: Point, radius: f64, start_angle: f64, end_angle: f64 },
    Polyline { points: Vec<Point>, closed: bool },
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
        }
    }

    pub fn ref_point(&self) -> Point {
        match self {
            ElementData::Line { start, .. } | ElementData::Dimension { start, .. } => *start,
            ElementData::Circle { center, .. } | ElementData::Arc { center, .. } => *center,
            ElementData::Polyline { points, .. } => {
                points.first().copied().unwrap_or(Point { x: 0.0, y: 0.0 })
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

#[derive(Debug)]
pub struct Camera {
    pub scale: f64,
    pub target_scale: f64,
    pub zoom_index: usize,
    pub offset: (f64, f64),
    pub target_offset: (f64, f64),
}

#[derive(Debug, Clone)]
pub enum RenderCommand {
    Line {
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        is_selected: bool,
    },
    Circle {
        cx: f64,
        cy: f64,
        radius: f64,
        is_selected: bool,
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
    },
    Polyline {
        points: Vec<(f64, f64)>,
        closed: bool,
        is_selected: bool,
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
        }
    }

    pub fn update(&mut self, dt: f64) -> bool {
        let lerp_factor = (dt * 10.0).min(1.0);
        let mut changed = false;

        let scale_ratio = (self.scale / self.target_scale).ln().abs();
        if scale_ratio > 0.00001 {
            self.scale += (self.target_scale - self.scale) * lerp_factor;
            changed = true;
        } else {
            self.scale = self.target_scale;
        }

        let dx = self.target_offset.0 - self.offset.0;
        let dy = self.target_offset.1 - self.offset.1;

        if dx.abs() > 0.01 || dy.abs() > 0.01 {
            self.offset.0 += dx * lerp_factor;
            self.offset.1 += dy * lerp_factor;
            changed = true;
        } else {
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
        // 1. หาจุดกึ่งกลางของวัตถุทั้งหมดในโลก
        let center_world = ((min_p.x + max_p.x) / 2.0, (min_p.y + max_p.y) / 2.0);

        // 2. คำนวณ Scale โดยเผื่อ Margin 10% (0.9)
        let world_w = (max_p.x - min_p.x).max(1.0); // กันหารศูนย์
        let world_h = (max_p.y - min_p.y).max(1.0);

        let scale_x = (screen_size.0 * 0.9) / world_w;
        let scale_y = (screen_size.1 * 0.9) / world_h;
        self.target_scale = scale_x.min(scale_y);

        // 3. คำนวณ Offset ใหม่ให้ตรงกลางพอดีเป๊ะ
        self.target_offset.0 = (screen_size.0 / 2.0) - (center_world.0 * self.target_scale);

        // สำหรับ Y-Up: Center Screen + (World Y * Scale)
        self.target_offset.1 = (screen_size.1 / 2.0) + (center_world.1 * self.target_scale);
    }

    pub fn zoom_in_at(&mut self, screen_x: f64, screen_y: f64) {
        // ใช้ค่า target ในการคำนวณ เพื่อให้พิกัดโลกนิ่งที่สุด
        let (wx, wy) = self.screen_to_world_target(screen_x, screen_y);

        self.target_scale = (self.target_scale * 1.2).min(50.0);

        // ล็อกตำแหน่งเดิม (wx, wy) ให้อยู่ที่พิกัดจอเดิม (screen_x, screen_y)
        self.target_offset.0 = screen_x - (wx * self.target_scale);
        self.target_offset.1 = screen_y - (-wy * self.target_scale);
    }

    pub fn zoom_out_at(&mut self, screen_x: f64, screen_y: f64) {
        let (wx, wy) = self.screen_to_world_target(screen_x, screen_y);

        self.target_scale = (self.target_scale / 1.2).max(0.01);

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
        // 1. ตั้งสเกลเป้าหมายใหม่
        self.target_scale *= factor;
        self.target_scale = self.target_scale.clamp(0.1, 10.0);

        // 2. คำนวณจุดกึ่งกลางหน้าจอ
        let center_x = screen_size.0 / 2.0;
        let center_y = screen_size.1 / 2.0;

        // 3. ตั้งเป้าหมาย Offset ให้จุดพิกัดโลก (world_x, world_y) มาอยู่กลางจอพอดี
        self.target_offset = (
            center_x - (world_x * self.target_scale),
            center_y - (world_y * self.target_scale),
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
    pub undo_stack: Vec<Vec<DrawingElement>>,
    pub redo_stack: Vec<Vec<DrawingElement>>,
    pub file_path: Option<PathBuf>,
    pub dirty: bool,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            spatial_index: SpatialIndex::new(8), // หนึ่งกิ่งเก็บได้ 8 ชิ้น
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
        }
    }

    pub fn select_at(&mut self, world_x: f64, world_y: f64, add_to_selection: bool) {
        let tolerance = 5.0 / self.camera.scale;
        let query_box = BoundingBox {
            min_x: world_x - tolerance,
            min_y: world_y - tolerance,
            max_x: world_x + tolerance,
            max_y: world_y + tolerance,
        };

        let mut hits = Vec::new();
        self.spatial_index.query(&query_box, &mut hits);
        let top = hits.into_iter().copied().max(); // id มากสุด = วาดทีหลังสุด = อยู่บนสุด

        if !add_to_selection {
            for el in &mut self.history {
                el.is_selected = false;
            }
        }

        if let Some(i) = top.and_then(|id| self.index_of(id)) {
            self.history[i].is_selected = true;
        }
    }

    pub fn deselect_all(&mut self) {
        for el in &mut self.history {
            el.is_selected = false;
        }
    }

    pub fn hover_at(&mut self, world_x: f64, world_y: f64) {
        let tolerance = 5.0 / self.camera.scale;
        let query_box = BoundingBox {
            min_x: world_x - tolerance,
            min_y: world_y - tolerance,
            max_x: world_x + tolerance,
            max_y: world_y + tolerance,
        };
        let mut hits = Vec::new();
        self.spatial_index.query(&query_box, &mut hits);
        let top = hits.into_iter().copied().max();
        self.hovered_id = top;
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
            let el = &mut self.history[i];
            if crossing {
                el.is_selected = true;
            } else {
                let bb = el.bounding_box();
                if bb.min_x >= sel_box.min_x
                    && bb.max_x <= sel_box.max_x
                    && bb.min_y >= sel_box.min_y
                    && bb.max_y <= sel_box.max_y
                {
                    el.is_selected = true;
                }
            }
        }
    }

    const MAX_UNDO: usize = 100;

    fn index_of(&self, id: u64) -> Option<usize> {
        self.history.binary_search_by_key(&id, |e| e.id).ok()
    }

    fn rebuild_index(&mut self) {
        let mut idx = SpatialIndex::new(8);
        for el in &self.history {
            idx.insert(el.id, el.bounding_box());
        }
        self.spatial_index = idx;
    }

    /// เรียก "ก่อน" แก้ไขอะไรก็ตามที่ต้อง undo ได้ (วาด, ย้าย, array)
    pub fn checkpoint(&mut self) {
        self.dirty = true;
        self.undo_stack.push(self.history.clone());
        if self.undo_stack.len() > Self::MAX_UNDO {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear(); // มี action ใหม่ → redo เดิมใช้ไม่ได้แล้ว
    }

    pub fn undo_last_action(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            self.dirty = true;
            let current = std::mem::replace(&mut self.history, prev);
            self.redo_stack.push(current);
            self.hovered_id = None;
            self.rebuild_index();
        }
    }

    pub fn redo_last_action(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.dirty = true;
            let current = std::mem::replace(&mut self.history, next);
            self.undo_stack.push(current);
            self.hovered_id = None;
            self.rebuild_index();
        }
    }

    pub fn move_selected(&mut self, dx: f64, dy: f64) {
        for el in self.history.iter_mut().filter(|e| e.is_selected) {
            el.data.translate(dx, dy);
        }
        self.rebuild_index();
    }

    pub fn move_and_snap_selected(&mut self, dx: f64, dy: f64, snap_enabled: bool) {
        // ย้ายก่อน
        self.move_selected(dx, dy);

        if !snap_enabled {
            return;
        }

        // หา reference point (จุดแรกของ selected object)
        let ref_point = self
            .history
            .iter()
            .find(|e| e.is_selected)
            .map(|e| {
                let p = e.data.ref_point();
                (p.x, p.y)
            });

        if let Some((rx, ry)) = ref_point {
            // snap reference point
            let (snx, sny) = self.get_snapped_pos(rx, ry, snap_enabled);
            let snap_dx = snx - rx;
            let snap_dy = sny - ry;

            // ถ้า snap offset มีนัยสำคัญ ให้ move ทุกตัวตาม snap offset
            if snap_dx.abs() > 0.001 || snap_dy.abs() > 0.001 {
                self.move_selected(snap_dx, snap_dy);
            }
        }
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
        _dx: f64, // ไม่ใช้ค่าตรงจาก UI แล้ว
        _dy: f64, // ไม่ใช้ค่าตรงจาก UI แล้ว
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

                // วาดเงาโดยใช้ระยะห่างที่กำลังเล่น Animation อยู่
                let offset_x = c as f64 * anim_dx;
                let offset_y = r as f64 * anim_dy;

                for el in &selected_elements {
                    match &el.data {
                        ElementData::Line { start, end } => {
                            // คำนวณจุดศูนย์กลางของเส้นเพื่อให้มันขยายออกจากกลางเส้น
                            let mid_x = (start.x + end.x) / 2.0;
                            let mid_y = (start.y + end.y) / 2.0;
                            let s = self.array_settings.anim_scale; // 0.0 -> 1.0

                            // ปรับตำแหน่งจุดเริ่มและจบให้ขยายออกจากจุดศูนย์กลางตาม anim_scale
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
                            });
                        }
                        ElementData::Circle { center, radius } => {
                            let c = self
                                .camera
                                .world_to_screen(center.x + offset_x, center.y + offset_y);
                            // คูณ anim_scale เข้าไปเพื่อให้มันมี animation ขยายตัว
                            let r_screen =
                                radius * self.camera.scale * self.array_settings.anim_scale;

                            previews.push(RenderCommand::Circle {
                                cx: c.0,
                                cy: c.1,
                                radius: r_screen,
                                is_selected: true,
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
                            });
                        }
                        ElementData::Dimension { .. } => {
                            // Skip dimension in array preview for now
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

        // เก็บ id ลำดับการโผล่ตาม row/col
        let reveal_order: Vec<u64> = new_elements.iter().map(|e| e.id).collect();
        let total = reveal_order.len() as i32;

        for el in new_elements {
            self.add_element(el);
        }

        // เริ่ม animation
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

    pub fn get_next_id(&mut self) -> u64 {
        self.id_counter += 1;
        self.id_counter
    }

    pub fn add_element(&mut self, element: DrawingElement) {
        let bounds = element.bounding_box();
        self.spatial_index.insert(element.id, bounds);
        self.history.push(element);
    }

    pub fn get_visible_elements(&self, viewport_bounds: BoundingBox) -> Vec<&DrawingElement> {
        let mut hits = Vec::new();
        self.spatial_index.query(&viewport_bounds, &mut hits);
        let mut ids: Vec<u64> = hits.into_iter().copied().collect();
        ids.sort_unstable(); // ลำดับการวาดคงที่ ไม่กะพริบ
        ids.into_iter()
            .filter_map(|id| self.index_of(id).map(|i| &self.history[i]))
            .collect()
    }

    pub fn get_all_elements(&self) -> Vec<&DrawingElement> {
        self.history.iter().collect()
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

        let camera_changed = self.camera.update(dt);
        let mut array_changed = false;
        let lerp_factor = (dt * 15.0).min(1.0);

        // apply animation
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

    pub fn zoom_to_fit(&mut self, screen_w: f64, screen_h: f64) {
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

        let snap_coord = |coord: f64, major: f64, minor: f64| {
            let nearest_major = (coord / major).round() * major;
            let dist_to_major = (coord - nearest_major).abs();
            if dist_to_major < (major * 0.3) {
                nearest_major
            } else {
                (coord / minor).round() * minor
            }
        };

        (
            snap_coord(wx, step_major, step_minor),
            snap_coord(wy, step_major, step_minor),
        )
    }

    /// หาจุดสำคัญของวัตถุที่ใกล้เมาส์ที่สุด (ระยะ 12px บนจอ)
    pub fn find_object_snap(
        &self,
        wx: f64,
        wy: f64,
        settings: &SnapSettings,
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
            .filter_map(|id| self.index_of(id).map(|i| &self.history[i].data))
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

        // จุดตัด: ตรวจเฉพาะคู่ของวัตถุที่อยู่ใกล้เมาส์ (จำกัด 32 ชิ้นกัน O(n²) บาน)
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
        if !enabled {
            return (wx, wy, None);
        }
        if let Some((p, kind)) = self.find_object_snap(wx, wy, settings) {
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
        // ใช้ screen_to_world หา 2 มุมของจอ
        let p1 = self.camera.screen_to_world(0.0, 0.0);
        let p2 = self.camera.screen_to_world(screen_w, screen_h);

        // BoundingBox::from_points จะหา min/max ให้เองจาก 2 จุดนี้
        let viewport_bounds = BoundingBox::from_points(p1.0, p1.1, p2.0, p2.1);

        // 2. Query หาเฉพาะสิ่งที่ต้องวาด
        let visible_elements = self.get_visible_elements(viewport_bounds);
        let mut commands = Vec::with_capacity(visible_elements.len());

        for el in visible_elements {
            let mut draw_scale = 1.0f64; // scale ปกติ

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
                    } // ยังไม่ถึงเวลา

                    // bounce scale: 0 -> 1.2 -> 1.0
                    draw_scale = if progress < 0.6 {
                        progress / 0.6 * 1.2 // ขยายขึ้น
                    } else {
                        1.2 - (progress - 0.6) / 0.4 * 0.2 // หดกลับ
                    };
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
                    commands.push(RenderCommand::Line {
                        x1: s.0,
                        y1: s.1,
                        x2: e.0,
                        y2: e.1,
                        is_selected: el.is_selected,
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
                    });
                }
            }
        }
        commands
    }
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
        _ => vec![], // Dimension ไม่นับ
    }
}

pub fn angle_in_range(a: f64, start: f64, end: f64) -> bool {
    (a - start).rem_euclid(TAU) <= (end - start) + 1e-9
}

pub fn arc_point(c: Point, r: f64, a: f64) -> Point {
    Point { x: c.x + r * a.cos(), y: c.y + r * a.sin() }
}

/// คืน (center, radius, start_angle, end_angle) โดย end > start และเส้นโค้งผ่านทั้งสามจุดตามลำดับ
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

/// ชิ้นส่วนพื้นฐานสำหรับคำนวณจุดตัด
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
        ElementData::Dimension { .. } => vec![],
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
