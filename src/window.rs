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
    DEFAULT_DIMENSION_OFFSET, ElementData, Point, RenderCommand, SnapKind,
    SnapSettings, ToolMode as DrawingMode, arc_from_three_points, linear_dimension_geometry,
};

use adw::subclass::prelude::*;
use gtk::gdk;
use gtk::prelude::*;
use adw::prelude::*;
use gtk::{gio, glib};
use std::cell::RefCell;
use crate::renderer::{CairoRenderer, Renderer};
use std::path::PathBuf;
use std::rc::Rc;
use crate::hatch::{HatchKind, HatchSpec};

glib::wrapper! {
    pub struct DraftingWindow(ObjectSubclass<imp::DraftingWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

const OFFSET_ANIM_SECS: f64 = 0.12;
const MIRROR_ANIM_SECS: f64 = 0.25;

#[derive(Debug, Clone)]
pub struct MirrorAnim {
    pub ids: Vec<u64>,
    pub a: (f64, f64),
    pub b: (f64, f64),
    pub keep: bool,
    pub boxed: bool, // เลือกเยอะ → เล่นแอนิเมชันเฉพาะกรอบ
    pub start_us: Option<i64>,
    pub t: f64, // 0.0 → 1.0
}
const OFFSET_FLIP_SECS: f64 = 0.04;

#[derive(Debug, Clone, Copy)]
pub struct OffsetAnim {
    pub source: u64,
    pub side: (f64, f64),
    pub dist: f64,
    pub start_us: Option<i64>,
    pub t: f64, // 0.0 → 1.0
}

mod layout_ui;

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
        pub btn_ortho: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub btn_arc: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_polyline: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_layout: TemplateChild<gtk::Button>,
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
        pub btn_mirror: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_hatch: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_text: TemplateChild<gtk::Button>,
        #[template_child]
        pub revealer_hatch: TemplateChild<gtk::Revealer>,
        #[template_child]
        pub dd_hatch_pattern: TemplateChild<gtk::DropDown>,
        #[template_child]
        pub spin_hatch_angle: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub spin_hatch_spacing: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub btn_layers: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub layer_list: TemplateChild<gtk::ListBox>,
        pub building_layers: Cell<bool>,
        pub space_down: Cell<bool>,
        pub space_pan: Cell<bool>,
        pub lw_drop: RefCell<Option<gtk::DropDown>>,
        pub lt_drop: RefCell<Option<gtk::DropDown>>,
        pub sel_box: RefCell<Option<gtk::Box>>,
        pub sel_label: RefCell<Option<gtk::Label>>,
        pub sel_lw_drop: RefCell<Option<gtk::DropDown>>,
        pub sel_lt_drop: RefCell<Option<gtk::DropDown>>,
        pub sel_key: Cell<(usize, u64)>,
        #[template_child]
        pub btn_layer_add: TemplateChild<gtk::Button>,
        #[template_child]
        pub btn_layer_move: TemplateChild<gtk::Button>,

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
        pub mirror_anim: RefCell<Option<MirrorAnim>>,
        pub ortho: Cell<bool>,
        pub cursor_size: Cell<f64>,
        /// เซฟสำรองอัตโนมัติ: id ไฟล์สำรองของแต่ละแท็บ + สถานะที่เซฟล่าสุด
        pub autosave_ids: RefCell<HashMap<adw::TabPage, String>>,
        pub autosave_keys: RefCell<HashMap<adw::TabPage, (usize, usize, u64, u64)>>,
        /// ตัวเลือกกล่องพิมพ์ล่าสุด (กระดาษ, แนว, พื้นที่, สเกล, สี)
        pub print_opts: Cell<(u32, u32, u32, u32, u32)>,
        /// ความสูงตัวอักษรที่ใช้ครั้งล่าสุด (0 = ยังไม่เคย)
        pub text_height: Cell<f64>,
        /// จุดจับ (snap แล้ว) ตอนเริ่มย้าย และระยะที่ย้ายไปแล้ว
        pub move_base: Cell<Option<(f64, f64)>>,
        pub move_applied: Cell<(f64, f64)>,
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
            self.cursor_size.set(5.0);
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

                    // กล่องโต้ตอบเปิดอยู่ (เช่น กล่องพิมพ์ข้อความ): ปล่อยคีย์ให้กล่องจัดการเอง
                    if obj.imp().is_dialog_open.get() {
                        return glib::Propagation::Proceed;
                    }

                    // Space ค้าง + ลากเมาส์ = เลื่อน canvas
                    if key == gdk::Key::space && !typing && !is_ctrl {
                        obj.imp().space_down.set(true);
                        return glib::Propagation::Stop;
                    }

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

                    // Undo: Ctrl+Z
                    if is_ctrl && (key == gdk::Key::z || key == gdk::Key::Z) {
                        obj.undo_action();
                        return glib::Propagation::Stop;
                    }

                    // Ortho: F8
                    if key == gdk::Key::F8 {
                        let o = &obj.imp().ortho;
                        o.set(!o.get());
                        if let Some(page) = obj.imp().tab_view.selected_page() {
                            page.child().queue_draw();
                        }
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

                    // แท็บ Layout: Delete / Esc จัดการเอง
                    if obj.layout_key(key) {
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
                            gdk::Key::h | gdk::Key::H => Some(DrawingMode::Hatch),
                            gdk::Key::x | gdk::Key::X => Some(DrawingMode::Text),
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
                    // Copy / Cut / Paste (Ctrl+Shift+V = วางที่พิกัดเดิม)
                    if is_ctrl && !typing {
                        if key == gdk::Key::c || key == gdk::Key::C {
                            obj.copy_action();
                            return glib::Propagation::Stop;
                        }
                        if key == gdk::Key::x || key == gdk::Key::X {
                            obj.cut_action();
                            return glib::Propagation::Stop;
                        }
                        if key == gdk::Key::v || key == gdk::Key::V {
                            obj.paste_action(is_shift);
                            return glib::Propagation::Stop;
                        }
                    }
                    if is_ctrl && (key == gdk::Key::o || key == gdk::Key::O) {
                        obj.show_open_dialog();
                        return glib::Propagation::Stop;
                    }
                    if is_ctrl && (key == gdk::Key::p || key == gdk::Key::P) {
                        obj.show_print_dialog();
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
                    if key == gdk::Key::space {
                        obj.imp().space_down.set(false);
                    }
                }
            ));

            obj.add_controller(key_ctrl);
            // สลับหน้าต่างตอนกด Space ค้าง จะไม่รับ key release → เคลียร์เอง
            obj.connect_is_active_notify(|w| {
                if !w.is_active() {
                    w.imp().space_down.set(false);
                }
            });

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

            // ปุ่ม Ortho (F8 / คำสั่ง ortho ไปเปลี่ยนค่า แล้ว sync_tool_buttons ตามให้ปุ่ม)
            self.btn_ortho.connect_toggled(glib::clone!(
                #[weak]
                obj,
                move |btn| {
                    obj.imp().ortho.set(btn.is_active());
                    if let Some(page) = obj.imp().tab_view.selected_page() {
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

            self.btn_layout.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| obj.layout_command()
            ));

            let layout_action = gtk::gio::SimpleAction::new("layout", None);
            layout_action.connect_activate(glib::clone!(
                #[weak]
                obj,
                move |_, _| obj.layout_command()
            ));
            obj.add_action(&layout_action);

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

            self.btn_mirror.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| obj.set_tool(DrawingMode::Mirror)
            ));

            self.btn_hatch.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| obj.set_tool(DrawingMode::Hatch)
            ));

            self.btn_text.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| obj.set_tool(DrawingMode::Text)
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
                        let handled = if first == "cursorsize" {
                            if let Some(v) = parts.next().and_then(|s| s.parse::<f64>().ok()) {
                                obj.imp().cursor_size.set(v.clamp(1.0, 100.0));
                            }
                            if let Some(page) = obj.imp().tab_view.selected_page() {
                                page.child().queue_draw();
                            }
                            true
                        } else if first == "of" || first == "offset" {
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
                            "ze" | "extents" => {
                                let pages = obj.imp().pages.borrow();
                                if let Some(d_ref) = pages.get(&page) {
                                    d_ref.borrow_mut().zoom_to_extents(w, h);
                                }
                            }
                            "s" | "select" => obj.imp().current_mode.set(DrawingMode::Select),
                            "l" | "line" => obj.imp().current_mode.set(DrawingMode::Line),
                            "c" | "circle" => obj.imp().current_mode.set(DrawingMode::Circle),
                            "ortho" | "or" => {
                                let o = &obj.imp().ortho;
                                o.set(!o.get());
                                page.child().queue_draw();
                            }
                            "u" | "undo" => obj.undo_action(),
                            "r" | "redo" => obj.redo_action(),
                            "save" => obj.save_action(),
                            "saveas" => obj.save_as_action(),
                            "o" | "open" => obj.show_open_dialog(),
                            "print" | "plot" => obj.show_print_dialog(),
                            "layout" | "lay" => obj.layout_command(),
                            "copy" => obj.copy_action(),
                            "cut" => obj.cut_action(),
                            "paste" => obj.paste_action(false),
                            "a" | "arc" => obj.set_tool(DrawingMode::Arc),
                            "pl" | "pline" | "polyline" => obj.set_tool(DrawingMode::Polyline),
                            "tr" | "trim" => obj.set_tool(DrawingMode::Trim),
                            "ex" | "extend" => obj.set_tool(DrawingMode::Extend),
                            "mi" | "mirror" => obj.set_tool(DrawingMode::Mirror),
                            "h" | "hatch" => obj.set_tool(DrawingMode::Hatch),
                            "tx" | "text" => obj.set_tool(DrawingMode::Text),
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
                move |_| {
                    obj.refresh_layers();
                    obj.sync_layout_chrome();
                }
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
                        obj.sync_line_style_ui();
                    }
                }
            ));

            self.btn_layer_add.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.with_engine(|d| {
                        let i = d.add_layer();
                        d.set_current_layer(i);
                    });
                    obj.refresh_layers();
                }
            ));

            self.btn_layer_move.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.with_engine(|d| d.move_selected_to_layer());
                    if let Some(page) = obj.imp().tab_view.selected_page() {
                        page.child().grab_focus();
                    }
                }
            ));

            obj.build_line_style_ui();
            obj.build_selection_style_ui();

            // สร้างหน้าแรกทันที
            obj.create_new_tab("Drawing 1");

            // เซฟสำรองทุก 2 นาที + ถามกู้คืนถ้าครั้งก่อนปิดผิดปกติ
            obj.start_autosave();
            glib::timeout_add_local_once(
                std::time::Duration::from_millis(600),
                glib::clone!(
                    #[weak]
                    obj,
                    move || obj.check_recovery()
                ),
            );

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
        let mut mode = mode;
        let mut hint = String::new();
        if mode == DrawingMode::Mirror {
            let any = imp
                .tab_view
                .selected_page()
                .and_then(|p| {
                    imp.pages
                        .borrow()
                        .get(&p)
                        .map(|d| d.borrow().history.iter().any(|e| e.is_selected))
                })
                .unwrap_or(false);
            if !any {
                mode = DrawingMode::Select;
                hint = "Select objects first, then Mirror (MI)".to_string();
            }
        }
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
        if !hint.is_empty() {
            *imp.offset_msg.borrow_mut() = hint;
            glib::timeout_add_local_once(
                std::time::Duration::from_millis(2500),
                glib::clone!(
                    #[weak(rename_to = obj)]
                    self,
                    move || {
                        obj.imp().offset_msg.borrow_mut().clear();
                        if let Some(page) = obj.imp().tab_view.selected_page() {
                            page.child().queue_draw();
                        }
                    }
                ),
            );
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
        if imp.btn_ortho.is_active() != imp.ortho.get() {
            imp.btn_ortho.set_active(imp.ortho.get());
        }
        let mode = imp.current_mode.get();
        if imp.synced_mode.get() == Some(mode) {
            return;
        }
        imp.synced_mode.set(Some(mode));
        imp.revealer_hatch.set_reveal_child(mode == DrawingMode::Hatch);

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

        let buttons: [(&gtk::Button, DrawingMode); 13] = [
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
            (&imp.btn_mirror, DrawingMode::Mirror),
            (&imp.btn_hatch, DrawingMode::Hatch),
            (&imp.btn_text, DrawingMode::Text),
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
            .cursor(&gdk::Cursor::from_name("none", None).unwrap())
            .build();

        let page = imp.tab_view.add_page(&canvas, None);
        page.set_title(title);

        let data = crate::engine::Engine::new();
        imp.pages
            .borrow_mut()
            .insert(page.clone(), RefCell::new(data));

        // --- 1. Zoom (Scroll) ---
        let scroll = gtk::EventControllerScroll::new(gtk::EventControllerScrollFlags::BOTH_AXES);
        scroll.connect_scroll(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[weak]
            page,
            #[upgrade_or]
            glib::Propagation::Stop,
            move |ctrl, dx, dy| {
                // ทัชแพด (หน่วยพิกเซล) = เลื่อน, ล้อเมาส์หรือ Ctrl+scroll = ซูม
                let touchpad = ctrl.unit() == gdk::ScrollUnit::Surface;
                let ctrl_down = ctrl
                    .current_event_state()
                    .contains(gdk::ModifierType::CONTROL_MASK);
                if !touchpad && dy == 0.0 {
                    return glib::Propagation::Proceed; // ล้อเอียงของเมาส์
                }
                let pages = obj.imp().pages.borrow();
                if let Some(d_ref) = pages.get(&page) {
                    let mut d = d_ref.borrow_mut();
                    let (mx, my) = obj.imp().mouse_pos.get();

                    if touchpad && !ctrl_down {
                        const PAN_GAIN: f64 = 1.5;
                        d.camera.target_offset.0 -= dx * PAN_GAIN;
                        d.camera.target_offset.1 -= dy * PAN_GAIN;
                        d.camera.offset = d.camera.target_offset;
                    } else if dy < 0.0 {
                        d.camera.zoom_in_at(mx, my);
                    } else {
                        d.camera.zoom_out_at(mx, my);
                    }
                }
                glib::Propagation::Stop
            }
        ));
        canvas.add_controller(scroll);

        // --- 1b. Pan: กดปุ่มกลาง (ล้อ) ค้างแล้วลาก ---
        let pan = gtk::GestureDrag::builder().button(2).build();
        let pan_origin: Rc<std::cell::Cell<(f64, f64)>> = Rc::new(std::cell::Cell::new((0.0, 0.0)));
        pan.connect_drag_begin(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[weak]
            page,
            #[strong]
            pan_origin,
            move |_, _, _| {
                let pages = obj.imp().pages.borrow();
                if let Some(d_ref) = pages.get(&page) {
                    pan_origin.set(d_ref.borrow().camera.target_offset);
                }
            }
        ));
        pan.connect_drag_update(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[weak]
            page,
            #[strong]
            pan_origin,
            move |_, dx, dy| {
                {
                    let pages = obj.imp().pages.borrow();
                    if let Some(d_ref) = pages.get(&page) {
                        let mut d = d_ref.borrow_mut();
                        let (ox, oy) = pan_origin.get();
                        let to = (ox + dx, oy + dy);
                        d.camera.target_offset = to;
                        d.camera.offset = to; // ตามเมาส์ทันที ไม่หน่วง
                    }
                }
                page.child().queue_draw();
            }
        ));
        canvas.add_controller(pan);

        // Space + ปุ่มซ้ายลาก
        let pan1 = gtk::GestureDrag::builder().button(1).build();
        let pan1_on: Rc<std::cell::Cell<bool>> = Rc::new(std::cell::Cell::new(false));
        pan1.connect_drag_begin(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[weak]
            page,
            #[strong]
            pan_origin,
            #[strong]
            pan1_on,
            move |_, _, _| {
                let on = obj.imp().space_down.get();
                pan1_on.set(on);
                if on {
                    let pages = obj.imp().pages.borrow();
                    if let Some(d_ref) = pages.get(&page) {
                        pan_origin.set(d_ref.borrow().camera.target_offset);
                    }
                }
            }
        ));
        pan1.connect_drag_update(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[weak]
            page,
            #[strong]
            pan_origin,
            #[strong]
            pan1_on,
            move |_, dx, dy| {
                if !pan1_on.get() {
                    return;
                }
                {
                    let pages = obj.imp().pages.borrow();
                    if let Some(d_ref) = pages.get(&page) {
                        let mut d = d_ref.borrow_mut();
                        let (ox, oy) = pan_origin.get();
                        let to = (ox + dx, oy + dy);
                        d.camera.target_offset = to;
                        d.camera.offset = to;
                    }
                }
                page.child().queue_draw();
            }
        ));
        canvas.add_controller(pan1);

        // ทัชแพด: pinch ซูมรอบจุดกึ่งกลางนิ้ว
        let pinch = gtk::GestureZoom::new();
        let pinch_base: Rc<std::cell::Cell<f64>> = Rc::new(std::cell::Cell::new(1.0));
        pinch.connect_begin(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[weak]
            page,
            #[strong]
            pinch_base,
            move |_, _| {
                let pages = obj.imp().pages.borrow();
                if let Some(d_ref) = pages.get(&page) {
                    pinch_base.set(d_ref.borrow().camera.target_scale);
                }
            }
        ));
        pinch.connect_scale_changed(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[weak]
            page,
            #[strong]
            pinch_base,
            move |g, scale| {
                let Some((sx, sy)) = g.bounding_box_center() else { return };
                let pages = obj.imp().pages.borrow();
                if let Some(d_ref) = pages.get(&page) {
                    let mut d = d_ref.borrow_mut();
                    let cam = &mut d.camera;
                    let wx = (sx - cam.target_offset.0) / cam.target_scale;
                    let wy = -(sy - cam.target_offset.1) / cam.target_scale;
                    cam.target_scale = (pinch_base.get() * scale).clamp(crate::engine::MIN_SCALE, crate::engine::MAX_SCALE);
                    cam.target_offset.0 = sx - wx * cam.target_scale;
                    cam.target_offset.1 = sy + wy * cam.target_scale;
                }
            }
        ));
        canvas.add_controller(pinch);

        // --- 2. Smooth Animation (Tick) ---
        canvas.add_tick_callback(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[weak]
            page,
            #[upgrade_or]
            glib::ControlFlow::Continue,
            move |area, clock| {
                // แท็บที่ซ่อนอยู่ไม่ต้องทำอะไร (กันแย่ง animation ของแท็บที่เปิด และประหยัด CPU)
                if obj.imp().tab_view.selected_page().as_ref() != Some(&page) {
                    return glib::ControlFlow::Continue;
                }
                obj.sync_tool_buttons();
                obj.sync_selection_ui();
                let pages = obj.imp().pages.borrow();
                if let Some(d_ref) = pages.get(&page) {
                    let mut d = d_ref.borrow_mut();

                    // เรียกใช้ฟังก์ชัน step ของ Engine
                    if d.step(clock.frame_time() as u64) {
                        area.queue_draw();
                    }

                    // Mirror: เดินเวลา แล้วสะท้อนจริงตอนจบ
                    let mut mirror_done: Option<MirrorAnim> = None;
                    {
                        let mut slot = obj.imp().mirror_anim.borrow_mut();
                        let is_current =
                            obj.imp().tab_view.selected_page().as_ref() == Some(&page);
                        if let Some(m) = slot.as_mut().filter(|_| is_current) {
                            let now = clock.frame_time();
                            let start = *m.start_us.get_or_insert(now);
                            m.t = ((now - start) as f64 / 1_000_000.0 / MIRROR_ANIM_SECS).min(1.0);
                            let finished = m.t >= 1.0;
                            if finished {
                                mirror_done = slot.take();
                            }
                            area.queue_draw();
                        }
                    }
                    if let Some(m) = mirror_done {
                        d.mirror_ids(
                            &m.ids,
                            Point { x: m.a.0, y: m.a.1 },
                            Point { x: m.b.0, y: m.b.1 },
                            m.keep,
                        );
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
                            d.finish_move();
                            obj.imp().move_base.set(None);
                            obj.imp().current_mode.set(DrawingMode::Select);
                            obj.imp().drag_start.set(None);
                        } else if let Some((bx, by)) = obj.imp().move_base.get() {
                            // ย้ายตามจุด snap (object snap → กริด) ลบจุดจับ
                            let (wx, wy) = d.camera.screen_to_world(x, y);
                            let (tx, ty, _) = d.get_input_snap_ex(
                                wx,
                                wy,
                                obj.imp().snap_enabled.get(),
                                &obj.imp().snap_settings.get(),
                                true,
                            );
                            let want = (tx - bx, ty - by);
                            let (ax, ay) = obj.imp().move_applied.get();
                            if (want.0 - ax).abs() > 1e-12 || (want.1 - ay).abs() > 1e-12 {
                                d.move_selected(want.0 - ax, want.1 - ay);
                                obj.imp().move_applied.set(want);
                            }
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
        let click = gtk::GestureClick::builder().button(1).build();
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
                if imp.space_down.get() {
                    imp.space_pan.set(true); // ให้ตอนปล่อยปุ่มรู้ว่าเป็นการเลื่อน
                    return;
                }
                let pages = imp.pages.borrow();

                if imp.current_mode.get() == DrawingMode::Move {
                    // snap ตำแหน่งสุดท้ายแล้วออกจาก Move mode
                    let pages = imp.pages.borrow();
                    if let Some(d_ref) = pages.get(&page) {
                        let mut d = d_ref.borrow_mut();
                        d.dirty = true;
                        d.finish_move();
                        imp.move_base.set(None);
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
                                let (bx, by, _) = d.get_input_snap_ex(
                                    wx,
                                    wy,
                                    imp.snap_enabled.get(),
                                    &imp.snap_settings.get(),
                                    false,
                                );
                                imp.move_base.set(Some((bx, by)));
                                imp.move_applied.set((0.0, 0.0));
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
                            let (sx, sy, sk) =
                                d.get_input_snap(wx, wy, imp.snap_enabled.get(), &imp.snap_settings.get());
                            let (sx, sy) = if sk.is_none() { obj.ortho_apply((sx, sy)) } else { (sx, sy) };
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
                            let (sx, sy, sk) = d.get_input_snap(
                                wx,
                                wy,
                                imp.snap_enabled.get(),
                                &imp.snap_settings.get(),
                            );
                            let (sx, sy) = if sk.is_none() { obj.ortho_apply((sx, sy)) } else { (sx, sy) };
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

                        DrawingMode::Mirror => {
                            let (sx, sy, sk) = d.get_input_snap(
                                wx,
                                wy,
                                imp.snap_enabled.get(),
                                &imp.snap_settings.get(),
                            );
                            let (sx, sy) =
                                if sk.is_none() { obj.ortho_apply((sx, sy)) } else { (sx, sy) };
                            match imp.start_pos.get() {
                                None => imp.start_pos.set(Some((sx, sy))),
                                Some((ax, ay)) => {
                                    if (sx - ax).hypot(sy - ay) > 1e-9 {
                                        let keep = !(gesture
                                            .current_event_state()
                                            .contains(gdk::ModifierType::SHIFT_MASK)
                                            || imp.shift_down.get());
                                        let ids: Vec<u64> = d
                                            .history
                                            .iter()
                                            .filter(|e| e.is_selected)
                                            .map(|e| e.id)
                                            .collect();
                                        if imp.mirror_anim.borrow().is_some() {
                                            d.mirror_selected(
                                                Point { x: ax, y: ay },
                                                Point { x: sx, y: sy },
                                                keep,
                                            );
                                        } else if !ids.is_empty() {
                                            *imp.mirror_anim.borrow_mut() = Some(MirrorAnim {
                                                boxed: ids.len() > 3000,
                                                ids,
                                                a: (ax, ay),
                                                b: (sx, sy),
                                                keep,
                                                start_us: None,
                                                t: 0.0,
                                            });
                                        }
                                        imp.start_pos.set(None);
                                        imp.current_mode.set(DrawingMode::Select);
                                    }
                                }
                            }
                        }

                        DrawingMode::Hatch => {
                            let (spec, solid) = obj.hatch_ui_spec();
                            match d.hatch_at(wx, wy, spec, solid) {
                                Ok(_) => imp.offset_msg.borrow_mut().clear(),
                                Err(m) => *imp.offset_msg.borrow_mut() = m.into(),
                            }
                        }

                        DrawingMode::Text => {
                            let (sx, sy, _) = d.get_input_snap(
                                wx,
                                wy,
                                imp.snap_enabled.get(),
                                &imp.snap_settings.get(),
                            );
                            let at = Point { x: sx, y: sy };
                            let scale = d.camera.scale;
                            // เปิดกล่องหลังจบการจัดการคลิกนี้ (ตอนนี้ engine ยังถูกยืมอยู่)
                            glib::idle_add_local_once(glib::clone!(
                                #[weak]
                                obj,
                                move || obj.prompt_new_text(at, scale)
                            ));
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
                if imp.space_pan.replace(false) {
                    return;
                }

                if imp.current_mode.get() == DrawingMode::Move {
                    let pages = imp.pages.borrow();
                    if let Some(d_ref) = pages.get(&page) {
                        let mut d = d_ref.borrow_mut();
                        d.dirty = true;
                        d.finish_move();
                        imp.move_base.set(None);
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
                        let (ex, ey, ek) = d.get_input_snap(
                            wx,
                            wy,
                            imp.snap_enabled.get(),
                            &imp.snap_settings.get(),
                        );
                        let (ex, ey) = if ek.is_none() { obj.ortho_apply((ex, ey)) } else { (ex, ey) };

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

        // --- 4b. ดับเบิลคลิก: แก้ Text / Hatch ---
        let dbl = gtk::GestureClick::builder().button(1).build();
        dbl.connect_pressed(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[weak]
            page,
            move |_, n_press, x, y| {
                if n_press != 2 {
                    return;
                }
                // ให้ตัวจัดการคลิกหลักทำงานจบก่อน (มันอาจเริ่มโหมดย้ายไปแล้ว)
                glib::idle_add_local_once(glib::clone!(
                    #[weak]
                    obj,
                    #[weak]
                    page,
                    move || obj.edit_at(&page, x, y)
                ));
            }
        ));
        canvas.add_controller(dbl);

        // --- 5. Draw Function ---
        let state: std::rc::Rc<RefCell<SceneState>> =
            std::rc::Rc::new(RefCell::new(SceneState::default()));
        canvas.set_draw_func(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[weak]
            page,
            move |_, cr, w, h| {
                obj.draw_page(&page, cr, w as f64, h as f64, &state);
            }
        ));

        imp.tab_view.set_selected_page(&page);
        canvas.grab_focus();
    }

    fn draw_page(
        &self,
        page: &adw::TabPage,
        cr: &gtk::cairo::Context,
        w: f64,
        h: f64,
        state: &std::rc::Rc<RefCell<SceneState>>,
    ) {
        let imp = self.imp();
        let pages = imp.pages.borrow();
        let d_ref = match pages.get(page) {
            Some(r) => r,
            None => return,
        };

        {
            let mut dm = d_ref.borrow_mut();
            if cam_moving(&dm.camera) {
                let (ts, toff) = (dm.camera.target_scale, dm.camera.target_offset);
                let want_key: SceneKey =
                    (w as i32, h as i32, ts.to_bits(), toff.0.to_bits(), toff.1.to_bits());
                let go = {
                    let mut st = state.borrow_mut();
                    let differs = st.cache.as_ref().map_or(true, |c| c.key != want_key);
                    let since = match st.want {
                        Some((k, t)) if k == want_key => t,
                        _ => {
                            let now = std::time::Instant::now();
                            st.want = Some((want_key, now));
                            now
                        }
                    };
                    differs
                        && !st.pending
                        && since.elapsed() >= std::time::Duration::from_millis(80)
                };
                if go {
                    let (mx, my) = scene_margin(w, h);
                    let s2w = |x: f64, y: f64| ((x - toff.0) / ts, -(y - toff.1) / ts);
                    let commands = dm.render_commands_at_target(-mx, -my, w + mx, h + my);
                    let rev = dm.scene_rev();
                    let (sx, sy) = cr.target().device_scale();
                    spawn_scene_build(
                        state,
                        page.child(),
                        SceneJob {
                            w,
                            h,
                            sx,
                            sy,
                            key: want_key,
                            rev,
                            tl: s2w(-mx, -my),
                            br: s2w(w + mx, h + my),
                            commands,
                        },
                    );
                }
            }
        }

        let d = d_ref.borrow();

        let cam = &d.camera;
        let scale = cam.scale;
        let (_ox, _oy) = cam.offset;
        let (rx, ry) = imp.mouse_pos.get();

        // --- Snap position ---
        let (wx, wy) = cam.screen_to_world(rx, ry);
        let (sn_x, sn_y, snap_kind) = d.get_input_snap_ex(
            wx,
            wy,
            imp.snap_enabled.get(),
            &imp.snap_settings.get(),
            imp.current_mode.get() == DrawingMode::Move,
        );

        let (sn_x, sn_y) = if snap_kind.is_none() {
            self.ortho_apply((sn_x, sn_y))
        } else {
            (sn_x, sn_y)
        };

        paint_grid(cr, cam, w, h);

        let (mx, my) = scene_margin(w, h);
        let (ew, eh) = (w + 2.0 * mx, h + 2.0 * my);
        let cur_key: SceneKey = (
            w as i32,
            h as i32,
            cam.scale.to_bits(),
            cam.offset.0.to_bits(),
            cam.offset.1.to_bits(),
        );

        if !cam_moving(cam) {
            let rev = d.scene_rev();
            let fresh = state
                .borrow()
                .cache
                .as_ref()
                .is_some_and(|c| c.key == cur_key && c.rev == rev);
            if !fresh {
                let cmds = d.get_render_commands_in(-mx, -my, w + mx, h + my);
                let (sx, sy) = cr.target().device_scale();
                let job = SceneJob {
                    w,
                    h,
                    sx,
                    sy,
                    key: cur_key,
                    rev,
                    tl: cam.screen_to_world(-mx, -my),
                    br: cam.screen_to_world(w + mx, h + my),
                    commands: cmds,
                };
                let (cheap, pending) = {
                    let st = state.borrow();
                    (st.last_cost < std::time::Duration::from_millis(8), st.pending)
                };
                if cheap {
                    let t0 = std::time::Instant::now();
                    if let Some(done) = render_scene_job(job) {
                        let mut st = state.borrow_mut();
                        install_scene(&mut st, done);
                        st.last_cost = t0.elapsed();
                    }
                } else if !pending {
                    spawn_scene_build(state, page.child(), job);
                }
            }
        }

        if let Some(c) = state.borrow().cache.as_ref() {
            if c.key.0 == w as i32 && c.key.1 == h as i32 {
                if c.key == cur_key {
                    let _ = cr.set_source_surface(&c.surface, -mx, -my);
                    let _ = cr.paint();
                } else {
                    let (p0x, p0y) = cam.world_to_screen(c.world_tl.0, c.world_tl.1);
                    let (p1x, p1y) = cam.world_to_screen(c.world_br.0, c.world_br.1);
                    cr.save().ok();
                    cr.translate(p0x, p0y);
                    cr.scale((p1x - p0x) / ew, (p1y - p0y) / eh);
                    cr.set_source_surface(&c.surface, 0.0, 0.0).ok();
                    cr.paint().ok();
                    cr.restore().ok();
                }
            }
        }
        cr.set_source_rgb(1.0, 1.0, 1.0);
        let lw = if scale < 0.2 { 1.0 } else { 2.0 };
        cr.set_line_width(lw);

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
                DrawingMode::Hatch | DrawingMode::Text => {}
                DrawingMode::Mirror => {
                    let (psx, psy) = cam.world_to_screen(sx, sy);
                    let (pex, pey) = cam.world_to_screen(sn_x, sn_y);
                    cr.move_to(psx, psy);
                    cr.line_to(pex, pey);
                }

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

                    RenderCommand::Fill { loops, .. } => {
                        cr.set_source_rgba(1.0, 0.5, 0.0, 0.4);
                        cr.set_line_width(1.5);
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
                        let _ = cr.stroke();
                    }

                    RenderCommand::Dimension { .. } | RenderCommand::Style { .. } => {} // placeholder
                    RenderCommand::Text { x, y, em_px, rotation, wrap_px, text, halign, valign, .. } => {
                        cr.set_source_rgba(1.0, 0.5, 0.0, 0.4);
                        crate::renderer::draw_text(cr, x, y, em_px, rotation, wrap_px, &text, halign, valign);
                    }
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

        // --- Mirror flip animation ---
        if let Some(m) = imp.mirror_anim.borrow().as_ref() {
            let ease = m.t * m.t * (3.0 - 2.0 * m.t);
            let mut s = (std::f64::consts::PI * ease).cos(); // +1 → -1
            if s.abs() < 1e-3 {
                s = 1e-3; // เมทริกซ์ห้ามเป็นศูนย์ ไม่งั้น Cairo เข้าสถานะ error
            }
            let (ax, ay) = cam.world_to_screen(m.a.0, m.a.1);
            let (bx, by) = cam.world_to_screen(m.b.0, m.b.1);
            let psi = (by - ay).atan2(bx - ax);
            cr.save().ok();
            cr.new_path();
            cr.translate(ax, ay);
            cr.rotate(psi);
            cr.scale(1.0, s);
            cr.rotate(-psi);
            cr.translate(-ax, -ay);
            if m.boxed {
                if let Some((p0, p1)) = d.selection_bbox() {
                    for (i, (x, y)) in [(p0.x, p0.y), (p1.x, p0.y), (p1.x, p1.y), (p0.x, p1.y)]
                        .iter()
                        .enumerate()
                    {
                        let (sx, sy) = cam.world_to_screen(*x, *y);
                        if i == 0 { cr.move_to(sx, sy); } else { cr.line_to(sx, sy); }
                    }
                    cr.close_path();
                }
            } else {
                for data in d.data_by_ids(&m.ids) {
                    trace_ghost(cr, cam, data);
                }
            }
            cr.restore().ok(); // path อยู่ต่อ แต่เส้นเขียนด้วยเมทริกซ์ปกติ ความหนาไม่เพี้ยน
            cr.set_source_rgba(1.0, 0.5, 0.0, 0.9);
            cr.set_line_width(1.5);
            let _ = cr.stroke();
        }

        // --- Mirror ghost ---
        if imp.current_mode.get() == DrawingMode::Mirror {
            if let Some((ax, ay)) = imp.start_pos.get() {
                let ghosts = d.mirror_preview(
                    Point { x: ax, y: ay },
                    Point { x: sn_x, y: sn_y },
                    3000,
                );
                cr.new_path();
                cr.set_source_rgba(1.0, 0.5, 0.0, 0.7);
                cr.set_line_width(1.5);
                for g in &ghosts {
                    trace_ghost(cr, cam, g);
                }
                // เลือกเยอะจนไม่มี ghost → แสดงกรอบที่สะท้อนแล้วแทน
                if ghosts.is_empty() {
                    if let Some((p0, p1)) = d.selection_bbox() {
                        let (la, lb) = (Point { x: ax, y: ay }, Point { x: sn_x, y: sn_y });
                        for (i, (x, y)) in [(p0.x, p0.y), (p1.x, p0.y), (p1.x, p1.y), (p0.x, p1.y)]
                            .iter()
                            .enumerate()
                        {
                            let r = crate::engine::reflect_point(Point { x: *x, y: *y }, la, lb);
                            let (sx, sy) = cam.world_to_screen(r.x, r.y);
                            if i == 0 { cr.move_to(sx, sy); } else { cr.line_to(sx, sy); }
                        }
                        cr.close_path();
                    }
                }
                let _ = cr.stroke();
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

        // Crosshair: เส้นสั้น (CURSORSIZE %) + pickbox ในโหมดที่เลือกวัตถุ
        {
            let (cx, cy) = cam.world_to_screen(sn_x, sn_y);
            let (cx, cy) = (cx.round() + 0.5, cy.round() + 0.5);
            let pct = imp.cursor_size.get();
            let half = if pct >= 100.0 { w.max(h) } else { w.min(h) * pct / 100.0 };
            cr.set_source_rgb(1.0, 0.0, 0.0);
            cr.set_line_width(1.0);
            cr.move_to((cx - half).max(0.0), cy);
            cr.line_to((cx + half).min(w), cy);
            cr.move_to(cx, (cy - half).max(0.0));
            cr.line_to(cx, (cy + half).min(h));
            let _ = cr.stroke();

            if matches!(
                imp.current_mode.get(),
                DrawingMode::Select | DrawingMode::Move | DrawingMode::Trim | DrawingMode::Extend
            ) {
                const PICK: f64 = 5.0; // ครึ่งหนึ่งของด้าน (px)
                cr.rectangle(cx - PICK, cy - PICK, 2.0 * PICK, 2.0 * PICK);
                let _ = cr.stroke();
            }
        }

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
            DrawingMode::Mirror => "MIRROR",
            DrawingMode::Hatch => "HATCH",
            DrawingMode::Text => "TEXT",
        };

        let offset_hint = if imp.current_mode.get() == DrawingMode::Hatch {
            let msg = imp.offset_msg.borrow();
            if msg.is_empty() {
                "  click inside a closed area".to_string()
            } else {
                format!("  click inside a closed area | {}", msg)
            }
        } else if imp.current_mode.get() == DrawingMode::Text {
            "  click where the text goes".to_string()
        } else if imp.current_mode.get() == DrawingMode::Mirror {
            if imp.start_pos.get().is_none() {
                "  click first point of mirror line".to_string()
            } else {
                "  click second point | Shift: delete originals".to_string()
            }
        } else if imp.current_mode.get() == DrawingMode::Offset {
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
        } else if !imp.offset_msg.borrow().is_empty() {
            format!("  {}", imp.offset_msg.borrow())
        } else {
            String::new()
        };

        let txt = format!(
            "MODE: [{}]  LAYER: {}  X: {:.2} Y: {:.2} {}{}{}",
            mode_text,
            d.layers[d.current_layer].name,
            sn_x,
            sn_y,
            format!(
                "{}{}",
                if imp.snap_enabled.get() { "[SNAP]" } else { "" },
                if imp.ortho.get() { "[ORTHO]" } else { "" }
            ),
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

    /// จุดอ้างอิงของ Ortho: จุดก่อนหน้าของเส้น/polyline ที่กำลังวาด
    fn ortho_base(&self) -> Option<(f64, f64)> {
        let imp = self.imp();
        if !imp.ortho.get() {
            return None;
        }
        match imp.current_mode.get() {
            DrawingMode::Line => imp.start_pos.get(),
            DrawingMode::Mirror => imp.start_pos.get(),
            DrawingMode::Dimension if imp.dimension_step.get() == 1 => imp.start_pos.get(),
            DrawingMode::Polyline => imp.pending_points.borrow().last().map(|p| (p.x, p.y)),
            _ => None,
        }
    }

    /// ล็อกจุดให้อยู่แนวนอนหรือแนวตั้งจากจุดอ้างอิง (ตามแกนที่ห่างมากกว่า)
    fn ortho_apply(&self, p: (f64, f64)) -> (f64, f64) {
        match self.ortho_base() {
            Some((bx, by)) => {
                if (p.0 - bx).abs() >= (p.1 - by).abs() {
                    (p.0, by)
                } else {
                    (bx, p.1)
                }
            }
            None => p,
        }
    }

    /// โฟกัสอยู่ในช่องที่รับข้อความ (ช่องคำสั่ง, spin ของ Array) หรือไม่
    fn is_typing(&self) -> bool {
        gtk::prelude::GtkWindowExt::focus(self)
            .map_or(false, |w| w.is::<gtk::Editable>() || w.is::<gtk::TextView>())
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

    // ---------------- Hatch / Text ----------------

    /// ค่าจากแผง Hatch: (รูปแบบ, solid?)  ตัวเลือก 0 = Lines, 1 = Cross, 2 = Solid
    fn hatch_ui_spec(&self) -> (HatchSpec, bool) {
        let imp = self.imp();
        imp.spin_hatch_angle.update();
        imp.spin_hatch_spacing.update();
        let sel = imp.dd_hatch_pattern.selected();
        let kind = if sel == 1 { HatchKind::Cross } else { HatchKind::Lines };
        let spec = HatchSpec {
            kind,
            angle: imp.spin_hatch_angle.value().to_radians(),
            spacing: imp.spin_hatch_spacing.value(),
        };
        (spec, sel == 2)
    }

    /// กล่องพิมพ์ข้อความ (หลายบรรทัด) + ความสูง + มุมหมุน (องศา); init = (ข้อความ, สูง, มุม°)
    fn show_text_dialog(
        &self,
        heading: &str,
        init: (String, f64, f64),
        on_ok: impl Fn(String, f64, f64) + 'static,
    ) {
        let imp = self.imp();
        let dialog = adw::AlertDialog::new(Some(heading), None);
        dialog.add_responses(&[("cancel", "_Cancel"), ("ok", "_OK")]);
        dialog.set_response_appearance("ok", adw::ResponseAppearance::Suggested);
        dialog.set_close_response("cancel");

        let tv = gtk::TextView::builder()
            .wrap_mode(gtk::WrapMode::WordChar)
            .accepts_tab(false)
            .left_margin(8)
            .right_margin(8)
            .top_margin(6)
            .bottom_margin(6)
            .build();
        tv.buffer().set_text(&init.0);
        let sw = gtk::ScrolledWindow::builder()
            .min_content_height(110)
            .min_content_width(380)
            .has_frame(true)
            .child(&tv)
            .build();

        let height = gtk::SpinButton::with_range(0.001, 100000.0, 1.0);
        height.set_digits(3);
        height.set_value(init.1);
        let rot = gtk::SpinButton::with_range(-360.0, 360.0, 5.0);
        rot.set_digits(1);
        rot.set_value(init.2);

        let lbl = |s: &str| gtk::Label::builder().label(s).xalign(0.0).build();
        let row = gtk::Box::builder().spacing(10).build();
        row.append(&lbl("Height"));
        row.append(&height);
        row.append(&lbl("Rotation (°)"));
        row.append(&rot);
        let bx = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(10)
            .build();
        bx.append(&sw);
        bx.append(&row);
        dialog.set_extra_child(Some(&bx));

        imp.is_dialog_open.set(true);
        dialog.connect_closed(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            move |_| {
                obj.imp().is_dialog_open.set(false);
                if let Some(page) = obj.imp().tab_view.selected_page() {
                    page.child().grab_focus();
                }
            }
        ));
        dialog.connect_response(
            None,
            glib::clone!(
                #[strong]
                tv,
                #[strong]
                height,
                #[strong]
                rot,
                move |_, response| {
                    if response != "ok" {
                        return;
                    }
                    height.update();
                    rot.update();
                    let buf = tv.buffer();
                    let text = buf.text(&buf.start_iter(), &buf.end_iter(), false).to_string();
                    let text = text.trim_end().to_string();
                    if text.trim().is_empty() {
                        return;
                    }
                    on_ok(text, height.value(), rot.value());
                }
            ),
        );
        dialog.present(Some(self));
        glib::idle_add_local_once(move || {
            tv.grab_focus();
        });
    }

    /// กล่องแก้ hatch ที่มีอยู่: รูปแบบ มุม ระยะห่าง
    fn show_hatch_dialog(&self, init: HatchSpec, on_ok: impl Fn(HatchSpec) + 'static) {
        let imp = self.imp();
        let dialog = adw::AlertDialog::new(Some("Edit hatch"), None);
        dialog.add_responses(&[("cancel", "_Cancel"), ("ok", "_OK")]);
        dialog.set_response_appearance("ok", adw::ResponseAppearance::Suggested);
        dialog.set_default_response(Some("ok"));
        dialog.set_close_response("cancel");

        let dd = gtk::DropDown::new(
            Some(gtk::StringList::new(&["Lines", "Cross"])),
            gtk::Expression::NONE,
        );
        dd.set_selected(if init.kind == HatchKind::Cross { 1 } else { 0 });
        let mut deg = init.angle.to_degrees() % 360.0;
        if deg > 180.0 {
            deg -= 360.0;
        }
        if deg <= -180.0 {
            deg += 360.0;
        }
        let ang = gtk::SpinButton::with_range(-360.0, 360.0, 5.0);
        ang.set_digits(1);
        ang.set_value(deg);
        let sp = gtk::SpinButton::with_range(0.001, 100000.0, 1.0);
        sp.set_digits(3);
        sp.set_value(init.spacing);

        let lbl = |s: &str| gtk::Label::builder().label(s).xalign(0.0).build();
        let grid = gtk::Grid::builder()
            .column_spacing(12)
            .row_spacing(8)
            .halign(gtk::Align::Center)
            .build();
        grid.attach(&lbl("Pattern"), 0, 0, 1, 1);
        grid.attach(&dd, 1, 0, 1, 1);
        grid.attach(&lbl("Angle (°)"), 0, 1, 1, 1);
        grid.attach(&ang, 1, 1, 1, 1);
        grid.attach(&lbl("Spacing"), 0, 2, 1, 1);
        grid.attach(&sp, 1, 2, 1, 1);
        dialog.set_extra_child(Some(&grid));

        imp.is_dialog_open.set(true);
        dialog.connect_closed(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            move |_| {
                obj.imp().is_dialog_open.set(false);
                if let Some(page) = obj.imp().tab_view.selected_page() {
                    page.child().grab_focus();
                }
            }
        ));
        dialog.connect_response(
            None,
            glib::clone!(
                #[strong]
                dd,
                #[strong]
                ang,
                #[strong]
                sp,
                move |_, response| {
                    if response != "ok" {
                        return;
                    }
                    ang.update();
                    sp.update();
                    on_ok(HatchSpec {
                        kind: if dd.selected() == 1 { HatchKind::Cross } else { HatchKind::Lines },
                        angle: ang.value().to_radians(),
                        spacing: sp.value(),
                    });
                }
            ),
        );
        dialog.present(Some(self));
    }

    /// คลิกวางข้อความ: เปิดกล่องพิมพ์ แล้วสร้างข้อความที่จุดนั้น
    fn prompt_new_text(&self, at: Point, scale: f64) {
        let imp = self.imp();
        let h0 = if imp.text_height.get() > 0.0 {
            imp.text_height.get()
        } else {
            nice_number(16.0 / scale) // ราว 16 px บนจอตอนนี้
        };
        self.show_text_dialog(
            "Add text",
            (String::new(), h0, 0.0),
            glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |text: String, h: f64, rot: f64| {
                    obj.imp().text_height.set(h);
                    obj.with_engine(|d| d.add_text(at, text, h, rot.to_radians()));
                    obj.cancel_action(false);
                }
            ),
        );
    }

    /// ดับเบิลคลิกที่ Text / Hatch → เปิดกล่องแก้ไข
    fn edit_at(&self, page: &adw::TabPage, x: f64, y: f64) {
        let imp = self.imp();
        let mode = imp.current_mode.get();
        if !matches!(mode, DrawingMode::Select | DrawingMode::Move) || imp.is_dialog_open.get() {
            return;
        }
        let target = {
            let pages = imp.pages.borrow();
            let Some(d_ref) = pages.get(page) else { return };
            let mut d = d_ref.borrow_mut();
            let (wx, wy) = d.camera.screen_to_world(x, y);
            let t = d.edit_target(wx, wy);
            // คลิกที่ 2 เริ่มการย้ายไปแล้ว (checkpoint ไว้) แต่ยังไม่ขยับ → ยกเลิกการย้ายนั้น
            if t.is_some() && mode == DrawingMode::Move && imp.move_applied.get() == (0.0, 0.0) {
                d.discard_last_checkpoint();
            }
            t
        };
        let Some((id, data)) = target else { return };
        imp.current_mode.set(DrawingMode::Select);
        imp.move_base.set(None);
        imp.drag_start.set(None);
        imp.is_dragging.set(false);
        match data {
            ElementData::Text { text, height, rotation, .. } => {
                self.show_text_dialog(
                    "Edit text",
                    (text, height, rotation.to_degrees()),
                    glib::clone!(
                        #[weak(rename_to = obj)]
                        self,
                        move |t: String, h: f64, r: f64| {
                            obj.imp().text_height.set(h);
                            obj.with_engine(|d| d.set_text(id, t, h, r.to_radians()));
                        }
                    ),
                );
            }
            ElementData::Hatch { spec, .. } => {
                self.show_hatch_dialog(
                    spec,
                    glib::clone!(
                        #[weak(rename_to = obj)]
                        self,
                        move |s: HatchSpec| {
                            obj.with_engine(|d| d.set_hatch_spec(id, s));
                        }
                    ),
                );
            }
            _ => {}
        }
    }

    // ---------------- Autosave / Recovery ----------------

    /// เซฟสำรองแท็บที่มีงานค้าง (เขียนไฟล์แยก ไม่แตะไฟล์จริง) เรียกทุก AUTOSAVE_SECS วินาที
    fn autosave_tick(&self) {
        let imp = self.imp();
        let dir = autosave_dir();
        let pages: Vec<adw::TabPage> = imp.pages.borrow().keys().cloned().collect();
        for page in pages {
            let job = {
                let pages = imp.pages.borrow();
                let Some(d_ref) = pages.get(&page) else { continue };
                let d = d_ref.borrow();
                if !d.dirty {
                    None
                } else {
                    let key = (
                        d.history.len(),
                        d.undo_stack.len(),
                        d.id_counter,
                        d.sel_rev.wrapping_add(d.layout_rev.wrapping_mul(1_000_003)),
                    );
                    if imp.autosave_keys.borrow().get(&page) == Some(&key) {
                        None // ไม่มีอะไรเปลี่ยนตั้งแต่รอบที่แล้ว
                    } else {
                        match engine_to_dxf_bytes(&d) {
                            Ok(b) => Some((b, key, d.file_path.clone())),
                            Err(e) => {
                                eprintln!("Autosave failed: {}", e);
                                None
                            }
                        }
                    }
                }
            };
            let Some((bytes, key, orig)) = job else { continue };
            let id = {
                let mut ids = imp.autosave_ids.borrow_mut();
                let n = ids.len();
                ids.entry(page.clone()).or_insert_with(|| new_autosave_id(n)).clone()
            };
            let meta = format!(
                "{}\n{}\n",
                page.title(),
                orig.map(|p| p.to_string_lossy().to_string()).unwrap_or_default()
            );
            let res = std::fs::create_dir_all(&dir)
                .and_then(|_| write_atomic(&dir.join(format!("{id}.meta")), meta.as_bytes()))
                .and_then(|_| write_atomic(&dir.join(format!("{id}.dxf")), &bytes));
            match res {
                Ok(()) => {
                    imp.autosave_keys.borrow_mut().insert(page, key);
                }
                Err(e) => eprintln!("Autosave failed: {}", e),
            }
        }
    }

    fn start_autosave(&self) {
        glib::timeout_add_seconds_local(
            AUTOSAVE_SECS,
            glib::clone!(
                #[weak(rename_to = obj)]
                self,
                #[upgrade_or]
                glib::ControlFlow::Break,
                move || {
                    obj.autosave_tick();
                    glib::ControlFlow::Continue
                }
            ),
        );
    }

    /// ลบไฟล์สำรองของแท็บนี้ (เซฟแล้ว หรือทิ้งงานแล้ว)
    fn forget_autosave(&self, page: &adw::TabPage) {
        let imp = self.imp();
        imp.autosave_keys.borrow_mut().remove(page);
        if let Some(id) = imp.autosave_ids.borrow_mut().remove(page) {
            remove_backup(&id);
        }
    }

    fn forget_all_autosave(&self) {
        let imp = self.imp();
        imp.autosave_keys.borrow_mut().clear();
        for (_, id) in imp.autosave_ids.borrow_mut().drain() {
            remove_backup(&id);
        }
    }

    /// ตอนเปิดแอป: ถ้ามีไฟล์สำรองค้างจากครั้งก่อน (แอปปิดผิดปกติ) ถามว่าจะกู้คืนไหม
    fn check_recovery(&self) {
        let dir = autosave_dir();
        let mut items: Vec<PathBuf> = std::fs::read_dir(&dir)
            .map(|rd| {
                rd.filter_map(|e| e.ok())
                    .map(|e| e.path())
                    .filter(|p| p.extension().is_some_and(|x| x == "dxf"))
                    .collect()
            })
            .unwrap_or_default();
        items.sort();
        if items.is_empty() {
            return;
        }
        let n = items.len();
        let dialog = adw::AlertDialog::new(
            Some("Recover unsaved work?"),
            Some(&format!(
                "Drafting didn't close normally last time. {} drawing{} backed up automatically.",
                n,
                if n == 1 { " was" } else { "s were" }
            )),
        );
        dialog.add_responses(&[
            ("later", "_Ask Later"),
            ("discard", "_Discard"),
            ("recover", "_Recover"),
        ]);
        dialog.set_response_appearance("discard", adw::ResponseAppearance::Destructive);
        dialog.set_response_appearance("recover", adw::ResponseAppearance::Suggested);
        dialog.set_default_response(Some("recover"));
        dialog.set_close_response("later");
        dialog.connect_response(
            None,
            glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |_, response| match response {
                    "recover" => {
                        for p in &items {
                            let (title, orig) = read_backup_meta(p);
                            obj.open_path_as(p.clone(), Some((title, orig)));
                        }
                    }
                    "discard" => {
                        for p in &items {
                            if let Some(stem) = p.file_stem() {
                                remove_backup(&stem.to_string_lossy());
                            }
                        }
                    }
                    _ => {}
                }
            ),
        );
        dialog.present(Some(self));
    }

    // ---------------- Print ----------------

    /// กล่องตั้งค่าพิมพ์: กระดาษ แนวกระดาษ พื้นที่ สเกล สี
    fn show_print_dialog(&self) {
        // แท็บ Layout พิมพ์ทั้งแผ่นด้วยการตั้งค่าของมันเอง
        if self.layout_print() {
            return;
        }
        let imp = self.imp();
        if imp.is_dialog_open.get() {
            return;
        }
        let dialog = adw::AlertDialog::new(
            Some("Print"),
            Some("In the next window choose a printer, or “Print to File” to save a PDF."),
        );
        dialog.add_responses(&[("cancel", "_Cancel"), ("ok", "_Print…")]);
        dialog.set_response_appearance("ok", adw::ResponseAppearance::Suggested);
        dialog.set_default_response(Some("ok"));
        dialog.set_close_response("cancel");

        let o = imp.print_opts.get();
        let mk = |items: &[&str], sel: u32| {
            let d = gtk::DropDown::new(Some(gtk::StringList::new(items)), gtk::Expression::NONE);
            d.set_selected(sel);
            d
        };
        let papers: Vec<&str> = PAPERS.iter().map(|p| p.0).collect();
        let scales: Vec<&str> = PRINT_SCALES.iter().map(|p| p.0).collect();
        let paper = mk(&papers, o.0);
        let orient = mk(&["Landscape", "Portrait"], o.1);
        let area = mk(&["Current view", "Everything"], o.2);
        let scale = mk(&scales, o.3);
        let color = mk(&["Black", "Colour"], o.4);

        let lbl = |s: &str| gtk::Label::builder().label(s).xalign(0.0).build();
        let grid = gtk::Grid::builder()
            .column_spacing(12)
            .row_spacing(8)
            .halign(gtk::Align::Center)
            .build();
        for (i, (name, w)) in [
            ("Paper", &paper),
            ("Orientation", &orient),
            ("Area", &area),
            ("Scale", &scale),
            ("Lines", &color),
        ]
        .iter()
        .enumerate()
        {
            grid.attach(&lbl(name), 0, i as i32, 1, 1);
            grid.attach(*w, 1, i as i32, 1, 1);
        }
        dialog.set_extra_child(Some(&grid));

        imp.is_dialog_open.set(true);
        dialog.connect_closed(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            move |_| {
                obj.imp().is_dialog_open.set(false);
                if let Some(page) = obj.imp().tab_view.selected_page() {
                    page.child().grab_focus();
                }
            }
        ));
        dialog.connect_response(
            None,
            glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |_, response| {
                    if response != "ok" {
                        return;
                    }
                    let sel = (
                        paper.selected(),
                        orient.selected(),
                        area.selected(),
                        scale.selected(),
                        color.selected(),
                    );
                    obj.imp().print_opts.set(sel);
                    // เปิดหน้าต่างพิมพ์หลังกล่องนี้ปิดสนิท
                    glib::idle_add_local_once(glib::clone!(
                        #[weak]
                        obj,
                        move || obj.run_print(
                            sel.0 as usize,
                            sel.1 == 0,
                            sel.2 == 1,
                            sel.3 as usize,
                            sel.4 == 0
                        )
                    ));
                }
            ),
        );
        dialog.present(Some(self));
    }

    fn run_print(&self, paper: usize, landscape: bool, whole: bool, scale: usize, mono: bool) {
        let imp = self.imp();
        let Some(page) = imp.tab_view.selected_page() else { return };
        let (Some(&(_, pw0, ph0)), Some(&(_, n))) = (PAPERS.get(paper), PRINT_SCALES.get(scale)) else {
            return;
        };
        let (pw, ph) = if landscape { (ph0, pw0) } else { (pw0, ph0) };
        // พื้นที่พิมพ์ในหน่วย "px" (1 px = 0.125 มม. ตามที่ความหนาเส้นใช้: 0.25 มม. = 2 px)
        let (bw, bh) = (
            (pw - 2.0 * PRINT_MARGIN_MM) / PX_MM,
            (ph - 2.0 * PRINT_MARGIN_MM) / PX_MM,
        );
        let (cw, ch) = (page.child().width() as f64, page.child().height() as f64);

        let cmds = self
            .with_engine(|d| {
                let (x0, y0, x1, y1) = if whole {
                    let (a, b) = d.get_elements_bounds()?;
                    (a.x, a.y, b.x, b.y)
                } else {
                    let a = d.camera.screen_to_world(0.0, 0.0);
                    let b = d.camera.screen_to_world(cw, ch);
                    (a.0.min(b.0), a.1.min(b.1), a.0.max(b.0), a.1.max(b.1))
                };
                let (dx, dy) = ((x1 - x0).max(1e-6), (y1 - y0).max(1e-6));
                let s = match n {
                    None => (bw / dx).min(bh / dy) * 0.98,
                    Some(n) => 1.0 / (n * PX_MM), // 1 มม. ในแบบ → 1/n มม. บนกระดาษ
                };
                let (cx, cy) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
                let saved = (d.camera.scale, d.camera.offset);
                d.camera.scale = s;
                d.camera.offset = (bw / 2.0 - cx * s, bh / 2.0 + cy * s);
                let cmds = d.get_render_commands_in(0.0, 0.0, bw, bh);
                d.camera.scale = saved.0;
                d.camera.offset = saved.1;
                Some(cmds)
            })
            .flatten();
        let Some(cmds) = cmds else {
            self.show_error("Nothing to print", "There are no objects to print.");
            return;
        };
        let cmds = print_prepare(cmds, mono);

        let setup = gtk::PageSetup::new();
        setup.set_paper_size(&gtk::PaperSize::new_custom(
            "drafting",
            "Drafting",
            pw0,
            ph0,
            gtk::Unit::Mm,
        ));
        setup.set_orientation(if landscape {
            gtk::PageOrientation::Landscape
        } else {
            gtk::PageOrientation::Portrait
        });
        setup.set_top_margin(PRINT_MARGIN_MM, gtk::Unit::Mm);
        setup.set_bottom_margin(PRINT_MARGIN_MM, gtk::Unit::Mm);
        setup.set_left_margin(PRINT_MARGIN_MM, gtk::Unit::Mm);
        setup.set_right_margin(PRINT_MARGIN_MM, gtk::Unit::Mm);

        let op = gtk::PrintOperation::new();
        op.set_job_name(page.title().as_str());
        op.set_n_pages(1);
        op.set_unit(gtk::Unit::Mm);
        op.set_default_page_setup(Some(&setup));
        op.connect_draw_page(move |_, ctx, _| {
            let cr = ctx.cairo_context();
            let (aw, ah) = (ctx.width() / PX_MM, ctx.height() / PX_MM);
            cr.scale(PX_MM, PX_MM);
            cr.translate(((aw - bw) / 2.0).max(0.0), ((ah - bh) / 2.0).max(0.0));
            cr.rectangle(0.0, 0.0, bw, bh);
            cr.clip();
            CairoRenderer::new(&cr).draw_commands(cmds.clone());
        });
        if let Err(e) = op.run(gtk::PrintOperationAction::PrintDialog, Some(self)) {
            self.show_error("Can't print", &e.to_string());
        }
    }

    // ---------------- Copy / Cut / Paste ----------------

    pub fn copy_action(&self) {
        if self.layout_clip(layout_ui::ClipOp::Copy) {
            return;
        }
        let items = self.with_engine(|d| d.copy_selected()).unwrap_or_default();
        if !items.is_empty() {
            layout_ui::clear_vp_clip();
            CLIPBOARD.with(|c| *c.borrow_mut() = items);
        }
    }

    pub fn cut_action(&self) {
        if self.layout_clip(layout_ui::ClipOp::Cut) {
            return;
        }
        self.copy_action();
        self.delete_action();
    }

    /// in_place = วางที่พิกัดเดิม; ไม่งั้นวางโดยให้กึ่งกลางของที่ copy อยู่ที่เมาส์
    pub fn paste_action(&self, in_place: bool) {
        let op = if in_place { layout_ui::ClipOp::PasteInPlace } else { layout_ui::ClipOp::Paste };
        if self.layout_clip(op) {
            return;
        }
        let items = CLIPBOARD.with(|c| c.borrow().clone());
        if items.is_empty() {
            return;
        }
        let (mx, my) = self.imp().mouse_pos.get();
        self.with_engine(|d| {
            let (dx, dy) = if in_place {
                (0.0, 0.0)
            } else {
                match crate::engine::clip_bounds(&items) {
                    Some((x0, y0, x1, y1)) => {
                        let (wx, wy) = d.camera.screen_to_world(mx, my);
                        (wx - (x0 + x1) / 2.0, wy - (y0 + y1) / 2.0)
                    }
                    None => (0.0, 0.0),
                }
            };
            d.paste_items(&items, dx, dy);
        });
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
                .spacing(10)
                .margin_start(8)
                .margin_end(8)
                .margin_top(4)
                .margin_bottom(4)
                .build();
            hb.set_opacity(if l.visible { 1.0 } else { 0.45 });

            let color_btn = gtk::ColorDialogButton::new(Some(
                gtk::ColorDialog::builder().with_alpha(false).title("Layer color").build(),
            ));
            color_btn.set_rgba(&gdk::RGBA::new(
                l.color.0 as f32 / 255.0,
                l.color.1 as f32 / 255.0,
                l.color.2 as f32 / 255.0,
                1.0,
            ));
            color_btn.set_valign(gtk::Align::Center);
            color_btn.set_tooltip_text(Some("Layer color"));
            color_btn.connect_rgba_notify(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |b| {
                    if obj.imp().building_layers.get() {
                        return;
                    }
                    let c = b.rgba();
                    let rgb = (
                        (c.red() * 255.0).round() as u8,
                        (c.green() * 255.0).round() as u8,
                        (c.blue() * 255.0).round() as u8,
                    );
                    obj.with_engine(|d| d.set_layer_color(i, rgb));
                }
            ));

            let name = gtk::Label::builder()
                .label(&l.name)
                .xalign(0.0)
                .hexpand(true)
                .width_chars(14)
                .max_width_chars(24)
                .ellipsize(gtk::pango::EllipsizeMode::End)
                .tooltip_text(&l.name)
                .build();

            if i != 0 {
                let entry = gtk::Entry::builder().text(&l.name).width_chars(18).build();
                let pop = gtk::Popover::new();
                pop.set_parent(&hb);

                let pbox = gtk::Box::new(gtk::Orientation::Vertical, 6);
                pbox.set_margin_top(6);
                pbox.set_margin_bottom(6);
                pbox.set_margin_start(6);
                pbox.set_margin_end(6);
                pbox.append(&entry);

                let del = gtk::Button::with_label("Delete layer");
                del.add_css_class("destructive-action");
                del.set_tooltip_text(Some("Objects on this layer move to layer 0"));
                pbox.append(&del);
                pop.set_child(Some(&pbox));

                entry.connect_activate(glib::clone!(
                    #[weak(rename_to = obj)]
                    self,
                    #[weak]
                    pop,
                    move |e| {
                        let text = e.text().to_string();
                        pop.popdown();
                        let _ = obj.with_engine(|d| d.rename_layer(i, &text));
                        glib::idle_add_local_once(glib::clone!(
                            #[weak]
                            obj,
                            move || obj.refresh_layers()
                        ));
                    }
                ));

                del.connect_clicked(glib::clone!(
                    #[weak(rename_to = obj)]
                    self,
                    #[weak]
                    pop,
                    move |_| {
                        pop.popdown();
                        let _ = obj.with_engine(|d| d.delete_layer(i));
                        glib::idle_add_local_once(glib::clone!(
                            #[weak]
                            obj,
                            move || {
                                obj.refresh_layers();
                                if let Some(p) = obj.imp().tab_view.selected_page() {
                                    p.child().queue_draw();
                                }
                            }
                        ));
                    }
                ));

                let right_click = gtk::GestureClick::builder().button(3).build();
                right_click.connect_pressed(glib::clone!(
                    #[weak]
                    pop,
                    #[weak]
                    entry,
                    move |_, _, _, _| {
                        pop.popup();
                        entry.grab_focus();
                        entry.select_region(0, -1);
                    }
                ));
                hb.add_controller(right_click);
                hb.connect_destroy(glib::clone!(
                    #[weak]
                    pop,
                    move |_| pop.unparent()
                ));
            }

            let hb_row = hb.clone();
            let eye = gtk::ToggleButton::builder()
                .icon_name(eye_icon(l.visible))
                .active(l.visible)
                .tooltip_text("Show / hide")
                .css_classes(["flat"])
                .build();
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
            if l.locked {
                lock.add_css_class("layer-locked");
            }
            lock.connect_toggled(glib::clone!(
                #[weak(rename_to = obj)]
                self,
                move |b| {
                    b.set_icon_name(lock_icon(b.is_active()));
                    if b.is_active() {
                        b.add_css_class("layer-locked");
                    } else {
                        b.remove_css_class("layer-locked");
                    }
                    obj.with_engine(|d| d.set_layer_locked(i, b.is_active()));
                }
            ));

            let btns = gtk::Box::builder()
                .css_classes(["linked"])
                .halign(gtk::Align::End)
                .build();
            btns.append(&eye);
            btns.append(&lock);

            hb.append(&color_btn);
            hb.append(&name);
            hb.append(&btns);
            row.set_child(Some(&hb));
            imp.layer_list.append(&row);
            if i == current {
                imp.layer_list.select_row(Some(&row));
            }
        }
        imp.building_layers.set(false);
        self.sync_line_style_ui();
    }

    /// แถบเลือกความหนา/ชนิดเส้นของ layer ปัจจุบัน (ใส่ใต้รายการ layer)
    fn build_line_style_ui(&self) {
        let imp = self.imp();
        let Some(add_box) = imp.btn_layer_add.parent() else { return };
        let Some(panel) = add_box.parent().and_then(|p| p.downcast::<gtk::Box>().ok()) else {
            return;
        };

        let labels: Vec<String> =
            crate::linestyle::WEIGHTS_MM.iter().map(|w| format!("{:.2} mm", w)).collect();
        let refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
        let lw = gtk::DropDown::new(Some(gtk::StringList::new(&refs)), gtk::Expression::NONE);
        lw.set_tooltip_text(Some("Line weight of the current layer"));
        let lt = gtk::DropDown::new(
            Some(gtk::StringList::new(&["Continuous"])),
            gtk::Expression::NONE,
        );
        lt.set_tooltip_text(Some("Linetype of the current layer"));
        lt.set_hexpand(true);

        lw.connect_selected_notify(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            move |d| {
                if obj.imp().building_layers.get() {
                    return;
                }
                if let Some(mm) = crate::linestyle::WEIGHTS_MM.get(d.selected() as usize) {
                    obj.with_engine(|e| {
                        let i = e.current_layer;
                        e.set_layer_weight(i, *mm);
                    });
                }
            }
        ));
        lt.connect_selected_notify(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            move |d| {
                if obj.imp().building_layers.get() {
                    return;
                }
                let idx = d.selected() as usize;
                obj.with_engine(|e| {
                    let lt = if idx == 0 { None } else { e.linetype_choices().get(idx - 1).cloned() };
                    let i = e.current_layer;
                    e.set_layer_linetype(i, lt);
                });
            }
        ));

        let row = gtk::Box::builder().spacing(6).build();
        row.append(&lw);
        row.append(&lt);
        panel.insert_child_after(&row, add_box.prev_sibling().as_ref());
        *imp.lw_drop.borrow_mut() = Some(lw);
        *imp.lt_drop.borrow_mut() = Some(lt);
    }

    /// แถบแก้ความหนา/ชนิดเส้นของวัตถุที่เลือก (ใต้แถบของ layer)
    fn build_selection_style_ui(&self) {
        let imp = self.imp();
        let Some(layer_row) = imp.lw_drop.borrow().as_ref().and_then(|w| w.parent()) else { return };
        let Some(panel) = layer_row.parent().and_then(|p| p.downcast::<gtk::Box>().ok()) else {
            return;
        };

        let mut labels: Vec<String> = vec!["ByLayer".to_string()];
        labels.extend(crate::linestyle::WEIGHTS_MM.iter().map(|w| format!("{:.2} mm", w)));
        let refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
        let lw = gtk::DropDown::new(Some(gtk::StringList::new(&refs)), gtk::Expression::NONE);
        lw.set_tooltip_text(Some("Line weight of the selected objects"));
        let lt = gtk::DropDown::new(
            Some(gtk::StringList::new(&["ByLayer", "Continuous"])),
            gtk::Expression::NONE,
        );
        lt.set_tooltip_text(Some("Linetype of the selected objects"));
        lt.set_hexpand(true);

        lw.connect_selected_notify(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            move |d| {
                if obj.imp().building_layers.get() {
                    return;
                }
                let idx = d.selected();
                if idx == gtk::INVALID_LIST_POSITION {
                    return;
                }
                let w = if idx == 0 {
                    None
                } else {
                    crate::linestyle::WEIGHTS_MM.get(idx as usize - 1).copied()
                };
                obj.with_engine(|e| e.set_selected_weight(w));
            }
        ));
        lt.connect_selected_notify(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            move |d| {
                if obj.imp().building_layers.get() {
                    return;
                }
                let idx = d.selected();
                if idx == gtk::INVALID_LIST_POSITION {
                    return;
                }
                let idx = idx as usize;
                obj.with_engine(|e| {
                    let v = match idx {
                        0 => None,
                        1 => Some(crate::linestyle::continuous()),
                        n => e.linetype_choices().get(n - 2).cloned(),
                    };
                    e.set_selected_linetype(v);
                });
            }
        ));

        let label = gtk::Label::builder().xalign(0.0).css_classes(["caption"]).build();
        let row = gtk::Box::builder().spacing(6).build();
        row.append(&lw);
        row.append(&lt);
        let bx = gtk::Box::builder().orientation(gtk::Orientation::Vertical).spacing(4).build();
        bx.append(&label);
        bx.append(&row);
        bx.set_sensitive(false);
        panel.insert_child_after(&bx, Some(&layer_row));

        *imp.sel_box.borrow_mut() = Some(bx);
        *imp.sel_label.borrow_mut() = Some(label);
        *imp.sel_lw_drop.borrow_mut() = Some(lw);
        *imp.sel_lt_drop.borrow_mut() = Some(lt);
        self.sync_selection_ui();
    }

    /// อัปเดตแถบวัตถุที่เลือกเมื่อการเลือก/สไตล์เปลี่ยน (เรียกทุกเฟรม แต่ทำงานเฉพาะตอนเปลี่ยน)
    fn sync_selection_ui(&self) {
        let imp = self.imp();
        let (Some(bx), Some(label), Some(lw), Some(lt)) = (
            imp.sel_box.borrow().clone(),
            imp.sel_label.borrow().clone(),
            imp.sel_lw_drop.borrow().clone(),
            imp.sel_lt_drop.borrow().clone(),
        ) else {
            return;
        };
        let Some(page) = imp.tab_view.selected_page() else { return };
        let pages = imp.pages.borrow();
        let Some(d_ref) = pages.get(&page) else { return };
        let d = d_ref.borrow();

        let key = (glib::prelude::ObjectType::as_ptr(&page) as usize, d.sel_rev);
        if imp.sel_key.get() == key {
            return;
        }
        imp.sel_key.set(key);

        let s = d.selection_style();
        let prev = imp.building_layers.replace(true); // กันไม่ให้การตั้งค่าตัวเลือกไปแก้วัตถุ
        bx.set_sensitive(s.count > 0);
        label.set_text(&if s.count > 0 {
            format!("Selected objects ({})", s.count)
        } else {
            "Selected objects".to_string()
        });

        if s.count == 0 {
            lw.set_selected(gtk::INVALID_LIST_POSITION);
            lt.set_selected(gtk::INVALID_LIST_POSITION);
        } else {
            let wi = if s.weight_mixed {
                gtk::INVALID_LIST_POSITION
            } else {
                s.weight.map_or(0, |w| crate::linestyle::nearest_weight_index(w) as u32 + 1)
            };
            lw.set_selected(wi);

            let choices = d.linetype_choices();
            let mut names: Vec<&str> = vec!["ByLayer", "Continuous"];
            names.extend(choices.iter().map(|c| c.name.as_str()));
            lt.set_model(Some(&gtk::StringList::new(&names)));
            let li = if s.lt_mixed {
                gtk::INVALID_LIST_POSITION
            } else {
                match &s.linetype {
                    None => 0,
                    Some(l) if l.pattern.is_empty() => 1,
                    Some(l) => choices
                        .iter()
                        .position(|c| c.name.eq_ignore_ascii_case(&l.name))
                        .map_or(gtk::INVALID_LIST_POSITION, |p| p as u32 + 2),
                }
            };
            lt.set_selected(li);
        }
        imp.building_layers.set(prev);
    }

    /// ให้แถบความหนา/ชนิดเส้นตรงกับ layer ปัจจุบันของแท็บที่เปิดอยู่
    fn sync_line_style_ui(&self) {
        let imp = self.imp();
        let (Some(lw), Some(lt)) = (imp.lw_drop.borrow().clone(), imp.lt_drop.borrow().clone())
        else {
            return;
        };
        let Some(page) = imp.tab_view.selected_page() else { return };
        let pages = imp.pages.borrow();
        let Some(d_ref) = pages.get(&page) else { return };
        let d = d_ref.borrow();
        let Some(l) = d.layers.get(d.current_layer) else { return };

        let prev = imp.building_layers.replace(true); // กันไม่ให้การตั้งค่าตัวเลือกไปแก้ layer
        lw.set_selected(crate::linestyle::nearest_weight_index(l.weight) as u32);

        let choices = d.linetype_choices();
        let mut names: Vec<&str> = vec!["Continuous"];
        names.extend(choices.iter().map(|c| c.name.as_str()));
        lt.set_model(Some(&gtk::StringList::new(&names)));
        let sel = l
            .linetype
            .as_ref()
            .and_then(|x| choices.iter().position(|c| c.name.eq_ignore_ascii_case(&x.name)))
            .map_or(0, |p| p + 1);
        lt.set_selected(sel as u32);
        imp.building_layers.set(prev);
    }

    pub fn undo_action(&self) {
        if let Some(page) = self.imp().tab_view.selected_page() {
            {
                let pages = self.imp().pages.borrow();
                if let Some(d_ref) = pages.get(&page) {
                    d_ref.borrow_mut().undo_last_action();
                }
            }
            page.child().queue_draw();
            self.refresh_layers();
        }
    }

    pub fn redo_action(&self) {
        if let Some(page) = self.imp().tab_view.selected_page() {
            {
                let pages = self.imp().pages.borrow();
                if let Some(d_ref) = pages.get(&page) {
                    d_ref.borrow_mut().redo_last_action();
                }
            }
            page.child().queue_draw();
            self.refresh_layers();
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

    pub fn save_page(&self, page: &adw::TabPage, on_done: impl Fn(bool) + 'static) {
        let path = self
            .imp()
            .pages
            .borrow()
            .get(page)
            .and_then(|d| d.borrow().file_path.clone());
        match path {
            Some(p) => self.write_page_guarded(page, p, std::rc::Rc::new(on_done)),
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
                move |res| match res {
                    Ok(file) => match file.path() {
                        Some(p) => obj.write_page_guarded(&page, p, std::rc::Rc::new(on_done)),
                        None => on_done(false),
                    },
                    Err(_) => on_done(false),
                }
            ),
        );
    }

    fn write_page_guarded(
        &self,
        page: &adw::TabPage,
        path: PathBuf,
        on_done: std::rc::Rc<dyn Fn(bool)>,
    ) {
        let (skipped, same) = {
            let pages = self.imp().pages.borrow();
            match pages.get(page) {
                Some(d) => {
                    let d = d.borrow();
                    (d.skipped, d.file_path.as_deref() == Some(path.as_path()))
                }
                None => (0, false),
            }
        };

        if skipped == 0 || !same {
            on_done(self.write_page_to(page, path));
            return;
        }

        let dialog = adw::AlertDialog::new(
            Some("Overwrite and lose objects?"),
            Some(&format!(
                "{} objects in this file aren't supported yet and won't be saved. \
                 Saving over it will delete them from the file.\n\n\
                 Use Save As (Ctrl+Shift+S) to keep the original.",
                skipped
            )),
        );
        dialog.add_responses(&[
            ("cancel", "_Cancel"),
            ("saveas", "Save _As…"),
            ("overwrite", "_Overwrite"),
        ]);
        dialog.set_response_appearance("overwrite", adw::ResponseAppearance::Destructive);
        dialog.set_default_response(Some("saveas"));
        dialog.set_close_response("cancel");

        dialog.connect_response(
            None,
            glib::clone!(
                #[weak(rename_to = obj)]
                self,
                #[weak]
                page,
                move |_, response| match response {
                    "overwrite" => on_done(obj.write_page_to(&page, path.clone())),
                    "saveas" => {
                        let cb = on_done.clone();
                        obj.save_page_as(&page, move |ok| cb(ok));
                    }
                    _ => on_done(false),
                }
            ),
        );
        dialog.present(Some(self));
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
        self.forget_autosave(page);
        if let Some(name) = path.file_name() {
            page.set_title(&name.to_string_lossy());
        }

        if d.file_path.as_deref() != Some(path.as_path()) {
            d.skipped = 0; // เซฟเป็นไฟล์ใหม่แล้ว ไม่ต้องเตือนซ้ำ
        }

        d.file_path = Some(path);
        true
    }

    pub fn show_open_dialog(&self) {
        let (_, filters, dxf_filter) = open_filters();
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
        self.open_path_as(path, None);
    }

    /// restore = Some((ชื่อแท็บ, path ไฟล์จริง)) เมื่อเปิดจากไฟล์สำรอง (แท็บจะถือว่ามีงานค้าง)
    fn open_path_as(&self, path: PathBuf, restore: Option<(String, Option<PathBuf>)>) {
        let obj = self.clone();
        glib::spawn_future_local(async move {
            let title = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "Untitled".to_string());
            let title = restore.as_ref().map(|r| r.0.clone()).unwrap_or(title);

            let p = path.clone();
            let result = gio::spawn_blocking(move || {
                let is_dwg = p.extension().is_some_and(|e| e.eq_ignore_ascii_case("dwg"));
                let doc = if is_dwg {
                    acadrust::DwgReader::from_file(&p).and_then(|mut r| r.read())
                } else {
                    acadrust::DxfReader::from_file(&p).and_then(|r| r.read())
                };
                doc.map(|d| dxf_to_elements(&d)).map_err(|e| e.to_string())
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
                for (data, layer, color, style) in imported.items {
                    d.push_imported(data, layer, color, style);
                }
                let is_dwg = path.extension().is_some_and(|e| e.eq_ignore_ascii_case("dwg"));
                d.file_path = if is_dwg { None } else { Some(path.clone()) };
                d.dirty = false;
                d.skipped = skipped;
                d.layouts = imported.layouts;
                if let Some((_, orig)) = &restore {
                    // กู้คืนแล้ว: ยังไม่เซฟลงไฟล์จริง → ถือว่ามีงานค้าง และใช้ไฟล์สำรองเดิมต่อ
                    d.file_path = orig.clone();
                    d.dirty = true;
                    if let Some(stem) = path.file_stem() {
                        obj.imp()
                            .autosave_ids
                            .borrow_mut()
                            .insert(page.clone(), stem.to_string_lossy().to_string());
                    }
                }
            }
            obj.refresh_layers();
            obj.open_layout_tabs(&page);

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
                            obj.forget_autosave(&page);
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
                        obj.forget_all_autosave();
                        obj.imp().force_close.set(true);
                        obj.close();
                    }
                }
            ),
        );
        dialog.present(Some(self));
    }
}

