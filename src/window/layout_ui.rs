//! แท็บ Layout (paper space): กระดาษ + Viewport หลายอันที่มองเข้าไปใน model ของอีกแท็บ
//! ตรรกะล้วนอยู่ใน crate::layout (มีเทสต์); ไฟล์นี้เป็นส่วน GTK
use super::{CLIPBOARD, DraftingWindow, PAPERS, PX_MM, dxf_to_elements, engine_to_dxf_bytes, print_prepare};
use crate::engine::{ClipItem, ElementData, Point, RenderCommand, clip_bounds};
use crate::layout::{
    LayoutSpec, MIN_VP, PaperItem, PaperView, STD_SCALES, Viewport, scale_index, scale_label, step_scale,
    unique_name,
};
use crate::renderer::{CairoRenderer, Renderer};
use adw::prelude::*;
use gtk::prelude::*;
use adw::subclass::prelude::*;
use gtk::gdk;
use gtk::glib;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

const DATA_KEY: &str = "drafting-layout";

thread_local! {
    /// overlay ที่เราซ่อนไว้ตอนอยู่ในแท็บ Layout (เพื่อคืนเฉพาะที่เราซ่อน)
    static HIDDEN: RefCell<Vec<glib::WeakRef<gtk::Widget>>> = const { RefCell::new(Vec::new()) };
}

enum Drag {
    Create { x0: f64, y0: f64, x1: f64, y1: f64 },
    Move { i: usize, ox: f64, oy: f64 },
    Resize { i: usize },
    PanContent { i: usize, lx: f64, ly: f64 },
    PanView { ox: f64, oy: f64 },
    MoveGroup { g: u32, lx: f64, ly: f64 },
    /// ย่อ/ขยายชุดของบนกระดาษโดยลากมุมขวาล่าง; จุดยึดคือมุมซ้ายบน (ax, ay แกน y ขึ้น)
    ScaleGroup { g: u32, ax: f64, ay: f64, d0: f64, orig: Vec<ElementData> },
}

struct LayoutTab {
    win: glib::WeakRef<DraftingWindow>,
    src: glib::WeakRef<adw::TabPage>,
    canvas: glib::WeakRef<gtk::DrawingArea>,
    /// ชื่อ Layout ในแบบ (engine.layouts); สำเนาทำงานของ spec อยู่ในฟิลด์ w, h, vps
    name: String,
    w: f64,
    h: f64,
    mono: bool,
    vps: Vec<Viewport>,
    sel: Option<usize>,
    /// ของบนกระดาษ (วางจาก Ctrl+V) พิกัด มม. แกน y ขึ้น; sel_group = ชุดที่เลือกอยู่
    items: Vec<PaperItem>,
    sel_group: Option<u32>,
    next_group: u32,
    items_changed: Cell<bool>,
    view: PaperView,
    /// เป้าหมายซูมแบบนุ่ม: z เป้าหมาย + จุดยึด (หน้าจอ, กระดาษ)
    zt: f64,
    zanchor: ((f64, f64), (f64, f64)),
    fitted: bool,
    add_mode: bool,
    inside: bool,
    drag: Option<Drag>,
    start: (f64, f64),
    mouse: (f64, f64),
    ui: Option<Rc<Ui>>,
}

impl LayoutTab {
    fn redraw(&self) {
        if let Some(c) = self.canvas.upgrade() {
            c.queue_draw();
        }
    }
    fn dims(&self) -> (f64, f64) {
        (self.w, self.h)
    }

    /// ลบสิ่งที่เลือกอยู่ (ชุดของบนกระดาษ หรือ viewport)
    fn delete_selection(&mut self) {
        if let Some(g) = self.sel_group.take() {
            self.items.retain(|p| p.group != g);
            self.items_changed.set(true);
            self.commit();
        } else if let Some(i) = self.sel.take() {
            if i < self.vps.len() {
                self.vps.remove(i);
            }
            self.commit();
        }
    }

    /// เขียนสำเนาทำงานกลับเข้า engine ของแบบ (ทำให้แบบ "มีงานค้าง" เพื่อให้เซฟ/autosave ตามทัน)
    fn commit(&self) {
        let (Some(win), Some(src)) = (self.win.upgrade(), self.src.upgrade()) else { return };
        {
            let pages = win.imp().pages.borrow();
            let Some(d_ref) = pages.get(&src) else { return };
            let Ok(mut d) = d_ref.try_borrow_mut() else { return };
            let changed = {
                let Some(sp) = d.layouts.iter_mut().find(|l| l.name == self.name) else { return };
                let items_ch = self.items_changed.replace(false);
                let ch = items_ch || sp.w != self.w || sp.h != self.h || sp.vps != self.vps;
                if ch {
                    sp.w = self.w;
                    sp.h = self.h;
                    sp.vps = self.vps.clone();
                    if items_ch {
                        sp.items = self.items.clone();
                    }
                }
                ch
            };
            if changed {
                d.dirty = true;
                d.layout_rev += 1;
            }
        }
        src.child().queue_draw();
    }
}

type Shared = Rc<RefCell<LayoutTab>>;

/// ตัวควบคุมที่ต้องซิงก์กับสถานะ (syncing = ตั้งค่าจากโค้ด ไม่ใช่ผู้ใช้ → ข้าม handler)
struct Ui {
    scale: gtk::DropDown,
    add: gtk::ToggleButton,
    paper: gtk::DropDown,
    orient: gtk::DropDown,
    syncing: Cell<bool>,
}

impl Ui {
    fn sync(&self, t: &LayoutTab) {
        self.syncing.set(true);
        let n = t.sel.and_then(|i| t.vps.get(i)).map(|v| v.n);
        self.scale.set_sensitive(n.is_some());
        // ช่อง 0 = "—" (ไม่ได้เลือก หรือสเกลที่ไม่ใช่สเกลมาตรฐาน)
        self.scale.set_selected(n.and_then(scale_index).map_or(0, |i| i as u32 + 1));
        self.add.set_active(t.add_mode);
        self.paper.set_selected(paper_index(t.w, t.h) as u32);
        self.orient.set_selected(if t.w >= t.h { 0 } else { 1 });
        self.syncing.set(false);
    }
}

/// ขนาดกระดาษมาตรฐานที่ตรงกับ w × h (ทั้งสองแนว); ไม่ตรง = ตัวสุดท้าย "Custom"
fn paper_index(w: f64, h: f64) -> usize {
    let (a, b) = (w.min(h), w.max(h));
    PAPERS
        .iter()
        .position(|p| (p.1 - a).abs() < 0.6 && (p.2 - b).abs() < 0.6)
        .unwrap_or(PAPERS.len())
}

fn dropdown(items: &[&str], sel: u32) -> gtk::DropDown {
    let d = gtk::DropDown::new(Some(gtk::StringList::new(items)), gtk::Expression::NONE);
    d.set_selected(sel);
    d
}

