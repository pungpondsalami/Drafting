// engine.rs - เป็นเครื่องยนต์สภาพต้นไม้อ่ะค่ะ

use crate::spatial::BoundingBox;
use crate::spatial::SpatialIndex;

#[allow(dead_code)]
pub const ZOOM_STEPS: [f64; 10] = [0.1, 0.25, 0.5, 0.75, 1.0, 1.5, 2.0, 3.0, 5.0, 10.0];
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
    Line {
        start: Point,
        end: Point,
    },
    Circle {
        center: Point,
        radius: f64,
    },
    Dimension {
        start: Point,
        end: Point,
        offset: f64,
    },
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
        }
    }

    pub fn cloned_with_offset(&self, id: u64, dx: f64, dy: f64) -> Self {
        let mut new_el = self.clone();
        new_el.id = id;
        new_el.is_selected = false; // ตัวก๊อปปี้ไม่ต้องติดสถานะเลือก

        match &mut new_el.data {
            ElementData::Line { start, end } => {
                start.x += dx;
                start.y += dy;
                end.x += dx;
                end.y += dy;
            }
            ElementData::Circle { center, .. } => {
                center.x += dx;
                center.y += dy;
            }
            ElementData::Dimension { start, end, .. } => {
                start.x += dx;
                start.y += dy;
                end.x += dx;
                end.y += dy;
            }
        }
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
    pub spatial_index: SpatialIndex<DrawingElement>,
    pub history: Vec<DrawingElement>,
    pub camera: Camera,
    pub active_tool: ToolMode,
    pub last_update_time: Option<u64>,
    pub id_counter: u64,
    pub array_settings: ArraySettings,
    pub show_array_ui: bool,
    pub apply_anim: ApplyAnimation,
    pub hovered_id: Option<u64>,
    pub move_snapshot: Option<Vec<DrawingElement>>,
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
            move_snapshot: None,
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

        // ถ้าไม่ได้ shift ค่อยเคลียร์
        if !add_to_selection {
            for el in &mut self.history {
                el.is_selected = false;
            }
        }

        if let Some(target) = hits.last() {
            let target_id = target.id;
            if let Some(el) = self.history.iter_mut().find(|e| e.id == target_id) {
                el.is_selected = true;
            }
        }

        self.spatial_index = SpatialIndex::new(8);
        for el in &self.history {
            let bounds = el.bounding_box();
            self.spatial_index.insert(el.clone(), bounds);
        }
    }

    pub fn deselect_all(&mut self) {
        for el in &mut self.history {
            el.is_selected = false;
        }
        self.spatial_index = SpatialIndex::new(8);
        for el in &self.history {
            let bounds = el.bounding_box();
            self.spatial_index.insert(el.clone(), bounds);
        }
    }

    pub fn snapshot_for_undo(&mut self) {
        // บันทึก state ปัจจุบันก่อน move
        self.move_snapshot = Some(self.history.clone());
    }

    pub fn undo_last_action(&mut self) {
        // ถ้ามี move snapshot ให้ restore ก่อน
        if let Some(snapshot) = self.move_snapshot.take() {
            self.history = snapshot;
            self.spatial_index = SpatialIndex::new(8);
            for el in &self.history {
                let bounds = el.bounding_box();
                self.spatial_index.insert(el.clone(), bounds);
            }
            return;
        }
        // ไม่มี snapshot ค่อย pop element เหมือนเดิม
        if self.history.pop().is_some() {
            self.spatial_index = SpatialIndex::new(8);
            for el in &self.history {
                let bounds = el.bounding_box();
                self.spatial_index.insert(el.clone(), bounds);
            }
        }
    }

    pub fn move_selected(&mut self, dx: f64, dy: f64) {
        let selected_ids: Vec<u64> = self
            .history
            .iter()
            .filter(|e| e.is_selected)
            .map(|e| e.id)
            .collect();

        for el in &mut self.history {
            if !selected_ids.contains(&el.id) {
                continue;
            }
            match &mut el.data {
                ElementData::Line { start, end } => {
                    start.x += dx;
                    start.y += dy;
                    end.x += dx;
                    end.y += dy;
                }
                ElementData::Circle { center, .. } => {
                    center.x += dx;
                    center.y += dy;
                }
                ElementData::Dimension { start, end, .. } => {
                    start.x += dx;
                    start.y += dy;
                    end.x += dx;
                    end.y += dy;
                }
            }
        }

        self.spatial_index = SpatialIndex::new(8);
        for el in &self.history {
            let bounds = el.bounding_box();
            self.spatial_index.insert(el.clone(), bounds);
        }
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
            .map(|e| match &e.data {
                ElementData::Line { start, .. } => (start.x, start.y),
                ElementData::Circle { center, .. } => (center.x, center.y),
                ElementData::Dimension { start, .. } => (start.x, start.y),
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
        self.hovered_id = hits.last().map(|e| e.id);
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

        // ถ้าไม่ shift ค่อยเคลียร์
        if !add_to_selection {
            for el in &mut self.history {
                el.is_selected = false;
            }
        }

        let mut hits = Vec::new();
        self.spatial_index.query(&sel_box, &mut hits);
        let hit_ids: Vec<u64> = hits.iter().map(|e| e.id).collect();

        for el in &mut self.history {
            if !hit_ids.contains(&el.id) {
                continue;
            }
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

        self.spatial_index = SpatialIndex::new(8);
        for el in &self.history {
            let bounds = el.bounding_box();
            self.spatial_index.insert(el.clone(), bounds);
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
        self.history.push(element.clone()); // เก็บประวัติไว้
        self.spatial_index.insert(element, bounds);
    }

    pub fn get_visible_elements(&self, viewport_bounds: BoundingBox) -> Vec<&DrawingElement> {
        let mut visible = Vec::new();
        self.spatial_index.query(&viewport_bounds, &mut visible);
        visible
    }

    pub fn get_all_elements(&self) -> Vec<&DrawingElement> {
        self.spatial_index
            .get_all_elements()
            .into_iter()
            .map(|(_bounds, data)| data)
            .collect()
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
            }
        }
        commands
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