/// ปัดเป็นเลขกลม 1 / 2 / 5 × 10^n ที่ใกล้ที่สุด
fn nice_number(x: f64) -> f64 {
    if !(x > 0.0 && x.is_finite()) {
        return 2.5;
    }
    let p = 10f64.powf(x.log10().floor());
    let m = x / p;
    let n = if m < 1.5 {
        1.0
    } else if m < 3.5 {
        2.0
    } else if m < 7.5 {
        5.0
    } else {
        10.0
    };
    n * p
}

const AUTOSAVE_SECS: u32 = 120;

/// กระดาษ (ชื่อ, กว้าง, ยาว) มม. แนวตั้ง
const PAPERS: [(&str, f64, f64); 6] = [
    ("A4", 210.0, 297.0),
    ("A3", 297.0, 420.0),
    ("A2", 420.0, 594.0),
    ("A1", 594.0, 841.0),
    ("A0", 841.0, 1189.0),
    ("Letter", 215.9, 279.4),
];

/// สเกลพิมพ์ 1:n (None = พอดีหน้ากระดาษ)
const PRINT_SCALES: [(&str, Option<f64>); 10] = [
    ("Fit to page", None),
    ("1:1", Some(1.0)),
    ("1:2", Some(2.0)),
    ("1:5", Some(5.0)),
    ("1:10", Some(10.0)),
    ("1:20", Some(20.0)),
    ("1:50", Some(50.0)),
    ("1:100", Some(100.0)),
    ("1:200", Some(200.0)),
    ("1:500", Some(500.0)),
];
const PRINT_MARGIN_MM: f64 = 10.0;
/// หน่วยที่ตัววาดใช้: 1 px = 0.125 มม. บนกระดาษ
const PX_MM: f64 = 0.125;