fn layout_of(page: &adw::TabPage) -> Option<Shared> {
    let child = page.child();
    // SAFETY: DATA_KEY ถูกตั้งด้วยชนิด Shared เท่านั้น (ใน new_layout_tab)
    unsafe { child.data::<Shared>(DATA_KEY).map(|p| p.as_ref().clone()) }
}

impl DraftingWindow {
    /// หน้าต่างแบบที่ Layout นี้ดูอยู่ (ถ้าเปิดอยู่เป็น Layout ใช้ต้นฉบับของมัน)
    fn layout_source(&self) -> Option<adw::TabPage> {
        let cur = self.imp().tab_view.selected_page()?;
        let src = match layout_of(&cur) {
            Some(t) => t.borrow().src.upgrade(),
            None => Some(cur),
        };
        let src = src?;
        self.imp().pages.borrow().contains_key(&src).then_some(src)
    }

    /// ชื่อ Layout ของแบบนี้ที่มีแท็บเปิดอยู่แล้ว
    fn open_layout_names(&self, src: &adw::TabPage) -> Vec<String> {
        let tv = &self.imp().tab_view;
        (0..tv.n_pages())
            .filter_map(|i| layout_of(&tv.nth_page(i)))
            .filter_map(|t| {
                let tb = t.borrow();
                (tb.src.upgrade().as_ref() == Some(src)).then(|| tb.name.clone())
            })
            .collect()
    }

    /// คำสั่ง `layout`: เปิด Layout ของแบบนี้ที่ปิดแท็บไปแล้ว; ถ้าเปิดครบแล้ว (หรือยังไม่มี) สร้างแผ่นใหม่
    pub(super) fn layout_command(&self) {
        let Some(src) = self.layout_source() else { return };
        let names: Vec<String> = {
            let pages = self.imp().pages.borrow();
            let Some(d_ref) = pages.get(&src) else { return };
            d_ref.borrow().layouts.iter().map(|l| l.name.clone()).collect()
        };
        let open = self.open_layout_names(&src);
        let closed: Vec<&String> = names.iter().filter(|n| !open.contains(n)).collect();
        if closed.is_empty() {
            self.create_layout(&src);
        } else {
            for n in closed {
                self.open_layout_tab(&src, n);
            }
        }
    }

    /// สร้าง Layout ใหม่ในแบบ (A3 แนวนอน) แล้วเปิดแท็บ
    fn create_layout(&self, src: &adw::TabPage) {
        let name = {
            let pages = self.imp().pages.borrow();
            let Some(d_ref) = pages.get(src) else { return };
            let mut d = d_ref.borrow_mut();
            let name = unique_name(&d.layouts, "");
            d.layouts.push(LayoutSpec::new(name.clone()));
            d.dirty = true;
            d.layout_rev += 1;
            name
        };
        src.child().queue_draw();
        self.open_layout_tab(src, &name);
    }

    /// เปิดแท็บของ Layout ทุกแผ่นในแบบ (ใช้หลังเปิดไฟล์) แล้วกลับมาที่แท็บแบบ
    pub(super) fn open_layout_tabs(&self, src: &adw::TabPage) {
        let names: Vec<String> = {
            let pages = self.imp().pages.borrow();
            let Some(d_ref) = pages.get(src) else { return };
            d_ref.borrow().layouts.iter().map(|l| l.name.clone()).collect()
        };
        if names.is_empty() {
            return;
        }
        for n in &names {
            self.open_layout_tab(src, n);
        }
        self.imp().tab_view.set_selected_page(src);
    }

