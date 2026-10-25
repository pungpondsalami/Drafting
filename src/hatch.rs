// hatch.rs - Hatch (ลายเส้นในพื้นที่ปิด): หาขอบเขตจากจุดที่คลิก + สร้างเส้นลาย
//
// หาขอบเขตแบบ AutoCAD: ยิงรังสีจากจุดคลิกไปทางขวา เจอเส้นแรกแล้ว "เดินวนรอบพื้นที่" (face walk)
// โดยเลี้ยวซ้ายสุดทุกจุดตัด ได้ลูปปิดที่ล้อมจุดคลิก; ลูปที่วนตามเข็มที่เจอระหว่างทางคือ "เกาะ" (island)
// ส่วนประกอบทางเรขาคณิตทั้งหมดไม่รู้จัก GTK จึงทดสอบแยกได้

use crate::engine::{ElementData, Point, arc_point};
use crate::spatial::BoundingBox;
use std::collections::HashSet;
use std::f64::consts::{FRAC_PI_2, TAU};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HatchKind {
    /// เส้นขนานเส้นเดียว
    Lines,
    /// เส้นตาราง (สองทิศตั้งฉากกัน)
    Cross,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HatchSpec {
    pub kind: HatchKind,
    /// เรเดียน ทวนเข็มจากแกน X
    pub angle: f64,
    /// ระยะห่างระหว่างเส้น (หน่วยของแบบ)
    pub spacing: f64,
}

impl Default for HatchSpec {
    fn default() -> Self {
        Self { kind: HatchKind::Lines, angle: std::f64::consts::FRAC_PI_4, spacing: 5.0 }
    }
}

/// จำนวนเส้นลายสูงสุดต่อหนึ่ง hatch ต่อหนึ่งทิศ (เกินนี้ถือว่าถี่เกิน)
pub const MAX_PATTERN_LINES: usize = 20_000;

// ---------------------------------------------------------------- เส้นลาย

/// เส้นลายของ loops (even-odd: loop ซ้อนในเป็นรู) ถูกตัดให้อยู่ในกรอบ view ถ้ามี
/// None = ถี่เกินไป / spacing ไม่ถูกต้อง
pub fn pattern_segments(
    loops: &[Vec<Point>],
    spec: &HatchSpec,
    view: Option<&BoundingBox>,
) -> Option<Vec<(Point, Point)>> {
    if !(spec.spacing > 0.0) || !spec.spacing.is_finite() || !spec.angle.is_finite() {
        return None;
    }
    let mut out = Vec::new();
    scan(loops, spec.angle, spec.spacing, view, &mut out)?;
    if spec.kind == HatchKind::Cross {
        scan(loops, spec.angle + FRAC_PI_2, spec.spacing, view, &mut out)?;
    }
    Some(out)
}

fn scan(
    loops: &[Vec<Point>],
    ang: f64,
    spacing: f64,
    view: Option<&BoundingBox>,
    out: &mut Vec<(Point, Point)>,
) -> Option<()> {
    let (s, c) = ang.sin_cos();
    let (u, n) = ((c, s), (-s, c));
    let dot_n = |p: &Point| p.x * n.0 + p.y * n.1;
    let dot_u = |p: &Point| p.x * u.0 + p.y * u.1;

    let mut smin = f64::INFINITY;
    let mut smax = f64::NEG_INFINITY;
    for p in loops.iter().flatten() {
        let v = dot_n(p);
        smin = smin.min(v);
        smax = smax.max(v);
    }
    if !(smin <= smax) {
        return Some(());
    }
    let mut t_lo = f64::NEG_INFINITY;
    let mut t_hi = f64::INFINITY;
    if let Some(v) = view {
        let corners = [
            Point { x: v.min_x, y: v.min_y },
            Point { x: v.max_x, y: v.min_y },
            Point { x: v.max_x, y: v.max_y },
            Point { x: v.min_x, y: v.max_y },
        ];
        let (mut a0, mut a1) = (f64::INFINITY, f64::NEG_INFINITY);
        let (mut b0, mut b1) = (f64::INFINITY, f64::NEG_INFINITY);
        for p in &corners {
            a0 = a0.min(dot_n(p));
            a1 = a1.max(dot_n(p));
            b0 = b0.min(dot_u(p));
            b1 = b1.max(dot_u(p));
        }
        smin = smin.max(a0);
        smax = smax.min(a1);
        t_lo = b0;
        t_hi = b1;
    }
    let k0 = (smin / spacing).ceil();
    let k1 = (smax / spacing).floor();
    if !(k1 >= k0) {
        return Some(());
    }
    if k1 - k0 + 1.0 > MAX_PATTERN_LINES as f64 {
        return None;
    }
    let (k0, k1) = (k0 as i64, k1 as i64);

    let mut ts: Vec<f64> = Vec::new();
    for k in k0..=k1 {
        let sline = k as f64 * spacing;
        ts.clear();
        for l in loops {
            let m = l.len();
            if m < 2 {
                continue;
            }
            for i in 0..m {
                let (a, b) = (l[i], l[(i + 1) % m]);
                let (sa, sb) = (dot_n(&a) - sline, dot_n(&b) - sline);
                // กฎครึ่งเปิด: จุดที่แตะเส้นพอดีนับเป็นด้านบวก กันนับซ้ำที่มุม
                if (sa >= 0.0) != (sb >= 0.0) {
                    let f = sa / (sa - sb);
                    let p = Point { x: a.x + (b.x - a.x) * f, y: a.y + (b.y - a.y) * f };
                    ts.push(dot_u(&p));
                }
            }
        }
        ts.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        for pair in ts.chunks_exact(2) {
            let (t0, t1) = (pair[0].max(t_lo), pair[1].min(t_hi));
            if t1 - t0 > 1e-12 {
                let at = |t: f64| Point { x: n.0 * sline + u.0 * t, y: n.1 * sline + u.1 * t };
                out.push((at(t0), at(t1)));
            }
        }
    }
    Some(())
}

// ---------------------------------------------------------------- รูปร่างพื้นฐาน

pub fn signed_area(poly: &[Point]) -> f64 {
    let n = poly.len();
    let mut s = 0.0;
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        s += a.x * b.y - b.x * a.y;
    }
    s * 0.5
}

