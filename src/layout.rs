//! Layout (paper space): ตรรกะล้วน ๆ ของกระดาษ + Viewport (ไม่พึ่ง GTK)
//!
//! หน่วย: กระดาษเป็น มม. แกน y ชี้ลง origin มุมซ้ายบน; model เป็น มม. แกน y ชี้ขึ้น
//! `n` = ตัวส่วนสเกล 1:n (n=100 → 1:100, n=0.5 → 2:1)

/// สเกลมาตรฐานที่เลือกได้
pub const STD_SCALES: [f64; 17] = [
    0.1, 0.2, 0.5, 1.0, 2.0, 5.0, 10.0, 20.0, 25.0, 50.0, 75.0, 100.0, 150.0, 200.0, 250.0, 500.0,
    1000.0,
];

pub const MIN_VP: f64 = 5.0;

fn num(v: f64) -> String {
    if (v - v.round()).abs() < 1e-9 {
        format!("{}", v.round() as i64)
    } else {
        format!("{}", v)
    }
}

pub fn scale_label(n: f64) -> String {
    if n >= 1.0 {
        format!("1:{}", num(n))
    } else {
        format!("{}:1", num(1.0 / n))
    }
}

/// ตำแหน่งของ n ในรายการสเกลมาตรฐาน (ถ้าเป็นสเกลอื่น = None)
pub fn scale_index(n: f64) -> Option<usize> {
    STD_SCALES.iter().position(|s| (s - n).abs() <= 1e-9 * s.max(1.0))
}

/// สเกลมาตรฐานที่เล็กที่สุดที่ยัดพื้นที่ view_w × view_h (มม. ใน model) ลงกรอบ vp_w × vp_h (มม. บนกระดาษ) ได้
pub fn fit_scale(view_w: f64, view_h: f64, vp_w: f64, vp_h: f64) -> f64 {
    let need = (view_w / vp_w.max(1e-9)).max(view_h / vp_h.max(1e-9));
    STD_SCALES
        .iter()
        .copied()
        .find(|&s| s >= need * (1.0 - 1e-9))
        .unwrap_or(STD_SCALES[STD_SCALES.len() - 1])
}

/// ขยับสเกลหนึ่งขั้น: dir > 0 = ซูมออก (n มากขึ้น), dir < 0 = ซูมเข้า
pub fn step_scale(n: f64, dir: i32) -> f64 {
    let last = STD_SCALES.len() - 1;
    let i = match scale_index(n) {
        Some(i) => {
            if dir > 0 { (i + 1).min(last) } else { i.saturating_sub(1) }
        }
        None => {
            if dir > 0 {
                STD_SCALES.iter().position(|&s| s > n).unwrap_or(last)
            } else {
                STD_SCALES.iter().rposition(|&s| s < n).unwrap_or(0)
            }
        }
    };
    STD_SCALES[i]
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    /// มุมซ้ายบนบนกระดาษ (มม.)
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    /// จุดใน model ที่อยู่กลางกรอบ
    pub cx: f64,
    pub cy: f64,
    /// สเกล 1:n
    pub n: f64,
}

impl Viewport {
    pub fn contains(&self, px: f64, py: f64) -> bool {
        px >= self.x && px <= self.x + self.w && py >= self.y && py <= self.y + self.h
    }

    /// จับมุมขวาล่าง (ย่อ/ขยาย) ได้ไหม; tol เป็น มม.
    pub fn corner_hit(&self, px: f64, py: f64, tol: f64) -> bool {
        (px - (self.x + self.w)).abs() <= tol && (py - (self.y + self.h)).abs() <= tol
    }

    /// กล้องของตัววาด (หน่วย "px" = px_mm มม. บนกระดาษ) ในพิกัดท้องถิ่นของกรอบ: (scale, offset_x, offset_y)
    pub fn camera(&self, px_mm: f64) -> (f64, f64, f64) {
        let s = 1.0 / (self.n * px_mm);
        let (bw, bh) = (self.w / px_mm, self.h / px_mm);
        (s, bw / 2.0 - self.cx * s, bh / 2.0 + self.cy * s)
    }