    fn open_layout_tab(&self, src: &adw::TabPage, name: &str) {
        let imp = self.imp();
        // เปิดอยู่แล้ว → แค่สลับไป
        let tv = &imp.tab_view;
        for i in 0..tv.n_pages() {
            let p = tv.nth_page(i);
            if let Some(t) = layout_of(&p) {
                let tb = t.borrow();
                if tb.name == name && tb.src.upgrade().as_ref() == Some(src) {
                    drop(tb);
                    tv.set_selected_page(&p);
                    return;
                }
            }
        }
        let spec = {
            let pages = imp.pages.borrow();
            let Some(d_ref) = pages.get(src) else { return };
            let Some(sp) = d_ref.borrow().layouts.iter().find(|l| l.name == name).cloned() else {
                return;
            };
            sp
        };

        let canvas = gtk::DrawingArea::builder()
            .hexpand(true)
            .vexpand(true)
            .focusable(true)
            .build();

        let t: Shared = Rc::new(RefCell::new(LayoutTab {
            win: self.downgrade(),
            src: src.downgrade(),
            canvas: canvas.downgrade(),
            name: spec.name.clone(),
            w: spec.w,
            h: spec.h,
            mono: false,
            vps: spec.vps.clone(),
            sel: None,
            next_group: spec.items.iter().map(|p| p.group).max().unwrap_or(0) + 1,
            items: spec.items.clone(),
            sel_group: None,
            items_changed: Cell::new(false),
            view: PaperView { z: 1.0, ox: 0.0, oy: 0.0 },
            zt: 1.0,
            zanchor: ((0.0, 0.0), (0.0, 0.0)),
            fitted: false,
            add_mode: false,
            inside: false,
            drag: None,
            start: (0.0, 0.0),
            mouse: (0.0, 0.0),
            ui: None,
        }));

        // ---------- แถบเครื่องมือ ----------
        let mut names: Vec<&str> = PAPERS.iter().map(|p| p.0).collect();
        names.push("Custom");
        let labels: Vec<String> = std::iter::once("—".to_string())
            .chain(STD_SCALES.iter().map(|&n| scale_label(n)))
            .collect();
        let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
        let paper_dd = dropdown(&names, paper_index(spec.w, spec.h) as u32);
        let orient_dd = dropdown(&["Landscape", "Portrait"], if spec.w >= spec.h { 0 } else { 1 });
        let add_btn = gtk::ToggleButton::with_label("New Viewport");
        add_btn.set_tooltip_text(Some("Drag a rectangle on the sheet. It shows the current view of the drawing tab."));
        let inside_btn = gtk::ToggleButton::with_label("Edit Inside");
        inside_btn.set_tooltip_text(Some("Drag inside a viewport to pan the drawing; scroll to change its scale."));
        let scale_dd = dropdown(&label_refs, 0);
        scale_dd.set_sensitive(false);
        let from_btn = gtk::Button::with_label("From Drawing View");
        from_btn.set_tooltip_text(Some("Show what the drawing tab currently shows in the selected viewport."));
        let del_btn = gtk::Button::with_label("Delete");
        let scale_obj_btn = gtk::Button::with_label("Scale Objects…");
        scale_obj_btn.set_tooltip_text(Some("Scale the selected objects on the sheet (or drag the square at their bottom-right corner)."));
        let mono_dd = dropdown(&["Colour", "Black"], 0);
        let fit_btn = gtk::Button::with_label("Fit");
        let new_lay_btn = gtk::Button::with_label("New Layout");
        let del_lay_btn = gtk::Button::with_label("Delete Layout");
        let print_btn = gtk::Button::with_label("Print");

        // toolbar_overlay: แถบลอยสไตล์เดียวกับ dock ของแอป วางทับกระดาษ ไม่กินที่
        let bar = gtk::Box::builder().spacing(6).margin_top(4).margin_bottom(4).margin_start(8).margin_end(8).build();
        let lbl = |s: &str| gtk::Label::new(Some(s));
        for w in [
            lbl("Paper").upcast::<gtk::Widget>(),
            paper_dd.clone().upcast(),
            orient_dd.clone().upcast(),
            gtk::Separator::new(gtk::Orientation::Vertical).upcast(),
            add_btn.clone().upcast(),
            inside_btn.clone().upcast(),
            lbl("Scale").upcast(),
            scale_dd.clone().upcast(),
            from_btn.clone().upcast(),
            del_btn.clone().upcast(),
            scale_obj_btn.clone().upcast(),
            gtk::Separator::new(gtk::Orientation::Vertical).upcast(),
            lbl("Lines").upcast(),
            mono_dd.clone().upcast(),
            fit_btn.clone().upcast(),
            gtk::Separator::new(gtk::Orientation::Vertical).upcast(),
            new_lay_btn.clone().upcast(),
            del_lay_btn.clone().upcast(),
            print_btn.clone().upcast(),
        ] {
            bar.append(&w);
        }
        let scrolled = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Automatic)
            .vscrollbar_policy(gtk::PolicyType::Never)
            .propagate_natural_width(true)
            .propagate_natural_height(true)
            .child(&bar)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Start)
            // เว้นสองข้างให้พ้นปุ่มไฟล์ (ซ้ายบน) และปุ่มหน้าต่าง (ขวาบน)
            .margin_top(8)
            .margin_start(224)
            .margin_end(176)
            .build();
        for c in ["osd", "toolbar", "drafting-floating"] {
            scrolled.add_css_class(c);
        }
        let root = gtk::Overlay::new();
        root.set_child(Some(&canvas));
        root.add_overlay(&scrolled);
        // SAFETY: อ่านกลับด้วยชนิดเดียวกันใน layout_of
        unsafe { root.set_data::<Shared>(DATA_KEY, t.clone()) };

        let ui = Rc::new(Ui {
            scale: scale_dd.clone(),
            add: add_btn.clone(),
            paper: paper_dd.clone(),
            orient: orient_dd.clone(),
            syncing: Cell::new(false),
        });
        t.borrow_mut().ui = Some(ui.clone());

        // ---------- วาด ----------
        canvas.set_draw_func(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[strong]
            t,
            move |_, cr, w, h| {
                {
                    let mut tb = t.borrow_mut();
                    if !tb.fitted {
                        let (pw, ph) = tb.dims();
                        // เว้นบนให้แถบเครื่องมือ ล่างให้ Command bar
                        let (top, bottom) = (72.0, 84.0);
                        let mut v = PaperView::fit(w as f64, (h as f64 - top - bottom).max(100.0), pw, ph);
                        v.oy += top;
                        tb.view = v;
                        tb.zt = v.z;
                        tb.fitted = true;
                    }
                }
                obj.layout_draw(&t.borrow(), cr, w as f64, h as f64);
            }
        ));

        // ---------- ซูม ----------
        let scroll = gtk::EventControllerScroll::new(gtk::EventControllerScrollFlags::VERTICAL);
        scroll.connect_scroll(glib::clone!(
            #[strong]
            t,
            #[strong]
            ui,
            move |_, _dx, dy| {
                {
                    let mut tb = t.borrow_mut();
                    let (mx, my) = tb.mouse;
                    let (px, py) = tb.view.to_paper(mx, my);
                    let over_sel = tb.inside
                        && tb.sel.and_then(|i| tb.vps.get(i)).is_some_and(|v| v.contains(px, py));
                    if over_sel {
                        let i = tb.sel.unwrap();
                        let n = tb.vps[i].n;
                        tb.vps[i].set_scale(step_scale(n, if dy < 0.0 { -1 } else { 1 }));
                        tb.commit();
                    } else {
                        // ซูมนุ่ม: ตั้งเป้าหมาย แล้วให้ tick ค่อยๆ เลื่อนเข้าหา (ยึดจุดเคอร์เซอร์)
                        if (tb.zt - tb.view.z).abs() < 1e-9 {
                            tb.zanchor = ((mx, my), (px, py));
                        } else {
                            tb.zanchor = ((mx, my), tb.view.to_paper(mx, my));
                        }
                        let k = if dy < 0.0 { 1.2 } else { 1.0 / 1.2 };
                        tb.zt = (tb.zt * k).clamp(0.05, 200.0);
                    }
                    ui.sync(&tb);
                    tb.redraw();
                }
                glib::Propagation::Stop
            }
        ));
        canvas.add_controller(scroll);

        // tick: เลื่อน z เข้าหาเป้าหมายแบบ exponential (~120 ms) แล้วหยุดเมื่อถึง
        let last_us = Cell::new(0i64);
        canvas.add_tick_callback(glib::clone!(
            #[strong]
            t,
            move |area, clock| {
                let now = clock.frame_time();
                let dt = ((now - last_us.replace(now)) as f64 / 1e6).clamp(0.0, 0.1);
                let Ok(mut tb) = t.try_borrow_mut() else { return glib::ControlFlow::Continue };
                if (tb.zt - tb.view.z).abs() < 1e-9 {
                    return glib::ControlFlow::Continue;
                }
                let k = 1.0 - (-dt / 0.04).exp();
                let mut z = tb.view.z + (tb.zt - tb.view.z) * k;
                if (z - tb.zt).abs() < tb.zt * 0.002 {
                    z = tb.zt;
                }
                let ((sx, sy), (px, py)) = tb.zanchor;
                tb.view.z = z;
                tb.view.ox = sx - px * z;
                tb.view.oy = sy - py * z;
                area.queue_draw();
                glib::ControlFlow::Continue
            }
        ));

        let motion = gtk::EventControllerMotion::new();
        motion.connect_motion(glib::clone!(
            #[strong]
            t,
            move |_, x, y| {
                t.borrow_mut().mouse = (x, y);
            }
        ));
        canvas.add_controller(motion);

        // ---------- ปุ่มกลางลาก = เลื่อนกระดาษ ----------
        let pan = gtk::GestureDrag::builder().button(2).build();
        pan.connect_drag_begin(glib::clone!(
            #[strong]
            t,
            move |_, _, _| {
                let mut tb = t.borrow_mut();
                tb.zt = tb.view.z;
                tb.drag = Some(Drag::PanView { ox: tb.view.ox, oy: tb.view.oy });
            }
        ));
        pan.connect_drag_update(glib::clone!(
            #[strong]
            t,
            move |_, dx, dy| {
                let mut tb = t.borrow_mut();
                if let Some(Drag::PanView { ox, oy }) = tb.drag {
                    tb.view.ox = ox + dx;
                    tb.view.oy = oy + dy;
                    tb.redraw();
                }
            }
        ));
        pan.connect_drag_end(glib::clone!(
            #[strong]
            t,
            move |_, _, _| {
                t.borrow_mut().drag = None;
            }
        ));
        canvas.add_controller(pan);

        // ---------- ปุ่มซ้าย: สร้าง / เลือก / ย้าย / ย่อขยาย / เลื่อนเนื้อหา ----------
        let drag = gtk::GestureDrag::builder().button(1).build();
        drag.connect_drag_begin(glib::clone!(
            #[strong]
            t,
            #[strong]
            ui,
            #[weak]
            canvas,
            move |_, x, y| {
                canvas.grab_focus();
                let mut tb = t.borrow_mut();
                tb.start = (x, y);
                let (px, py) = tb.view.to_paper(x, y);
                tb.drag = None;
                if tb.add_mode {
                    tb.drag = Some(Drag::Create { x0: px, y0: py, x1: px, y1: py });
                } else if let Some(h) = scale_handle(&tb, px, py) {
                    tb.drag = Some(h);
                } else if let Some(g) = pick_group(&tb.items, px, tb.h - py, 6.0 / tb.view.z) {
                    tb.sel = None;
                    tb.sel_group = Some(g);
                    tb.drag = Some(Drag::MoveGroup { g, lx: 0.0, ly: 0.0 });
                } else {
                    tb.sel_group = None;
                    let tol = 8.0 / tb.view.z;
                    let on_corner = tb
                        .sel
                        .filter(|&i| tb.vps.get(i).is_some_and(|v| v.corner_hit(px, py, tol)));
                    if let Some(i) = on_corner {
                        tb.drag = Some(Drag::Resize { i });
                    } else if let Some(i) = tb.vps.iter().rposition(|v| v.contains(px, py)) {
                        tb.sel = Some(i);
                        tb.drag = Some(if tb.inside {
                            Drag::PanContent { i, lx: 0.0, ly: 0.0 }
                        } else {
                            Drag::Move { i, ox: tb.vps[i].x, oy: tb.vps[i].y }
                        });
                    } else {
                        tb.sel = None;
                    }
                }
                ui.sync(&tb);
                tb.redraw();
            }
        ));
        drag.connect_drag_update(glib::clone!(
            #[strong]
            t,
            move |_, dx, dy| {
                let mut tb = t.borrow_mut();
                let z = tb.view.z;
                let (sx, sy) = tb.start;
                let (px, py) = tb.view.to_paper(sx + dx, sy + dy);
                // เอา drag ออกมาก่อนเพื่อแก้ tb ได้โดยไม่ยืมซ้อน
                let Some(mut d) = tb.drag.take() else { return };
                let mut edited = true;
                match &mut d {
                    Drag::Create { x1, y1, .. } => {
                        *x1 = px;
                        *y1 = py;
                        edited = false;
                    }
                    Drag::Move { i, ox, oy } => {
                        if let Some(v) = tb.vps.get_mut(*i) {
                            v.x = *ox + dx / z;
                            v.y = *oy + dy / z;
                        }
                    }
                    Drag::Resize { i } => {
                        if let Some(v) = tb.vps.get_mut(*i) {
                            v.resize_to(px, py);
                        }
                    }
                    Drag::PanContent { i, lx, ly } => {
                        let (nx, ny) = (dx / z, dy / z);
                        if let Some(v) = tb.vps.get_mut(*i) {
                            v.pan_content(nx - *lx, ny - *ly);
                        }
                        *lx = nx;
                        *ly = ny;
                    }
                    Drag::PanView { .. } => edited = false,
                    Drag::ScaleGroup { g, ax, ay, d0, orig } => {
                        let (qx, qy) = (px, tb.h - py);
                        let k = ((qx - *ax).hypot(qy - *ay) / *d0).clamp(0.01, 100.0);
                        let mut n = 0;
                        for it in tb.items.iter_mut().filter(|p| p.group == *g) {
                            if let Some(o) = orig.get(n) {
                                it.item.data = o.clone();
                                it.item.data.scale_about(Point { x: *ax, y: *ay }, k);
                            }
                            n += 1;
                        }
                        tb.items_changed.set(true);
                    }
                    Drag::MoveGroup { g, lx, ly } => {
                        let (nx, ny) = (dx / z, -dy / z);
                        let (ddx, ddy) = (nx - *lx, ny - *ly);
                        for it in tb.items.iter_mut().filter(|p| p.group == *g) {
                            it.item.data.translate(ddx, ddy);
                        }
                        *lx = nx;
                        *ly = ny;
                        tb.items_changed.set(true);
                    }
                }
                tb.drag = Some(d);
                if edited {
                    tb.commit();
                }
                tb.redraw();
            }
        ));
        drag.connect_drag_end(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[strong]
            t,
            #[strong]
            ui,
            move |_, _, _| {
                let created = {
                    let mut tb = t.borrow_mut();
                    let d = tb.drag.take();
                    if let Some(Drag::Create { x0, y0, x1, y1 }) = d {
                        let (w, h) = ((x1 - x0).abs(), (y1 - y0).abs());
                        if w >= 2.0 * MIN_VP && h >= 2.0 * MIN_VP {
                            let mut v = Viewport {
                                x: x0.min(x1),
                                y: y0.min(y1),
                                w,
                                h,
                                cx: 0.0,
                                cy: 0.0,
                                n: 100.0,
                            };
                            if let Some(src) = tb.src.upgrade() {
                                if let Some((a, b, c, e)) = obj.model_view_area(&src) {
                                    v.show_area(a, b, c, e);
                                }
                            }
                            tb.vps.push(v);
                            tb.sel = Some(tb.vps.len() - 1);
                            tb.add_mode = false;
                            tb.commit();
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                };
                if created {
                    ui.sync(&t.borrow());
                }
                t.borrow().redraw();
            }
        ));
        canvas.add_controller(drag);

        // ---------- ปุ่มต่าง ๆ ----------
        paper_dd.connect_selected_notify(glib::clone!(
            #[strong]
            t,
            #[strong]
            ui,
            move |d| {
                if ui.syncing.get() {
                    return;
                }
                let i = d.selected() as usize;
                let mut tb = t.borrow_mut();
                if let Some(&(_, a, b)) = PAPERS.get(i) {
                    let landscape = tb.w >= tb.h;
                    (tb.w, tb.h) = if landscape { (b, a) } else { (a, b) };
                    tb.fitted = false;
                    tb.commit();
                }
                ui.sync(&tb);
                tb.redraw();
            }
        ));
        orient_dd.connect_selected_notify(glib::clone!(
            #[strong]
            t,
            #[strong]
            ui,
            move |d| {
                if ui.syncing.get() {
                    return;
                }
                let want_landscape = d.selected() == 0;
                let mut tb = t.borrow_mut();
                let (a, b) = (tb.w.min(tb.h), tb.w.max(tb.h));
                (tb.w, tb.h) = if want_landscape { (b, a) } else { (a, b) };
                tb.fitted = false;
                tb.commit();
                tb.redraw();
            }
        ));
        mono_dd.connect_selected_notify(glib::clone!(
            #[strong]
            t,
            move |d| {
                let mut tb = t.borrow_mut();
                tb.mono = d.selected() == 1;
                tb.redraw();
            }
        ));
        add_btn.connect_toggled(glib::clone!(
            #[strong]
            t,
            #[strong]
            ui,
            move |b| {
                if ui.syncing.get() {
                    return;
                }
                t.borrow_mut().add_mode = b.is_active();
            }
        ));
        inside_btn.connect_toggled(glib::clone!(
            #[strong]
            t,
            move |b| {
                t.borrow_mut().inside = b.is_active();
            }
        ));
        scale_dd.connect_selected_notify(glib::clone!(
            #[strong]
            t,
            #[strong]
            ui,
            move |d| {
                if ui.syncing.get() {
                    return;
                }
                let i = d.selected() as usize;
                if i == 0 {
                    return;
                }
                let mut tb = t.borrow_mut();
                if let (Some(s), Some(&n)) = (tb.sel, STD_SCALES.get(i - 1)) {
                    if let Some(v) = tb.vps.get_mut(s) {
                        v.set_scale(n);
                    }
                    tb.commit();
                }
                tb.redraw();
            }
        ));
        from_btn.connect_clicked(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[strong]
            t,
            #[strong]
            ui,
            move |_| {
                let mut tb = t.borrow_mut();
                let Some(src) = tb.src.upgrade() else { return };
                let Some((a, b, c, e)) = obj.model_view_area(&src) else { return };
                if let Some(v) = tb.sel.and_then(|i| tb.vps.get_mut(i)) {
                    v.show_area(a, b, c, e);
                    tb.commit();
                }
                ui.sync(&tb);
                tb.redraw();
            }
        ));
        del_btn.connect_clicked(glib::clone!(
            #[strong]
            t,
            #[strong]
            ui,
            move |_| {
                let mut tb = t.borrow_mut();
                tb.delete_selection();
                ui.sync(&tb);
                tb.redraw();
            }
        ));
        fit_btn.connect_clicked(glib::clone!(
            #[strong]
            t,
            move |_| {
                let mut tb = t.borrow_mut();
                tb.fitted = false;
                tb.redraw();
            }
        ));
        new_lay_btn.connect_clicked(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[strong]
            t,
            move |_| {
                let src = t.borrow().src.upgrade();
                if let Some(src) = src {
                    obj.create_layout(&src);
                }
            }
        ));
        del_lay_btn.connect_clicked(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[strong]
            t,
            #[weak]
            root,
            move |_| obj.confirm_delete_layout(&t, &root)
        ));
        scale_obj_btn.connect_clicked(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[strong]
            t,
            move |_| obj.prompt_scale_group(&t)
        ));
        print_btn.connect_clicked(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[strong]
            t,
            move |_| obj.layout_print_tab(&t)
        ));

        let page = imp.tab_view.add_page(&root, None);
        page.set_title(&format!("{} – {}", spec.name, src.title()));
        imp.tab_view.set_selected_page(&page);
        canvas.grab_focus();
    }

    fn confirm_delete_layout(&self, t: &Shared, root: &gtk::Overlay) {
        let name = t.borrow().name.clone();
        let dialog = adw::AlertDialog::new(
            Some("Delete layout?"),
            Some(&format!("“{}” and its viewports will be removed from the drawing.", name)),
        );
        dialog.add_responses(&[("cancel", "_Cancel"), ("delete", "_Delete")]);
        dialog.set_response_appearance("delete", adw::ResponseAppearance::Destructive);
        dialog.set_default_response(Some("cancel"));
        dialog.set_close_response("cancel");
        dialog.connect_response(
            None,
            glib::clone!(
                #[weak(rename_to = obj)]
                self,
                #[strong]
                t,
                #[weak]
                root,
                move |_, response| {
                    if response != "delete" {
                        return;
                    }
                    let src = t.borrow().src.upgrade();
                    if let Some(src) = &src {
                        if let Some(d_ref) = obj.imp().pages.borrow().get(src) {
                            let mut d = d_ref.borrow_mut();
                            let name = t.borrow().name.clone();
                            d.layouts.retain(|l| l.name != name);
                            d.dirty = true;
                            d.layout_rev += 1;
                        }
                        src.child().queue_draw();
                    }
                    let tv = &obj.imp().tab_view;
                    let page = tv.page(&root);
                    tv.close_page(&page);
                }
            ),
        );
        dialog.present(Some(self));
    }

    /// พื้นที่ model (x0,y0,x1,y1) ที่แท็บต้นฉบับกำลังแสดงอยู่
    fn model_view_area(&self, src: &adw::TabPage) -> Option<(f64, f64, f64, f64)> {
        let (w, h) = (src.child().width() as f64, src.child().height() as f64);
        if w < 1.0 || h < 1.0 {
            return None;
        }
        let pages = self.imp().pages.borrow();
        let d = pages.get(src)?.try_borrow().ok()?;
        let a = d.camera.screen_to_world(0.0, 0.0);
        let b = d.camera.screen_to_world(w, h);
        Some((a.0.min(b.0), a.1.min(b.1), a.0.max(b.0), a.1.max(b.1)))
    }

    /// คำสั่งวาดของ Viewport หนึ่งอัน (พิกัดท้องถิ่นของกรอบ หน่วย "px" = PX_MM มม.)
    fn vp_commands(&self, src: &adw::TabPage, vp: &Viewport, mono: bool) -> Option<Vec<RenderCommand>> {
        let pages = self.imp().pages.borrow();
        let mut d = pages.get(src)?.try_borrow_mut().ok()?;
        let (bw, bh) = (vp.w / PX_MM, vp.h / PX_MM);
        let (s, ox, oy) = vp.camera(PX_MM);
        let saved = (d.camera.scale, d.camera.offset);
        d.camera.scale = s;
        d.camera.offset = (ox, oy);
        let cmds = d.get_render_commands_in(0.0, 0.0, bw, bh);
        d.camera.scale = saved.0;
        d.camera.offset = saved.1;
        Some(print_prepare(cmds, mono))
    }

    fn layout_draw(&self, t: &LayoutTab, cr: &gtk::cairo::Context, _w: f64, _h: f64) {
        cr.set_source_rgb(0.78, 0.78, 0.80);
        let _ = cr.paint();
        let (pw, ph) = t.dims();
        let v = t.view;
        let (x0, y0) = v.to_screen(0.0, 0.0);
        cr.set_source_rgba(0.0, 0.0, 0.0, 0.25);
        cr.rectangle(x0 + 4.0, y0 + 4.0, pw * v.z, ph * v.z);
        let _ = cr.fill();
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.rectangle(x0, y0, pw * v.z, ph * v.z);
        let _ = cr.fill();

        let src = t.src.upgrade();
        for (i, vp) in t.vps.iter().enumerate() {
            let (sx, sy) = v.to_screen(vp.x, vp.y);
            let (sw, sh) = (vp.w * v.z, vp.h * v.z);
            let _ = cr.save();
            cr.rectangle(sx, sy, sw, sh);
            cr.clip();
            match src.as_ref().and_then(|p| self.vp_commands(p, vp, t.mono)) {
                Some(cmds) => {
                    cr.translate(sx, sy);
                    cr.scale(v.z * PX_MM, v.z * PX_MM);
                    CairoRenderer::new(cr).draw_commands(cmds);
                }
                None => {
                    cr.set_source_rgb(0.5, 0.5, 0.5);
                    cr.move_to(sx + 8.0, sy + 20.0);
                    let _ = cr.show_text("Drawing not available");
                }
            }
            let _ = cr.restore();

            let selected = t.sel == Some(i);
            if selected {
                cr.set_source_rgb(0.10, 0.40, 0.90);
                cr.set_line_width(2.0);
            } else {
                cr.set_source_rgb(0.25, 0.25, 0.25);
                cr.set_line_width(1.0);
            }
            cr.rectangle(sx, sy, sw, sh);
            let _ = cr.stroke();
            if selected {
                cr.rectangle(sx + sw - 5.0, sy + sh - 5.0, 10.0, 10.0);
                let _ = cr.fill();
                // ป้ายสเกล
                cr.set_source_rgb(0.10, 0.40, 0.90);
                cr.move_to(sx + 4.0, sy - 5.0);
                let _ = cr.show_text(&scale_label(vp.n));
            }
        }

        // ของบนกระดาษอยู่เหนือ viewport
        let cmds = paper_commands(&t.items, pw, ph, t.mono);
        if !cmds.is_empty() {
            let _ = cr.save();
            cr.translate(x0, y0);
            cr.scale(v.z * PX_MM, v.z * PX_MM);
            CairoRenderer::new(cr).draw_commands(cmds);
            let _ = cr.restore();
        }
        if let Some(g) = t.sel_group {
            let group: Vec<ClipItem> = t.items.iter().filter(|p| p.group == g).map(|p| p.item.clone()).collect();
            if let Some((bx0, by0, bx1, by1)) = clip_bounds(&group) {
                let (a, b) = v.to_screen(bx0, ph - by1);
                cr.set_source_rgb(0.10, 0.40, 0.90);
                cr.set_line_width(1.0);
                cr.set_dash(&[6.0, 4.0], 0.0);
                cr.rectangle(a - 3.0, b - 3.0, (bx1 - bx0) * v.z + 6.0, (by1 - by0) * v.z + 6.0);
                let _ = cr.stroke();
                cr.set_dash(&[], 0.0);
                // จับมุมขวาล่างเพื่อย่อ/ขยาย
                let (hx, hy) = v.to_screen(bx1, ph - by0);
                cr.rectangle(hx - 5.0, hy - 5.0, 10.0, 10.0);
                let _ = cr.fill();
            }
        }

        if let Some(Drag::Create { x0: ax, y0: ay, x1, y1 }) = t.drag {
            let (a, b) = v.to_screen(ax.min(x1), ay.min(y1));
            cr.set_source_rgb(0.10, 0.40, 0.90);
            cr.set_line_width(1.0);
            cr.set_dash(&[6.0, 4.0], 0.0);
            cr.rectangle(a, b, (x1 - ax).abs() * v.z, (y1 - ay).abs() * v.z);
            let _ = cr.stroke();
            cr.set_dash(&[], 0.0);
        }
    }

    /// ซ่อน dock ลอยด้านซ้าย/ขวาที่บังกระดาษตอนอยู่ในแท็บ Layout และคืนเมื่อกลับไปแท็บแบบ
    pub(super) fn sync_layout_chrome(&self) {
        let imp = self.imp();
        let is_layout = imp.tab_view.selected_page().is_some_and(|p| layout_of(&p).is_some());
        if is_layout {
            let Some(overlay) = imp.tab_view.parent() else { return };
            let mut child = overlay.first_child();
            while let Some(c) = child {
                let next = c.next_sibling();
                let side = c.valign() == gtk::Align::Center
                    && matches!(c.halign(), gtk::Align::Start | gtk::Align::End);
                if side && c.is_visible() {
                    c.set_visible(false);
                    HIDDEN.with(|h| h.borrow_mut().push(c.downgrade()));
                }
                child = next;
            }
        } else {
            HIDDEN.with(|h| {
                for w in h.borrow_mut().drain(..) {
                    if let Some(w) = w.upgrade() {
                        w.set_visible(true);
                    }
                }
            });
        }
    }

    /// ถ้าแท็บที่เปิดอยู่เป็น Layout: จัดการ Delete / Esc ให้ แล้วคืน true
    pub(super) fn layout_key(&self, key: gdk::Key) -> bool {
        let Some(page) = self.imp().tab_view.selected_page() else { return false };
        let Some(t) = layout_of(&page) else { return false };
        let mut tb = t.borrow_mut();
        if key == gdk::Key::Delete || key == gdk::Key::KP_Delete {
            tb.delete_selection();
        } else if key == gdk::Key::Escape {
            if tb.add_mode {
                tb.add_mode = false;
            } else {
                tb.sel = None;
                tb.sel_group = None;
            }
        } else {
            return false;
        }
        if let Some(ui) = tb.ui.clone() {
            ui.sync(&tb);
        }
        tb.redraw();
        true
    }

    /// Ctrl+P ในแท็บ Layout: พิมพ์ทั้งแผ่น; คืน true ถ้าจัดการแล้ว
    pub(super) fn layout_print(&self) -> bool {
        let Some(page) = self.imp().tab_view.selected_page() else { return false };
        let Some(t) = layout_of(&page) else { return false };
        self.layout_print_tab(&t);
        true
    }

    fn layout_print_tab(&self, t: &Shared) {
        let (src, pw, ph, mono, vps, name) = {
            let tb = t.borrow();
            (tb.src.upgrade(), tb.w, tb.h, tb.mono, tb.vps.clone(), tb.name.clone())
        };
        let Some(src) = src else {
            self.show_error("Can't print", "The drawing for this layout is closed.");
            return;
        };
        let jobs: Vec<(Viewport, Vec<RenderCommand>)> = vps
            .iter()
            .filter_map(|vp| self.vp_commands(&src, vp, mono).map(|c| (*vp, c)))
            .collect();
        let paper_cmds = paper_commands(&t.borrow().items, pw, ph, mono);
        if jobs.is_empty() && paper_cmds.is_empty() {
            self.show_error("Nothing to print", "Add a viewport or paste something onto the sheet first.");
            return;
        }
        let landscape = pw >= ph;
        let (pw0, ph0) = (pw.min(ph), pw.max(ph));

        let setup = gtk::PageSetup::new();
        setup.set_paper_size(&gtk::PaperSize::new_custom("drafting", "Drafting", pw0, ph0, gtk::Unit::Mm));
        setup.set_orientation(if landscape {
            gtk::PageOrientation::Landscape
        } else {
            gtk::PageOrientation::Portrait
        });
        setup.set_top_margin(0.0, gtk::Unit::Mm);
        setup.set_bottom_margin(0.0, gtk::Unit::Mm);
        setup.set_left_margin(0.0, gtk::Unit::Mm);
        setup.set_right_margin(0.0, gtk::Unit::Mm);

        let op = gtk::PrintOperation::new();
        op.set_job_name(&name);
        op.set_n_pages(1);
        op.set_unit(gtk::Unit::Mm);
        op.set_default_page_setup(Some(&setup));
        op.connect_draw_page(move |_, ctx, _| {
            let cr = ctx.cairo_context();
            let (dx, dy) = (((ctx.width() - pw) / 2.0).max(0.0), ((ctx.height() - ph) / 2.0).max(0.0));
            for (vp, cmds) in &jobs {
                let _ = cr.save();
                cr.translate(dx + vp.x, dy + vp.y);
                cr.rectangle(0.0, 0.0, vp.w, vp.h);
                cr.clip();
                cr.scale(PX_MM, PX_MM);
                CairoRenderer::new(&cr).draw_commands(cmds.clone());
                let _ = cr.restore();
            }
            if !paper_cmds.is_empty() {
                let _ = cr.save();
                cr.translate(dx, dy);
                cr.scale(PX_MM, PX_MM);
                CairoRenderer::new(&cr).draw_commands(paper_cmds.clone());
                let _ = cr.restore();
            }
        });
        if let Err(e) = op.run(gtk::PrintOperationAction::PrintDialog, Some(self)) {
            self.show_error("Can't print", &e.to_string());
        }
    }
}


