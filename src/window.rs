/* window.rs
 *
 * Copyright 2026 Supakit Suptorranee
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use crate::engine::{
    DEFAULT_DIMENSION_OFFSET, DrawingElement, ElementData, Point, RenderCommand,
    ToolMode as DrawingMode, linear_dimension_geometry,
};
use adw::subclass::prelude::*;
use dxf;
use gtk::gdk;
use gtk::prelude::*;
use gtk::{gio, glib};
use std::cell::RefCell;

glib::wrapper! {
    pub struct DraftingWindow(ObjectSubclass<imp::DraftingWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

mod imp {
    use super::*;
    use gtk::CompositeTemplate;
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;

    #[derive(Debug, CompositeTemplate, Default)]
    #[template(resource = "/com/pungpondsalami/drafting/window.ui")]
    pub struct DraftingWindow {
        #[template_child]
        pub tab_view: TemplateChild<adw::TabView>,
        #[template_child]
        pub btn_line: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_circle: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_snap: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub cmd_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub btn_new: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_save: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_undo: TemplateChild<gtk::Button>,
        #[template_child]
        pub spin_rows: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub spin_cols: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub spin_spacing_x: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub spin_spacing_y: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub right_dock: TemplateChild<gtk::Stack>,
        #[template_child]
        pub btn_array: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_apply_array: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_cancel_array: TemplateChild<gtk::Button>,
        #[template_child]
        pub revealer_array_ui: TemplateChild<gtk::Revealer>,
        #[template_child]
        pub btn_redo: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_dimension: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_auto_pan: TemplateChild<gtk::ToggleButton>,

        pub current_mode: Cell<crate::engine::ToolMode>,
        pub mouse_pos: Cell<(f64, f64)>,
        pub start_pos: Cell<Option<(f64, f64)>>,
        pub snap_enabled: Cell<bool>,
        pub is_dialog_open: Cell<bool>,
        pub pages: RefCell<HashMap<adw::TabPage, RefCell<crate::engine::Engine>>>,
        pub array_base_pos: Cell<Option<(f64, f64)>>, // จุดเริ่มต้นของ Array
        pub array_count: Cell<i32>,                   // จำนวนที่จะ copy
        pub is_cancelling_array: Cell<bool>,
        pub drag_start: Cell<Option<(f64, f64)>>, // จุดเริ่มลาก (screen)
        pub is_dragging: Cell<bool>,
        pub is_mouse_pressed: Cell<bool>,
        pub dimension_step: Cell<u8>, // 0: เริ่ม, 1: ได้จุดแรก, 2: ได้จุดสอง (กำลังดึงระยะ)
        pub dim_temp_start: Cell<Option<Point>>,
        pub dim_temp_end: Cell<Option<Point>>,
        pub auto_pan_enabled: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DraftingWindow {
        const NAME: &'static str = "DraftingWindow";
        type Type = super::DraftingWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DraftingWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            // ตั้งค่าเริ่มต้น
            self.snap_enabled.set(true);
            self.btn_snap.set_active(true);
            self.current_mode.set(DrawingMode::Select);

            // สร้าง Controller สำหรับดักคีย์บอร์ดระดับ Window
            let key_ctrl = gtk::EventControllerKey::new();
            // ตั้งค่า Capture เพื่อให้ Window ดักคีย์ได้ก่อนที่ปุ่มหรือช่องพิมพ์จะแย่งไป
            key_ctrl.set_propagation_phase(gtk::PropagationPhase::Capture);

            key_ctrl.connect_key_pressed(glib::clone!(
                #[weak]
                obj,
                #[upgrade_or]
                glib::Propagation::Proceed,
                move |_, key, _, state| {
                    let is_ctrl = state.contains(gdk::ModifierType::CONTROL_MASK);

                    // 1. Undo: ต้องกด Ctrl + Z
                    if is_ctrl && (key == gdk::Key::z || key == gdk::Key::Z) {
                        obj.undo_action();
                        return glib::Propagation::Stop;
                    }

                    // 2. Snap: กด F9
                    // เช็กแค่ key == F9 อย่างเดียว
                    if key == gdk::Key::F9 {
                        let btn = &obj.imp().btn_snap;
                        let new_state = !btn.is_active();
                        btn.set_active(new_state); // สลับสถานะปุ่ม Snap บน UI
                        println!("F9 Pressed: Snap toggled to {}", new_state);
                        return glib::Propagation::Stop;
                    }

                    // 3. Cancel Action: กด Esc
                    if key == gdk::Key::Escape {
                        obj.imp().start_pos.set(None);
                        obj.imp().current_mode.set(DrawingMode::Select);
                        obj.imp().drag_start.set(None);

                        // ปิด revealer ก่อน แล้วค่อย switch stack
                        obj.imp().revealer_array_ui.set_reveal_child(false);
                        glib::timeout_add_local_once(
                            std::time::Duration::from_millis(300),
                            glib::clone!(
                                #[weak]
                                obj,
                                move || {
                                    obj.imp().right_dock.set_visible_child_name("default");
                                }
                            ),
                        );

                        if let Some(page) = obj.imp().tab_view.selected_page() {
                            let pages = obj.imp().pages.borrow();
                            if let Some(d_ref) = pages.get(&page) {
                                d_ref.borrow_mut().deselect_all();
                            }
                            page.child().queue_draw();
                        }
                        return glib::Propagation::Stop;
                    }

                    // 4. Zoom to Fit: Ctrl + F
                    if is_ctrl && (key == gdk::Key::f || key == gdk::Key::F) {
                        if let Some(page) = obj.imp().tab_view.selected_page() {
                            let pages = obj.imp().pages.borrow();
                            if let Some(d_ref) = pages.get(&page) {
                                let mut d = d_ref.borrow_mut();
                                let w = page.child().width() as f64;
                                let h = page.child().height() as f64;
                                d.zoom_to_fit(w, h);
                            }
                        }
                        return glib::Propagation::Stop;
                    }

                    if key == gdk::Key::z || key == gdk::Key::Z {
                        if let Some(page) = obj.imp().tab_view.selected_page() {
                            let (w, h) =
                                (page.child().width() as f64, page.child().height() as f64);
                            let pages = obj.imp().pages.borrow();
                            if let Some(d_ref) = pages.get(&page) {
                                let mut d = d_ref.borrow_mut();
                                let (mx, my) = obj.imp().mouse_pos.get();
                                let (wx, wy) = d.camera.screen_to_world(mx, my);

                                // พุ่งเข้าหาจุดที่เมาส์ชี้ ให้มาอยู่กลางจอ
                                d.camera.zoom_to_point(wx, wy, (w, h), 2.0);
                            }
                        }
                        return glib::Propagation::Stop;
                    }

                    glib::Propagation::Proceed
                }
            ));
            obj.add_controller(key_ctrl);

            // ปิด Revealer เมื่อกดปุ่ม Apply
            self.btn_apply_array.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    let imp = obj.imp();
                    let rows = imp.spin_rows.value() as i32;
                    let cols = imp.spin_cols.value() as i32;
                    let dx = imp.spin_spacing_x.value();
                    let dy = imp.spin_spacing_y.value();

                    if let Some(page) = imp.tab_view.selected_page() {
                        let pages = imp.pages.borrow();
                        if let Some(d_ref) = pages.get(&page) {
                            let mut d = d_ref.borrow_mut();
                            d.apply_array_grid(rows, cols, dx, dy);
                            page.child().queue_draw();
                        }
                    }

                    imp.revealer_array_ui.set_reveal_child(false);
                    glib::timeout_add_local_once(
                        std::time::Duration::from_millis(300),
                        glib::clone!(
                            #[weak]
                            imp,
                            move || {
                                imp.right_dock.set_visible_child_name("default");
                                imp.current_mode.set(DrawingMode::Select);
                            }
                        ),
                    );
                }
            ));

            self.spin_rows.connect_value_changed(glib::clone!(
                #[weak]
                obj,
                move |spin| {
                    if let Some(page) = obj.imp().tab_view.selected_page() {
                        let pages = obj.imp().pages.borrow();
                        if let Some(d_ref) = pages.get(&page) {
                            let mut d = d_ref.borrow_mut();
                            d.array_settings.rows = spin.value() as i32;
                            d.array_settings.anim_spacing_x *= 0.0;
                            d.array_settings.anim_spacing_y *= 0.0;
                            d.array_settings.anim_scale = 0.8;
                        }
                        page.child().queue_draw();
                    }
                }
            ));

            self.spin_cols.connect_value_changed(glib::clone!(
                #[weak]
                obj,
                move |spin| {
                    if let Some(page) = obj.imp().tab_view.selected_page() {
                        let pages = obj.imp().pages.borrow();
                        if let Some(d_ref) = pages.get(&page) {
                            let mut d = d_ref.borrow_mut();
                            d.array_settings.cols = spin.value() as i32;
                            d.array_settings.anim_spacing_x *= 0.0;
                            d.array_settings.anim_spacing_y *= 0.0;
                            d.array_settings.anim_scale = 0.8;
                        }
                        page.child().queue_draw();
                    }
                }
            ));

            self.spin_spacing_x.connect_value_changed(glib::clone!(
                #[weak]
                obj,
                move |spin| {
                    if let Some(page) = obj.imp().tab_view.selected_page() {
                        let pages = obj.imp().pages.borrow();
                        if let Some(d_ref) = pages.get(&page) {
                            let mut d = d_ref.borrow_mut();
                            // ส่งค่าเป้าหมายใหม่ไปให้ Engine
                            d.array_settings.spacing_x = spin.value();
                        }
                        page.child().queue_draw();
                    }
                }
            ));

            self.spin_spacing_y.connect_value_changed(glib::clone!(
                #[weak]
                obj,
                move |spin| {
                    if let Some(page) = obj.imp().tab_view.selected_page() {
                        let pages = obj.imp().pages.borrow();
                        if let Some(d_ref) = pages.get(&page) {
                            let mut d = d_ref.borrow_mut();
                            // ส่งค่าเป้าหมายใหม่ไปให้ Engine
                            d.array_settings.spacing_y = spin.value();
                        }
                        page.child().queue_draw();
                    }
                }
            ));

            // ตรวจสอบความถูกต้องของปุ่ม Snap
            self.btn_snap.connect_toggled(glib::clone!(
                #[weak]
                obj,
                move |btn| {
                    obj.imp().snap_enabled.set(btn.is_active());
                    if let Some(page) = obj.imp().tab_view.selected_page() {
                        page.child().queue_draw();
                    }
                }
            ));

            // ลบ Tab
            self.tab_view.connect_close_page(glib::clone!(
                #[weak]
                obj,
                #[upgrade_or]
                glib::Propagation::Proceed,
                move |_view, page| {
                    obj.imp().pages.borrow_mut().remove(page);
                    println!("Closed tab and cleared memory.");
                    glib::Propagation::Proceed
                }
            ));

            // --- UI Actions ---

            // dimension
            self.btn_dimension.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.imp().current_mode.set(DrawingMode::Dimension);
                    obj.imp().dimension_step.set(0); // เริ่มนับหนึ่งใหม่
                    obj.imp().start_pos.set(None);
                }
            ));

            self.btn_auto_pan.connect_toggled(glib::clone!(
                #[weak]
                obj,
                move |btn| {
                    obj.imp().auto_pan_enabled.set(btn.is_active());
                }
            ));

            self.btn_new.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.create_new_tab("Untitled");
                }
            ));

            self.btn_line.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.imp().current_mode.set(DrawingMode::Line);
                }
            ));

            self.btn_circle.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.imp().current_mode.set(DrawingMode::Circle);
                }
            ));

            self.btn_undo.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| obj.undo_action()
            ));
            self.btn_redo.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| obj.redo_action()
            ));
            self.btn_save.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| obj.show_save_dialog()
            ));

            self.btn_cancel_array.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    let imp = obj.imp();
                    if let Some(page) = imp.tab_view.selected_page() {
                        let pages = imp.pages.borrow();
                        if let Some(d_ref) = pages.get(&page) {
                            d_ref.borrow_mut().cancel_array_preview();
                        }
                    }
                    imp.revealer_array_ui.set_reveal_child(false);
                    imp.is_cancelling_array.set(true);
                    glib::timeout_add_local_once(
                        std::time::Duration::from_millis(300),
                        glib::clone!(
                            #[weak]
                            imp,
                            move || {
                                imp.right_dock.set_visible_child_name("default");
                            }
                        ),
                    );
                }
            ));

            self.btn_array.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    let imp = obj.imp();

                    // reset anim ให้เริ่มจาก 0 ใหม่ทุกครั้งที่เปิด
                    if let Some(page) = imp.tab_view.selected_page() {
                        let pages = imp.pages.borrow();
                        if let Some(d_ref) = pages.get(&page) {
                            let mut d = d_ref.borrow_mut();
                            d.array_settings.anim_spacing_x = 0.0;
                            d.array_settings.anim_spacing_y = 0.0;
                            d.array_settings.anim_scale = 0.0;
                            d.array_settings.target_anim_scale = 1.0;
                        }
                    }

                    imp.right_dock.set_visible_child_name("array_settings");
                    glib::timeout_add_local_once(
                        std::time::Duration::from_millis(50),
                        glib::clone!(
                            #[weak]
                            imp,
                            move || {
                                imp.revealer_array_ui.set_reveal_child(true);
                            }
                        ),
                    );
                    imp.current_mode.set(DrawingMode::Array);
                }
            ));

            self.cmd_entry.connect_activate(glib::clone!(
                #[weak]
                obj,
                move |entry| {
                    let text = entry.text().to_string().to_lowercase().trim().to_string();

                    if let Some(page) = obj.imp().tab_view.selected_page() {
                        let (w, h) = (page.child().width() as f64, page.child().height() as f64);

                        match text.as_str() {
                            "fit" => {
                                let pages = obj.imp().pages.borrow();
                                if let Some(d_ref) = pages.get(&page) {
                                    d_ref.borrow_mut().zoom_to_fit(w, h);
                                }
                            }
                            "z" => {
                                let pages = obj.imp().pages.borrow();
                                if let Some(d_ref) = pages.get(&page) {
                                    let mut d = d_ref.borrow_mut();
                                    let (mx, my) = obj.imp().mouse_pos.get();
                                    let (wx, wy) = d.camera.screen_to_world(mx, my);
                                    d.camera.zoom_to_point(wx, wy, (w, h), 1.5);
                                }
                            }
                            "s" | "select" => obj.imp().current_mode.set(DrawingMode::Select),
                            "l" | "line" => obj.imp().current_mode.set(DrawingMode::Line),
                            "c" | "circle" => obj.imp().current_mode.set(DrawingMode::Circle),
                            "u" | "undo" => obj.undo_action(),
                            "save" => obj.show_save_dialog(),
                            "ar" | "array" => {
                                let imp = obj.imp();

                                if let Some(page) = imp.tab_view.selected_page() {
                                    let pages = imp.pages.borrow();
                                    if let Some(d_ref) = pages.get(&page) {
                                        let mut d = d_ref.borrow_mut();
                                        d.array_settings.anim_spacing_x = 0.0;
                                        d.array_settings.anim_spacing_y = 0.0;
                                        d.array_settings.anim_scale = 0.0;
                                        d.array_settings.target_anim_scale = 1.0;
                                    }
                                }

                                imp.right_dock.set_visible_child_name("array_settings");
                                glib::timeout_add_local_once(
                                    std::time::Duration::from_millis(50),
                                    glib::clone!(
                                        #[weak]
                                        imp,
                                        move || {
                                            imp.revealer_array_ui.set_reveal_child(true);
                                        }
                                    ),
                                );
                                imp.current_mode.set(DrawingMode::Array);
                                imp.array_base_pos.set(None);
                                println!("Mode: ARRAY - Click base point");
                            }
                            _ => {
                                println!("Unknown command: {}", text);
                            }
                        }
                    }
                    entry.set_text("");
                }
            ));

            // สร้างหน้าแรกทันที
            obj.create_new_tab("Drawing 1");

            self.btn_line.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.imp().current_mode.set(DrawingMode::Line);
                    obj.imp().start_pos.set(None); // ล้างพิกัดเก่าทิ้ง
                }
            ));

            self.btn_circle.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.imp().current_mode.set(DrawingMode::Circle);
                    obj.imp().start_pos.set(None);
                }
            ));
        }
    }

    impl WidgetImpl for DraftingWindow {}
    impl WindowImpl for DraftingWindow {}
    impl ApplicationWindowImpl for DraftingWindow {}
    impl AdwApplicationWindowImpl for DraftingWindow {}
}

impl DraftingWindow {
    pub fn new<P: glib::prelude::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub fn create_new_tab(&self, title: &str) {
        let imp = self.imp();
        let canvas = gtk::DrawingArea::builder()
            .hexpand(true)
            .vexpand(true)
            .focusable(true)
            .can_focus(true)
            .cursor(&gdk::Cursor::from_name("crosshair", None).unwrap())
            .build();

        let page = imp.tab_view.add_page(&canvas, None);
        page.set_title(title);

        let data = crate::engine::Engine::new();
        imp.pages
            .borrow_mut()
            .insert(page.clone(), RefCell::new(data));

        // --- 1. Zoom (Scroll) ---
        let scroll = gtk::EventControllerScroll::new(gtk::EventControllerScrollFlags::VERTICAL);
        scroll.connect_scroll(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[weak]
            page,
            #[upgrade_or]
            glib::Propagation::Stop,
            move |_, _, dy| {
                let pages = obj.imp().pages.borrow();
                if let Some(d_ref) = pages.get(&page) {
                    let mut d = d_ref.borrow_mut();
                    let (mx, my) = obj.imp().mouse_pos.get();

                    if dy < 0.0 {
                        d.camera.zoom_in_at(mx, my);
                    } else {
                        d.camera.zoom_out_at(mx, my);
                    }
                }
                glib::Propagation::Stop
            }
        ));
        canvas.add_controller(scroll);

        // --- 2. Smooth Animation (Tick) ---
        canvas.add_tick_callback(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[weak]
            page,
            #[upgrade_or]
            glib::ControlFlow::Continue,
            move |area, clock| {
                let pages = obj.imp().pages.borrow();
                if let Some(d_ref) = pages.get(&page) {
                    let mut d = d_ref.borrow_mut();

                    // เรียกใช้ฟังก์ชัน step ของ Engine
                    if d.step(clock.frame_time() as u64) {
                        area.queue_draw();
                    }

                    // ถ้า cancel animation จบแล้ว ค่อยเปลี่ยน mode
                    if obj.imp().is_cancelling_array.get() && d.array_settings.anim_scale < 0.01 {
                        obj.imp().is_cancelling_array.set(false);
                        obj.imp().current_mode.set(DrawingMode::Select);
                    }
                }
                glib::ControlFlow::Continue
            }
        ));

        // --- 3. Mouse Motion ---
        let motion = gtk::EventControllerMotion::new();
        motion.connect_motion(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[weak]
            page,
            #[weak]
            canvas,
            move |controller, x, y| {
                obj.imp().mouse_pos.set((x, y));

                let pages = obj.imp().pages.borrow();
                if let Some(d_ref) = pages.get(&page) {
                    let mut d = d_ref.borrow_mut();
                    let (w, h) = (canvas.width() as f64, canvas.height() as f64);
                    if obj.imp().auto_pan_enabled.get() {
                        d.update_auto_pan(x, y, w, h);
                    }

                    if obj.imp().current_mode.get() == DrawingMode::Select {
                        let (wx, wy) = d.camera.screen_to_world(x, y);
                        d.hover_at(wx, wy);

                        if let Some((sx, sy)) = obj.imp().drag_start.get() {
                            let dist = ((x - sx).powi(2) + (y - sy).powi(2)).sqrt();
                            if dist > 5.0 {
                                // ถ้า mode ไม่ใช่ Move ให้ทำ selection box
                                obj.imp().is_dragging.set(true);
                            }
                        }
                    }

                    if obj.imp().current_mode.get() == DrawingMode::Move {
                        // เช็ค button 1 (left click) กำลัง press อยู่มั้ย
                        let is_pressed = controller
                            .current_event_state()
                            .contains(gdk::ModifierType::BUTTON1_MASK);

                        if !is_pressed {
                            obj.imp().current_mode.set(DrawingMode::Select);
                            obj.imp().drag_start.set(None);
                        } else if let Some((sx, sy)) = obj.imp().drag_start.get() {
                            let (wx1, wy1) = d.camera.screen_to_world(sx, sy);
                            let (wx2, wy2) = d.camera.screen_to_world(x, y);
                            let dx = wx2 - wx1;
                            let dy = wy2 - wy1;
                            d.move_selected(dx, dy);
                            obj.imp().drag_start.set(Some((x, y)));
                        }
                    }
                }

                canvas.queue_draw();
            }
        ));
        canvas.add_controller(motion);

        // --- 4. Drawing (Click) ---
        let click = gtk::GestureClick::new();
        click.connect_pressed(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[weak]
            page,
            #[weak]
            canvas,
            move |_, _, x, y| {
                canvas.grab_focus();
                let imp = obj.imp();
                imp.is_mouse_pressed.set(true);
                let pages = imp.pages.borrow();

                if imp.current_mode.get() == DrawingMode::Move {
                    // snap ตำแหน่งสุดท้ายแล้วออกจาก Move mode
                    let pages = imp.pages.borrow();
                    if let Some(d_ref) = pages.get(&page) {
                        let mut d = d_ref.borrow_mut();
                        d.move_and_snap_selected(0.0, 0.0, imp.snap_enabled.get());
                    }
                    imp.current_mode.set(DrawingMode::Select);
                    imp.drag_start.set(None);
                    page.child().queue_draw();
                    return;
                }

                if let Some(d_ref) = pages.get(&page) {
                    let mut d = d_ref.borrow_mut();

                    // 1. แปลงพิกัดจอเป็นพิกัดโลก
                    let (wx, wy) = d.camera.screen_to_world(x, y);

                    // 2. เช็คโหมดปัจจุบัน
                    match imp.current_mode.get() {
                        DrawingMode::Select => {
                            let (_wx, _wy) = d.camera.screen_to_world(x, y);
                            imp.drag_start.set(Some((x, y)));
                            imp.is_dragging.set(false);
                            d.snapshot_for_undo();

                            let hit_selected = d
                                .hovered_id
                                .map(|id| d.history.iter().any(|e| e.id == id && e.is_selected))
                                .unwrap_or(false);

                            if hit_selected {
                                obj.imp().current_mode.set(DrawingMode::Move);
                            }
                        }
                        DrawingMode::Line | DrawingMode::Circle => {
                            // Logic การวาดเดิม (มิติ: คลิกแรกเก็บจุดเริ่ม)
                            if imp.start_pos.get().is_none() {
                                let (sx, sy) = d.get_snapped_pos(wx, wy, imp.snap_enabled.get());
                                imp.start_pos.set(Some((sx, sy)));
                            }
                        }

                        // ในส่วน match imp.current_mode.get()
                        DrawingMode::Array => {
                            // แปลงพิกัดเมาส์ (x, y) จากหน้าจอเป็นพิกัดโลก (world) ก่อนใช้
                            let (world_x, world_y) = d.camera.screen_to_world(x, y);

                            if let Some(base) = imp.array_base_pos.get() {
                                // --- จังหวะที่ 2: คลิกเพื่อ "ยืนยัน" ---
                                // base ในที่นี้คือ (f64, f64) ต้องเข้าถึงด้วย .0 และ .1
                                let dx = world_x - base.0;
                                let dy = world_y - base.1;

                                let rows = imp.spin_rows.value() as i32;
                                let cols = imp.spin_cols.value() as i32;

                                d.apply_array_grid(rows, cols, dx, dy);

                                imp.array_base_pos.set(None);
                                imp.current_mode.set(DrawingMode::Select);
                            } else {
                                // --- จังหวะที่ 1: คลิกเพื่อ "เลือกจุดเริ่ม" ---
                                let (wx, wy) = d.camera.screen_to_world(x, y);
                                imp.array_base_pos.set(Some((wx, wy)));

                                // เซ็ตค่า anim ให้เท่ากับค่าเป้าหมายปัจจุบันทันที เพื่อให้มันไม่ "พุ่ง" มาจาก 0
                                d.array_settings.anim_spacing_x = d.array_settings.spacing_x;
                                d.array_settings.anim_spacing_y = d.array_settings.spacing_y;
                                d.array_settings.anim_scale = 1.0;
                            }
                        }
                        DrawingMode::Dimension => {
                            let step = imp.dimension_step.get();
                            let (sx, sy) = d.get_snapped_pos(wx, wy, imp.snap_enabled.get());
                            let p = Point { x: sx, y: sy };
                            match step {
                                0 => {
                                    imp.dim_temp_start.set(Some(p));
                                    imp.dimension_step.set(1);
                                    imp.start_pos.set(Some((sx, sy))); // ใช้ preview เดิม
                                }
                                1 => {
                                    imp.dim_temp_end.set(Some(p));
                                    imp.dimension_step.set(2);
                                    // start_pos ยังคงเป็นจุดแรก — preview จะใช้ mouse เป็น offset
                                }
                                2 => {
                                    // click 3: ยืนยัน offset
                                    if let (Some(start), Some(end)) = (
                                        imp.dim_temp_start.get(),
                                        imp.dim_temp_end.get(),
                                    ) {
                                        // offset = ระยะห่างจากเส้น start-end ไปยัง mouse
                                        let dx_line = end.x - start.x;
                                        let dy_line = end.y - start.y;
                                        let len = (dx_line*dx_line + dy_line*dy_line).sqrt().max(0.001);
                                        // perpendicular direction
                                        let px = -dy_line / len;
                                        let py =  dx_line / len;
                                        // project mouse onto perpendicular
                                        let dmx = wx - start.x;
                                        let dmy = wy - start.y;
                                        let offset = dmx * px + dmy * py;

                                        let next_id = d.get_next_id();
                                        d.add_element(DrawingElement {
                                            id: next_id,
                                            is_selected: false,
                                            data: ElementData::Dimension { start, end, offset },
                                        });
                                        // reset
                                        imp.start_pos.set(None);
                                        imp.dimension_step.set(0);
                                        imp.dim_temp_start.set(None);
                                        imp.dim_temp_end.set(None);
                                        imp.current_mode.set(DrawingMode::Select);
                                    }
                                }
                                _ => {}
                            }
                        }
                        DrawingMode::Move => {}
                    }
                }
                canvas.queue_draw();
            }
        ));

        click.connect_released(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[weak]
            page,
            move |gesture, _, x, y| {
                let is_shift = gesture
                    .current_event_state()
                    .contains(gdk::ModifierType::SHIFT_MASK);
                let imp = obj.imp();
                imp.is_mouse_pressed.set(false);

                if imp.current_mode.get() == DrawingMode::Move {
                    let pages = imp.pages.borrow();
                    if let Some(d_ref) = pages.get(&page) {
                        let mut d = d_ref.borrow_mut();
                        d.move_and_snap_selected(0.0, 0.0, imp.snap_enabled.get());
                    }
                    imp.current_mode.set(DrawingMode::Select);
                    imp.drag_start.set(None);
                    page.child().queue_draw();
                    return;
                }

                // --- Select mode แยกออกมาก่อนเลย ---
                if imp.current_mode.get() == DrawingMode::Select {
                    let pages = imp.pages.borrow();
                    if let Some(d_ref) = pages.get(&page) {
                        let mut d = d_ref.borrow_mut();
                        if imp.is_dragging.get() {
                            if let Some((sx, sy)) = imp.drag_start.get() {
                                let (wx1, wy1) = d.camera.screen_to_world(sx, sy);
                                let (wx2, wy2) = d.camera.screen_to_world(x, y);
                                let crossing = x < sx;
                                d.select_in_box(wx1, wy1, wx2, wy2, crossing, is_shift);
                            }
                        } else {
                            // คลิกธรรมดา
                            let (wx, wy) = d.camera.screen_to_world(x, y);
                            d.select_at(wx, wy, is_shift);
                        }
                    }
                    imp.drag_start.set(None);
                    imp.is_dragging.set(false);
                    page.child().queue_draw();
                    return;
                }

                // --- Line/Circle mode ---
                if let Some((sx, sy)) = imp.start_pos.get() {
                    let pages = imp.pages.borrow();
                    if let Some(d_ref) = pages.get(&page) {
                        let mut d = d_ref.borrow_mut();
                        let (wx, wy) = d.camera.screen_to_world(x, y);
                        let (ex, ey) = d.get_snapped_pos(wx, wy, imp.snap_enabled.get());

                        match imp.current_mode.get() {
                            DrawingMode::Line => {
                                let next_id = d.get_next_id();
                                d.add_element(DrawingElement {
                                    id: next_id,
                                    is_selected: false,
                                    data: ElementData::Line {
                                        start: Point { x: sx, y: sy },
                                        end: Point { x: ex, y: ey },
                                    },
                                });
                                imp.start_pos.set(Some((ex, ey)));
                            }
                            DrawingMode::Circle => {
                                let dx = ex - sx;
                                let dy = ey - sy;
                                let radius = (dx * dx + dy * dy).sqrt();
                                let next_id = d.get_next_id();
                                d.add_element(DrawingElement {
                                    id: next_id,
                                    is_selected: false,
                                    data: ElementData::Circle {
                                        center: Point { x: sx, y: sy },
                                        radius,
                                    },
                                });
                                imp.start_pos.set(None);
                            }
                            _ => {}
                        }
                        page.child().queue_draw();
                    }
                }
            }
        ));
        canvas.add_controller(click);

        // --- 5. Draw Function ---
        canvas.set_draw_func(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[weak]
            page,
            move |_, cr, w, h| {
                obj.draw_page(&page, cr, w as f64, h as f64);
            }
        ));

        imp.tab_view.set_selected_page(&page);
        canvas.grab_focus();
    }

    fn draw_page(&self, page: &adw::TabPage, cr: &gtk::cairo::Context, w: f64, h: f64) {
        let imp = self.imp();
        let pages = imp.pages.borrow();
        let d_ref = match pages.get(page) {
            Some(r) => r,
            None => return,
        };
        let d = d_ref.borrow();

        let cam = &d.camera;
        let scale = cam.scale;
        let (_ox, _oy) = cam.offset;
        let (rx, ry) = imp.mouse_pos.get();

        // 1. Background
        cr.set_source_rgb(0.1, 0.1, 0.1);
        let _ = cr.paint();

        // 2. Grid & Snap Calculation
        let grid_base = 100.0;
        let lod_raw = scale.log10();
        let lod = lod_raw.floor();
        let fraction = lod_raw - lod;

        let step_major = grid_base / 10.0f64.powf(lod); // ตารางปัจจุบัน
        let step_minor = step_major / 10.0; // ตารางย่อยที่จะโผล่มา
        let step_giant = step_major * 10.0; // ตารางใหญ่ที่จะหายไป

        // --- คำนวณ Snap Position แบบคาดเดาได้ (Predictable) ---
        let (wx, wy) = cam.screen_to_world(rx, ry);
        let (sn_x, sn_y) = d.get_snapped_pos(wx, wy, imp.snap_enabled.get());

        // 3. Grid Drawing Function
        let draw_grid_line = |s: f64, alpha: f64, lw: f64| {
            if alpha <= 0.02 {
                return;
            }
            cr.set_source_rgba(0.5, 0.5, 0.5, alpha);
            cr.set_line_width(lw);

            // หาขอบเขตของโลก (World Bounds) ที่ปรากฏบนจอขณะนี้
            let (w_min_x, w_max_y) = cam.screen_to_world(0.0, 0.0);
            let (w_max_x, w_min_y) = cam.screen_to_world(w, h);

            // วาดเส้นแนวตั้ง (Vertical Lines) - วิ่งตามแกน X
            let mut x_cursor = (w_min_x / s).floor() * s;
            while x_cursor <= w_max_x {
                let (sx, _) = cam.world_to_screen(x_cursor, 0.0);
                cr.move_to(sx, 0.0);
                cr.line_to(sx, h);
                x_cursor += s;
            }

            let mut y_cursor = (w_min_y / s).floor() * s;
            while y_cursor <= w_max_y {
                let (_, sy) = cam.world_to_screen(0.0, y_cursor);
                cr.move_to(0.0, sy);
                cr.line_to(w, sy);
                y_cursor += s;
            }
            let _ = cr.stroke();
        };

        // วาด Grid 3 ชั้น (Fade เข้า-ออก)

        // --- Grid Logic (Professional Subtle Version) ---

        // ชั้นที่ 1: ตารางใหญ่ (Giant)
        // ลด Alpha และความหนาลง เพื่อให้เป็นแค่ไกด์ห่างๆ
        draw_grid_line(step_giant, (1.0 - fraction) * 0.15, 1.0);

        // ชั้นที่ 2: ตารางหลัก (Major)
        // ลดจาก 0.6 เหลือ 0.3-0.4 เพื่อไม่ให้แข่งกับเส้นขาวของ Diode
        draw_grid_line(step_major, 0.35, 0.8);

        // ชั้นที่ 3: ตารางละเอียด (Minor)
        // ให้บางและจางที่สุด เป็นเหมือนพื้นผิวสัมผัสเบาๆ
        draw_grid_line(step_minor, fraction * 0.15, 0.75);

        // 4. Draw Elements
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.set_line_width(2.0);

        // ขอรายการสิ่งที่ต้องวาดจาก Engine
        let commands = d.get_render_commands(w, h);

        for cmd in commands {
            match cmd {
                RenderCommand::Line {
                    x1,
                    y1,
                    x2,
                    y2,
                    is_selected,
                } => {
                    if is_selected {
                        cr.set_source_rgb(1.0, 0.0, 0.0); // สีแดง
                        cr.set_line_width(3.0); // หนาขึ้น
                    } else {
                        cr.set_source_rgb(1.0, 1.0, 1.0); // สีขาวปกติ
                        cr.set_line_width(2.0);
                    }
                    cr.move_to(x1, y1);
                    cr.line_to(x2, y2);
                    let _ = cr.stroke();
                }
                RenderCommand::Circle {
                    cx,
                    cy,
                    radius,
                    is_selected,
                } => {
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
                RenderCommand::Dimension {
                    text_x, text_y, text, is_selected,
                    dim_x1, dim_y1, dim_x2, dim_y2,
                    ext1_x1, ext1_y1, ext1_x2, ext1_y2,
                    ext2_x1, ext2_y1, ext2_x2, ext2_y2,
                } => {
                    let matrix = cr.matrix();
                    let scale = (matrix.xx().powi(2) + matrix.xy().powi(2)).sqrt();

                    if is_selected {
                        cr.set_source_rgb(1.0, 0.35, 0.35);
                        cr.set_line_width(2.5 / scale);
                    } else {
                        cr.set_source_rgb(0.9, 0.9, 0.0);
                        cr.set_line_width(1.2 / scale);
                    }

                    // extension line 1
                    cr.move_to(ext1_x1, ext1_y1); cr.line_to(ext1_x2, ext1_y2);
                    let _ = cr.stroke();
                    // extension line 2
                    cr.move_to(ext2_x1, ext2_y1); cr.line_to(ext2_x2, ext2_y2);
                    let _ = cr.stroke();
                    // dim line
                    cr.move_to(dim_x1, dim_y1); cr.line_to(dim_x2, dim_y2);
                    let _ = cr.stroke();

                    // text
                    cr.select_font_face("Sans", gtk::cairo::FontSlant::Normal, gtk::cairo::FontWeight::Normal);
                    cr.set_font_size(13.0 / scale);
                    if let Ok(ext) = cr.text_extents(&text) {
                        let offset_y = 8.0 / scale;
                        cr.move_to(
                            text_x - ext.width()/2.0 - ext.x_bearing(),
                            text_y - ext.height()/2.0 - ext.y_bearing() - offset_y,
                        );
                        let _ = cr.show_text(&text);
                    }
                    cr.new_path();
                }
            }
            let _ = cr.stroke();
        }

        let draw_dim_preview = |cr: &gtk::cairo::Context,
        cam: &crate::engine::Camera,
        g: &crate::engine::DimensionGeometry| {
            let (a0x, a0y) = cam.world_to_screen(g.start.x,    g.start.y);
            let (a1x, a1y) = cam.world_to_screen(g.end.x,      g.end.y);
            let (dfx, dfy) = cam.world_to_screen(g.dim_from.x, g.dim_from.y);
            let (dtx, dty) = cam.world_to_screen(g.dim_to.x,   g.dim_to.y);

            // extension line 1
            cr.move_to(a0x, a0y); cr.line_to(dfx, dfy);
            let _ = cr.stroke();
            // extension line 2
            cr.move_to(a1x, a1y); cr.line_to(dtx, dty);
            let _ = cr.stroke();
            // dim line
            cr.move_to(dfx, dfy); cr.line_to(dtx, dty);
            let _ = cr.stroke();
        };

        // 5. Preview Drawing
        if let Some((sx, sy)) = imp.start_pos.get() {
            cr.set_source_rgba(1.0, 1.0, 1.0, 0.5);

            match imp.current_mode.get() {
                DrawingMode::Line => {
                    let (psx, psy) = cam.world_to_screen(sx, sy);
                    let (pex, pey) = cam.world_to_screen(sn_x, sn_y);
                    cr.move_to(psx, psy);
                    cr.line_to(pex, pey);
                }

                DrawingMode::Circle => {
                    let r = ((sn_x - sx).powi(2) + (sn_y - sy).powi(2)).sqrt();
                    let (psx, psy) = cam.world_to_screen(sx, sy);
                    cr.arc(psx, psy, r * cam.scale, 0.0, 2.0 * std::f64::consts::PI);
                }

                // --- มีไว้คั่นหู ---
                DrawingMode::Select => {}
                DrawingMode::Array => {}
                DrawingMode::Move => {}

                // --- Dimension ---
                DrawingMode::Dimension => {
                    let step = imp.dimension_step.get();
                        match step {
                            1 => {
                                // step 1: วาด preview เส้นจาก start → mouse
                                if let Some(g) = linear_dimension_geometry(
                                    Point { x: sx, y: sy },
                                    Point { x: sn_x, y: sn_y },
                                    DEFAULT_DIMENSION_OFFSET,
                                ) {
                                    draw_dim_preview(cr, cam, &g);
                                }
                            }
                            2 => {
                                // step 2: จุดปลายล็อกแล้ว — ลาก offset ตาม mouse
                                if let (Some(start), Some(end)) = (
                                    imp.dim_temp_start.get(),
                                    imp.dim_temp_end.get(),
                                ) {
                                    // คำนวณ offset จาก mouse
                                    let dx_line = end.x - start.x;
                                    let dy_line = end.y - start.y;
                                    let len = (dx_line*dx_line + dy_line*dy_line).sqrt().max(0.001);
                                    let px = -dy_line / len;
                                    let py =  dx_line / len;
                                    let dmx = sn_x - start.x;
                                    let dmy = sn_y - start.y;
                                    let offset = dmx * px + dmy * py;

                                    if let Some(g) = linear_dimension_geometry(start, end, offset) {
                                        draw_dim_preview(cr, cam, &g);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
            }
            let _ = cr.stroke();
        }

        // --- Array Preview ---
        if imp.current_mode.get() == DrawingMode::Array || imp.is_cancelling_array.get() {
            let _pages = imp.pages.borrow();
            let rows = imp.spin_rows.value() as i32;
            let cols = imp.spin_cols.value() as i32;
            let _dx = imp.spin_spacing_x.value();
            let _dy = imp.spin_spacing_y.value();

            let ghosts = d.get_array_preview_grid(rows, cols, 0.0, 0.0);

            for cmd in ghosts {
                match cmd {
                    RenderCommand::Line { x1, y1, x2, y2, .. } => {
                        cr.set_source_rgba(1.0, 0.5, 0.0, 0.4);
                        cr.set_line_width(1.5);
                        cr.move_to(x1, y1);
                        cr.line_to(x2, y2);
                        let _ = cr.stroke();
                    }
                    RenderCommand::Circle { cx, cy, radius, .. } => {
                        cr.set_source_rgba(1.0, 0.5, 0.0, 0.4);
                        cr.set_line_width(1.5);
                        cr.arc(cx, cy, radius, 0.0, 2.0 * std::f64::consts::PI);
                        let _ = cr.stroke();
                    }
                    RenderCommand::Dimension { .. } => {} // placeholder
                }
            }
        }

        // --- Selection Box ---
        if imp.is_dragging.get() && imp.current_mode.get() == DrawingMode::Select {
            if let Some((sx, sy)) = imp.drag_start.get() {
                let (mx, my) = imp.mouse_pos.get();
                let is_crossing = mx < sx; // ขวา→ซ้าย

                if is_crossing {
                    cr.set_source_rgba(0.0, 0.9, 0.3, 0.15); // เขียว fill
                } else {
                    cr.set_source_rgba(0.2, 0.6, 1.0, 0.15); // ฟ้า fill
                }
                cr.rectangle(sx, sy, mx - sx, my - sy);
                let _ = cr.fill();

                if is_crossing {
                    cr.set_source_rgba(0.0, 0.9, 0.3, 0.8); // เขียว border
                    // dashed line สำหรับ crossing
                    cr.set_dash(&[8.0, 4.0], 0.0);
                } else {
                    cr.set_source_rgba(0.2, 0.6, 1.0, 0.8); // ฟ้า border
                    cr.set_dash(&[], 0.0); // solid line
                }
                cr.set_line_width(1.0);
                cr.rectangle(sx, sy, mx - sx, my - sy);
                let _ = cr.stroke();
                cr.set_dash(&[], 0.0); // reset dash
            }
        }

        // 6. Crosshair (ใช้ sn_x, sn_y ที่คำนวณไว้ข้างบน)
        let (cx, cy) = cam.world_to_screen(sn_x, sn_y);
        cr.set_source_rgb(1.0, 0.0, 0.0);
        cr.set_line_width(1.0);
        cr.move_to(cx, 0.0);
        cr.line_to(cx, h);
        cr.move_to(0.0, cy);
        cr.line_to(w, cy);
        let _ = cr.stroke();

        // 7. OSD
        let mode_text = match imp.current_mode.get() {
            DrawingMode::Line => "LINE",
            DrawingMode::Circle => "CIRCLE",
            DrawingMode::Select => "SELECT",
            DrawingMode::Array => "ARRAY (LINEAR)",
            DrawingMode::Move => "MOVE",
            DrawingMode::Dimension => "DIM (LINEAR)",
        };

        let txt = format!(
            "MODE: [{}]  X: {:.2} Y: {:.2} {}",
            mode_text,
            sn_x,
            sn_y,
            if imp.snap_enabled.get() { "[SNAP]" } else { "" }
        );

        let layout = gtk::pango::Layout::new(&page.child().pango_context());
        layout.set_text(&txt);
        let font_desc = gtk::pango::FontDescription::from_string("Monospace 11");
        layout.set_font_description(Some(&font_desc));

        let padding = 10.0;

        cr.set_source_rgb(1.0, 1.0, 0.0);
        cr.move_to(10.0 + padding, 70.0 + padding);
        let _ = pangocairo::functions::show_layout(cr, &layout);
    }

    pub fn undo_action(&self) {
        // 1. ดึงหน้าปัจจุบัน (page) ที่กำลังเปิดอยู่ขึ้นมา
        if let Some(page) = self.imp().tab_view.selected_page() {
            // 2. ยืม (borrow) HashMap ที่เก็บ Engine (pages) ออกมาจาก RefCell
            let pages = self.imp().pages.borrow();

            // 3. หา Engine ที่ตรงกับหน้าเพจนั้น
            if let Some(d_ref) = pages.get(&page) {
                let mut d = d_ref.borrow_mut();

                // 4. เรียกใช้ชื่อใหม่ที่เราแก้ไปใน engine.rs
                d.undo_last_action();

                // 5. สั่งให้หน้าจอนั้นวาดใหม่ (ตอนนี้มีตัวแปร page แล้ว)
                page.child().queue_draw();
            }
        }
    }

    pub fn redo_action(&self) {
        // TODO: implement redo
        println!("Redo not yet implemented");
    }

    pub fn show_save_dialog(&self) {
        if let Some(page) = self.imp().tab_view.selected_page() {
            let dialog = gtk::FileDialog::builder()
                .title("Save Drawing")
                .accept_label("_Save")
                .build();

            dialog.save(
                None::<&gtk::Window>,
                gtk::gio::Cancellable::NONE,
                glib::clone!(
                    #[weak(rename_to = obj)]
                    self,
                    #[weak]
                    page,
                    move |res| {
                        if let Ok(file) = res {
                            let pages = obj.imp().pages.borrow();
                            if let Some(d_ref) = pages.get(&page) {
                                let data = d_ref.borrow();
                                let all_elements = data.get_all_elements();
                                let mut drawing = dxf::Drawing::new();

                                for el in all_elements {
                                    match &el.data {
                                        ElementData::Line { start, end } => {
                                            let line = dxf::entities::Line {
                                                p1: dxf::Point::new(start.x, start.y, 0.0),
                                                p2: dxf::Point::new(end.x, end.y, 0.0),
                                                ..Default::default()
                                            };
                                            drawing.add_entity(dxf::entities::Entity::new(
                                                dxf::entities::EntityType::Line(line),
                                            ));
                                        }
                                        ElementData::Circle { center, radius } => {
                                            let circle = dxf::entities::Circle {
                                                center: dxf::Point::new(center.x, center.y, 0.0),
                                                radius: *radius,
                                                ..Default::default()
                                            };
                                            drawing.add_entity(dxf::entities::Entity::new(
                                                dxf::entities::EntityType::Circle(circle),
                                            ));
                                        }
                                        ElementData::Dimension { start, end, offset } => {
                                            if let Some(g) =
                                                linear_dimension_geometry(*start, *end, *offset)
                                            {
                                                for (a, b) in [
                                                    (g.start, g.dim_from),
                                                    (g.end, g.dim_to),
                                                    (g.dim_from, g.dim_to),
                                                ] {
                                                    let line = dxf::entities::Line {
                                                        p1: dxf::Point::new(a.x, a.y, 0.0),
                                                        p2: dxf::Point::new(b.x, b.y, 0.0),
                                                        ..Default::default()
                                                    };
                                                    drawing.add_entity(dxf::entities::Entity::new(
                                                        dxf::entities::EntityType::Line(line),
                                                    ));
                                                }
                                            }
                                        }
                                    }
                                }

                                let mut buf = std::io::Cursor::new(Vec::new());
                                if let Err(e) = drawing.save(&mut buf) {
                                    eprintln!("Error generating DXF: {}", e);
                                    return;
                                }

                                // ใช้ path จาก dialog แต่เขียนไปที่ home แทน
                                let home =
                                    std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
                                let filename = file.basename().unwrap_or_else(|| {
                                    std::ffi::OsString::from("drawing.dxf").into()
                                });
                                let mut save_path = std::path::PathBuf::from(home);
                                save_path.push(std::path::PathBuf::from(filename));

                                if save_path.extension().is_none() {
                                    save_path.set_extension("dxf");
                                }

                                println!("Writing to: {:?}", save_path);
                                if let Err(e) = std::fs::write(&save_path, buf.into_inner()) {
                                    eprintln!("Error: {}", e);
                                } else {
                                    println!("Saved OK to {:?}", save_path);
                                }
                            }
                        }
                    }
                ),
            );
        }
    }
}