    /// ลากเนื้อหาในกรอบไป (dx, dy) มม. บนกระดาษ
    pub fn pan_content(&mut self, dx: f64, dy: f64) {
        self.cx -= dx * self.n;
        self.cy += dy * self.n;
    }

    /// ลากมุมขวาล่างไปที่ (px, py) บนกระดาษ โดยเนื้อหาไม่ขยับบนกระดาษ
    pub fn resize_to(&mut self, px: f64, py: f64) {
        let nw = (px - self.x).max(MIN_VP);
        let nh = (py - self.y).max(MIN_VP);
        self.cx += (nw - self.w) / 2.0 * self.n;
        self.cy -= (nh - self.h) / 2.0 * self.n;
        self.w = nw;
        self.h = nh;
    }

    /// เปลี่ยนสเกลโดยจุดกลางกรอบยังเดิม
    pub fn set_scale(&mut self, n: f64) {
        self.n = n.max(1e-6);
    }

    /// ให้กรอบมองพื้นที่ model (x0,y0)-(x1,y1) ด้วยสเกลมาตรฐานที่ใกล้ที่สุดที่พอดี
    pub fn show_area(&mut self, x0: f64, y0: f64, x1: f64, y1: f64) {
        self.cx = (x0 + x1) / 2.0;
        self.cy = (y0 + y1) / 2.0;
        self.n = fit_scale((x1 - x0).abs(), (y1 - y0).abs(), self.w, self.h);
    }
}

/// มุมมองของหน้าจอที่ดูกระดาษ: z = พิกเซลหน้าจอต่อ 1 มม. กระดาษ
#[derive(Clone, Copy, Debug)]
pub struct PaperView {
    pub z: f64,
    pub ox: f64,
    pub oy: f64,
}