/// กรอบรวมของชุดของบนกระดาษที่คลิกโดน (ใกล้เส้นจริง tol มม.); พิกัดกระดาษ แกน y ขึ้น
fn pick_group(items: &[PaperItem], x: f64, y: f64, tol: f64) -> Option<u32> {
    let p = Point { x, y };
    items.iter().rev().find(|it| it.item.data.distance_to(p) <= tol).map(|it| it.group)
}

/// คำสั่งวาดของบนกระดาษ (หน่วย "px" = PX_MM มม. จากมุมซ้ายบนของกระดาษ)
/// ใช้ Engine ชั่วคราวเป็นตัวสร้างคำสั่งวาด เหมือนที่ viewport ใช้กับแบบ
fn paper_commands(items: &[PaperItem], w: f64, h: f64, mono: bool) -> Vec<RenderCommand> {
    if items.is_empty() {
        return Vec::new();
    }
    let mut e = crate::engine::Engine::new();
    let clips: Vec<ClipItem> = items.iter().map(|p| p.item.clone()).collect();
    e.paste_items(&clips, 0.0, 0.0);
    e.camera.scale = 1.0 / PX_MM;
    e.camera.offset = (0.0, h / PX_MM);
    let cmds = e.get_render_commands_in(0.0, 0.0, w / PX_MM, h / PX_MM);
    print_prepare(cmds, mono)
}