fn autosave_dir() -> PathBuf {
    glib::user_data_dir().join("drafting").join("autosave")
}

fn new_autosave_id(n: usize) -> String {
    let ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    format!("{:x}-{}", ms, n)
}

fn write_atomic(path: &std::path::Path, bytes: &[u8]) -> std::io::Result<()> {
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, bytes)?;
    std::fs::rename(&tmp, path)
}

fn remove_backup(id: &str) {
    let dir = autosave_dir();
    let _ = std::fs::remove_file(dir.join(format!("{id}.dxf")));
    let _ = std::fs::remove_file(dir.join(format!("{id}.meta")));
}

/// (ชื่อแท็บ, path ไฟล์จริงถ้ามี) จาก .meta ที่อยู่ข้างไฟล์สำรอง
fn read_backup_meta(dxf: &std::path::Path) -> (String, Option<PathBuf>) {
    let text = std::fs::read_to_string(dxf.with_extension("meta")).unwrap_or_default();
    let mut lines = text.lines();
    let title = lines
        .next()
        .filter(|t| !t.is_empty())
        .unwrap_or("Recovered drawing")
        .to_string();
    let orig = lines.next().filter(|p| !p.is_empty()).map(PathBuf::from);
    (title, orig)
}

/// เตรียมคำสั่งวาดสำหรับพิมพ์: ไม่มีสีเลือก, เส้นสีดำ (หรือสีจริงแต่ขาว→ดำ), มิติเป็นสีดำ
fn print_prepare(cmds: Vec<RenderCommand>, mono: bool) -> Vec<RenderCommand> {
    let fix = |c: (f64, f64, f64)| {
        if mono || (c.0 > 0.9 && c.1 > 0.9 && c.2 > 0.9) { (0.0, 0.0, 0.0) } else { c }
    };
    let black = (0.0, 0.0, 0.0);
    let mut cur: (f64, Vec<f64>) = (2.0, Vec::new());
    let mut out = Vec::with_capacity(cmds.len());
    for c in cmds {
        match c {
            RenderCommand::Line { x1, y1, x2, y2, color, .. } => {
                out.push(RenderCommand::Line { x1, y1, x2, y2, is_selected: false, color: fix(color) })
            }
            RenderCommand::Circle { cx, cy, radius, color, .. } => {
                out.push(RenderCommand::Circle { cx, cy, radius, is_selected: false, color: fix(color) })
            }
            RenderCommand::Arc { cx, cy, radius, start_angle, end_angle, color, .. } => {
                out.push(RenderCommand::Arc {
                    cx,
                    cy,
                    radius,
                    start_angle,
                    end_angle,
                    is_selected: false,
                    color: fix(color),
                })
            }
            RenderCommand::Polyline { points, closed, color, .. } => out.push(RenderCommand::Polyline {
                points,
                closed,
                is_selected: false,
                color: fix(color),
            }),
            RenderCommand::Fill { loops, filled, color, .. } => out.push(RenderCommand::Fill {
                loops,
                filled,
                is_selected: false,
                color: fix(color),
            }),
            RenderCommand::Style { lw, dash } => {
                cur = (lw, dash.clone());
                out.push(RenderCommand::Style { lw, dash });
            }
            RenderCommand::Text { x, y, em_px, rotation, wrap_px, text, halign, valign, color, .. } => {
                out.push(RenderCommand::Text {
                    x,
                    y,
                    em_px,
                    rotation,
                    wrap_px,
                    text,
                    halign,
                    valign,
                    is_selected: false,
                    color: fix(color),
                })
            }
            RenderCommand::Dimension {
                text_x, text_y, text,
                dim_x1, dim_y1, dim_x2, dim_y2,
                ext1_x1, ext1_y1, ext1_x2, ext1_y2,
                ext2_x1, ext2_y1, ext2_x2, ext2_y2,
                ..
            } => {
                // ตัววาดมิติใช้สีเหลืองบนจอ → บนกระดาษแตกเป็นเส้นดำ + ตัวเลขดำ
                out.push(RenderCommand::Style { lw: 1.5, dash: Vec::new() });
                for (a, b, c, d) in [
                    (ext1_x1, ext1_y1, ext1_x2, ext1_y2),
                    (ext2_x1, ext2_y1, ext2_x2, ext2_y2),
                    (dim_x1, dim_y1, dim_x2, dim_y2),
                ] {
                    out.push(RenderCommand::Line {
                        x1: a,
                        y1: b,
                        x2: c,
                        y2: d,
                        is_selected: false,
                        color: black,
                    });
                }
                out.push(RenderCommand::Text {
                    x: text_x,
                    y: text_y - 6.0,
                    em_px: 24.0,
                    rotation: 0.0,
                    wrap_px: 0.0,
                    text,
                    halign: crate::text::HAlign::Center,
                    valign: crate::text::VAlign::Bottom,
                    is_selected: false,
                    color: black,
                });
                out.push(RenderCommand::Style { lw: cur.0, dash: cur.1.clone() });
            }
        }
    }
    out
}

