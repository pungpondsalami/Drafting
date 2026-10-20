use crate::engine::RenderCommand;
use crate::text::{HAlign, VAlign};
use gtk::pango;

pub trait Renderer {
    fn draw_commands(&mut self, commands: Vec<RenderCommand>);
}

pub struct CairoRenderer<'a> {
    cr: &'a gtk::cairo::Context,
    lw: f64,
    dash: Vec<f64>,
}

impl<'a> CairoRenderer<'a> {
    pub fn new(cr: &'a gtk::cairo::Context) -> Self {
        Self { cr, lw: 2.0, dash: Vec::new() }
    }

    /// เลือกสีเส้น: ถูกเลือก = แดงหนา, ปกติ = สีของวัตถุ/layer
    fn pen(&self, is_selected: bool, color: (f64, f64, f64)) {
        if is_selected {
            self.cr.set_source_rgb(1.0, 0.0, 0.0);
            self.cr.set_line_width((self.lw + 1.0).max(3.0));
        } else {
            self.cr.set_source_rgb(color.0, color.1, color.2);
            self.cr.set_line_width(self.lw);
        }
        self.cr.set_dash(&self.dash, 0.0);
    }
}

/// วาดข้อความด้วย Pango ที่จุดยึด (x, y) บนจอ ด้วยสีที่ตั้งไว้ใน cr ตอนนี้
///
/// จัดวางที่ขนาดคงที่ 100 หน่วยแล้วย่อ/ขยายด้วย cr.scale ทำให้การตัดบรรทัดไม่เปลี่ยนตามระดับซูม
#[allow(clippy::too_many_arguments)]
pub fn draw_text(
    cr: &gtk::cairo::Context,
    x: f64,
    y: f64,
    em_px: f64,
    rotation: f64,
    wrap_px: f64,
    text: &str,
    halign: HAlign,
    valign: VAlign,
) {
    const NOMINAL: f64 = 100.0;
    let k = em_px / NOMINAL;
    if !(k > 0.0) || !k.is_finite() {
        return;
    }
    let scale = pango::SCALE as f64;

    let _ = cr.save();
    cr.translate(x, y);
    cr.rotate(-rotation); // แกน Y บนจอกลับด้าน
    cr.scale(k, k);

    let layout = pangocairo::functions::create_layout(cr);
    let mut font = pango::FontDescription::new();
    font.set_family("Sans");
    font.set_absolute_size(NOMINAL * scale);
    layout.set_font_description(Some(&font));
    layout.set_text(text);
    layout.set_alignment(match halign {
        HAlign::Left => pango::Alignment::Left,
        HAlign::Center => pango::Alignment::Center,
        HAlign::Right => pango::Alignment::Right,
    });
    let wrap = wrap_px > 0.0;
    if wrap {
        layout.set_width(((wrap_px / k) * scale) as i32);
        layout.set_wrap(pango::WrapMode::WordChar);
    }

    let (_, logical) = layout.extents();
    let lw = logical.width() as f64 / scale;
    let lh = logical.height() as f64 / scale;
    let box_w = if wrap { wrap_px / k } else { lw };
    let baseline = layout.baseline() as f64 / scale;

    let dx = match halign {
        HAlign::Left => 0.0,
        HAlign::Center => -box_w / 2.0,
        HAlign::Right => -box_w,
    };
    let dy = match valign {
        VAlign::Top => 0.0,
        VAlign::Middle => -lh / 2.0,
        VAlign::Bottom => -lh,
        VAlign::Baseline => -baseline,
    };
    cr.move_to(dx, dy);
    pangocairo::functions::show_layout(cr, &layout);

    let _ = cr.restore();
    cr.new_path();
}