thread_local! {
    /// viewport ที่ copy ไว้ (ถ้ามี จะถูกวางแทนของที่ copy จากแบบ)
    static VP_CLIP: RefCell<Option<Viewport>> = const { RefCell::new(None) };
}

/// copy จากแท็บแบบ → ทิ้ง viewport ที่ copy ไว้ (ไม่ให้ Ctrl+V วางผิดชิ้น)
pub(super) fn clear_vp_clip() {
    VP_CLIP.with(|c| *c.borrow_mut() = None);
}

#[derive(Clone, Copy, PartialEq)]
pub(super) enum ClipOp {
    Copy,
    Cut,
    Paste,
    PasteInPlace,
}

impl DraftingWindow {
    /// Copy/Cut/Paste ในแท็บ Layout; คืน false ถ้าแท็บที่เปิดอยู่ไม่ใช่ Layout
    pub(super) fn layout_clip(&self, op: ClipOp) -> bool {
        let Some(page) = self.imp().tab_view.selected_page() else { return false };
        let Some(t) = layout_of(&page) else { return false };
        let mut tb = t.borrow_mut();
        match op {
            ClipOp::Copy | ClipOp::Cut => {
                if let Some(g) = tb.sel_group {
                    let items: Vec<ClipItem> =
                        tb.items.iter().filter(|p| p.group == g).map(|p| p.item.clone()).collect();
                    if !items.is_empty() {
                        CLIPBOARD.with(|c| *c.borrow_mut() = items);
                        clear_vp_clip();
                    }
                } else if let Some(vp) = tb.sel.and_then(|i| tb.vps.get(i).copied()) {
                    VP_CLIP.with(|c| *c.borrow_mut() = Some(vp));
                } else {
                    return true;
                }
                if op == ClipOp::Cut {
                    tb.delete_selection();
                }
            }
            ClipOp::Paste | ClipOp::PasteInPlace => {
                let (mx, my) = tb.mouse;
                let (px, py) = tb.view.to_paper(mx, my);
                if let Some(mut vp) = VP_CLIP.with(|c| *c.borrow()) {
                    // วางซ้ำ: เลื่อนจากต้นฉบับเล็กน้อย (หรือไปที่เมาส์ถ้าไม่ใช่ in place)
                    if op == ClipOp::PasteInPlace {
                        vp.x += 10.0;
                        vp.y += 10.0;
                    } else {
                        vp.x = px - vp.w / 2.0;
                        vp.y = py - vp.h / 2.0;
                    }
                    tb.vps.push(vp);
                    tb.sel = Some(tb.vps.len() - 1);
                    tb.sel_group = None;
                    tb.commit();
                } else {
                    let items = CLIPBOARD.with(|c| c.borrow().clone());
                    let Some((x0, y0, x1, y1)) = clip_bounds(&items) else { return true };
                    let (dx, dy) = if op == ClipOp::PasteInPlace {
                        (0.0, 0.0)
                    } else {
                        (px - (x0 + x1) / 2.0, (tb.h - py) - (y0 + y1) / 2.0)
                    };
                    let g = tb.next_group;
                    tb.next_group += 1;
                    for mut it in items {
                        it.data.translate(dx, dy);
                        tb.items.push(PaperItem { group: g, item: it });
                    }
                    tb.sel = None;
                    tb.sel_group = Some(g);
                    tb.items_changed.set(true);
                    tb.commit();
                }
            }
        }
        if let Some(ui) = tb.ui.clone() {
            ui.sync(&tb);
        }
        tb.redraw();
        true
    }
}

