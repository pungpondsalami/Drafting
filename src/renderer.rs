use crate::engine::RenderCommand;

pub trait Renderer {
    fn draw_commands(&mut self, commands: Vec<RenderCommand>);
}

pub struct CairoRenderer<'a> {
    cr: &'a gtk::cairo::Context,
}

impl<'a> CairoRenderer<'a> {
    pub fn new(cr: &'a gtk::cairo::Context) -> Self {
        Self { cr }
    }
}

impl Renderer for CairoRenderer<'_> {
    fn draw_commands(&mut self, commands: Vec<RenderCommand>) {
        let cr = self.cr;
        for cmd in commands {
            match cmd {
                RenderCommand::Line { x1, y1, x2, y2, is_selected } => {
                    if is_selected {
                        cr.set_source_rgb(1.0, 0.0, 0.0);
                        cr.set_line_width(3.0);
                    } else {
                        cr.set_source_rgb(1.0, 1.0, 1.0);
                        cr.set_line_width(2.0);
                    }
                    cr.move_to(x1, y1);
                    cr.line_to(x2, y2);
                    let _ = cr.stroke();
                }
                RenderCommand::Circle { cx, cy, radius, is_selected } => {
                    if is_selected {
                        cr.set_source_rgb(1.0, 0.0, 0.0);
                        cr.set_line_width(3.0);
                    } else {
                        cr.set_source_rgb(1.0, 1.0, 1.0);
                        cr.set_line_width(2.0);
                    }
                    cr.arc(cx, cy, radius, 0.0, 2.0 * std::f64::consts::PI);
                    let _ = cr.stroke();
                }
                RenderCommand::Arc { cx, cy, radius, start_angle, end_angle, is_selected } => {
                    if is_selected {
                        cr.set_source_rgb(1.0, 0.0, 0.0);
                        cr.set_line_width(3.0);
                    } else {
                        cr.set_source_rgb(1.0, 1.0, 1.0);
                        cr.set_line_width(2.0);
                    }
                    // แกน Y บนจอกลับด้าน: มุมเปลี่ยนเครื่องหมาย และ CCW ในโลก = arc_negative บนจอ
                    cr.arc_negative(cx, cy, radius, -start_angle, -end_angle);
                    let _ = cr.stroke();
                }
                RenderCommand::Polyline { points, closed, is_selected } => {
                    if is_selected {
                        cr.set_source_rgb(1.0, 0.0, 0.0);
                        cr.set_line_width(3.0);
                    } else {
                        cr.set_source_rgb(1.0, 1.0, 1.0);
                        cr.set_line_width(2.0);
                    }
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
                RenderCommand::Dimension {
                    text_x, text_y, text, is_selected,
                    dim_x1, dim_y1, dim_x2, dim_y2,
                    ext1_x1, ext1_y1, ext1_x2, ext1_y2,
                    ext2_x1, ext2_y1, ext2_x2, ext2_y2,
                } => {
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