impl Renderer for CairoRenderer<'_> {
    fn draw_commands(&mut self, commands: Vec<RenderCommand>) {
        let cr = self.cr;
        for cmd in commands {
            match cmd {
                RenderCommand::Line { x1, y1, x2, y2, is_selected, color } => {
                    self.pen(is_selected, color);
                    cr.move_to(x1, y1);
                    cr.line_to(x2, y2);
                    let _ = cr.stroke();
                }
                RenderCommand::Circle { cx, cy, radius, is_selected, color } => {
                    self.pen(is_selected, color);
                    cr.arc(cx, cy, radius, 0.0, 2.0 * std::f64::consts::PI);
                    let _ = cr.stroke();
                }
                RenderCommand::Arc { cx, cy, radius, start_angle, end_angle, is_selected, color } => {
                    self.pen(is_selected, color);
                    // แกน Y บนจอกลับด้าน: มุมเปลี่ยนเครื่องหมาย และ CCW ในโลก = arc_negative บนจอ
                    cr.arc_negative(cx, cy, radius, -start_angle, -end_angle);
                    let _ = cr.stroke();
                }
                RenderCommand::Polyline { points, closed, is_selected, color } => {
                    self.pen(is_selected, color);
                    for (i, (x, y)) in points.iter().enumerate() {
                        if i == 0 {
                            cr.move_to(*x, *y);
                        } else {
                            cr.line_to(*x, *y);
                        }
                    }
                    if closed {
                        cr.close_path();
                    }
                    let _ = cr.stroke();
                }

                RenderCommand::Fill { loops, filled, is_selected, color } => {
                    for l in &loops {
                        for (i, (x, y)) in l.iter().enumerate() {
                            if i == 0 {
                                cr.move_to(*x, *y);
                            } else {
                                cr.line_to(*x, *y);
                            }
                        }
                        cr.close_path();
                    }
                    if filled {
                        if is_selected {
                            cr.set_source_rgba(1.0, 0.0, 0.0, 0.7);
                        } else {
                            cr.set_source_rgb(color.0, color.1, color.2);
                        }
                        cr.set_fill_rule(gtk::cairo::FillRule::EvenOdd);
                        if is_selected {
                            let _ = cr.fill_preserve();
                            cr.set_source_rgb(1.0, 0.0, 0.0);
                            cr.set_line_width(3.0);
                            let _ = cr.stroke();
                        } else {
                            // เติมแล้ววาดขอบบาง ๆ ทับ ปิดรอยต่อระหว่างสามเหลี่ยมที่อยู่ติดกัน
                            let _ = cr.fill_preserve();
                            cr.set_dash(&[], 0.0);
                            cr.set_line_width(0.6);
                            let _ = cr.stroke();
                        }
                        cr.set_fill_rule(gtk::cairo::FillRule::Winding);
                    } else {
                        self.pen(is_selected, color);
                        let _ = cr.stroke();
                    }
                }

                RenderCommand::Style { lw, dash } => {
                    self.lw = lw;
                    self.dash = dash;
                }

                RenderCommand::Text {
                    x, y, em_px, rotation, wrap_px, text, halign, valign, is_selected, color,
                } => {
                    if is_selected {
                        cr.set_source_rgb(1.0, 0.0, 0.0);
                    } else {
                        cr.set_source_rgb(color.0, color.1, color.2);
                    }
                    draw_text(cr, x, y, em_px, rotation, wrap_px, &text, halign, valign);
                }

                RenderCommand::Dimension {
                    text_x, text_y, text, is_selected,
                    dim_x1, dim_y1, dim_x2, dim_y2,
                    ext1_x1, ext1_y1, ext1_x2, ext1_y2,
                    ext2_x1, ext2_y1, ext2_x2, ext2_y2,
                } => {
                    cr.set_dash(&[], 0.0);
                    if is_selected {
                        cr.set_source_rgb(1.0, 0.35, 0.35);
                        cr.set_line_width(2.5);
                    } else {
                        cr.set_source_rgb(0.9, 0.9, 0.0);
                        cr.set_line_width(1.2);
                    }
                    cr.move_to(ext1_x1, ext1_y1); cr.line_to(ext1_x2, ext1_y2);
                    let _ = cr.stroke();
                    cr.move_to(ext2_x1, ext2_y1); cr.line_to(ext2_x2, ext2_y2);
                    let _ = cr.stroke();
                    cr.move_to(dim_x1, dim_y1); cr.line_to(dim_x2, dim_y2);
                    let _ = cr.stroke();

                    cr.select_font_face(
                        "Sans",
                        gtk::cairo::FontSlant::Normal,
                        gtk::cairo::FontWeight::Normal,
                    );
                    cr.set_font_size(13.0);
                    if let Ok(ext) = cr.text_extents(&text) {
                        cr.move_to(
                            text_x - ext.width() / 2.0 - ext.x_bearing(),
                            text_y - ext.height() / 2.0 - ext.y_bearing() - 8.0,
                        );
                        let _ = cr.show_text(&text);
                    }
                    cr.new_path();
                }
            }
        }
    }
}