/// เขียนของบนกระดาษลง paper space ของ layout ใน DXF
/// (ใช้ตัวแปลงเดียวกับโมเดล: แปลงผ่าน Engine ชั่วคราว แล้วย้ายเอนทิตีเข้า layout)
pub(super) fn write_paper_items(doc: &mut acadrust::CadDocument, specs: &[LayoutSpec]) -> Result<(), String> {
    for sp in specs {
        if sp.items.is_empty() {
            continue;
        }
        let mut e = crate::engine::Engine::new();
        let clips: Vec<ClipItem> = sp.items.iter().map(|p| p.item.clone()).collect();
        e.paste_items(&clips, 0.0, 0.0);
        let bytes = engine_to_dxf_bytes(&e)?;
        let tmp = acadrust::DxfReader::from_reader(std::io::Cursor::new(bytes))
            .and_then(|r| r.read())
            .map_err(|e| e.to_string())?;
        for l in tmp.layers.iter() {
            if !doc.layers.contains(&l.name) {
                let _ = doc.layers.add(l.clone());
            }
        }
        for lt in tmp.line_types.iter() {
            if !doc.line_types.contains(&lt.name) {
                let _ = doc.line_types.add(lt.clone());
            }
        }
        for ent in tmp.entities() {
            let mut en = ent.clone();
            en.common_mut().handle = acadrust::Handle::NULL;
            en.common_mut().owner_handle = acadrust::Handle::NULL;
            doc.add_entity_to_layout(en, &sp.name).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

/// อ่านของบนกระดาษ (เท่าที่โปรแกรมรองรับ) จาก paper space ของแต่ละ layout
pub(super) fn read_paper_items(doc: &acadrust::CadDocument, specs: &mut [LayoutSpec]) {
    use acadrust::objects::ObjectType;
    use acadrust::{CadDocument, EntityType, Handle};
    for sp in specs.iter_mut() {
        let Some(br) = doc.objects.values().find_map(|o| match o {
            ObjectType::Layout(l) if l.name == sp.name => Some(l.block_record),
            _ => None,
        }) else {
            continue;
        };
        let mut tmp = CadDocument::new();
        for l in doc.layers.iter() {
            if !tmp.layers.contains(&l.name) {
                let _ = tmp.layers.add(l.clone());
            } else if let Some(t) = tmp.layers.get_mut(&l.name) {
                t.color = l.color.clone();
            }
        }
        for lt in doc.line_types.iter() {
            if !tmp.line_types.contains(&lt.name) {
                let _ = tmp.line_types.add(lt.clone());
            }
        }
        let mut n = 0usize;
        for e in doc.entities() {
            if matches!(e, EntityType::Viewport(_)) || e.common().owner_handle != br {
                continue;
            }
            let mut en = e.clone();
            en.common_mut().handle = Handle::NULL;
            en.common_mut().owner_handle = Handle::NULL;
            if tmp.add_entity(en).is_ok() {
                n += 1;
            }
        }
        if n == 0 {
            continue;
        }
        let imp = dxf_to_elements(&tmp);
        for (i, (data, li, color, style)) in imp.items.into_iter().enumerate() {
            let layer = imp
                .layers
                .get(li)
                .cloned()
                .unwrap_or_else(|| crate::engine::Layer::new("0", (255, 255, 255)));
            sp.items.push(PaperItem { group: i as u32 + 1, item: ClipItem { layer, color, style, data } });
        }
    }
}


/// จับมุมขวาล่างของชุดที่เลือกอยู่ได้ไหม (px, py = พิกัดกระดาษ แกน y ลง)
fn scale_handle(tb: &LayoutTab, px: f64, py: f64) -> Option<Drag> {
    let g = tb.sel_group?;
    let datas: Vec<&PaperItem> = tb.items.iter().filter(|p| p.group == g).collect();
    let clips: Vec<ClipItem> = datas.iter().map(|p| p.item.clone()).collect();
    let (x0, y0, x1, y1) = clip_bounds(&clips)?;
    let tol = 8.0 / tb.view.z;
    if (px - x1).abs() > tol || (py - (tb.h - y0)).abs() > tol {
        return None;
    }
    let d0 = (x1 - x0).hypot(y0 - y1);
    if d0 < 1e-6 {
        return None;
    }
    Some(Drag::ScaleGroup {
        g,
        ax: x0,
        ay: y1,
        d0,
        orig: datas.iter().map(|p| p.item.data.clone()).collect(),
    })
}

impl DraftingWindow {
    /// ย่อ/ขยายชุดของบนกระดาษที่เลือก ตามตัวคูณที่พิมพ์ (ยึดกึ่งกลางของชุด)
    fn prompt_scale_group(&self, t: &Shared) {
        if t.borrow().sel_group.is_none() {
            self.show_error("Scale Objects", "Click the objects on the sheet that you pasted first.");
            return;
        }
        let dialog = adw::AlertDialog::new(Some("Scale objects"), Some("Scale factor (2 doubles the size, 0.5 halves it)."));
        dialog.add_responses(&[("cancel", "_Cancel"), ("ok", "_Scale")]);
        dialog.set_response_appearance("ok", adw::ResponseAppearance::Suggested);
        dialog.set_default_response(Some("ok"));
        dialog.set_close_response("cancel");
        let spin = gtk::SpinButton::with_range(0.01, 100.0, 0.1);
        spin.set_digits(3);
        spin.set_value(1.0);
        spin.set_activates_default(true);
        dialog.set_extra_child(Some(&spin));
        dialog.connect_response(
            None,
            glib::clone!(
                #[strong]
                t,
                move |_, response| {
                    if response != "ok" {
                        return;
                    }
                    let k = spin.value();
                    let mut tb = t.borrow_mut();
                    let Some(g) = tb.sel_group else { return };
                    let clips: Vec<ClipItem> =
                        tb.items.iter().filter(|p| p.group == g).map(|p| p.item.clone()).collect();
                    let Some((x0, y0, x1, y1)) = clip_bounds(&clips) else { return };
                    let c = Point { x: (x0 + x1) / 2.0, y: (y0 + y1) / 2.0 };
                    for it in tb.items.iter_mut().filter(|p| p.group == g) {
                        it.item.data.scale_about(c, k);
                    }
                    tb.items_changed.set(true);
                    tb.commit();
                    tb.redraw();
                }
            ),
        );
        dialog.present(Some(self));
    }
}