thread_local! {
    /// Copy/Paste ใช้ร่วมกันทุกแท็บ
    static CLIPBOARD: RefCell<Vec<crate::engine::ClipItem>> = const { RefCell::new(Vec::new()) };
}

fn scene_margin(w: f64, h: f64) -> (f64, f64) {
    ((w * SCENE_MARGIN).round(), (h * SCENE_MARGIN).round())
}

const SCENE_MARGIN: f64 = 0.25;
type SceneKey = (i32, i32, u64, u64, u64);

#[derive(Debug)]
struct SceneCache {
    key: SceneKey,
    rev: u64,
    world_tl: (f64, f64),
    world_br: (f64, f64),
    cmds: Vec<RenderCommand>,
    surface: gtk::cairo::ImageSurface,
}

#[derive(Debug, Default)]
struct SceneState {
    cache: Option<SceneCache>,
    pending: bool,
    last_cost: std::time::Duration,
    want: Option<(SceneKey, std::time::Instant)>,
}

struct SceneJob {
    w: f64,
    h: f64,
    sx: f64,
    sy: f64,
    key: SceneKey,
    rev: u64,
    tl: (f64, f64),
    br: (f64, f64),
    commands: Vec<RenderCommand>,
}

struct SceneDone {
    key: SceneKey,
    rev: u64,
    tl: (f64, f64),
    br: (f64, f64),
    commands: Vec<RenderCommand>,
    data: Vec<u8>,
    wpx: i32,
    hpx: i32,
    stride: i32,
    sx: f64,
    sy: f64,
}