impl PaperView {
    pub fn fit(avail_w: f64, avail_h: f64, pw: f64, ph: f64) -> Self {
        let pad = 24.0;
        let z = ((avail_w - 2.0 * pad) / pw).min((avail_h - 2.0 * pad) / ph).max(0.05);
        PaperView { z, ox: (avail_w - pw * z) / 2.0, oy: (avail_h - ph * z) / 2.0 }
    }
    pub fn to_screen(&self, px: f64, py: f64) -> (f64, f64) {
        (self.ox + px * self.z, self.oy + py * self.z)
    }
    pub fn to_paper(&self, sx: f64, sy: f64) -> (f64, f64) {
        ((sx - self.ox) / self.z, (sy - self.oy) / self.z)
    }
    pub fn zoom_at(&mut self, sx: f64, sy: f64, factor: f64) {
        let (px, py) = self.to_paper(sx, sy);
        self.z = (self.z * factor).clamp(0.05, 200.0);
        self.ox = sx - px * self.z;
        self.oy = sy - py * self.z;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vp() -> Viewport {
        Viewport { x: 20.0, y: 30.0, w: 200.0, h: 100.0, cx: 500.0, cy: 300.0, n: 50.0 }
    }

    #[test]
    fn labels() {
        assert_eq!(scale_label(100.0), "1:100");
        assert_eq!(scale_label(1.0), "1:1");
        assert_eq!(scale_label(0.5), "2:1");
        assert_eq!(scale_label(0.1), "10:1");
        assert_eq!(scale_label(2.5), "1:2.5");
    }

    #[test]
    fn fit_picks_smallest_std() {
        // 10 ม. × 4 ม. ลงกรอบ 200×100 มม. → ต้อง >= 50 → 1:50 พอดี
        assert_eq!(fit_scale(10000.0, 4000.0, 200.0, 100.0), 50.0);
        // ล้นนิดเดียว → ขั้นถัดไป
        assert_eq!(fit_scale(10001.0, 4000.0, 200.0, 100.0), 75.0);
        // แกน y เป็นตัวกำหนด
        assert_eq!(fit_scale(100.0, 4000.0, 200.0, 100.0), 50.0);
        // ใหญ่เกินสเกลสูงสุด → ใช้สเกลสูงสุด
        assert_eq!(fit_scale(1e9, 1e9, 200.0, 100.0), 1000.0);
        // เล็กมาก → ขยายสูงสุด 10:1
        assert_eq!(fit_scale(1.0, 1.0, 200.0, 100.0), 0.1);
    }

    #[test]
    fn stepping() {
        assert_eq!(step_scale(50.0, 1), 75.0);
        assert_eq!(step_scale(50.0, -1), 25.0);
        assert_eq!(step_scale(1000.0, 1), 1000.0);
        assert_eq!(step_scale(0.1, -1), 0.1);
        // สเกลกำหนดเอง 60 → ขึ้น 75, ลง 50
        assert_eq!(step_scale(60.0, 1), 75.0);
        assert_eq!(step_scale(60.0, -1), 50.0);
        assert_eq!(scale_index(60.0), None);
        assert_eq!(scale_index(100.0), Some(11));
    }

    #[test]
    fn camera_centres_model_point() {
        let v = vp();
        let px_mm = 0.125;
        let (s, ox, oy) = v.camera(px_mm);
        // จุด (cx, cy) ต้องไปอยู่กลางกรอบ (หน่วย px)
        let sx = ox + v.cx * s;
        let sy = oy - v.cy * s;
        assert!((sx - v.w / px_mm / 2.0).abs() < 1e-9);
        assert!((sy - v.h / px_mm / 2.0).abs() < 1e-9);
        // 1 มม. model = 1/50 มม. กระดาษ = (1/50)/0.125 px
        assert!((s - 0.16).abs() < 1e-12);
        // model x ที่ห่างกัน 5000 มม. ห่างกันบนกระดาษ 100 มม.
        let a = ox + 0.0 * s;
        let b = ox + 5000.0 * s;
        assert!(((b - a) * px_mm - 100.0).abs() < 1e-9);
    }

    #[test]
    fn pan_content_moves_with_mouse() {
        let mut v = vp();
        let px_mm = 0.125;
        let before = {
            let (s, ox, oy) = v.camera(px_mm);
            (ox + 520.0 * s, oy - 310.0 * s)
        };
        v.pan_content(10.0, -4.0); // ลากเมาส์ขวา 10 มม. ขึ้น 4 มม.
        let after = {
            let (s, ox, oy) = v.camera(px_mm);
            (ox + 520.0 * s, oy - 310.0 * s)
        };
        assert!(((after.0 - before.0) * px_mm - 10.0).abs() < 1e-9);
        assert!(((after.1 - before.1) * px_mm - (-4.0)).abs() < 1e-9);
    }

    #[test]
    fn resize_keeps_content_still() {
        let mut v = vp();
        let px_mm = 0.125;
        let paper_pos = |v: &Viewport| {
            let (s, ox, oy) = v.camera(px_mm);
            (v.x + (ox + 480.0 * s) * px_mm, v.y + (oy - 280.0 * s) * px_mm)
        };
        let before = paper_pos(&v);
        v.resize_to(300.0, 200.0);
        assert_eq!((v.w, v.h), (280.0, 170.0));
        let after = paper_pos(&v);
        assert!((before.0 - after.0).abs() < 1e-9 && (before.1 - after.1).abs() < 1e-9);
        // ไม่ให้เล็กกว่าขั้นต่ำ
        v.resize_to(0.0, 0.0);
        assert_eq!((v.w, v.h), (MIN_VP, MIN_VP));
    }

    #[test]
    fn hit_tests() {
        let v = vp();
        assert!(v.contains(30.0, 40.0));
        assert!(!v.contains(10.0, 40.0));
        assert!(v.corner_hit(220.5, 130.5, 2.0));
        assert!(!v.corner_hit(120.0, 80.0, 2.0));
    }

    #[test]
    fn show_area_fits() {
        let mut v = vp();
        v.show_area(0.0, 0.0, 10000.0, 4000.0);
        assert_eq!((v.cx, v.cy, v.n), (5000.0, 2000.0, 50.0));
    }

    #[test]
    fn paper_view_roundtrip() {
        let mut pv = PaperView::fit(1000.0, 700.0, 420.0, 297.0);
        let (sx, sy) = pv.to_screen(100.0, 50.0);
        let (px, py) = pv.to_paper(sx, sy);
        assert!((px - 100.0).abs() < 1e-9 && (py - 50.0).abs() < 1e-9);
        pv.zoom_at(300.0, 200.0, 1.5);
        let (qx, qy) = pv.to_paper(300.0, 200.0);
        let (rx, ry) = PaperView::fit(1000.0, 700.0, 420.0, 297.0).to_paper(300.0, 200.0);
        assert!((qx - rx).abs() < 1e-9 && (qy - ry).abs() < 1e-9);
    }
}

// ---------------------------------------------------------------------------
// Layout ที่เก็บในแบบ (LayoutSpec) และการแปลงจาก/ไป DXF
// ---------------------------------------------------------------------------

/// ของที่วางบนกระดาษ (paper space) พิกัด มม. จากมุมซ้ายล่าง แกน y ขึ้น; group = ชิ้นที่เลือก/ย้ายด้วยกัน
#[derive(Clone, Debug)]
pub struct PaperItem {
    pub group: u32,
    pub item: crate::engine::ClipItem,
}

/// Layout หนึ่งแผ่น: ชื่อ ขนาดกระดาษ (มม. เท่าที่เห็น กว้าง × สูง) Viewport และของบนกระดาษ
#[derive(Clone, Debug)]
pub struct LayoutSpec {
    pub name: String,
    pub w: f64,
    pub h: f64,
    pub vps: Vec<Viewport>,
    pub items: Vec<PaperItem>,
}

impl LayoutSpec {
    pub fn new(name: impl Into<String>) -> Self {
        // A3 แนวนอน
        LayoutSpec { name: name.into(), w: 420.0, h: 297.0, vps: Vec::new(), items: Vec::new() }
    }
}

/// ชื่อ Layout ใหม่ที่ไม่ซ้ำ: Layout1, Layout2, ...
pub fn unique_name(existing: &[LayoutSpec], base: &str) -> String {
    let taken = |n: &str| existing.iter().any(|l| l.name.eq_ignore_ascii_case(n));
    if !base.is_empty() && !taken(base) {
        return base.to_string();
    }
    let mut i = 1;
    loop {
        let n = format!("Layout{}", i);
        if !taken(&n) {
            return n;
        }
        i += 1;
    }
}

/// เขียน Layout ลง DXF: กระดาษ (หน่วย มม.) + Viewport ที่เป็นมุมมองแปลนตั้งฉาก
/// พิกัดกระดาษใน DXF แกน y ชี้ขึ้นจากมุมซ้ายล่าง
pub fn write_layouts(doc: &mut acadrust::CadDocument, specs: &[LayoutSpec]) -> Result<(), String> {
    use acadrust::entities::Viewport as AcVp;
    use acadrust::{EntityType, Vector3};
    let mut next_id: i16 = 2;
    for spec in specs {
        // CadDocument::new() มี "Layout1" เปล่า ๆ อยู่แล้ว → ใช้ซ้ำแทนการเพิ่มซ้ำ
        let exists = doc.objects.values().any(|o| {
            matches!(o, acadrust::objects::ObjectType::Layout(l) if l.name == spec.name)
        });
        if !exists {
            doc.add_layout(&spec.name).map_err(|e| e.to_string())?;
        }
        let handle = doc
            .objects
            .values_mut()
            .find_map(|o| match o {
                acadrust::objects::ObjectType::Layout(l) if l.name == spec.name => Some(l),
                _ => None,
            })
            .ok_or_else(|| "layout vanished".to_string())?;
        handle.paper_width = spec.w;
        handle.paper_height = spec.h;
        handle.plot_paper_units = 1;
        handle.plot_rotation = 0;
        handle.paper_size = format!("User_({:.2}_x_{:.2}_MM)", spec.w, spec.h);
        handle.min_limits = (0.0, 0.0);
        handle.max_limits = (spec.w, spec.h);
        handle.min_extents = (0.0, 0.0, 0.0);
        handle.max_extents = (spec.w, spec.h, 0.0);

        for vp in &spec.vps {
            let mut v = AcVp::new();
            v.id = next_id;
            next_id += 1;
            v.center = Vector3::new(vp.x + vp.w / 2.0, spec.h - (vp.y + vp.h / 2.0), 0.0);
            v.width = vp.w;
            v.height = vp.h;
            // มุมมองแปลน: เป้าหมายที่จุดกลางกรอบ → view_center เป็นศูนย์ใน DCS
            v.view_target = Vector3::new(vp.cx, vp.cy, 0.0);
            v.view_center = Vector3::new(0.0, 0.0, 0.0);
            v.view_direction = Vector3::new(0.0, 0.0, 1.0);
            v.view_height = vp.h * vp.n;
            doc.add_entity_to_layout(EntityType::Viewport(Box::new(v)), &spec.name)
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

/// อ่าน Layout (ไม่รวม Model) พร้อม Viewport ที่ใช้ได้: เปิดอยู่ ไม่ใช่ viewport ใหญ่ id=1 และมองแปลน
pub fn read_layouts(doc: &acadrust::CadDocument) -> Vec<LayoutSpec> {
    use acadrust::EntityType;
    use acadrust::objects::ObjectType;
    let mut layouts: Vec<&acadrust::objects::Layout> = doc
        .objects
        .values()
        .filter_map(|o| match o {
            ObjectType::Layout(l) if !l.name.eq_ignore_ascii_case("Model") => Some(l),
            _ => None,
        })
        .collect();
    layouts.sort_by_key(|l| l.tab_order);
    let mut out = Vec::new();
    for l in layouts {
        let (mut w, mut h) = (l.paper_width, l.paper_height);
        if l.plot_rotation == 1 || l.plot_rotation == 3 {
            std::mem::swap(&mut w, &mut h);
        }
        let untouched = !(w > 1.0 && h > 1.0);
        if untouched {
            (w, h) = (420.0, 297.0);
        }
        let mut spec = LayoutSpec { name: l.name.clone(), w, h, vps: Vec::new(), items: Vec::new() };
        // viewport ของ layout: จากรายการใน layout และจากเอนทิตีที่เป็นของ block record ของมัน
        // (ไฟล์ DXF จาก AutoCAD มักมีแต่แบบหลัง)
        let mut found: Vec<&acadrust::entities::Viewport> = Vec::new();
        for h_ in &l.viewports {
            if let Some(EntityType::Viewport(v)) = doc.get_entity(*h_) {
                found.push(v);
            }
        }
        for e in doc.entities() {
            if let EntityType::Viewport(v) = e {
                if v.common.owner_handle == l.block_record
                    && !found.iter().any(|x| x.common.handle == v.common.handle)
                {
                    found.push(v);
                }
            }
        }
        for v in found {
            let plan = v.view_direction.x.abs() < 1e-9 && v.view_direction.y.abs() < 1e-9;
            if v.id <= 1 || !plan || v.width < MIN_VP || v.height < MIN_VP || v.view_height <= 0.0 {
                continue;
            }
            spec.vps.push(Viewport {
                x: v.center.x - v.width / 2.0,
                y: spec.h - (v.center.y + v.height / 2.0),
                w: v.width,
                h: v.height,
                cx: v.view_center.x + v.view_target.x,
                cy: v.view_center.y + v.view_target.y,
                n: v.view_height / v.height,
            });
        }
        // Layout1 เปล่าที่มากับไฟล์ใหม่ (ไม่มีขนาดกระดาษ ไม่มี viewport) ไม่นับ
        if untouched && spec.vps.is_empty() {
            continue;
        }
        out.push(spec);
    }
    out
}

#[cfg(test)]
mod dxf_tests {
    use super::*;

    fn sample() -> Vec<LayoutSpec> {
        let mut a = LayoutSpec::new("Plan");
        a.vps.push(Viewport { x: 20.0, y: 30.0, w: 200.0, h: 100.0, cx: 5000.0, cy: 2000.0, n: 50.0 });
        a.vps.push(Viewport { x: 240.0, y: 30.0, w: 150.0, h: 100.0, cx: 100.0, cy: -50.0, n: 0.5 });
        let mut b = LayoutSpec::new("Detail");
        b.w = 297.0;
        b.h = 210.0;
        vec![a, b]
    }

    #[test]
    fn roundtrip_through_dxf_text() {
        let mut doc = acadrust::CadDocument::new();
        write_layouts(&mut doc, &sample()).unwrap();
        let bytes = acadrust::DxfWriter::new(&doc).write_to_vec().unwrap();
        let text = String::from_utf8_lossy(&bytes).to_string();
        assert!(text.contains("VIEWPORT"));
        let doc2 = acadrust::DxfReader::from_reader(std::io::Cursor::new(bytes.clone())).unwrap().read().unwrap();
        let got = read_layouts(&doc2);
        let want = sample();
        assert_eq!(got.len(), 2, "{:?}", got);
        for (g, w) in got.iter().zip(&want) {
            assert_eq!(g.name, w.name);
            assert!((g.w - w.w).abs() < 1e-6 && (g.h - w.h).abs() < 1e-6, "{:?}", g);
            assert_eq!(g.vps.len(), w.vps.len(), "{:?}", g);
            for (a, b) in g.vps.iter().zip(&w.vps) {
                let close = |p: f64, q: f64| (p - q).abs() < 1e-6 * q.abs().max(1.0);
                assert!(
                    close(a.x, b.x) && close(a.y, b.y) && close(a.w, b.w) && close(a.h, b.h)
                        && close(a.cx, b.cx) && close(a.cy, b.cy) && close(a.n, b.n),
                    "{:?} vs {:?}", a, b
                );
            }
        }
    }

    #[test]
    fn plain_file_has_no_layouts() {
        let doc = acadrust::CadDocument::new();
        let bytes = acadrust::DxfWriter::new(&doc).write_to_vec().unwrap();
        let doc2 = acadrust::DxfReader::from_reader(std::io::Cursor::new(bytes)).unwrap().read().unwrap();
        assert!(read_layouts(&doc2).is_empty());
    }

    #[test]
    fn layout1_is_reused_and_empty_layout_survives() {
        let mut a = LayoutSpec::new("Layout1");
        a.vps.push(Viewport { x: 10.0, y: 10.0, w: 100.0, h: 80.0, cx: 0.0, cy: 0.0, n: 10.0 });
        let b = LayoutSpec::new("Empty");
        let mut doc = acadrust::CadDocument::new();
        write_layouts(&mut doc, &[a.clone(), b.clone()]).unwrap();
        let bytes = acadrust::DxfWriter::new(&doc).write_to_vec().unwrap();
        let doc2 = acadrust::DxfReader::from_reader(std::io::Cursor::new(bytes)).unwrap().read().unwrap();
        let got = read_layouts(&doc2);
        assert_eq!(got.len(), 2, "{:?}", got);
        assert_eq!(got[0].name, "Layout1");
        assert_eq!(got[0].vps.len(), 1);
        assert_eq!(got[1].name, "Empty");
    }

    #[test]
    fn unique_names() {
        let mut v = vec![LayoutSpec::new("Layout1")];
        assert_eq!(unique_name(&v, ""), "Layout2");
        assert_eq!(unique_name(&v, "layout1"), "Layout2");
        assert_eq!(unique_name(&v, "Sheet"), "Sheet");
        v.push(LayoutSpec::new("Layout2"));
        assert_eq!(unique_name(&v, ""), "Layout3");
    }
}
