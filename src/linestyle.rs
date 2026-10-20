// linestyle.rs - ความหนาเส้น (lineweight) และชนิดเส้น (linetype)
//
// ความหนาเก็บเป็น mm (ตามไฟล์ DXF) แล้วแปลงเป็นพิกเซลคงที่ตอนวาด: 0.25 mm = 2 px (เท่าเดิมก่อนมีฟีเจอร์นี้)
// ชนิดเส้นเก็บเป็นลวดลายหน่วยโลก (+ขีด, -ช่องว่าง, 0 จุด) ย่อ/ขยายตามการซูม และถ้าถี่เกินไปจนมองไม่ออก
// จะวาดเป็นเส้นทึบ

use acadrust::entities::EntityCommon;
use acadrust::tables::LineType as AcLineType;
use acadrust::tables::LineTypeElement;
use acadrust::types::LineWeight;
use acadrust::CadDocument;
use std::collections::HashMap;
use std::sync::Arc;

pub const DEFAULT_WEIGHT: f64 = 0.25;

#[derive(Debug, PartialEq)]
pub struct LineType {
    pub name: String,
    /// ว่าง = เส้นทึบ
    pub pattern: Vec<f64>,
}

pub type Lt = Arc<LineType>;

/// สไตล์เฉพาะวัตถุ: None = ใช้ของ layer
#[derive(Debug, Clone, PartialEq)]
pub struct ElementStyle {
    /// มิลลิเมตร
    pub weight: Option<f64>,
    /// Some(เส้นทึบ) = บังคับทึบ, Some(ลวดลาย) = บังคับลวดลายนั้น
    pub linetype: Option<Lt>,
    pub lt_scale: f64,
}

impl Default for ElementStyle {
    fn default() -> Self {
        Self { weight: None, linetype: None, lt_scale: 1.0 }
    }
}

/// มม. → พิกเซลบนจอ
pub fn weight_px(mm: f64) -> f64 {
    if mm <= 0.0 {
        1.0
    } else {
        (1.0 + mm * 4.0).min(8.0)
    }
}

/// ลวดลาย (หน่วยโลก) → อาร์เรย์ dash ของ cairo (พิกเซล); ว่าง = เส้นทึบ
pub fn dash_px(pattern: &[f64], k: f64) -> Vec<f64> {
    if pattern.is_empty() || !(k > 0.0) || !k.is_finite() {
        return Vec::new();
    }
    let mut out: Vec<f64> = Vec::with_capacity(pattern.len() + 2);
    let mut expect_on = true;
    for &p in pattern {
        let len = if p == 0.0 { 0.6 } else { p.abs() * k };
        let is_dash = p >= 0.0;
        if is_dash == expect_on {
            out.push(len);
            expect_on = !expect_on;
        } else {
            out.push(0.0);
            out.push(len);
        }
    }
    let period: f64 = out.iter().sum();
    if !(3.0..=1.0e5).contains(&period) {
        return Vec::new();
    }
    out
}

/// สไตล์ที่ใช้วาดจริงของวัตถุ: (ความหนา px, dash px)
pub fn resolve(
    layer_weight: f64,
    layer_lt: &Option<Lt>,
    st: &ElementStyle,
    cam_scale: f64,
) -> (f64, Vec<f64>) {
    let w = st.weight.unwrap_or(layer_weight);
    let lt = st.linetype.as_ref().or(layer_lt.as_ref());
    let dash = match lt {
        Some(l) => dash_px(&l.pattern, st.lt_scale * cam_scale),
        None => Vec::new(),
    };
    // ซูมออกไกลเส้นที่หนา 2 px จะดูเบลอ/อืด: ลดลงเหลือ 60% (ไม่ต่ำกว่า 1 px)
    let far = if cam_scale < 0.2 { 0.6 } else { 1.0 };
    ((weight_px(w) * far).max(1.0), dash)
}

// ---------------- DXF → engine ----------------

fn is_inherit(name: &str) -> bool {
    name.is_empty() || name.eq_ignore_ascii_case("bylayer") || name.eq_ignore_ascii_case("byblock")
}

/// ตาราง linetype ของไฟล์ (ชื่อตัวพิมพ์เล็ก → ลวดลาย)
pub fn linetype_map(doc: &CadDocument) -> HashMap<String, Lt> {
    let mut m = HashMap::new();
    for lt in doc.line_types.iter() {
        if is_inherit(&lt.name) {
            continue;
        }
        let pattern: Vec<f64> = lt.elements.iter().map(|e| e.length).collect();
        // ลวดลายที่เป็นศูนย์หมด/ไม่มีเลย = เส้นทึบ
        let solid = pattern.iter().all(|p| *p == 0.0) && pattern.len() != 1 || pattern.is_empty();
        m.insert(
            lt.name.to_lowercase(),
            Arc::new(LineType { name: lt.name.clone(), pattern: if solid { Vec::new() } else { pattern } }),
        );
    }
    m
}

