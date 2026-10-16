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
    Offset,
    Trim,
    Extend,
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
    pub data: ElementData,
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

pub type Rgb = (u8, u8, u8);

#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub color: Rgb,
    pub visible: bool,
    pub locked: bool,
}

impl Layer {
    pub fn new(name: &str, color: Rgb) -> Self {
        Self { name: name.to_string(), color, visible: true, locked: false }
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
        self.target_scale = (self.target_scale * 1.2).min(50.0);
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
        self.target_scale *= factor;
        self.target_scale = self.target_scale.clamp(0.1, 10.0);
        let center_x = screen_size.0 / 2.0;
        let center_y = screen_size.1 / 2.0;
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
    pub layers: Vec<Layer>,
    pub current_layer: usize,
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
            layers: vec![Layer::new("0", (255, 255, 255))],
            current_layer: 0,
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
            let d = self.history[i].data.distance_to(p);
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
        self.hovered_id = self.pick_at(world_x, world_y);
    }

    pub fn offset_pick(&mut self, wx: f64, wy: f64) -> Option<u64> {
        let id = self.pick_at(wx, wy)?;
        let i = self.index_of(id)?;
        if matches!(self.history[i].data, ElementData::Dimension { .. }) {
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

    pub fn offset_apply(&mut self, id: u64, side: Point, dist: f64) -> Result<(), &'static str> {
        let i = self.index_of(id).ok_or("Object no longer exists")?;
        let data = offset_data(&self.history[i].data, side, dist)?;
        let (layer, color) = (self.history[i].layer, self.history[i].color);
        self.checkpoint();
        let id = self.get_next_id();
        self.add_element(DrawingElement { id, is_selected: false, layer, color, data });
        Ok(())
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
        let (layer, color) = (self.history[i].layer, self.history[i].color);
        self.checkpoint();
        self.history.remove(i);
        for data in pieces {
            let nid = self.get_next_id();
            self.history.push(DrawingElement { id: nid, is_selected: false, layer, color, data });
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
        for el in &mut self.history {
            el.is_selected = false;
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

    pub fn checkpoint(&mut self) {
        self.dirty = true;
        self.undo_stack.push(self.history.clone());
        if self.undo_stack.len() > Self::MAX_UNDO {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
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
        self.add_element(DrawingElement { id, is_selected: false, layer, color: None, data });
        id
    }

    pub fn push_imported(&mut self, data: ElementData, layer: usize, color: Option<Rgb>) {
        let id = self.get_next_id();
        self.add_element(DrawingElement { id, is_selected: false, layer, color, data });
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

    pub fn set_current_layer(&mut self, i: usize) {
        if i < self.layers.len() {
            self.current_layer = i;
        }
    }

    fn drop_selection_on_layer(&mut self, layer: usize) {
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

    pub fn get_visible_elements(&self, viewport_bounds: BoundingBox) -> Vec<&DrawingElement> {
        let mut hits = Vec::new();
        self.spatial_index.query(&viewport_bounds, &mut hits);
        let mut ids: Vec<u64> = hits.into_iter().copied().collect();
        ids.sort_unstable();
        ids.into_iter()
            .filter_map(|id| self.index_of(id).map(|i| &self.history[i]))
            .filter(|e| self.layers[e.layer].visible)
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
            .filter_map(|id| self.index_of(id))
            .filter(|&i| self.layer_visible(i))
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
        let p1 = self.camera.screen_to_world(0.0, 0.0);
        let p2 = self.camera.screen_to_world(screen_w, screen_h);
        let viewport_bounds = BoundingBox::from_points(p1.0, p1.1, p2.0, p2.1);
        let visible_elements = self.get_visible_elements(viewport_bounds);
        let mut commands = Vec::with_capacity(visible_elements.len());
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
        }
    }
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
        ElementData::Dimension { .. } => vec![],
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