fn cam_moving(c: &crate::engine::Camera) -> bool {
    (c.scale / c.target_scale).ln().abs() > 0.00001
        || (c.target_offset.0 - c.offset.0).abs() > 0.01
        || (c.target_offset.1 - c.offset.1).abs() > 0.01
}

fn render_scene_job(job: SceneJob) -> Option<SceneDone> {
    use gtk::cairo::{Context, Format, ImageSurface};
    let (mx, my) = scene_margin(job.w, job.h);
    let (ew, eh) = (job.w + 2.0 * mx, job.h + 2.0 * my);
    let (wpx, hpx) = ((ew * job.sx).ceil() as i32, (eh * job.sy).ceil() as i32);
    let mut surface = ImageSurface::create(Format::ARgb32, wpx, hpx).ok()?;
    surface.set_device_scale(job.sx, job.sy);
    {
        let sc = Context::new(&surface).ok()?;
        sc.translate(mx, my);
        paint_scene(&sc, job.commands.clone());
    }
    surface.flush();
    let stride = surface.stride();
    let data = surface.data().ok()?.to_vec();
    Some(SceneDone {
        key: job.key,
        rev: job.rev,
        tl: job.tl,
        br: job.br,
        commands: job.commands,
        data,
        wpx,
        hpx,
        stride,
        sx: job.sx,
        sy: job.sy,
    })
}

