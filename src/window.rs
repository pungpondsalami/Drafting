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
    DEFAULT_DIMENSION_OFFSET, DrawingElement, ElementData, Point, RenderCommand, SnapKind,
    SnapSettings, ToolMode as DrawingMode, arc_from_three_points, linear_dimension_geometry,
};

use adw::subclass::prelude::*;
use dxf;
use gtk::gdk;
use gtk::prelude::*;
use adw::prelude::*;
use gtk::{gio, glib};
use std::cell::RefCell;
use crate::renderer::{CairoRenderer, Renderer};
use std::path::PathBuf;

glib::wrapper! {
    pub struct DraftingWindow(ObjectSubclass<imp::DraftingWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

const OFFSET_ANIM_SECS: f64 = 0.12;
const OFFSET_FLIP_SECS: f64 = 0.04;

#[derive(Debug, Clone, Copy)]
pub struct OffsetAnim {
    pub source: u64,
    pub side: (f64, f64),
    pub dist: f64,
    pub start_us: Option<i64>,
    pub t: f64, // 0.0 → 1.0
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
        #[template_child]
        pub btn_open: TemplateChild<gtk::Button>,
        #[template_child]
        pub chk_snap_endpoint: TemplateChild<gtk::CheckButton>,
        #[template_child]
        pub chk_snap_midpoint: TemplateChild<gtk::CheckButton>,
        #[template_child]
        pub chk_snap_center: TemplateChild<gtk::CheckButton>,
        #[template_child]
        pub chk_snap_quadrant: TemplateChild<gtk::CheckButton>,
        #[template_child]
        pub chk_snap_intersection: TemplateChild<gtk::CheckButton>,
        #[template_child]
        pub btn_snap_menu: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub btn_arc: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_polyline: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_select: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_offset: TemplateChild<gtk::Button>,
        #[template_child]
        pub spin_offset_dist: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub btn_offset_done: TemplateChild<gtk::Button>,
        #[template_child]
        pub revealer_offset_ui: TemplateChild<gtk::Revealer>,
        #[template_child]
        pub btn_offset_cancel: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_delete: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_trim: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_extend: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_layers: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub layer_list: TemplateChild<gtk::ListBox>,
        pub building_layers: Cell<bool>,

        pub current_mode: Cell<crate::engine::ToolMode>,
        pub mouse_pos: Cell<(f64, f64)>,
        pub start_pos: Cell<Option<(f64, f64)>>,
        pub snap_enabled: Cell<bool>,
        pub is_dialog_open: Cell<bool>,
        pub pages: RefCell<HashMap<adw::TabPage, RefCell<crate::engine::Engine>>>,
        pub array_base_pos: Cell<Option<(f64, f64)>>,
        pub array_count: Cell<i32>,
        pub is_cancelling_array: Cell<bool>,
        pub drag_start: Cell<Option<(f64, f64)>>,
        pub is_dragging: Cell<bool>,
        pub is_mouse_pressed: Cell<bool>,
        pub dimension_step: Cell<u8>,
        pub dim_temp_start: Cell<Option<Point>>,
        pub dim_temp_end: Cell<Option<Point>>,
        pub auto_pan_enabled: Cell<bool>,
        pub force_close: Cell<bool>,
        pub snap_settings: Cell<SnapSettings>,
        pub pending_points: RefCell<Vec<Point>>,
        pub synced_mode: Cell<Option<DrawingMode>>,
        pub offset_distance: Cell<f64>,
        pub offset_source: Cell<Option<u64>>,
        pub offset_msg: RefCell<String>,
        pub offset_anim: RefCell<Option<OffsetAnim>>,
        pub offset_fade: Cell<f64>,
        pub last_frame_us: Cell<i64>,
        pub offset_stable: Cell<Option<(f64, f64)>>,
        pub offset_k: Cell<f64>,
        pub offset_phase: Cell<u8>,
        pub shift_down: Cell<bool>,
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
            let checks: [&gtk::CheckButton; 5] = [
                &self.chk_snap_endpoint,
                &self.chk_snap_midpoint,
                &self.chk_snap_center,
                &self.chk_snap_quadrant,
                &self.chk_snap_intersection,
            ];
            for chk in checks {
                chk.connect_toggled(glib::clone!(
                    #[weak]
                    obj,
                    move |_| obj.sync_snap_settings()
                ));
            }

            // ตั้งค่าเริ่มต้น
            self.snap_enabled.set(true);
            self.btn_snap.set_active(true);
            self.current_mode.set(DrawingMode::Select);
            self.auto_pan_enabled.set(true);
            self.offset_distance.set(self.spin_offset_dist.value());

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
                    let is_shift = state.contains(gdk::ModifierType::SHIFT_MASK);
                    let typing = obj.is_typing();

                    if key == gdk::Key::Shift_L || key == gdk::Key::Shift_R {
                        obj.set_shift(true);
                        return glib::Propagation::Proceed;
                    }

                    // Redo: Ctrl+Y หรือ Ctrl+Shift+Z
                    if is_ctrl
                        && (key == gdk::Key::y
                            || key == gdk::Key::Y
                            || (is_shift && (key == gdk::Key::z || key == gdk::Key::Z)))
                    {
                        obj.redo_action();
                        return glib::Propagation::Stop;
                    }

                    // Undo: Ctrl + Z
                    if is_ctrl && (key == gdk::Key::z || key == gdk::Key::Z) {
                        obj.undo_action();
                        return glib::Propagation::Stop;
                    }

                    // Snap: F9
                    if key == gdk::Key::F9 {
                        let btn = &obj.imp().btn_snap;
                        btn.set_active(!btn.is_active());
                        return glib::Propagation::Stop;
                    }

                    // Enter: จบ polyline
                    if (key == gdk::Key::Return || key == gdk::Key::KP_Enter)
                        && !typing
                        && obj.imp().current_mode.get() == DrawingMode::Polyline
                        && !obj.imp().pending_points.borrow().is_empty()
                    {
                        obj.finish_polyline();
                        return glib::Propagation::Stop;
                    }

                    // Cancel Action: Esc
                    if key == gdk::Key::Escape {
                        obj.cancel_action(true);
                        return glib::Propagation::Stop;
                    }

                    // Delete
                    if (key == gdk::Key::Delete || key == gdk::Key::KP_Delete) && !typing {
                        obj.delete_action();
                        return glib::Propagation::Stop;
                    }

                    // Zoom to Fit: Ctrl + F
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

                    // Zoom to cursor: Z
                    if (key == gdk::Key::z || key == gdk::Key::Z) && !typing {
                        if let Some(page) = obj.imp().tab_view.selected_page() {
                            let (w, h) =
                                (page.child().width() as f64, page.child().height() as f64);
                            let pages = obj.imp().pages.borrow();
                            if let Some(d_ref) = pages.get(&page) {
                                let mut d = d_ref.borrow_mut();
                                let (mx, my) = obj.imp().mouse_pos.get();
                                let (wx, wy) = d.camera.screen_to_world(mx, my);
                                d.camera.zoom_to_point(wx, wy, (w, h), 2.0);
                            }
                        }
                        return glib::Propagation::Stop;
                    }

                    // คีย์ลัดเลือกเครื่องมือ (ไม่มี Ctrl/Alt และไม่ได้พิมพ์อยู่)
                    if !is_ctrl && !typing && !state.contains(gdk::ModifierType::ALT_MASK) {
                        let tool = match key {
                            gdk::Key::l | gdk::Key::L => Some(DrawingMode::Line),
                            gdk::Key::c | gdk::Key::C => Some(DrawingMode::Circle),
                            gdk::Key::a | gdk::Key::A => Some(DrawingMode::Arc),
                            gdk::Key::p | gdk::Key::P => Some(DrawingMode::Polyline),
                            gdk::Key::d | gdk::Key::D => Some(DrawingMode::Dimension),
                            gdk::Key::o | gdk::Key::O => Some(DrawingMode::Offset),
                            gdk::Key::t | gdk::Key::T => Some(DrawingMode::Trim),
                            gdk::Key::e | gdk::Key::E => Some(DrawingMode::Extend),
                            _ => None,
                        };
                        if let Some(mode) = tool {
                            obj.set_tool(mode);
                            return glib::Propagation::Stop;
                        }
                        if key == gdk::Key::s || key == gdk::Key::S {
                            obj.cancel_action(false);
                            return glib::Propagation::Stop;
                        }
                    }

                    // Save / Save As / Open
                    if is_ctrl && (key == gdk::Key::s || key == gdk::Key::S) {
                        if is_shift {
                            obj.save_as_action();
                        } else {
                            obj.save_action();
                        }
                        return glib::Propagation::Stop;
                    }
                    if is_ctrl && (key == gdk::Key::o || key == gdk::Key::O) {
                        obj.show_open_dialog();
                        return glib::Propagation::Stop;
                    }

                    glib::Propagation::Proceed
                }
            ));

            key_ctrl.connect_key_released(glib::clone!(
                #[weak]
                obj,
                move |_, key, _, _| {
                    if key == gdk::Key::Shift_L || key == gdk::Key::Shift_R {
                        obj.set_shift(false);
                    }
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
                    let dirty = obj
                        .imp()
                        .pages
                        .borrow()
                        .get(page)
                        .map(|d| d.borrow().dirty)
                        .unwrap_or(false);
                    if !dirty {
                        obj.imp().pages.borrow_mut().remove(page);
                        return glib::Propagation::Proceed;
                    }
                    obj.confirm_close_page(page);
                    glib::Propagation::Stop
                }
            ));

            obj.connect_close_request(glib::clone!(
                #[weak]
                obj,
                #[upgrade_or]
                glib::Propagation::Proceed,
                move |_| {
                    if obj.imp().force_close.get() {
                        return glib::Propagation::Proceed;
                    }
                    let any_dirty = obj.imp().pages.borrow().values().any(|d| d.borrow().dirty);
                    if !any_dirty {
                        return glib::Propagation::Proceed;
                    }
                    obj.confirm_quit();
                    glib::Propagation::Stop
                }
            ));

            self.btn_open.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| obj.show_open_dialog()
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
                move |_| obj.save_action()
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

            self.btn_select.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| obj.cancel_action(false)
            ));

            self.btn_arc.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| obj.set_tool(DrawingMode::Arc)
            ));

            self.btn_polyline.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| obj.set_tool(DrawingMode::Polyline)
            ));

            self.btn_offset.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.set_tool(DrawingMode::Offset);
                    glib::timeout_add_local_once(
                        std::time::Duration::from_millis(120),
                        glib::clone!(
                            #[weak]
                            obj,
                            move || {
                                if obj.imp().current_mode.get() == DrawingMode::Offset {
                                    obj.imp().spin_offset_dist.grab_focus();
                                }
                            }
                        ),
                    );
                }
            ));

            self.btn_offset_cancel.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| obj.cancel_action(true) // ออก + ยกเลิกการเลือก
            ));

            self.spin_offset_dist.connect_value_changed(glib::clone!(
                #[weak]
                obj,
                move |spin| obj.set_offset_distance(spin.value())
            ));

            // Enter ในช่องกรอก → คืนโฟกัสให้ canvas
            self.spin_offset_dist.connect_activate(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    if let Some(page) = obj.imp().tab_view.selected_page() {
                        page.child().grab_focus();
                    }
                }
            ));

            self.btn_offset_done.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| obj.cancel_action(false)
            ));

            self.btn_trim.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| obj.set_tool(DrawingMode::Trim)
            ));

            self.btn_extend.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| obj.set_tool(DrawingMode::Extend)
            ));

            self.btn_delete.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.delete_action();
                    if let Some(page) = obj.imp().tab_view.selected_page() {
                        page.child().grab_focus();
                    }
                }
            ));

            self.cmd_entry.connect_activate(glib::clone!(
                #[weak]
                obj,
                move |entry| {
                    let text = entry.text().to_string().to_lowercase().trim().to_string();

                    // offset / of [ระยะ]  หรือพิมพ์ตัวเลขล้วนตอนอยู่ในโหมด Offset
                    {
                        let mut parts = text.split_whitespace();
                        let first = parts.next().unwrap_or("");
                        let handled = if first == "of" || first == "offset" {
                            obj.set_tool(DrawingMode::Offset);
                            if let Some(v) = parts.next().and_then(|s| s.parse::<f64>().ok()) {
                                obj.set_offset_distance(v);
                            }
                            true
                        } else if obj.imp().current_mode.get() == DrawingMode::Offset {
                            match text.parse::<f64>() {
                                Ok(v) => {
                                    obj.set_offset_distance(v);
                                    true
                                }
                                Err(_) => false,
                            }
                        } else {
                            false
                        };
                        if handled {
                            entry.set_text("");
                            if let Some(page) = obj.imp().tab_view.selected_page() {
                                page.child().grab_focus();
                            }
                            return;
                        }
                    }

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
                            "r" | "redo" => obj.redo_action(),
                            "save" => obj.save_action(),
                            "saveas" => obj.save_as_action(),
                            "o" | "open" => obj.show_open_dialog(),
                            "a" | "arc" => obj.set_tool(DrawingMode::Arc),
                            "pl" | "pline" | "polyline" => obj.set_tool(DrawingMode::Polyline),
                            "tr" | "trim" => obj.set_tool(DrawingMode::Trim),
                            "ex" | "extend" => obj.set_tool(DrawingMode::Extend),
                            "del" | "delete" | "erase" => obj.delete_action(),
                            "la" | "layer" | "layers" => {
                                let b = &obj.imp().btn_layers;
                                b.set_active(!b.is_active());
                            }
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
                    if let Some(page) = obj.imp().tab_view.selected_page() {
                        page.child().grab_focus();
                    }
                }
            ));

            self.tab_view.connect_selected_page_notify(glib::clone!(
                #[weak]
                obj,
                move |_| obj.refresh_layers()
            ));

            self.layer_list.connect_row_selected(glib::clone!(
                #[weak]
                obj,
                move |_, row| {
                    if obj.imp().building_layers.get() {
                        return;
                    }
                    if let Some(row) = row {
                        let i = row.index() as usize;
                        obj.with_engine(|d| d.set_current_layer(i));
                    }
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

    pub fn set_tool(&self, mode: DrawingMode) {
        let imp = self.imp();
        imp.current_mode.set(mode);
        imp.start_pos.set(None);
        imp.dimension_step.set(0);
        imp.pending_points.borrow_mut().clear();
        imp.offset_source.set(None);
        *imp.offset_anim.borrow_mut() = None;
        imp.offset_stable.set(None);
        imp.offset_k.set(1.0);
        imp.offset_phase.set(0);
        imp.offset_fade.set(0.0);
        imp.offset_msg.borrow_mut().clear();
        if let Some(page) = imp.tab_view.selected_page() {
            page.child().queue_draw();
        }
        self.sync_tool_buttons();
    }

    fn set_shift(&self, down: bool) {
        let imp = self.imp();
        if imp.shift_down.replace(down) != down {
            if let Some(page) = imp.tab_view.selected_page() {
                page.child().queue_draw();
            }
        }
    }

    pub fn set_offset_distance(&self, v: f64) {
        let imp = self.imp();
        if v > 0.0 {
            imp.offset_distance.set(v);
            imp.offset_msg.borrow_mut().clear();
            if (imp.spin_offset_dist.value() - v).abs() > 1e-9 {
                imp.spin_offset_dist.set_value(v); // จากช่องคำสั่ง `of 25`
            }
        } else {
            *imp.offset_msg.borrow_mut() = "Distance must be greater than 0".into();
        }
        if let Some(page) = imp.tab_view.selected_page() {
            page.child().queue_draw();
        }
    }

    /// ให้ปุ่มของเครื่องมือที่ใช้อยู่ขึ้นสถานะ active (เรียกจาก tick ทุกเฟรม แต่ทำงานเฉพาะตอนโหมดเปลี่ยน)
    fn sync_tool_buttons(&self) {
        let imp = self.imp();
        let mode = imp.current_mode.get();
        if imp.synced_mode.get() == Some(mode) {
            return;
        }
        imp.synced_mode.set(Some(mode));

        let offset_page_shown =
            imp.right_dock.visible_child_name().as_deref() == Some("offset_settings");

        if mode == DrawingMode::Offset {
            imp.right_dock.set_visible_child_name("offset_settings");
            glib::timeout_add_local_once(
                std::time::Duration::from_millis(50),
                glib::clone!(
                    #[weak(rename_to = obj)]
                    self,
                    move || {
                        if obj.imp().current_mode.get() == DrawingMode::Offset {
                            obj.imp().revealer_offset_ui.set_reveal_child(true);
                        }
                    }
                ),
            );
        } else {
            // เลื่อนออกก่อน แล้วค่อยสลับ stack กลับ (เหมือน Array)
            imp.revealer_offset_ui.set_reveal_child(false);
            if offset_page_shown {
                glib::timeout_add_local_once(
                    std::time::Duration::from_millis(300),
                    glib::clone!(
                        #[weak(rename_to = obj)]
                        self,
                        move || {
                            let imp = obj.imp();
                            if imp.current_mode.get() != DrawingMode::Offset
                                && imp.right_dock.visible_child_name().as_deref()
                                    == Some("offset_settings")
                            {
                                imp.right_dock.set_visible_child_name("default");
                            }
                        }
                    ),
                );
            }
        }

        let buttons: [(&gtk::Button, DrawingMode); 10] = [
            (&imp.btn_select, DrawingMode::Select),
            (&imp.btn_line, DrawingMode::Line),
            (&imp.btn_circle, DrawingMode::Circle),
            (&imp.btn_arc, DrawingMode::Arc),
            (&imp.btn_polyline, DrawingMode::Polyline),
            (&imp.btn_dimension, DrawingMode::Dimension),
            (&imp.btn_array, DrawingMode::Array),
            (&imp.btn_offset, DrawingMode::Offset),
            (&imp.btn_trim, DrawingMode::Trim),
            (&imp.btn_extend, DrawingMode::Extend),
        ];

        for (btn, m) in buttons {
            // ระหว่างลากย้าย (Move) ถือว่ายังอยู่ใน Select
            let active = m == mode || (m == DrawingMode::Select && mode == DrawingMode::Move);
            if active {
                btn.add_css_class("tool-active");
            } else {
                btn.remove_css_class("tool-active");
            }
        }
    }

    /// ยกเลิกสิ่งที่ค้างอยู่แล้วกลับ Select (Esc = deselect ด้วย, ปุ่ม Select = ไม่ deselect)
    pub fn cancel_action(&self, deselect: bool) {
        let imp = self.imp();
        imp.start_pos.set(None);
        imp.pending_points.borrow_mut().clear();
        imp.offset_source.set(None);
        *imp.offset_anim.borrow_mut() = None;
        imp.offset_fade.set(0.0);
        imp.offset_msg.borrow_mut().clear();
        imp.dimension_step.set(0);
        imp.dim_temp_start.set(None);
        imp.dim_temp_end.set(None);
        imp.drag_start.set(None);
        imp.current_mode.set(DrawingMode::Select);

        // ปิด revealer ก่อน แล้วค่อย switch stack
        imp.revealer_array_ui.set_reveal_child(false);
        glib::timeout_add_local_once(
            std::time::Duration::from_millis(300),
            glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move || obj.imp().right_dock.set_visible_child_name("default")
            ),
        );

        if let Some(page) = imp.tab_view.selected_page() {
            page.child().grab_focus();
            if deselect {
                let pages = imp.pages.borrow();
                if let Some(d_ref) = pages.get(&page) {
                    d_ref.borrow_mut().deselect_all();
                }
            }
            page.child().queue_draw();
        }
    }

    /// จบ polyline แบบเปิด (กด Enter)
    fn finish_polyline(&self) {
        let imp = self.imp();
        let points: Vec<Point> = imp.pending_points.borrow_mut().drain(..).collect();
        if points.len() < 2 {
            return;
        }
        if let Some(page) = imp.tab_view.selected_page() {
            {
                let pages = imp.pages.borrow();
                if let Some(d_ref) = pages.get(&page) {
                    let mut d = d_ref.borrow_mut();
                    d.checkpoint();
                    d.push_new(ElementData::Polyline { points, closed: false });
                }
            }
            page.child().queue_draw();
        }
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
                obj.sync_tool_buttons();
                let pages = obj.imp().pages.borrow();
                if let Some(d_ref) = pages.get(&page) {
                    let mut d = d_ref.borrow_mut();

                    // เรียกใช้ฟังก์ชัน step ของ Engine
                    if d.step(clock.frame_time() as u64) {
                        area.queue_draw();
                    }

                    let anim_copy = *obj.imp().offset_anim.borrow();
                    if let Some(mut a) = anim_copy {
                        let now = clock.frame_time();
                        let start = a.start_us.unwrap_or(now);
                        a.start_us = Some(start);
                        a.t = ((now - start) as f64 / 1_000_000.0 / OFFSET_ANIM_SECS).min(1.0);
                        const OFFSET_FLIP_SECS: f64 = 0.09;

                        if a.t >= 1.0 {
                            let side = Point { x: a.side.0, y: a.side.1 };
                            match d.offset_apply(a.source, side, a.dist) {
                                Ok(()) => {
                                    d.deselect_all();
                                    obj.imp().offset_source.set(None);
                                    obj.imp().offset_msg.borrow_mut().clear();
                                    obj.imp().offset_fade.set(0.0);
                                }
                                Err(msg) => *obj.imp().offset_msg.borrow_mut() = msg.into(),
                            }
                            *obj.imp().offset_anim.borrow_mut() = None;
                        } else {
                            *obj.imp().offset_anim.borrow_mut() = Some(a);
                        }
                        area.queue_draw();
                    }

                    let imp = obj.imp();
                    let active = imp.tab_view.selected_page().as_ref() == Some(&page)
                        && imp.current_mode.get() == DrawingMode::Offset
                        && imp.offset_anim.borrow().is_none();
                    match (active, imp.offset_source.get()) {
                        (true, Some(id)) => {
                            let now = clock.frame_time();
                            let last = imp.last_frame_us.replace(now);
                            let dt = if last == 0 { 0.016 } else { ((now - last) as f64 / 1e6).min(0.1) };

                            let (mx, my) = imp.mouse_pos.get();
                            let target = d.camera.screen_to_world(mx, my);
                            let dist = imp.offset_distance.get();
                            let mut stable = imp.offset_stable.get().unwrap_or(target);

                            // ฝั่งของเมาส์ตอนนี้ต่างจากฝั่งที่ ghost แสดงอยู่ไหม
                            let differs = dist > 0.0 && {
                                let a = d.offset_preview(id, Point { x: stable.0, y: stable.1 }, dist);
                                let b = d.offset_preview(id, Point { x: target.0, y: target.1 }, dist);
                                format!("{:?}", a) != format!("{:?}", b)
                            };

                            let mut k = imp.offset_k.get();
                            let mut phase = imp.offset_phase.get();
                            if differs && phase != 1 {
                                phase = 1;
                            }
                            match phase {
                                1 => {
                                    k -= dt / OFFSET_FLIP_SECS;
                                    if k <= 0.0 {
                                        k = 0.0;
                                        stable = target; // สลับฝั่งตอนหดจนสุด
                                        phase = 2;
                                    }
                                }
                                2 => {
                                    k += dt / OFFSET_FLIP_SECS;
                                    if k >= 1.0 {
                                        k = 1.0;
                                        phase = 0;
                                    }
                                    if !differs {
                                        stable = target;
                                    }
                                }
                                _ => {
                                    if !differs {
                                        stable = target;
                                    }
                                }
                            }
                            imp.offset_stable.set(Some(stable));
                            imp.offset_k.set(k);
                            imp.offset_phase.set(phase);
                            imp.offset_fade.set((imp.offset_fade.get() + dt / 0.12).min(1.0));
                            area.queue_draw();
                        }
                        _ => imp.last_frame_us.set(0),
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
            move |gesture, _, x, y| {
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

                            let hit_selected = d
                                .hovered_id
                                .map(|id| d.history.iter().any(|e| e.id == id && e.is_selected))
                                .unwrap_or(false);

                            if hit_selected {
                                d.checkpoint(); // ก่อนเริ่มย้าย
                                obj.imp().current_mode.set(DrawingMode::Move);
                            }
                        }

                        DrawingMode::Line | DrawingMode::Circle => {
                            // Logic การวาดเดิม (มิติ: คลิกแรกเก็บจุดเริ่ม)
                            if imp.start_pos.get().is_none() {
                                let (sx, sy, _) = d.get_input_snap(wx, wy, imp.snap_enabled.get(), &imp.snap_settings.get());
                                imp.start_pos.set(Some((sx, sy)));
                            }
                        }

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
                            let (sx, sy, _) =
                                d.get_input_snap(wx, wy, imp.snap_enabled.get(), &imp.snap_settings.get());
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
                                }
                                2 => {
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

                                        d.checkpoint();
                                        d.push_new(ElementData::Dimension { start, end, offset });
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
                        DrawingMode::Arc => {
                            let (sx, sy, _) = d.get_input_snap(
                                wx,
                                wy,
                                imp.snap_enabled.get(),
                                &imp.snap_settings.get(),
                            );
                            let mut pts = imp.pending_points.borrow_mut();
                            pts.push(Point { x: sx, y: sy });
                            if pts.len() == 3 {
                                if let Some((center, radius, s, e)) =
                                    arc_from_three_points(pts[0], pts[1], pts[2])
                                {
                                    d.checkpoint();
                                    d.push_new(ElementData::Arc { center, radius, start_angle: s, end_angle: e });
                                }
                                pts.clear();
                            }
                        }
                        DrawingMode::Polyline => {
                            let (sx, sy, _) = d.get_input_snap(
                                wx,
                                wy,
                                imp.snap_enabled.get(),
                                &imp.snap_settings.get(),
                            );
                            let p = Point { x: sx, y: sy };
                            let mut pts = imp.pending_points.borrow_mut();
                            let close = pts.len() >= 3 && {
                                let f = pts[0];
                                (p.x - f.x).hypot(p.y - f.y) <= 8.0 / d.camera.scale
                            };
                            if close {
                                let points: Vec<Point> = pts.drain(..).collect();
                                d.checkpoint();
                                d.push_new(ElementData::Polyline { points, closed: true });
                            } else if pts
                                .last()
                                .map_or(true, |l| (l.x - p.x).hypot(l.y - p.y) > 1e-9)
                            {
                                pts.push(p);
                            }
                        }

                        DrawingMode::Offset => {
                            let dist = imp.offset_distance.get();
                            if dist <= 0.0 {
                                *imp.offset_msg.borrow_mut() =
                                    "Type a distance in the command bar first".into();
                            } else if imp.offset_anim.borrow().is_some() {
                                // กำลังเล่นอนิเมชัน ไม่รับคลิก
                            } else if let Some(id) = imp.offset_source.get() {
                                let side = Point { x: wx, y: wy };
                                if d.offset_preview(id, side, dist).is_some() {
                                    *imp.offset_anim.borrow_mut() = Some(OffsetAnim {
                                        source: id,
                                        side: (wx, wy),
                                        dist,
                                        start_us: None,
                                        t: 0.0,
                                    });
                                    imp.offset_msg.borrow_mut().clear();
                                } else if let Err(msg) = d.offset_apply(id, side, dist) {
                                    *imp.offset_msg.borrow_mut() = msg.into();
                                }
                            } else if let Some(id) = d.offset_pick(wx, wy) {
                                imp.offset_source.set(Some(id));
                                imp.offset_fade.set(0.0);
                                imp.offset_msg.borrow_mut().clear();
                            }
                        }

                        DrawingMode::Trim | DrawingMode::Extend => {
                            let swap = gesture.current_event_state().contains(gdk::ModifierType::SHIFT_MASK)
                                || imp.shift_down.get();
                            let trimming = (imp.current_mode.get() == DrawingMode::Trim) != swap;
                            let r = if trimming { d.trim_at(wx, wy) } else { d.extend_at(wx, wy) };
                            match r {
                                Ok(()) => imp.offset_msg.borrow_mut().clear(),
                                Err(m) => *imp.offset_msg.borrow_mut() = m.into(),
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
                        let (ex, ey, _) = d.get_input_snap(
                            wx,
                            wy,
                            imp.snap_enabled.get(),
                            &imp.snap_settings.get(),
                        );

                        match imp.current_mode.get() {
                            DrawingMode::Line => {
                                d.checkpoint();
                                d.push_new(ElementData::Line {
                                    start: Point { x: sx, y: sy },
                                    end: Point { x: ex, y: ey },
                                });
                                imp.start_pos.set(Some((ex, ey)));
                            }
                            DrawingMode::Circle => {
                                let dx = ex - sx;
                                let dy = ey - sy;
                                let radius = (dx * dx + dy * dy).sqrt();
                                d.checkpoint();
                                d.push_new(ElementData::Circle {
                                    center: Point { x: sx, y: sy },
                                    radius,
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

        // Background
        cr.set_source_rgb(0.1, 0.1, 0.1);
        let _ = cr.paint();

        // Grid & Snap Calculation
        let grid_base = 100.0;
        let lod_raw = scale.log10();
        let lod = lod_raw.floor();
        let fraction = lod_raw - lod;

        let step_major = grid_base / 10.0f64.powf(lod);
        let step_minor = step_major / 10.0;
        let step_giant = step_major * 10.0;

        // --- คำนวณ Snap Position แบบคาดเดาได้ (Predictable) ---
        let (wx, wy) = cam.screen_to_world(rx, ry);
        let (sn_x, sn_y, snap_kind) =
            d.get_input_snap(wx, wy, imp.snap_enabled.get(), &imp.snap_settings.get());

        // Grid Drawing Function
        let draw_grid_line = |s: f64, alpha: f64, lw: f64| {
            if alpha <= 0.02 || s * scale < 8.0 {
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

        // --- Grid Logic ---

        draw_grid_line(step_giant, (1.0 - fraction) * 0.15, 1.0);
        draw_grid_line(step_major, 0.35, 0.8);
        draw_grid_line(step_minor, fraction * 0.15, 0.75);

        // Draw Elements
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.set_line_width(2.0);

        // ขอรายการสิ่งที่ต้องวาดจาก Engine
        let commands = d.get_render_commands(w, h);
        CairoRenderer::new(cr).draw_commands(commands);

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
                DrawingMode::Arc | DrawingMode::Polyline | DrawingMode::Offset => {}
                DrawingMode::Trim => {}
                DrawingMode::Extend => {}

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
                    RenderCommand::Arc { cx, cy, radius, start_angle, end_angle, .. } => {
                        cr.set_source_rgba(1.0, 0.5, 0.0, 0.4);
                        cr.set_line_width(1.5);
                        cr.arc_negative(cx, cy, radius, -start_angle, -end_angle);
                        let _ = cr.stroke();
                    }
                    RenderCommand::Polyline { points, closed, .. } => {
                        cr.set_source_rgba(1.0, 0.5, 0.0, 0.4);
                        cr.set_line_width(1.5);
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
                    RenderCommand::Dimension { .. } => {} // placeholder
                }
            }
        }

        // --- Arc / Polyline preview ---
        {
            let pending = imp.pending_points.borrow();
            let mode = imp.current_mode.get();
            if !pending.is_empty() && (mode == DrawingMode::Arc || mode == DrawingMode::Polyline) {
                cr.set_source_rgba(1.0, 1.0, 1.0, 0.5);
                cr.set_line_width(2.0);
                let cursor = Point { x: sn_x, y: sn_y };
                let mut pts: Vec<Point> = pending.clone();
                match mode {
                    DrawingMode::Arc if pts.len() == 2 => {
                        if let Some((c, r, s, e)) = arc_from_three_points(pts[0], pts[1], cursor) {
                            let (cx, cy) = cam.world_to_screen(c.x, c.y);
                            cr.arc_negative(cx, cy, r * cam.scale, -s, -e);
                        } else {
                            pts.push(cursor);
                        }
                    }
                    _ => pts.push(cursor),
                }
                if !(mode == DrawingMode::Arc && pts.len() == 2 && pending.len() == 2) {
                    for (i, p) in pts.iter().enumerate() {
                        let (px, py) = cam.world_to_screen(p.x, p.y);
                        if i == 0 {
                            cr.move_to(px, py);
                        } else {
                            cr.line_to(px, py);
                        }
                    }
                }
                let _ = cr.stroke();
            }
        }

        // --- Offset preview ---
        if imp.current_mode.get() == DrawingMode::Offset {
            let anim = *imp.offset_anim.borrow();
            let (src, side, dist) = match anim {
                Some(a) => (
                    Some(a.source),
                    Point { x: a.side.0, y: a.side.1 },
                    a.dist, // ระยะเต็ม ไม่ขยายจาก 0 แล้ว
                ),
                None => {
                    let k = imp.offset_k.get();
                    let e = k * k * (3.0 - 2.0 * k); // smoothstep
                    let s = imp.offset_stable.get().unwrap_or((wx, wy));
                    (
                        imp.offset_source.get(),
                        Point { x: s.0, y: s.1 },
                        (imp.offset_distance.get() * e).max(1e-6),
                    )
                }
            };

            let alpha = if anim.is_some() { 1.0 } else { imp.offset_fade.get() };

            if let Some(id) = src {
                if let Some(data) = d.offset_preview(id, side, dist) {
                    cr.new_path();
                    let c = anim.map_or(0.0, |a| 1.0 - (1.0 - a.t).powi(3)); // 0 → 1 ตอนวาง
                    cr.set_source_rgba(1.0, 0.5 + 0.5 * c, c, 0.8 * alpha + 0.2 * c);
                    cr.set_line_width(1.5);
                    match data {
                        ElementData::Line { start, end } => {
                            let (a, b) = (cam.world_to_screen(start.x, start.y), cam.world_to_screen(end.x, end.y));
                            cr.move_to(a.0, a.1);
                            cr.line_to(b.0, b.1);
                        }
                        ElementData::Circle { center, radius } => {
                            let c = cam.world_to_screen(center.x, center.y);
                            cr.arc(c.0, c.1, radius * cam.scale, 0.0, std::f64::consts::TAU);
                        }
                        ElementData::Arc { center, radius, start_angle, end_angle } => {
                            let c = cam.world_to_screen(center.x, center.y);
                            cr.arc_negative(c.0, c.1, radius * cam.scale, -start_angle, -end_angle);
                        }
                        ElementData::Polyline { points, closed } => {
                            for (i, p) in points.iter().enumerate() {
                                let s = cam.world_to_screen(p.x, p.y);
                                if i == 0 { cr.move_to(s.0, s.1); } else { cr.line_to(s.0, s.1); }
                            }
                            if closed { cr.close_path(); }
                        }
                        _ => {}
                    }
                    let _ = cr.stroke();
                }
            }
        }

        // --- Trim / Extend preview ---
        {
            let mode = imp.current_mode.get();
            if matches!(mode, DrawingMode::Trim | DrawingMode::Extend) {
                let trimming = (mode == DrawingMode::Trim) != imp.shift_down.get();
                let prev = if trimming { d.trim_preview(wx, wy) } else { d.extend_preview(wx, wy) };
                if let Some(data) = prev {
                    cr.new_path();
                    if trimming {
                        cr.set_source_rgba(1.0, 0.25, 0.25, 0.9); // แดง = ส่วนที่จะหาย
                    } else {
                        cr.set_source_rgba(0.3, 1.0, 0.5, 0.9); // เขียว = ส่วนที่จะเพิ่ม
                    }
                    cr.set_line_width(3.0);
                    trace_element(cr, cam, &data);
                    let _ = cr.stroke();
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

        // Crosshair
        let (cx, cy) = cam.world_to_screen(sn_x, sn_y);
        cr.set_source_rgb(1.0, 0.0, 0.0);
        cr.set_line_width(1.0);
        cr.move_to(cx, 0.0);
        cr.line_to(cx, h);
        cr.move_to(0.0, cy);
        cr.line_to(w, cy);
        let _ = cr.stroke();

        // Object snap marker
        if let Some(kind) = snap_kind {
            let (mx, my) = cam.world_to_screen(sn_x, sn_y);
            let r = 6.0;
            cr.set_source_rgb(0.0, 1.0, 0.4);
            cr.set_line_width(2.0);
            cr.new_path();
            match kind {
                SnapKind::Endpoint => cr.rectangle(mx - r, my - r, 2.0 * r, 2.0 * r),
                SnapKind::Midpoint => {
                    cr.move_to(mx, my - r);
                    cr.line_to(mx + r, my + r);
                    cr.line_to(mx - r, my + r);
                    cr.close_path();
                }
                SnapKind::Center => {
                    cr.new_sub_path();
                    cr.arc(mx, my, r, 0.0, 2.0 * std::f64::consts::PI);
                }
                SnapKind::Quadrant => {
                    cr.move_to(mx, my - r);
                    cr.line_to(mx + r, my);
                    cr.line_to(mx, my + r);
                    cr.line_to(mx - r, my);
                    cr.close_path();
                }
                SnapKind::Intersection => {
                    cr.move_to(mx - r, my - r);
                    cr.line_to(mx + r, my + r);
                    cr.move_to(mx + r, my - r);
                    cr.line_to(mx - r, my + r);
                }
            }
            let _ = cr.stroke();
        }

        // OSD
        let mode_text = match imp.current_mode.get() {
            DrawingMode::Line => "LINE",
            DrawingMode::Circle => "CIRCLE",
            DrawingMode::Select => "SELECT",
            DrawingMode::Array => "ARRAY (LINEAR)",
            DrawingMode::Move => "MOVE",
            DrawingMode::Dimension => "DIM (LINEAR)",
            DrawingMode::Arc => "ARC (3-POINT)",
            DrawingMode::Polyline => "POLYLINE",
            DrawingMode::Offset => "OFFSET",
            DrawingMode::Trim => "TRIM",
            DrawingMode::Extend => "EXTEND",
        };

        let offset_hint = if imp.current_mode.get() == DrawingMode::Offset {
            let dist = imp.offset_distance.get();
            let msg = imp.offset_msg.borrow();
            let step = if dist <= 0.0 {
                "type distance + Enter".to_string()
            } else if imp.offset_source.get().is_none() {
                format!("d={} | pick object", dist)
            } else {
                format!("d={} | click side", dist)
            };
            if msg.is_empty() { format!("  {}", step) } else { format!("  {} | {}", step, msg) }
        } else if matches!(imp.current_mode.get(), DrawingMode::Trim | DrawingMode::Extend) {
            let msg = imp.offset_msg.borrow();
            let trimming = (imp.current_mode.get() == DrawingMode::Trim) != imp.shift_down.get();
            let step = if trimming {
                "click the part to remove | Shift: extend"
            } else {
                "click near the end to extend | Shift: trim"
            };
            if msg.is_empty() { format!("  {}", step) } else { format!("  {} | {}", step, msg) }
        } else {
            String::new()
        };

        let txt = format!(
            "MODE: [{}]  LAYER: {}  X: {:.2} Y: {:.2} {}{}{}",
            mode_text,
            d.layers[d.current_layer].name,
            sn_x,
            sn_y,
            if imp.snap_enabled.get() { "[SNAP]" } else { "" },
            snap_kind.map(|k| format!(" [{}]", k.label())).unwrap_or_default(),
            offset_hint,
        );

        let layout = gtk::pango::Layout::new(&page.child().pango_context());
        layout.set_text(&txt);
        let font_desc = gtk::pango::FontDescription::from_string("Monospace 11");
        layout.set_font_description(Some(&font_desc));

        let padding = 10.0;

        cr.set_source_rgb(1.0, 1.0, 0.0);
        cr.move_to(220.0 + padding, 15.0 + padding);
        let _ = pangocairo::functions::show_layout(cr, &layout);
    }

    /// โฟกัสอยู่ในช่องที่รับข้อความ (ช่องคำสั่ง, spin ของ Array) หรือไม่
    fn is_typing(&self) -> bool {
        gtk::prelude::GtkWindowExt::focus(self)
            .map_or(false, |w| w.is::<gtk::Editable>())
    }

    fn sync_snap_settings(&self) {
        let imp = self.imp();
        imp.snap_settings.set(SnapSettings {
            endpoint: imp.chk_snap_endpoint.is_active(),
            midpoint: imp.chk_snap_midpoint.is_active(),
            center: imp.chk_snap_center.is_active(),
            quadrant: imp.chk_snap_quadrant.is_active(),
            intersection: imp.chk_snap_intersection.is_active(),
        });
        if let Some(page) = imp.tab_view.selected_page() {
            page.child().queue_draw();
        }
    }

    pub fn delete_action(&self) {
        if let Some(page) = self.imp().tab_view.selected_page() {
            let pages = self.imp().pages.borrow();
            if let Some(d_ref) = pages.get(&page) {
                d_ref.borrow_mut().delete_selected();
                page.child().queue_draw();
            }
        }
    }

    fn with_engine<R>(&self, f: impl FnOnce(&mut crate::engine::Engine) -> R) -> Option<R> {
        let imp = self.imp();
        let page = imp.tab_view.selected_page()?;
        let pages = imp.pages.borrow();
        let d_ref = pages.get(&page)?;
        let r = f(&mut d_ref.borrow_mut());
        page.child().queue_draw();
        Some(r)
    }

    pub fn refresh_layers(&self) {
        let imp = self.imp();
        imp.building_layers.set(true);
        while let Some(row) = imp.layer_list.first_child() {
            imp.layer_list.remove(&row);
        }

        let snapshot = imp.tab_view.selected_page().and_then(|page| {
            let pages = imp.pages.borrow();
            pages.get(&page).map(|d| {
                let d = d.borrow();
                (d.layers.clone(), d.current_layer)
            })
        });
        let Some((layers, current)) = snapshot else {
            imp.building_layers.set(false);
            return;
        };

        let eye_icon = |on: bool| if on { "view-reveal-symbolic" } else { "view-conceal-symbolic" };
        let lock_icon = |on: bool| if on { "changes-prevent-symbolic" } else { "changes-allow-symbolic" };

        for (i, l) in layers.iter().enumerate() {
            let row = gtk::ListBoxRow::new();
            let hb = gtk::Box::builder()
                .spacing(6)
                .margin_start(4)
                .margin_end(4)
                .margin_top(2)
                .margin_bottom(2)
                .build();

            let dot = gtk::DrawingArea::builder()
                .content_width(14)
                .content_height(14)
                .valign(gtk::Align::Center)
                .build();
            let rgb = l.color;
            dot.set_draw_func(move |_, cr, w, h| {
                cr.set_source_rgb(rgb.0 as f64 / 255.0, rgb.1 as f64 / 255.0, rgb.2 as f64 / 255.0);
                cr.arc(w as f64 / 2.0, h as f64 / 2.0, (w.min(h) as f64) / 2.0, 0.0, std::f64::consts::TAU);
                let _ = cr.fill();
            });

            let name = gtk::Label::builder()
                .label(&l.name)
                .xalign(0.0)
                .hexpand(true)
                .halign(gtk::Align::Fill)
                .width_chars(14)
                .max_width_chars(24)
                .ellipsize(gtk::pango::EllipsizeMode::End)
                .build();

            let eye = gtk::ToggleButton::builder()
                .icon_name(eye_icon(l.visible))
                .active(l.visible)
                .tooltip_text("Show / hide")
                .css_classes(["flat"])
                .build();

            let hb_row = hb.clone();
            eye.connect_toggled(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |b| {
                    b.set_icon_name(eye_icon(b.is_active()));
                    hb_row.set_opacity(if b.is_active() { 1.0 } else { 0.45 });
                    obj.with_engine(|d| d.set_layer_visible(i, b.is_active()));
                }
            ));

            let lock = gtk::ToggleButton::builder()
                .icon_name(lock_icon(l.locked))
                .active(l.locked)
                .tooltip_text("Lock / unlock")
                .css_classes(["flat"])
                .build();
            lock.connect_toggled(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |b| {
                    b.set_icon_name(lock_icon(b.is_active()));
                    obj.with_engine(|d| d.set_layer_locked(i, b.is_active()));
                    if b.is_active() { b.add_css_class("layer-locked") } else { b.remove_css_class("layer-locked") }
                    if b.is_active() {
                        b.add_css_class("layer-locked");
                    } else {
                        b.remove_css_class("layer-locked");
                    }
                }
            ));

            let btns = gtk::Box::builder()
                .css_classes(["linked"])
                .halign(gtk::Align::End)
                .build();
            btns.append(&eye);
            btns.append(&lock);

            hb.append(&dot);
            hb.append(&name);
            hb.append(&btns);

            row.set_child(Some(&hb));
            imp.layer_list.append(&row);
            if i == current {
                imp.layer_list.select_row(Some(&row));
            }
        }
        imp.building_layers.set(false);
    }

    pub fn undo_action(&self) {
        if let Some(page) = self.imp().tab_view.selected_page() {
            let pages = self.imp().pages.borrow();
            if let Some(d_ref) = pages.get(&page) {
                let mut d = d_ref.borrow_mut();
                d.undo_last_action();
                page.child().queue_draw();
            }
        }
    }

    pub fn redo_action(&self) {
        if let Some(page) = self.imp().tab_view.selected_page() {
            let pages = self.imp().pages.borrow();
            if let Some(d_ref) = pages.get(&page) {
                d_ref.borrow_mut().redo_last_action();
                page.child().queue_draw();
            }
        }
    }

    pub fn save_action(&self) {
        if let Some(page) = self.imp().tab_view.selected_page() {
            self.save_page(&page, |_| {});
        }
    }

    pub fn save_as_action(&self) {
        if let Some(page) = self.imp().tab_view.selected_page() {
            self.save_page_as(&page, |_| {});
        }
    }

    /// มี path แล้วเขียนเลย ไม่มีก็เปิด Save As
    pub fn save_page(&self, page: &adw::TabPage, on_done: impl Fn(bool) + 'static) {
        let path = self
            .imp()
            .pages
            .borrow()
            .get(page)
            .and_then(|d| d.borrow().file_path.clone());
        match path {
            Some(p) => {
                let ok = self.write_page_to(page, p);
                on_done(ok);
            }
            None => self.save_page_as(page, on_done),
        }
    }

    pub fn save_page_as(&self, page: &adw::TabPage, on_done: impl Fn(bool) + 'static) {
        let (_, filters, dxf_filter) = dxf_filters();
        let current = self
            .imp()
            .pages
            .borrow()
            .get(page)
            .and_then(|d| d.borrow().file_path.clone());

        let dialog = gtk::FileDialog::builder()
            .title("Save Drawing")
            .accept_label("_Save")
            .filters(&filters)
            .default_filter(&dxf_filter)
            .build();
        match current {
            Some(p) => dialog.set_initial_file(Some(&gio::File::for_path(p))),
            None => dialog.set_initial_name(Some("drawing.dxf")),
        }

        dialog.save(
            Some(self),
            gio::Cancellable::NONE,
            glib::clone!(
                #[weak(rename_to = obj)]
                self,
                #[weak]
                page,
                move |res| {
                    let ok = match res {
                        Ok(file) => file
                            .path()
                            .map(|p| obj.write_page_to(&page, p))
                            .unwrap_or(false),
                        Err(_) => false, // ผู้ใช้กดยกเลิก
                    };
                    on_done(ok);
                }
            ),
        );
    }

    fn write_page_to(&self, page: &adw::TabPage, mut path: PathBuf) -> bool {
        if path.extension().is_none() {
            path.set_extension("dxf");
        }
        let pages = self.imp().pages.borrow();
        let Some(d_ref) = pages.get(page) else { return false };

        let bytes = match engine_to_dxf_bytes(&d_ref.borrow()) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("Error generating DXF: {}", e);
                return false;
            }
        };
        if let Err(e) = std::fs::write(&path, bytes) {
            eprintln!("Error writing {:?}: {}", path, e);
            return false;
        }

        let mut d = d_ref.borrow_mut();
        d.dirty = false;
        if let Some(name) = path.file_name() {
            page.set_title(&name.to_string_lossy());
        }
        d.file_path = Some(path);
        true
    }

    pub fn show_open_dialog(&self) {
        let (_, filters, dxf_filter) = dxf_filters();
        let dialog = gtk::FileDialog::builder()
            .title("Open Drawing")
            .accept_label("_Open")
            .filters(&filters)
            .default_filter(&dxf_filter)
            .build();

        dialog.open(
            Some(self),
            gio::Cancellable::NONE,
            glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |res| {
                    if let Ok(file) = res {
                        if let Some(path) = file.path() {
                            obj.open_path(path);
                        }
                    }
                }
            ),
        );
    }

    pub fn open_path(&self, path: PathBuf) {
        let obj = self.clone();
        glib::spawn_future_local(async move {
            let title = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "Untitled".to_string());

            let p = path.clone();
            let result = gio::spawn_blocking(move || {
                acadrust::DxfReader::from_file(&p)
                    .and_then(|r| r.read())
                    .map(|d| dxf_to_elements(&d))
                    .map_err(|e| e.to_string())
            })
            .await;

            let imported = match result {
                Ok(Ok(v)) => v,
                Ok(Err(e)) => {
                    eprintln!("Error opening {:?}: {}", path, e);
                    obj.show_error("Can't open file", &format!("{}\n\n{}", title, e));
                    return;
                }
                Err(_) => {
                    obj.show_error(
                        "Can't open file",
                        &format!("{}\n\nThe DXF reader failed on this file.", title),
                    );
                    return;
                }
            };
            let skipped = imported.skipped;

            obj.create_new_tab(&title);
            let Some(page) = obj.imp().tab_view.selected_page() else { return };
            {
                let pages = obj.imp().pages.borrow();
                let Some(d_ref) = pages.get(&page) else { return };
                let mut d = d_ref.borrow_mut();
                d.layers = imported.layers;
                d.current_layer = 0;
                for (data, layer, color) in imported.items {
                    d.push_imported(data, layer, color);
                }
                d.file_path = Some(path.clone());
                d.dirty = false;
            }
            obj.refresh_layers();

            if skipped > 0 {
                eprintln!("Skipped {} unsupported entities", skipped);
            }

            glib::timeout_add_local_once(
                std::time::Duration::from_millis(100),
                glib::clone!(
                    #[weak]
                    obj,
                    #[weak]
                    page,
                    move || {
                        let pages = obj.imp().pages.borrow();
                        if let Some(d_ref) = pages.get(&page) {
                            let (w, h) = (page.child().width() as f64, page.child().height() as f64);
                            d_ref.borrow_mut().zoom_to_fit(w, h);
                        }
                        page.child().queue_draw();
                    }
                ),
            );
        });
    }

    fn show_error(&self, heading: &str, body: &str) {
        let dialog = adw::AlertDialog::new(Some(heading), Some(body));
        dialog.add_responses(&[("ok", "_OK")]);
        dialog.set_default_response(Some("ok"));
        dialog.set_close_response("ok");
        dialog.present(Some(self));
    }

    fn confirm_close_page(&self, page: &adw::TabPage) {
        let dialog = adw::AlertDialog::new(
            Some("Save changes?"),
            Some(&format!("“{}” has unsaved changes.", page.title())),
        );
        dialog.add_responses(&[("cancel", "_Cancel"), ("discard", "_Discard"), ("save", "_Save")]);
        dialog.set_response_appearance("discard", adw::ResponseAppearance::Destructive);
        dialog.set_response_appearance("save", adw::ResponseAppearance::Suggested);
        dialog.set_default_response(Some("save"));
        dialog.set_close_response("cancel");

        dialog.connect_response(
            None,
            glib::clone!(
                #[weak(rename_to = obj)]
                self,
                #[weak]
                page,
                move |_, response| {
                    let view = obj.imp().tab_view.clone();
                    match response {
                        "save" => {
                            let (obj2, view2, page2) = (obj.clone(), view.clone(), page.clone());
                            obj.save_page(&page, move |ok| {
                                if ok {
                                    obj2.imp().pages.borrow_mut().remove(&page2);
                                }
                                view2.close_page_finish(&page2, ok);
                            });
                        }
                        "discard" => {
                            obj.imp().pages.borrow_mut().remove(&page);
                            view.close_page_finish(&page, true);
                        }
                        _ => view.close_page_finish(&page, false),
                    }
                }
            ),
        );
        dialog.present(Some(self));
    }

    fn confirm_quit(&self) {
        let dialog = adw::AlertDialog::new(
            Some("Discard unsaved changes?"),
            Some("Some drawings have unsaved changes that will be lost."),
        );
        dialog.add_responses(&[("cancel", "_Cancel"), ("discard", "_Discard and Quit")]);
        dialog.set_response_appearance("discard", adw::ResponseAppearance::Destructive);
        dialog.set_close_response("cancel");
        dialog.connect_response(
            None,
            glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |_, response| {
                    if response == "discard" {
                        obj.imp().force_close.set(true);
                        obj.close();
                    }
                }
            ),
        );
        dialog.present(Some(self));
    }
}

fn trace_element(cr: &gtk::cairo::Context, cam: &crate::engine::Camera, data: &ElementData) {
    match data {
        ElementData::Line { start, end } => {
            let (a, b) = (cam.world_to_screen(start.x, start.y), cam.world_to_screen(end.x, end.y));
            cr.move_to(a.0, a.1);
            cr.line_to(b.0, b.1);
        }
        ElementData::Circle { center, radius } => {
            let c = cam.world_to_screen(center.x, center.y);
            cr.arc(c.0, c.1, radius * cam.scale, 0.0, std::f64::consts::TAU);
        }
        ElementData::Arc { center, radius, start_angle, end_angle } => {
            let c = cam.world_to_screen(center.x, center.y);
            cr.arc_negative(c.0, c.1, radius * cam.scale, -start_angle, -end_angle);
        }

        ElementData::Polyline { points, closed } => {
            let mut it = points.iter();
            if let Some(p0) = it.next() {
                let (x, y) = cam.world_to_screen(p0.x, p0.y);
                cr.move_to(x, y);
                for p in it {
                    let (x, y) = cam.world_to_screen(p.x, p.y);
                    cr.line_to(x, y);
                }
                if *closed {
                    cr.close_path();
                }
            }
        }

        _ => {}
    }
}

// ===== วางใน src/window.rs ระดับบนสุด (ใกล้ bulge_segment) =====

/// ประมาณ B-spline/NURBS เป็นจุดบนเส้นโค้ง (de Boor) ถ้า knots ไม่ครบ คืน None
fn spline_points(
    degree: usize,
    knots: &[f64],
    ctrl: &[(f64, f64)],
    weights: &[f64],
) -> Option<Vec<Point>> {
    let n = ctrl.len();
    if degree == 0 || n <= degree || knots.len() != n + degree + 1 {
        return None;
    }
    // พิกัด homogeneous (x*w, y*w, w) ใช้ได้ทั้ง B-spline ธรรมดาและ NURBS
    let rational = weights.len() == n;
    let hp: Vec<[f64; 3]> = ctrl
        .iter()
        .enumerate()
        .map(|(i, (x, y))| {
            let w = if rational { weights[i] } else { 1.0 };
            [x * w, y * w, w]
        })
        .collect();

    let (u0, u1) = (knots[degree], knots[n]);
    if !(u1 > u0) {
        return None;
    }

    let eval = |u: f64, k: usize| -> Point {
        let mut d: Vec<[f64; 3]> = (0..=degree).map(|j| hp[j + k - degree]).collect();
        for r in 1..=degree {
            for j in (r..=degree).rev() {
                let i = j + k - degree;
                let denom = knots[i + degree + 1 - r] - knots[i];
                let a = if denom.abs() < 1e-14 { 0.0 } else { (u - knots[i]) / denom };
                for c in 0..3 {
                    d[j][c] = d[j - 1][c] * (1.0 - a) + d[j][c] * a;
                }
            }
        }
        let p = d[degree];
        let w = if p[2].abs() < 1e-14 { 1.0 } else { p[2] };
        Point { x: p[0] / w, y: p[1] / w }
    };

    const PER_SPAN: usize = 8;
    let mut out = Vec::new();
    for k in degree..n {
        let (a, b) = (knots[k], knots[k + 1]);
        if !(b > a) {
            continue; // ช่วงว่าง (knot ซ้ำ)
        }
        let last = k == n - 1;
        for s in 0..PER_SPAN {
            out.push(eval(a + (b - a) * s as f64 / PER_SPAN as f64, k));
        }
        if last {
            out.push(eval(b, k)); // ปลายสุด
        }
    }
    if out.len() >= 2 { Some(out) } else { None }
}

/// เส้นผ่านจุด fit (ใช้เมื่อ spline ไม่มี control points)
fn polyline_through(pts: &[(f64, f64)]) -> Vec<Point> {
    pts.iter().map(|(x, y)| Point { x: *x, y: *y }).collect()
}

fn ellipse_points(
    center: (f64, f64),
    major: (f64, f64),
    ratio: f64,
    start: f64,
    end: f64,
) -> (Vec<Point>, bool) {
    let minor = (-major.1 * ratio, major.0 * ratio);
    let mut e = end;
    while e <= start {
        e += std::f64::consts::TAU;
    }
    let span = e - start;
    let full = (span - std::f64::consts::TAU).abs() < 1e-6;
    let steps = ((span / std::f64::consts::TAU * 96.0).ceil() as usize).max(16);
    let count = if full { steps } else { steps + 1 };
    let pts = (0..count)
        .map(|i| {
            let t = start + span * i as f64 / steps as f64;
            Point {
                x: center.0 + major.0 * t.cos() + minor.0 * t.sin(),
                y: center.1 + major.1 * t.cos() + minor.1 * t.sin(),
            }
        })
        .collect();
    (pts, full)
}

// arm ใหม่ใน match ของ dxf_to_elements (ก่อน `_ => ...`)
//
// EntityType::Spline(s) => {
//     let ctrl: Vec<(f64, f64)> = s.control_points.iter().map(|v| (v.x, v.y)).collect();
//     let pts = spline_points(s.degree.max(0) as usize, &s.knots, &ctrl, &s.weights)
//         .or_else(|| {
//             let src: Vec<(f64, f64)> = if s.fit_points.len() >= 2 {
//                 s.fit_points.iter().map(|v| (v.x, v.y)).collect()
//             } else {
//                 ctrl.clone()
//             };
//             (src.len() >= 2).then(|| polyline_through(&src))
//         });
//     match pts {
//         Some(points) => out.push(ElementData::Polyline { points, closed: false }),
//         None => skipped += 1,
//     }
// }
// EntityType::Ellipse(el) => {
//     let (points, full) = ellipse_points(
//         (el.center.x, el.center.y),
//         (el.major_axis.x, el.major_axis.y),
//         el.minor_axis_ratio,
//         el.start_parameter,
//         el.end_parameter,
//     );
//     out.push(ElementData::Polyline { points, closed: full });
// }

/// bulge = tan(มุมโค้ง/4), บวก = ทวนเข็ม (CCW) จาก p1 ไป p2
fn bulge_segment(p1: Point, p2: Point, bulge: f64) -> ElementData {
    let (dx, dy) = (p2.x - p1.x, p2.y - p1.y);
    let c = dx.hypot(dy);
    if bulge.abs() < 1e-9 || c < 1e-12 {
        return ElementData::Line { start: p1, end: p2 };
    }
    let (lx, ly) = (-dy / c, dx / c); // เวกเตอร์ซ้ายของคอร์ด
    let k = (c / 2.0) * (1.0 - bulge * bulge) / (2.0 * bulge);
    let center = Point {
        x: (p1.x + p2.x) / 2.0 + lx * k,
        y: (p1.y + p2.y) / 2.0 + ly * k,
    };
    let radius = c * (1.0 + bulge * bulge) / (4.0 * bulge.abs());
    let a1 = (p1.y - center.y).atan2(p1.x - center.x);
    let a2 = (p2.y - center.y).atan2(p2.x - center.x);
    let (s, mut e) = if bulge > 0.0 { (a1, a2) } else { (a2, a1) };
    while e <= s {
        e += std::f64::consts::TAU;
    }
    ElementData::Arc { center, radius, start_angle: s, end_angle: e }
}

fn dxf_filters() -> (gtk::FileFilter, gio::ListStore, gtk::FileFilter) {
    let f = gtk::FileFilter::new();
    f.set_name(Some("DXF Drawing (*.dxf)"));
    f.add_pattern("*.dxf");
    f.add_pattern("*.DXF");
    let list = gio::ListStore::new::<gtk::FileFilter>();
    list.append(&f);
    (f.clone(), list, f)
}

type Rgb = (u8, u8, u8);

struct Imported {
    layers: Vec<crate::engine::Layer>,
    items: Vec<(ElementData, usize, Option<Rgb>)>,
    skipped: usize,
}

fn dxf_to_elements(doc: &acadrust::CadDocument) -> Imported {
    use crate::engine::Layer;
    use acadrust::EntityType;
    use std::collections::HashMap;
    let mut layers: Vec<Layer> = vec![Layer::new("0", (255, 255, 255))];
    let mut index: HashMap<String, usize> = HashMap::from([("0".to_string(), 0)]);
    for l in doc.layers.iter() {
        let color = l.color.rgb().unwrap_or((255, 255, 255));
        let (visible, locked) = (l.is_visible(), l.is_locked());
        match index.get(&l.name.to_lowercase()) {
            Some(&i) => {
                layers[i].color = color;
                layers[i].visible = visible;
                layers[i].locked = locked;
            }
            None => {
                index.insert(l.name.to_lowercase(), layers.len());
                layers.push(Layer { name: l.name.clone(), color, visible, locked });
            }
        }
    }

    let mut items = Vec::new();
    let mut skipped = 0usize;

    for e in doc.model_space_entities() {
        let c = e.common();
        let lname = if c.layer.is_empty() { "0" } else { c.layer.as_str() };
        let li = *index.entry(lname.to_lowercase()).or_insert_with(|| {
            layers.push(Layer::new(lname, (255, 255, 255)));
            layers.len() - 1
        });
        // ByLayer / ByBlock → None (ใช้สีของ layer), Index / Rgb → สีของวัตถุเอง
        let col = c.color.rgb();

        let mut out: Vec<ElementData> = Vec::new();
        match e {
            EntityType::Line(l) => out.push(ElementData::Line {
                start: Point { x: l.start.x, y: l.start.y },
                end: Point { x: l.end.x, y: l.end.y },
            }),
            EntityType::Circle(c) => out.push(ElementData::Circle {
                center: Point { x: c.center.x, y: c.center.y },
                radius: c.radius,
            }),
            EntityType::Arc(a) => {
                let s = a.start_angle;
                let mut en = a.end_angle;
                while en <= s {
                    en += std::f64::consts::TAU;
                }
                out.push(ElementData::Arc {
                    center: Point { x: a.center.x, y: a.center.y },
                    radius: a.radius,
                    start_angle: s,
                    end_angle: en,
                });
            }
            EntityType::LwPolyline(p) if p.vertices.len() >= 2 => {
                let pts: Vec<Point> = p.vertices.iter()
                    .map(|v| Point { x: v.location.x, y: v.location.y })
                    .collect();
                if p.vertices.iter().any(|v| v.bulge.abs() > 1e-9) {
                    let n = pts.len();
                    let segs = if p.is_closed { n } else { n - 1 };
                    for i in 0..segs {
                        out.push(bulge_segment(pts[i], pts[(i + 1) % n], p.vertices[i].bulge));
                    }
                } else {
                    out.push(ElementData::Polyline { points: pts, closed: p.is_closed });
                }
            }

            EntityType::Spline(s) => {
                let ctrl: Vec<(f64, f64)> = s.control_points.iter().map(|v| (v.x, v.y)).collect();
                let pts = spline_points(s.degree.max(0) as usize, &s.knots, &ctrl, &s.weights)
                    .or_else(|| {
                        let src: Vec<(f64, f64)> = if s.fit_points.len() >= 2 {
                            s.fit_points.iter().map(|v| (v.x, v.y)).collect()
                        } else {
                            ctrl.clone()
                        };
                        (src.len() >= 2).then(|| polyline_through(&src))
                    });
                match pts {
                    Some(points) => out.push(ElementData::Polyline { points, closed: false }),
                    None => skipped += 1,
                }
            }
            EntityType::Ellipse(el) => {
                let (points, full) = ellipse_points(
                    (el.center.x, el.center.y),
                    (el.major_axis.x, el.major_axis.y),
                    el.minor_axis_ratio,
                    el.start_parameter,
                    el.end_parameter,
                );
                out.push(ElementData::Polyline { points, closed: full });
            }
            _ => skipped += 1,
        }
        for data in out {
            items.push((data, li, col));
        }
    }
    Imported { layers, items, skipped }
}

fn aci_of(c: Rgb) -> u8 {
    acadrust::Color::Rgb { r: c.0, g: c.1, b: c.2 }
        .approximate_index()
        .clamp(1, 255) as u8
}

fn engine_to_dxf_bytes(data: &crate::engine::Engine) -> Result<Vec<u8>, String> {
    let mut drawing = dxf::Drawing::new();
    for l in &data.layers {
        let color = dxf::Color::from_index(aci_of(l.color));
        if let Some(ex) = drawing.layers_mut().find(|x| x.name.eq_ignore_ascii_case(&l.name)) {
            ex.color = color;
            ex.is_layer_on = l.visible;
        } else {
            drawing.add_layer(dxf::tables::Layer {
                name: l.name.clone(),
                color,
                is_layer_on: l.visible,
                ..Default::default()
            });
        }
    }

    for el in data.get_all_elements() {
        let layer_name = data.layers[el.layer].name.clone();
        let color = el.color.map(|c| dxf::Color::from_index(aci_of(c)));
        // ใส่ชื่อ layer + สี (ถ้ามี) ให้ทุก entity ที่เขียนออก
        let put = |drawing: &mut dxf::Drawing, et: dxf::entities::EntityType| {
            let mut e = dxf::entities::Entity::new(et);
            e.common.layer = layer_name.clone();
            if let Some(c) = &color {
                e.common.color = c.clone();
            }
            drawing.add_entity(e);
        };
        let put_line = |drawing: &mut dxf::Drawing, a: Point, b: Point| {
            put(
                drawing,
                dxf::entities::EntityType::Line(dxf::entities::Line {
                    p1: dxf::Point::new(a.x, a.y, 0.0),
                    p2: dxf::Point::new(b.x, b.y, 0.0),
                    ..Default::default()
                }),
            );
        };

        match &el.data {
            ElementData::Line { start, end } => put_line(&mut drawing, *start, *end),
            ElementData::Circle { center, radius } => put(
                &mut drawing,
                dxf::entities::EntityType::Circle(dxf::entities::Circle {
                    center: dxf::Point::new(center.x, center.y, 0.0),
                    radius: *radius,
                    ..Default::default()
                }),
            ),
            ElementData::Dimension { start, end, offset } => {
                if let Some(g) = linear_dimension_geometry(*start, *end, *offset) {
                    put_line(&mut drawing, g.start, g.dim_from);
                    put_line(&mut drawing, g.end, g.dim_to);
                    put_line(&mut drawing, g.dim_from, g.dim_to);
                }
            }
            ElementData::Arc { center, radius, start_angle, end_angle } => put(
                &mut drawing,
                dxf::entities::EntityType::Arc(dxf::entities::Arc {
                    center: dxf::Point::new(center.x, center.y, 0.0),
                    radius: *radius,
                    start_angle: start_angle.to_degrees().rem_euclid(360.0),
                    end_angle: end_angle.to_degrees().rem_euclid(360.0),
                    ..Default::default()
                }),
            ),
            ElementData::Polyline { points, closed } => put(
                &mut drawing,
                dxf::entities::EntityType::LwPolyline(dxf::entities::LwPolyline {
                    vertices: points
                        .iter()
                        .map(|p| dxf::LwPolylineVertex { x: p.x, y: p.y, ..Default::default() })
                        .collect(),
                    flags: if *closed { 1 } else { 0 },
                    ..Default::default()
                }),
            ),
        }
    }

    let mut buf = std::io::Cursor::new(Vec::new());
    drawing.save(&mut buf).map_err(|e| e.to_string())?;
    Ok(buf.into_inner())
}