pub fn point_in_poly(p: Point, poly: &[Point]) -> bool {
    let n = poly.len();
    if n < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = n - 1;
    for i in 0..n {
        let (a, b) = (poly[i], poly[j]);
        if (a.y > p.y) != (b.y > p.y) && p.x < (b.x - a.x) * (p.y - a.y) / (b.y - a.y) + a.x {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn dist(a: Point, b: Point) -> f64 {
    (a.x - b.x).hypot(a.y - b.y)
}

fn seg_dist(p: Point, a: Point, b: Point) -> f64 {
    let (dx, dy) = (b.x - a.x, b.y - a.y);
    let l2 = dx * dx + dy * dy;
    let t = if l2 < 1e-30 { 0.0 } else { (((p.x - a.x) * dx + (p.y - a.y) * dy) / l2).clamp(0.0, 1.0) };
    dist(p, Point { x: a.x + dx * t, y: a.y + dy * t })
}

fn cross(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    ax * by - ay * bx
}

/// ลบจุดซ้ำติดกัน และ "หนาม" (ไปแล้วย้อนกลับทางเดิม)
pub fn simplify(poly: &[Point], eps: f64) -> Vec<Point> {
    let mut out: Vec<Point> = Vec::with_capacity(poly.len());
    for &p in poly {
        if out.last().is_some_and(|l| dist(*l, p) <= eps) {
            continue;
        }
        if out.len() >= 2 && dist(out[out.len() - 2], p) <= eps {
            out.pop();
            continue;
        }
        out.push(p);
    }
    loop {
        let n = out.len();
        if n >= 2 && dist(out[0], out[n - 1]) <= eps {
            out.pop();
        } else if n >= 3 && dist(out[1], out[n - 1]) <= eps {
            out.remove(0);
        } else if n >= 3 && dist(out[0], out[n - 2]) <= eps {
            out.pop();
        } else {
            break;
        }
    }
    out
}

// ---------------------------------------------------------------- เส้นของวัตถุ

#[derive(Debug, Clone, Copy)]
pub struct Seg {
    pub a: Point,
    pub b: Point,
    /// id ของวัตถุต้นทาง
    pub src: u64,
    /// ลำดับชิ้นภายในวัตถุ
    pub idx: u32,
}

impl Seg {
    fn key(&self) -> (u64, u32) {
        (self.src, self.idx)
    }
    pub fn bbox(&self) -> BoundingBox {
        BoundingBox::from_points(self.a.x, self.a.y, self.b.x, self.b.y)
    }
}

/// จำนวนท่อนต่อวงกลมเต็ม (วงกลม/ส่วนโค้งถูกแทนด้วยเส้นตรงสั้น ๆ)
pub const CIRCLE_STEPS: usize = 256;

pub fn circle_points(center: Point, r: f64) -> Vec<Point> {
    (0..CIRCLE_STEPS).map(|i| arc_point(center, r, TAU * i as f64 / CIRCLE_STEPS as f64)).collect()
}

/// แตกวัตถุเป็นเส้นตรงสั้น ๆ (เฉพาะชนิดที่เป็นขอบเขตได้)
pub fn element_segs(id: u64, d: &ElementData, out: &mut Vec<Seg>) {
    let mut n: u32 = 0;
    let mut push = |a: Point, b: Point, out: &mut Vec<Seg>| {
        if dist(a, b) > 1e-12 {
            out.push(Seg { a, b, src: id, idx: n });
            n += 1;
        }
    };
    match d {
        ElementData::Line { start, end } => push(*start, *end, out),
        ElementData::Polyline { points, closed } => {
            let n = points.len();
            let segs = if *closed { n } else { n.saturating_sub(1) };
            for i in 0..segs {
                push(points[i], points[(i + 1) % n], out);
            }
        }
        ElementData::Circle { center, radius } => {
            let pts = circle_points(*center, *radius);
            for i in 0..pts.len() {
                push(pts[i], pts[(i + 1) % pts.len()], out);
            }
        }
        ElementData::Arc { center, radius, start_angle, end_angle } => {
            let span = end_angle - start_angle;
            let steps = ((span.abs() / TAU * CIRCLE_STEPS as f64).ceil() as usize).max(4);
            let pt = |i: usize| arc_point(*center, *radius, start_angle + span * i as f64 / steps as f64);
            let mut prev = pt(0);
            for i in 1..=steps {
                let cur = pt(i);
                push(prev, cur, out);
                prev = cur;
            }
        }
        _ => {}
    }
}

fn expand(b: &BoundingBox, e: f64) -> BoundingBox {
    BoundingBox { min_x: b.min_x - e, min_y: b.min_y - e, max_x: b.max_x + e, max_y: b.max_y + e }
}

fn point_at(s: &Seg, t: f64) -> Point {
    if t <= 0.0 {
        s.a
    } else if t >= 1.0 {
        s.b
    } else {
        Point { x: s.a.x + (s.b.x - s.a.x) * t, y: s.a.y + (s.b.y - s.a.y) * t }
    }
}

fn param_of(s: &Seg, p: Point, eps: f64) -> f64 {
    if dist(p, s.a) <= eps {
        return 0.0;
    }
    if dist(p, s.b) <= eps {
        return 1.0;
    }
    let (dx, dy) = (s.b.x - s.a.x, s.b.y - s.a.y);
    (((p.x - s.a.x) * dx + (p.y - s.a.y) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0)
}

/// ค่าพารามิเตอร์ t (a→b, 0..1) ของทุกจุดที่เส้นอื่นแตะ/ตัดเส้นนี้ รวม 0 และ 1
fn node_params(s: &Seg, near: &[Seg], eps: f64) -> Vec<f64> {
    let (rx, ry) = (s.b.x - s.a.x, s.b.y - s.a.y);
    let len = rx.hypot(ry);
    let mut ts = vec![0.0, 1.0];
    if len < 1e-12 {
        return ts;
    }
    for t in near {
        if t.key() == s.key() {
            continue;
        }
        for p in [t.a, t.b] {
            let u = ((p.x - s.a.x) * rx + (p.y - s.a.y) * ry) / (len * len);
            if u > 0.0 && u < 1.0 && seg_dist(p, s.a, s.b) <= eps {
                ts.push(u);
            }
        }
        let (sx, sy) = (t.b.x - t.a.x, t.b.y - t.a.y);
        let slen = sx.hypot(sy);
        let den = cross(rx, ry, sx, sy);
        if slen > 1e-12 && den.abs() > 1e-12 * len * slen {
            let (wx, wy) = (t.a.x - s.a.x, t.a.y - s.a.y);
            let tt = cross(wx, wy, sx, sy) / den;
            let uu = cross(wx, wy, rx, ry) / den;
            let (tol_t, tol_u) = (eps / len, eps / slen);
            if tt > -tol_t && tt < 1.0 + tol_t && uu > -tol_u && uu < 1.0 + tol_u {
                ts.push(tt.clamp(0.0, 1.0));
            }
        }
    }
    ts.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let epst = eps / len;
    let mut out: Vec<f64> = Vec::with_capacity(ts.len());
    for t in ts {
        match out.last_mut() {
            Some(l) if t - *l <= epst => {
                if t >= 1.0 {
                    *l = 1.0; // ปลายสุดต้องคงเป็น 1 พอดี
                }
            }
            _ => out.push(t),
        }
    }
    out
}

// ---------------------------------------------------------------- หาพื้นที่ปิด

#[derive(Debug)]
pub struct Region {
    pub outer: Vec<Point>,
    /// เกาะที่เจอระหว่างทาง (ลูปที่วนตามเข็ม)
    pub holes: Vec<Vec<Point>>,
    /// id ของวัตถุที่ถูกใช้เป็นขอบ/เกาะแล้ว
    pub used: HashSet<u64>,
}

const MAX_STEPS: usize = 100_000;
const MAX_TRACES: usize = 64;

/// เดินวนรอบพื้นที่ที่อยู่ "ซ้าย" ของเส้น start (ทิศ fwd) เริ่มที่ชิ้นที่ครอบจุด h
fn trace_face(
    fetch: &mut dyn FnMut(&BoundingBox) -> Vec<Seg>,
    start: Seg,
    fwd0: bool,
    h: Point,
    eps: f64,
) -> Option<(Vec<Point>, Vec<(u64, u32)>)> {
    let mut s = start;
    let mut fwd = fwd0;

    // ชิ้นเริ่มต้น: node ก่อนจุด h ตามทิศเดิน
    let len = dist(s.a, s.b);
    let near = fetch(&expand(&s.bbox(), eps));
    let nodes = node_params(&s, &near, eps);
    let th = param_of(&s, h, eps);
    let epst = eps / len;
    let mut t_cur = if fwd {
        nodes.iter().copied().filter(|t| *t < th - epst).last().unwrap_or(0.0)
    } else {
        nodes.iter().copied().find(|t| *t > th + epst).unwrap_or(1.0)
    };

    let p_start = point_at(&s, t_cur);
    let start_key = (s.key(), fwd);
    let mut pts = vec![p_start];
    let mut keys: Vec<(u64, u32)> = Vec::new();
    let mut first_near = Some(near);

    for _ in 0..MAX_STEPS {
        let len = dist(s.a, s.b);
        let epst = eps / len;
        let near = match first_near.take() {
            Some(n) => n,
            None => fetch(&expand(&s.bbox(), eps)),
        };
        let nodes = node_params(&s, &near, eps);
        let t_next = if fwd {
            nodes.iter().copied().find(|t| *t > t_cur + epst).unwrap_or(1.0)
        } else {
            nodes.iter().rev().copied().find(|t| *t < t_cur - epst).unwrap_or(0.0)
        };
        keys.push(s.key());
        let v = point_at(&s, t_next);

        // ที่จุด v: เลือกเส้นที่ "เลี้ยวซ้ายสุด" (วนตามเข็มจากทิศที่ย้อนกลับน้อยสุด)
        let din = if fwd { (s.b.x - s.a.x, s.b.y - s.a.y) } else { (s.a.x - s.b.x, s.a.y - s.b.y) };
        let ang_rev = (-din.1).atan2(-din.0);
        let inc = fetch(&expand(&BoundingBox { min_x: v.x, min_y: v.y, max_x: v.x, max_y: v.y }, eps * 2.0));
        let mut best: Option<(f64, Seg, bool)> = None;
        for t in &inc {
            if seg_dist(v, t.a, t.b) > eps * 2.0 {
                continue;
            }
            for (ok, f) in [(dist(v, t.b) > eps, true), (dist(v, t.a) > eps, false)] {
                if !ok {
                    continue;
                }
                let d = if f { (t.b.x - t.a.x, t.b.y - t.a.y) } else { (t.a.x - t.b.x, t.a.y - t.b.y) };
                let mut diff = (ang_rev - d.1.atan2(d.0)).rem_euclid(TAU);
                if diff < 1e-9 || diff > TAU - 1e-9 {
                    diff = TAU; // ย้อนกลับทางเดิม = ทางเลือกสุดท้าย (ปลายตัน)
                }
                if best.as_ref().is_none_or(|(bd, _, _)| diff < *bd) {
                    best = Some((diff, *t, f));
                }
            }
        }
        let (_, s2, f2) = best?;
        if (s2.key(), f2) == start_key && dist(v, p_start) <= eps * 4.0 {
            return Some((pts, keys));
        }
        pts.push(v);
        t_cur = param_of(&s2, v, eps);
        s = s2;
        fwd = f2;
    }
    None
}

/// หาพื้นที่ปิดที่ครอบจุด click; fetch(bbox) ต้องคืนเส้นของทุกวัตถุที่กรอบแตะ bbox
pub fn find_region(
    fetch: &mut dyn FnMut(&BoundingBox) -> Vec<Seg>,
    click: Point,
    extent: &BoundingBox,
    eps: f64,
) -> Result<Region, &'static str> {
    const NO_BOUNDARY: &str = "No closed boundary around this point";
    let strip = BoundingBox {
        min_x: click.x,
        max_x: extent.max_x.max(click.x) + 1.0,
        min_y: click.y - eps,
        max_y: click.y + eps,
    };
    let mut hits: Vec<(f64, Seg)> = Vec::new();
    for s in fetch(&strip) {
        if (s.a.y > click.y) != (s.b.y > click.y) {
            let x = s.a.x + (click.y - s.a.y) * (s.b.x - s.a.x) / (s.b.y - s.a.y);
            if x > click.x + eps {
                hits.push((x, s));
            }
        }
    }
    hits.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    if hits.is_empty() {
        return Err(NO_BOUNDARY);
    }

    let mut traced: HashSet<(u64, u32)> = HashSet::new();
    let mut used: HashSet<u64> = HashSet::new();
    let mut islands: Vec<Vec<Point>> = Vec::new();
    let mut outer: Option<Vec<Point>> = None;
    let mut traces = 0;

    for (x, seg) in &hits {
        if traced.contains(&seg.key()) {
            continue;
        }
        if traces >= MAX_TRACES {
            break;
        }
        traces += 1;
        let side = (seg.b.x - seg.a.x) * (click.y - seg.a.y) - (seg.b.y - seg.a.y) * (click.x - seg.a.x);
        let fwd = side > 0.0;
        let h = Point { x: *x, y: click.y };
        let Some((poly, keys)) = trace_face(fetch, *seg, fwd, h, eps) else {
            traced.insert(seg.key());
            continue;
        };
        for k in &keys {
            traced.insert(*k);
            used.insert(k.0);
        }
        traced.insert(seg.key());
        let poly = simplify(&poly, eps);
        if poly.len() < 3 {
            continue;
        }
        let area = signed_area(&poly);
        if area > eps * eps && point_in_poly(click, &poly) {
            outer = Some(poly);
            break;
        } else if area < -(eps * eps) {
            islands.push(poly);
        }
    }

    let Some(outer) = outer else { return Err(NO_BOUNDARY) };
    let holes = islands
        .into_iter()
        .filter(|l| l.iter().all(|p| point_in_poly(*p, &outer) || outer.iter().any(|q| dist(*p, *q) <= eps)))
        .collect();
    Ok(Region { outer, holes, used })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn line(id: u64, a: Point, b: Point) -> Seg {
        Seg { a, b, src: id, idx: 0 }
    }

    fn rect(id0: u64, x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<Seg> {
        vec![
            line(id0, p(x0, y0), p(x1, y0)),
            line(id0 + 1, p(x1, y0), p(x1, y1)),
            line(id0 + 2, p(x1, y1), p(x0, y1)),
            line(id0 + 3, p(x0, y1), p(x0, y0)),
        ]
    }

    fn region(segs: &[Seg], click: Point) -> Result<Region, &'static str> {
        let ext = BoundingBox { min_x: -1000.0, min_y: -1000.0, max_x: 1000.0, max_y: 1000.0 };
        let all = segs.to_vec();
        let mut f = move |bb: &BoundingBox| -> Vec<Seg> {
            all.iter().filter(|s| s.bbox().intersects(bb)).copied().collect()
        };
        find_region(&mut f, click, &ext, 1e-7)
    }

    #[test]
    fn rectangle() {
        let r = region(&rect(1, 0.0, 0.0, 10.0, 6.0), p(3.0, 2.0)).unwrap();
        assert!((signed_area(&r.outer) - 60.0).abs() < 1e-9);
        assert!(r.holes.is_empty());
    }

    #[test]
    fn outside_is_error() {
        assert!(region(&rect(1, 0.0, 0.0, 10.0, 6.0), p(-3.0, 2.0)).is_err());
        assert!(region(&rect(1, 0.0, 0.0, 10.0, 6.0), p(30.0, 2.0)).is_err());
    }

    #[test]
    fn open_gap_is_error() {
        let mut s = rect(1, 0.0, 0.0, 10.0, 6.0);
        s.remove(3); // เอาด้านซ้ายออก
        assert!(region(&s, p(3.0, 2.0)).is_err());
    }

    #[test]
    fn t_junction_divider() {
        let mut s = rect(1, 0.0, 0.0, 10.0, 6.0);
        s.push(line(9, p(4.0, 0.0), p(4.0, 6.0)));
        let left = region(&s, p(2.0, 3.0)).unwrap();
        assert!((signed_area(&left.outer) - 24.0).abs() < 1e-9, "{}", signed_area(&left.outer));
        let right = region(&s, p(7.0, 3.0)).unwrap();
        assert!((signed_area(&right.outer) - 36.0).abs() < 1e-9);
    }

    #[test]
    fn crossing_lines_make_triangles() {
        // กากบาท: เส้นทแยงสองเส้นตัดกลางสี่เหลี่ยม คลิกในสามเหลี่ยมซ้าย
        let mut s = rect(1, 0.0, 0.0, 10.0, 10.0);
        s.push(line(9, p(0.0, 0.0), p(10.0, 10.0)));
        s.push(line(10, p(0.0, 10.0), p(10.0, 0.0)));
        let r = region(&s, p(1.0, 5.0)).unwrap();
        assert!((signed_area(&r.outer) - 25.0).abs() < 1e-9, "{}", signed_area(&r.outer));
        let top = region(&s, p(5.0, 9.0)).unwrap();
        assert!((signed_area(&top.outer) - 25.0).abs() < 1e-9);
    }

    #[test]
    fn lines_extending_past_corners() {
        // เส้นไม่ได้หยุดที่มุม แต่ยื่นเลยออกไป (แบบวาดกำแพงด้วยเส้นยาว)
        let s = vec![
            line(1, p(-3.0, 0.0), p(13.0, 0.0)),
            line(2, p(10.0, -3.0), p(10.0, 9.0)),
            line(3, p(13.0, 6.0), p(-3.0, 6.0)),
            line(4, p(0.0, 9.0), p(0.0, -3.0)),
        ];
        let r = region(&s, p(5.0, 3.0)).unwrap();
        assert!((signed_area(&r.outer) - 60.0).abs() < 1e-9, "{}", signed_area(&r.outer));
    }

    #[test]
    fn island_on_the_ray_becomes_hole() {
        let mut s = rect(1, 0.0, 0.0, 20.0, 20.0);
        s.extend(rect(10, 8.0, 8.0, 12.0, 12.0)); // เกาะสี่เหลี่ยมวาดด้วยเส้น
        let r = region(&s, p(4.0, 10.0)).unwrap(); // รังสีไปทางขวาชนเกาะก่อน
        assert!((signed_area(&r.outer) - 400.0).abs() < 1e-9);
        assert_eq!(r.holes.len(), 1);
        assert!((signed_area(&r.holes[0]).abs() - 16.0).abs() < 1e-9);
    }

    #[test]
    fn element_segs_circle_and_arc() {
        let mut v = Vec::new();
        element_segs(7, &ElementData::Circle { center: p(0.0, 0.0), radius: 5.0 }, &mut v);
        assert_eq!(v.len(), CIRCLE_STEPS);
        let poly: Vec<Point> = v.iter().map(|s| s.a).collect();
        let a = signed_area(&poly);
        assert!(a > 0.0 && (a - std::f64::consts::PI * 25.0).abs() < 0.1);
        // ปลายต่อปลาย
        for i in 0..v.len() {
            assert!(dist(v[i].b, v[(i + 1) % v.len()].a) < 1e-12);
        }
        let mut w = Vec::new();
        element_segs(8, &ElementData::Arc { center: p(0.0, 0.0), radius: 5.0, start_angle: 0.0, end_angle: std::f64::consts::PI }, &mut w);
        assert!(w.len() >= 100);
    }

    #[test]
    fn circle_boundary() {
        let mut v = Vec::new();
        element_segs(1, &ElementData::Circle { center: p(0.0, 0.0), radius: 5.0 }, &mut v);
        let r = region(&v, p(1.0, 1.0)).unwrap();
        assert!((signed_area(&r.outer) - std::f64::consts::PI * 25.0).abs() < 0.1);
        assert!(r.used.contains(&1));
    }

    #[test]
    fn annulus_click_left_of_inner() {
        let mut v = Vec::new();
        element_segs(1, &ElementData::Circle { center: p(0.0, 0.0), radius: 10.0 }, &mut v);
        element_segs(2, &ElementData::Circle { center: p(0.0, 0.0), radius: 4.0 }, &mut v);
        // อยู่ซ้ายของวงใน: รังสีไปขวาชนวงในก่อน
        let r = region(&v, p(-7.0, 0.0)).unwrap();
        assert!((signed_area(&r.outer) - std::f64::consts::PI * 100.0).abs() < 0.5);
        assert_eq!(r.holes.len(), 1);
        assert!((signed_area(&r.holes[0]).abs() - std::f64::consts::PI * 16.0).abs() < 0.2);
    }

    #[test]
    fn overlapping_circles_lens() {
        let mut v = Vec::new();
        element_segs(1, &ElementData::Circle { center: p(0.0, 0.0), radius: 5.0 }, &mut v);
        element_segs(2, &ElementData::Circle { center: p(6.0, 0.0), radius: 5.0 }, &mut v);
        let r = region(&v, p(3.0, 0.0)).unwrap(); // เลนส์ตรงกลาง
        // พื้นที่เลนส์ของวงกลมรัศมี 5 ระยะห่างศูนย์กลาง 6
        let d: f64 = 6.0;
        let rr: f64 = 5.0;
        let lens = 2.0 * rr * rr * (d / (2.0 * rr)).acos() - 0.5 * d * (4.0 * rr * rr - d * d).sqrt();
        assert!((signed_area(&r.outer) - lens).abs() < 0.3, "{} vs {}", signed_area(&r.outer), lens);
    }

    #[test]
    fn pattern_horizontal_lines() {
        let sq = vec![vec![p(0.0, 0.0), p(10.0, 0.0), p(10.0, 10.0), p(0.0, 10.0)]];
        let spec = HatchSpec { kind: HatchKind::Lines, angle: 0.0, spacing: 2.0 };
        let segs = pattern_segments(&sq, &spec, None).unwrap();
        // y = 0 (ขอบล่าง นับด้านบวก) ... ขอบบน y=10 ไม่นับ → 5 เส้น ยาวเส้นละ 10
        let total: f64 = segs.iter().map(|(a, b)| dist(*a, *b)).sum();
        assert!((total - 50.0).abs() < 1e-9, "{total}");
        assert_eq!(segs.len(), 5);
    }

    #[test]
    fn pattern_diagonal_total_length() {
        let sq = vec![vec![p(0.0, 0.0), p(10.0, 0.0), p(10.0, 10.0), p(0.0, 10.0)]];
        let spec = HatchSpec { kind: HatchKind::Lines, angle: std::f64::consts::FRAC_PI_4, spacing: 1.0 };
        let segs = pattern_segments(&sq, &spec, None).unwrap();
        let total: f64 = segs.iter().map(|(a, b)| dist(*a, *b)).sum();
        // ความยาวรวม ≈ พื้นที่ / ระยะห่าง
        assert!((total - 100.0).abs() < 2.0, "{total}");
        for (a, b) in &segs {
            for q in [a, b] {
                assert!(q.x >= -1e-9 && q.x <= 10.0 + 1e-9 && q.y >= -1e-9 && q.y <= 10.0 + 1e-9);
            }
        }
    }

    #[test]
    fn pattern_hole_and_cross_and_view() {
        let outer = vec![p(0.0, 0.0), p(10.0, 0.0), p(10.0, 10.0), p(0.0, 10.0)];
        let hole = vec![p(4.0, 4.0), p(6.0, 4.0), p(6.0, 6.0), p(4.0, 6.0)];
        let spec = HatchSpec { kind: HatchKind::Lines, angle: 0.0, spacing: 1.0 };
        let with = pattern_segments(&[outer.clone(), hole.clone()], &spec, None).unwrap();
        let without = pattern_segments(&[outer.clone()], &spec, None).unwrap();
        let len = |v: &Vec<(Point, Point)>| v.iter().map(|(a, b)| dist(*a, *b)).sum::<f64>();
        assert!((len(&without) - len(&with) - 4.0).abs() < 1e-9); // รูกว้าง 2 ตัดสองเส้น (y=4,5) → หาย 4
        let cross = pattern_segments(&[outer.clone()], &HatchSpec { kind: HatchKind::Cross, ..spec }, None).unwrap();
        assert!(cross.len() >= 2 * without.len() - 2);
        let view = BoundingBox { min_x: 2.0, min_y: 2.0, max_x: 5.0, max_y: 4.0 };
        let clipped = pattern_segments(&[outer], &spec, Some(&view)).unwrap();
        assert_eq!(clipped.len(), 3); // y = 2, 3, 4
        for (a, b) in &clipped {
            assert!(a.x >= 2.0 - 1e-9 && b.x <= 5.0 + 1e-9);
        }
    }

    #[test]
    fn pattern_too_dense_is_none() {
        let sq = vec![vec![p(0.0, 0.0), p(1000.0, 0.0), p(1000.0, 1000.0), p(0.0, 1000.0)]];
        let spec = HatchSpec { kind: HatchKind::Lines, angle: 0.3, spacing: 0.001 };
        assert!(pattern_segments(&sq, &spec, None).is_none());
        let bad = HatchSpec { spacing: 0.0, ..spec };
        assert!(pattern_segments(&sq, &bad, None).is_none());
    }
}