fn install_scene(state: &mut SceneState, done: SceneDone) {
    use gtk::cairo::{Format, ImageSurface};
    if let Ok(surface) =
        ImageSurface::create_for_data(done.data, Format::ARgb32, done.wpx, done.hpx, done.stride)
    {
        surface.set_device_scale(done.sx, done.sy);
        state.cache = Some(SceneCache {
            key: done.key,
            rev: done.rev,
            world_tl: done.tl,
            world_br: done.br,
            cmds: done.commands,
            surface,
        });
    }
}

fn spawn_scene_build(
    state: &std::rc::Rc<RefCell<SceneState>>,
    area: gtk::Widget,
    job: SceneJob,
) {
    state.borrow_mut().pending = true;
    let state = state.clone();
    glib::spawn_future_local(async move {
        let t0 = std::time::Instant::now();
        let res = gio::spawn_blocking(move || render_scene_job(job)).await;
        {
            let mut st = state.borrow_mut();
            st.pending = false;
            if let Ok(Some(done)) = res {
                install_scene(&mut st, done);
                st.last_cost = t0.elapsed();
            }
        }
        area.queue_draw();
    });
}

fn paint_grid(cr: &gtk::cairo::Context, cam: &crate::engine::Camera, w: f64, h: f64) {
    let scale = cam.scale;

    cr.set_source_rgb(0.1, 0.1, 0.1);
    let _ = cr.paint();

    let grid_base = 100.0;
    let lod_raw = scale.log10();
    let lod = lod_raw.floor();
    let fraction = lod_raw - lod;
    let step_major = grid_base / 10.0f64.powf(lod);
    let step_minor = step_major / 10.0;
    let step_giant = step_major * 10.0;

    let draw_grid_line = |s: f64, alpha: f64, lw: f64| {
        if alpha <= 0.02 || s * scale < 8.0 {
            return;
        }
        cr.set_source_rgba(0.5, 0.5, 0.5, alpha);
        cr.set_line_width(lw);

        let (w_min_x, w_max_y) = cam.screen_to_world(0.0, 0.0);
        let (w_max_x, w_min_y) = cam.screen_to_world(w, h);

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

    draw_grid_line(step_giant, (1.0 - fraction) * 0.15, 1.0);
    draw_grid_line(step_major, 0.35, 0.8);
    draw_grid_line(step_minor, fraction * 0.15, 0.75);
}

/// เฉพาะวัตถุ วาดลง surface โปร่งใสที่แคชไว้
fn paint_scene(cr: &gtk::cairo::Context, commands: Vec<RenderCommand>) {
    cr.set_source_rgb(1.0, 1.0, 1.0);
    cr.set_line_width(2.0);
    CairoRenderer::new(cr).draw_commands(commands);
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

/// วาดภาพเงา (ทุกชนิดวัตถุ) ลง path ปัจจุบัน
fn trace_ghost(cr: &gtk::cairo::Context, cam: &crate::engine::Camera, data: &ElementData) {
    match data {
        ElementData::Dimension { start, end, offset } => {
            if let Some(g) = linear_dimension_geometry(*start, *end, *offset) {
                for (a, b) in [(g.start, g.dim_from), (g.end, g.dim_to), (g.dim_from, g.dim_to)] {
                    let (ax, ay) = cam.world_to_screen(a.x, a.y);
                    let (bx, by) = cam.world_to_screen(b.x, b.y);
                    cr.move_to(ax, ay);
                    cr.line_to(bx, by);
                }
            }
        }
        ElementData::Fill { loops, .. } | ElementData::Hatch { loops, .. } => {
            for l in loops {
                for (i, p) in l.iter().enumerate() {
                    let (x, y) = cam.world_to_screen(p.x, p.y);
                    if i == 0 {
                        cr.move_to(x, y);
                    } else {
                        cr.line_to(x, y);
                    }
                }
                cr.close_path();
            }
        }
        ElementData::Text { pos, .. } => {
            let (x, y) = cam.world_to_screen(pos.x, pos.y);
            cr.move_to(x - 4.0, y);
            cr.line_to(x + 4.0, y);
            cr.move_to(x, y - 4.0);
            cr.line_to(x, y + 4.0);
        }
        other => trace_element(cr, cam, other),
    }
}

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

fn sample_arc(c: Point, r: f64, a0: f64, a1: f64, ccw: bool) -> Vec<Point> {
    let mut e = a1;
    if ccw {
        while e <= a0 {
            e += std::f64::consts::TAU;
        }
    } else {
        while e >= a0 {
            e -= std::f64::consts::TAU;
        }
    }
    let span = e - a0;
    let steps = ((span.abs() / std::f64::consts::TAU * 64.0).ceil() as usize).max(4);
    (0..=steps)
        .map(|i| {
            let a = a0 + span * i as f64 / steps as f64;
            Point { x: c.x + r * a.cos(), y: c.y + r * a.sin() }
        })
        .collect()
}

/// ช่วง bulge จาก p1 ไป p2 เป็นจุดต่อเนื่อง (รวมปลายทั้งสอง)
fn bulge_points(p1: Point, p2: Point, bulge: f64) -> Vec<Point> {
    match bulge_segment(p1, p2, bulge) {
        ElementData::Arc { center, radius, start_angle, end_angle } => {
            let mut v = sample_arc(center, radius, start_angle, end_angle, true);
            // bulge ลบ: bulge_segment สลับหัวท้ายไว้ ให้หันกลับให้ตรงทิศ p1→p2
            if bulge < 0.0 {
                v.reverse();
            }
            v
        }
        _ => vec![p1, p2],
    }
}

fn push_unique(out: &mut Vec<Point>, p: Point) {
    if out.last().map_or(true, |l| (l.x - p.x).hypot(l.y - p.y) > 1e-9) {
        out.push(p);
    }
}

/// edge เดียวของ hatch boundary → จุด (ตามทิศของ edge)
fn hatch_edge_points(e: &acadrust::entities::BoundaryEdge) -> Vec<Point> {
    use acadrust::entities::BoundaryEdge as E;
    match e {
        E::Line(l) => vec![
            Point { x: l.start.x, y: l.start.y },
            Point { x: l.end.x, y: l.end.y },
        ],
        E::CircularArc(a) => sample_arc(
            Point { x: a.center.x, y: a.center.y },
            a.radius,
            a.start_angle,
            a.end_angle,
            a.counter_clockwise,
        ),
        E::EllipticArc(a) => {
            let (mut pts, _) = ellipse_points(
                (a.center.x, a.center.y),
                (a.major_axis_endpoint.x, a.major_axis_endpoint.y),
                a.minor_axis_ratio,
                a.start_angle,
                a.end_angle,
            );
            if !a.counter_clockwise {
                pts.reverse();
            }
            pts
        }
        E::Spline(s) => {
            let src: Vec<(f64, f64)> = if s.fit_points.len() >= 2 {
                s.fit_points.iter().map(|v| (v.x, v.y)).collect()
            } else {
                s.control_points.iter().map(|v| (v.x, v.y)).collect()
            };
            polyline_through(&src)
        }
        E::Polyline(p) => {
            let n = p.vertices.len();
            let segs = if p.is_closed { n } else { n.saturating_sub(1) };
            let mut out = Vec::new();
            for i in 0..segs {
                let (a, b) = (p.vertices[i], p.vertices[(i + 1) % n]);
                let (pa, pb) = (Point { x: a.x, y: a.y }, Point { x: b.x, y: b.y });
                for q in bulge_points(pa, pb, a.z) {
                    push_unique(&mut out, q);
                }
            }
            if n == 1 {
                out.push(Point { x: p.vertices[0].x, y: p.vertices[0].y });
            }
            out
        }
    }
}

/// Solid/Trace → พื้นที่ทึบ (ใช้ลำดับมุมตามขอบจริง)
fn fill_from_solid(s: &acadrust::entities::Solid) -> ElementData {
    let pts: Vec<Point> =
        s.boundary_corners().iter().map(|v| Point { x: v.x, y: v.y }).collect();
    ElementData::Fill { loops: vec![pts], filled: true }
}

/// Hatch → loops ของขอบเขต (วาดทึบถ้าเป็น solid/gradient, ไม่งั้นวาดแค่ขอบ)
fn fill_from_hatch(h: &acadrust::entities::Hatch) -> Option<ElementData> {
    let mut loops: Vec<Vec<Point>> = Vec::new();
    for path in &h.paths {
        let mut pts: Vec<Point> = Vec::new();
        for edge in &path.edges {
            let mut seg = hatch_edge_points(edge);
            if seg.is_empty() {
                continue;
            }
            // edge ที่ต่อมาเป็นทิศกลับ → กลับด้านให้ต่อกันได้
            if let Some(last) = pts.last() {
                let d = |p: &Point| (p.x - last.x).hypot(p.y - last.y);
                if d(&seg[0]) > d(&seg[seg.len() - 1]) {
                    seg.reverse();
                }
            }
            for q in seg {
                push_unique(&mut pts, q);
            }
        }
        if pts.len() >= 2 {
            if let (Some(f), Some(l)) = (pts.first().copied(), pts.last().copied()) {
                if pts.len() > 2 && (f.x - l.x).hypot(f.y - l.y) < 1e-9 {
                    pts.pop();
                }
            }
        }
        if pts.len() >= 3 {
            loops.push(pts);
        }
    }
    if loops.is_empty() {
        return None;
    }
    let filled = h.is_solid || h.gradient_color.enabled;
    Some(ElementData::Fill { loops, filled })
}

// ===== arm ใหม่ใน match ของ dxf_to_elements (ก่อน `_ => skipped += 1`) =====
//
//     EntityType::Solid(s) => out.push(fill_from_solid(s)),
//     EntityType::Hatch(h) => match fill_from_hatch(h) {
//         Some(d) => out.push(d),
//         None => skipped += 1,
//     },

fn dxf_filters() -> (gtk::FileFilter, gio::ListStore, gtk::FileFilter) {
    let f = gtk::FileFilter::new();
    f.set_name(Some("DXF Drawing (*.dxf)"));
    f.add_pattern("*.dxf");
    f.add_pattern("*.DXF");
    let list = gio::ListStore::new::<gtk::FileFilter>();
    list.append(&f);
    (f.clone(), list, f)
}

/// filter ของ dialog เปิดไฟล์: DXF และ DWG (เซฟยังเป็น DXF อย่างเดียว)
fn open_filters() -> (gtk::FileFilter, gio::ListStore, gtk::FileFilter) {
    let all = gtk::FileFilter::new();
    all.set_name(Some("DXF and DWG drawings"));
    for p in ["*.dxf", "*.DXF", "*.dwg", "*.DWG"] {
        all.add_pattern(p);
    }
    let (dxf, _, _) = dxf_filters();
    let dwg = gtk::FileFilter::new();
    dwg.set_name(Some("DWG Drawing (*.dwg)"));
    dwg.add_pattern("*.dwg");
    dwg.add_pattern("*.DWG");
    let list = gio::ListStore::new::<gtk::FileFilter>();
    list.append(&all);
    list.append(&dxf);
    list.append(&dwg);
    (all.clone(), list, all)
}

type Rgb = (u8, u8, u8);

struct Imported {
    layers: Vec<crate::engine::Layer>,
    items: Vec<(ElementData, usize, Option<Rgb>, crate::linestyle::ElementStyle)>,
    skipped: usize,
    layouts: Vec<crate::layout::LayoutSpec>,
}

fn dxf_to_elements(doc: &acadrust::CadDocument) -> Imported {
    use crate::engine::Layer;
    use acadrust::EntityType;
    use std::collections::HashMap;
    let lt_map = crate::linestyle::linetype_map(doc);
    let global_lt_scale = if doc.header.linetype_scale > 0.0 { doc.header.linetype_scale } else { 1.0 };
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
                (layers[i].weight, layers[i].linetype) = crate::linestyle::layer_style(l, &lt_map);
            }
            None => {
                index.insert(l.name.to_lowercase(), layers.len());
                let (weight, linetype) = crate::linestyle::layer_style(l, &lt_map);
                layers.push(Layer { name: l.name.clone(), color, visible, locked, weight, linetype });
            }
        }
    }

    let mut items = Vec::new();
    let mut skipped = 0usize;

    // INSERT (ซ้อนได้) และ DIMENSION ถูกแตกเป็นวัตถุธรรมดาก่อน
    let flat = crate::blocks::flatten_model_space(doc);
    for e in &flat {
        let c = e.common();
        let lname = if c.layer.is_empty() { "0" } else { c.layer.as_str() };
        let li = *index.entry(lname.to_lowercase()).or_insert_with(|| {
            layers.push(Layer::new(lname, (255, 255, 255)));
            layers.len() - 1
        });
        // ByLayer / ByBlock → None (ใช้สีของ layer), Index / Rgb → สีของวัตถุเอง
        let col = c.color.rgb();
        let style = crate::linestyle::entity_style(c, &lt_map, global_lt_scale);

        let mut out: Vec<ElementData> = Vec::new();
        match e {
            EntityType::Line(l) => out.push(ElementData::Line {
                start: Point { x: l.start.x, y: l.start.y },
                end: Point { x: l.end.x, y: l.end.y },
            }),
            EntityType::Circle(c) => out.push(crate::blocks::circle_element(c)),
            EntityType::Arc(a) => out.push(crate::blocks::arc_element(a)),
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
            EntityType::Ellipse(el) if crate::blocks::ellipse_round(el).is_some() => {
                out.extend(crate::blocks::ellipse_round(el));
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

            EntityType::Solid(s) => out.push(fill_from_solid(s)),
            EntityType::Hatch(h) => match fill_from_hatch(h) {
                Some(d) => out.push(d),
                None => skipped += 1,
            },

            EntityType::Text(t) => match crate::text::from_dxf_text(t) {
                Some(d) => out.push(d),
                None => skipped += 1,
            },
            EntityType::MText(m) => match crate::text::from_dxf_mtext(m) {
                Some(d) => out.push(d),
                None => skipped += 1,
            },

            _ => skipped += 1,
        }
        for data in out {
            items.push((data, li, col, style.clone()));
        }
    }
    let mut layouts = crate::layout::read_layouts(doc);
    layout_ui::read_paper_items(doc, &mut layouts);
    Imported { layers, items, skipped, layouts }
}