fn weight_mm(w: &LineWeight) -> Option<f64> {
    match w {
        LineWeight::Value(v) => Some((*v).max(0) as f64 / 100.0),
        LineWeight::Default => Some(DEFAULT_WEIGHT),
        _ => None, // ByLayer / ByBlock
    }
}

pub fn entity_style(c: &EntityCommon, map: &HashMap<String, Lt>, global_scale: f64) -> ElementStyle {
    let linetype = if is_inherit(&c.linetype) {
        None
    } else {
        map.get(&c.linetype.to_lowercase()).cloned()
    };
    let s = if c.linetype_scale > 0.0 { c.linetype_scale } else { 1.0 };
    ElementStyle { weight: weight_mm(&c.line_weight), linetype, lt_scale: s * global_scale }
}

/// (ความหนา mm, linetype) ของ layer; linetype ทึบ → None
pub fn layer_style(l: &acadrust::tables::Layer, map: &HashMap<String, Lt>) -> (f64, Option<Lt>) {
    let w = weight_mm(&l.line_weight).unwrap_or(DEFAULT_WEIGHT);
    let lt = map.get(&l.line_type.to_lowercase()).filter(|t| !t.pattern.is_empty()).cloned();
    (w, lt)
}

// ---------------- engine → DXF ----------------

pub fn to_ac_weight(mm: f64) -> LineWeight {
    LineWeight::Value((mm * 100.0).round().clamp(0.0, 211.0) as i16)
}

pub fn ac_linetype_name(lt: &Option<Lt>) -> String {
    match lt {
        Some(l) => l.name.clone(),
        None => "Continuous".to_string(),
    }
}

/// เพิ่ม linetype ที่ใช้อยู่ลงตาราง (ถ้ายังไม่มี) ก่อนเขียนไฟล์
pub fn ensure_linetypes<'a>(doc: &mut CadDocument, used: impl Iterator<Item = &'a Lt>) {
    for l in used {
        if l.pattern.is_empty() || doc.line_types.contains(&l.name) {
            continue;
        }
        let mut t = AcLineType::new(l.name.clone());
        for &p in &l.pattern {
            t.add_element(LineTypeElement { length: p, complex: None });
        }
        t.pattern_length = l.pattern.iter().map(|p| p.abs()).sum();
        let _ = doc.line_types.add(t);
    }
}

// ---------------- UI ----------------

/// ความหนามาตรฐานของ CAD (มม.)
pub const WEIGHTS_MM: [f64; 24] = [
    0.00, 0.05, 0.09, 0.13, 0.15, 0.18, 0.20, 0.25, 0.30, 0.35, 0.40, 0.50, 0.53, 0.60, 0.70, 0.80,
    0.90, 1.00, 1.06, 1.20, 1.40, 1.58, 2.00, 2.11,
];

/// index ของความหนามาตรฐานที่ใกล้ค่านี้ที่สุด
pub fn nearest_weight_index(mm: f64) -> usize {
    let mut best = (f64::MAX, 0);
    for (i, w) in WEIGHTS_MM.iter().enumerate() {
        let d = (w - mm).abs();
        if d < best.0 {
            best = (d, i);
        }
    }
    best.1
}

/// ชนิดเส้นสำเร็จรูป (หน่วยเป็น mm แบบ acadiso)
pub fn presets() -> Vec<Lt> {
    let p = |name: &str, pat: &[f64]| Arc::new(LineType { name: name.into(), pattern: pat.to_vec() });
    vec![
        p("DASHED", &[12.0, -6.0]),
        p("HIDDEN", &[6.0, -3.0]),
        p("CENTER", &[32.0, -6.0, 6.0, -6.0]),
        p("DASHDOT", &[12.0, -6.0, 0.0, -6.0]),
        p("DOT", &[0.0, -6.0]),
        p("PHANTOM", &[30.0, -6.0, 6.0, -6.0, 6.0, -6.0]),
    ]
}

/// เส้นทึบที่ตั้งตรง ๆ ที่วัตถุ (ต่างจาก None = ตาม layer)
pub fn continuous() -> Lt {
    Arc::new(LineType { name: "Continuous".into(), pattern: Vec::new() })
}

/// สรุปสไตล์ของวัตถุที่เลือกอยู่ (ใช้แสดงในแถบ Selected objects)
#[derive(Debug, Default, Clone)]
pub struct SelStyle {
    pub count: usize,
    pub weight: Option<f64>,
    pub linetype: Option<Lt>,
    /// ค่าไม่เหมือนกันในวัตถุที่เลือก
    pub weight_mixed: bool,
    pub lt_mixed: bool,
}
