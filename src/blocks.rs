// ใช้ของ acadrust ส่วนไฟล์นี้ดูแลเรื่องที่มันทำให้ไม่ตรงกับโครงสร้างของเรา

use crate::engine::{ElementData, Point};
use acadrust::entities::{Arc, Circle, Dimension, Ellipse, Entity, Insert};
use acadrust::types::{Matrix4, Transform};
use acadrust::{CadDocument, EntityType};
use std::f64::consts::{PI, TAU};

/// บล็อกซ้อนบล็อกลึกสุดกี่ชั้น (กันไฟล์ที่อ้างวนกัน)
pub const MAX_DEPTH: u8 = 8;

/// วัตถุข้างในบล็อกของ INSERT แปลงเป็นพิกัดโลกแล้ว; None = ไม่พบบล็อกชื่อนี้
pub fn explode_insert(doc: &CadDocument, ins: &Insert) -> Option<Vec<EntityType>> {
    let br = doc.block_records.get(&ins.block_name)?;
    let mut ents: Vec<EntityType> = doc
        .entities_in_block(&ins.block_name)
        .cloned()
        .collect();
    // acadrust ไม่คิดจุดฐานของบล็อก (BLOCK 10/20): ย้ายให้จุดฐานอยู่ที่ origin ก่อนแตก
    let b = br.base_point;
    if b.x != 0.0 || b.y != 0.0 || b.z != 0.0 {
        let t = Transform::from_matrix(Matrix4::translation(-b.x, -b.y, -b.z));
        for e in ents.iter_mut() {
            e.as_entity_mut().apply_transform(&t);
        }
    }
    Some(ins.explode(&ents))
}

/// เส้นมิติ/ลูกศร/ตัวเลขของ DIMENSION อยู่ในบล็อกนิรนาม (*D1, ...) พิกัดโลกอยู่แล้ว
pub fn dimension_entities(doc: &CadDocument, d: &Dimension) -> Option<Vec<EntityType>> {
    let name = &d.base().block_name;
    if name.is_empty() {
        return None;
    }
    doc.block_records.get(name)?;
    Some(
        doc.entities_in_block(name)
            .filter(|e| {
                !matches!(
                    e,
                    EntityType::Block(_)
                        | EntityType::BlockEnd(_)
                        | EntityType::AttributeDefinition(_)
                        | EntityType::Point(_) // จุดนิยามมิติ (DEFPOINTS) ไม่ต้องแสดง
                )
            })
            .cloned()
            .collect(),
    )
}

/// ตัวแตก INSERT เปลี่ยน CIRCLE/ARC สเกลไม่เท่ากันเป็นวงรี; ถ้ายังกลมอยู่ให้คืนเป็น Circle/Arc
pub fn ellipse_round(el: &Ellipse) -> Option<ElementData> {
    if (el.minor_axis_ratio - 1.0).abs() > 1e-9 {
        return None;
    }
    let r = el.major_axis.x.hypot(el.major_axis.y);
    if !(r > 0.0) {
        return None;
    }
    let center = Point { x: el.center.x, y: el.center.y };
    let span = el.end_parameter - el.start_parameter;
    if (span - TAU).abs() < 1e-6 || span.abs() < 1e-12 {
        return Some(ElementData::Circle { center, radius: r });
    }
    let a0 = el.major_axis.y.atan2(el.major_axis.x);
    let start = a0 + el.start_parameter;
    let mut end = a0 + el.end_parameter;
    while end <= start {
        end += TAU;
    }
    Some(ElementData::Arc { center, radius: r, start_angle: start, end_angle: end })
}

/// ARC → ElementData; normal ที่ชี้ลง (-Z) กลับแกน X ของพิกัด OCS (เกิดตอนบล็อกถูกกระจก)
pub fn arc_element(a: &Arc) -> ElementData {
    let (mut s, mut e, mut cx) = (a.start_angle, a.end_angle, a.center.x);
    if a.normal.z < 0.0 {
        cx = -cx;
        (s, e) = (PI - e, PI - s);
    }
    while e <= s {
        e += TAU;
    }
    ElementData::Arc {
        center: Point { x: cx, y: a.center.y },
        radius: a.radius,
        start_angle: s,
        end_angle: e,
    }
}

pub fn circle_element(c: &Circle) -> ElementData {
    let cx = if c.normal.z < 0.0 { -c.center.x } else { c.center.x };
    ElementData::Circle { center: Point { x: cx, y: c.center.y }, radius: c.radius }
}

/// พื้นที่โมเดลทั้งหมดหลังแตก INSERT (ซ้อนได้) และ DIMENSION แล้ว
pub fn flatten_model_space(doc: &CadDocument) -> Vec<EntityType> {
    let mut out = Vec::new();
    for e in doc.model_space_entities() {
        flatten_into(doc, e, 0, &mut out);
    }
    out
}

fn flatten_into(doc: &CadDocument, e: &EntityType, depth: u8, out: &mut Vec<EntityType>) {
    let children = match e {
        EntityType::Insert(ins) if depth < MAX_DEPTH => explode_insert(doc, ins),
        EntityType::Dimension(d) if depth < MAX_DEPTH => dimension_entities(doc, d),
        _ => None,
    };
    match children {
        Some(list) => {
            for c in &list {
                flatten_into(doc, c, depth + 1, out);
            }
        }
        // ธรรมดา หรือแตกไม่ได้ (ไม่มีบล็อก/ลึกเกิน): ส่งต่อตามเดิม
        // INSERT/DIMENSION ที่ค้างอยู่จะไปตกที่ `_ => skipped += 1` ให้นับเป็นวัตถุที่ข้าม
        None => out.push(e.clone()),
    }
}