fn to_ac_color(c: Rgb) -> acadrust::Color {
    if c == (255, 255, 255) {
        acadrust::Color::Index(7)
    } else {
        acadrust::Color::Rgb { r: c.0, g: c.1, b: c.2 }
    }
}

fn engine_to_dxf_bytes(data: &crate::engine::Engine) -> Result<Vec<u8>, String> {
    use acadrust::entities::{
        Arc, BoundaryEdge, BoundaryPath, Circle, Hatch, Line, LwPolyline, LwVertex, PolylineEdge, Solid,
    };
    use acadrust::tables::Layer as AcLayer;
    use acadrust::{CadDocument, DxfWriter, EntityType, Vector2, Vector3};

    let mut doc = CadDocument::new();

    crate::linestyle::ensure_linetypes(
        &mut doc,
        data.layers
            .iter()
            .filter_map(|l| l.linetype.as_ref())
            .chain(data.history.iter().filter_map(|e| e.style.linetype.as_ref())),
    );

    // ตาราง layer ("0" มีอยู่แล้วในไฟล์ใหม่ → แก้ของเดิม ไม่เพิ่มซ้ำ)
    for l in &data.layers {
        if let Some(ex) = doc.layers.get_mut(&l.name) {
            ex.color = to_ac_color(l.color);
            ex.flags.off = !l.visible;
            ex.flags.locked = l.locked;
            ex.line_weight = crate::linestyle::to_ac_weight(l.weight);
            ex.line_type = crate::linestyle::ac_linetype_name(&l.linetype);
        } else {
            let mut nl = AcLayer::new(l.name.clone());
            nl.color = to_ac_color(l.color);
            nl.flags.off = !l.visible;
            nl.flags.locked = l.locked;
            nl.line_weight = crate::linestyle::to_ac_weight(l.weight);
            nl.line_type = crate::linestyle::ac_linetype_name(&l.linetype);
            doc.layers.add(nl)?;
        }
    }

    for el in data.get_all_elements() {
        let layer_name = data.layers[el.layer].name.clone();
        let color = el.color.map(to_ac_color);
        let style = el.style.clone();
        let mut push = |mut e: EntityType| -> Result<(), String> {
            {
                let c = e.common_mut();
                c.layer = layer_name.clone();
                if let Some(col) = &color {
                    c.color = col.clone();
                }
                if let Some(w) = style.weight {
                    c.line_weight = crate::linestyle::to_ac_weight(w);
                }
                if let Some(lt) = &style.linetype {
                    c.linetype = lt.name.clone();
                }
                c.linetype_scale = style.lt_scale;
            }
            doc.add_entity(e).map(|_| ()).map_err(|e| e.to_string())
        };
        let line = |a: Point, b: Point| {
            let mut l = Line::new();
            l.start = Vector3::new(a.x, a.y, 0.0);
            l.end = Vector3::new(b.x, b.y, 0.0);
            EntityType::Line(l)
        };

        match &el.data {
            ElementData::Line { start, end } => push(line(*start, *end))?,
            ElementData::Circle { center, radius } => push(EntityType::Circle(
                Circle::from_center_radius(Vector3::new(center.x, center.y, 0.0), *radius),
            ))?,
            ElementData::Dimension { start, end, offset } => {
                if let Some(g) = linear_dimension_geometry(*start, *end, *offset) {
                    push(line(g.start, g.dim_from))?;
                    push(line(g.end, g.dim_to))?;
                    push(line(g.dim_from, g.dim_to))?;
                }
            }
            // acadrust เก็บมุม Arc เป็นเรเดียนเหมือนเรา (DxfWriter แปลงเป็นองศาเอง)
            ElementData::Arc { center, radius, start_angle, end_angle } => {
                push(EntityType::Arc(Arc::from_center_radius_angles(
                    Vector3::new(center.x, center.y, 0.0),
                    *radius,
                    *start_angle,
                    *end_angle,
                )))?
            }
            ElementData::Fill { loops, filled } => {
                let v3 = |p: &Point| Vector3::new(p.x, p.y, 0.0);
                let simple = loops.len() == 1 && (loops[0].len() == 3 || loops[0].len() == 4);
                if *filled && simple {
                    // ลำดับมุมที่เก็บใน DXF: 1, 2, 4, 3 (สลับสองตัวท้าย)
                    let l = &loops[0];
                    let sol = if l.len() == 3 {
                        Solid::triangle(v3(&l[0]), v3(&l[1]), v3(&l[2]))
                    } else {
                        Solid::new(v3(&l[0]), v3(&l[1]), v3(&l[3]), v3(&l[2]))
                    };
                    push(EntityType::Solid(sol))?;
                } else if *filled {
                    let mut h = Hatch::solid();
                    for l in loops {
                        let mut path = BoundaryPath::external();
                        path.add_edge(BoundaryEdge::Polyline(PolylineEdge::new(
                            l.iter().map(|p| Vector2::new(p.x, p.y)).collect(),
                            true,
                        )));
                        h.add_path(path);
                    }
                    push(EntityType::Hatch(Box::new(h)))?;
                } else {
                    for l in loops {
                        let mut p = LwPolyline::new();
                        p.vertices = l.iter().map(|q| LwVertex::from_coords(q.x, q.y)).collect();
                        p.is_closed = true;
                        push(EntityType::LwPolyline(p))?;
                    }
                }
            }
            ElementData::Hatch { loops, spec } => {
                // DXF ที่เราเขียนยังไม่มี pattern → บันทึกเป็นเส้นลายแต่ละเส้น (LINE)
                match crate::hatch::pattern_segments(loops, spec, None) {
                    Some(segs) => {
                        for (a, b) in segs {
                            push(line(a, b))?;
                        }
                    }
                    None => eprintln!("Hatch is too dense to save as lines; skipped"),
                }
            }
            ElementData::Text { pos, height, rotation, text, halign, valign, width } => push(
                crate::text::to_dxf_entity(*pos, *height, *rotation, text, *halign, *valign, *width),
            )?,
            ElementData::Polyline { points, closed } => {
                let mut p = LwPolyline::new();
                p.vertices = points.iter().map(|q| LwVertex::from_coords(q.x, q.y)).collect();
                p.is_closed = *closed;
                push(EntityType::LwPolyline(p))?
            }
        }
    }

    crate::layout::write_layouts(&mut doc, &data.layouts)?;
    layout_ui::write_paper_items(&mut doc, &data.layouts)?;

    DxfWriter::new(&doc).write_to_vec().map_err(|e| e.to_string())
}
