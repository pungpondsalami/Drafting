/* application.rs
 *
 * Copyright 2026 Supakit Suptorranee
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
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

use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::DraftingWindow;
use crate::config::VERSION;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct DraftingApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for DraftingApplication {
        const NAME: &'static str = "DraftingApplication";
        type Type = super::DraftingApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for DraftingApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<control>q"]);
        }
    }

    impl ApplicationImpl for DraftingApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.obj();

            // โหลด CSS — drafting-floating บังคับโปร่งทับแคนวาส (Yaru/OS บางตัวทำ osd/toolbar ทึบ)
            let provider = gtk::CssProvider::new();
            provider.load_from_string(r#"
                stack {
                    transition: min-width 300ms cubic-bezier(0.34, 1.56, 0.64, 1),
                                min-height 300ms cubic-bezier(0.34, 1.56, 0.64, 1);
                }

                .drafting-floating {
                    background-color: alpha(#2d2d2d, 0.68);
                    background-image: none;
                    color: #f6f5f4;
                    border-radius: 18px;
                    padding: 8px;
                    border: 1px solid alpha(white, 0.12);
                    box-shadow: 0 3px 16px alpha(black, 0.45);
                }

                .drafting-floating label {
                    color: #f6f5f4;
                }

                /* ปุ่ม suggested-action พื้นหลังสว่าง — ตัวหนังสือดำ (override กฎด้านบนเพราะ specificity สูงกว่า) */
                .drafting-floating button.drafting-apply-array,
                .drafting-floating button.drafting-apply-array label {
                    color: #1c1c1c;
                }

                .drafting-floating separator {
                    background: alpha(white, 0.18);
                    min-width: 1px;
                    min-height: 1px;
                }

                .drafting-floating entry {
                    background-color: alpha(black, 0.35);
                    color: #f6f5f4;
                    border: 1px solid alpha(white, 0.14);
                    border-radius: 10px;
                }
                /* ลดขนาดปุ่ม Window Controls ให้เล็กกะทัดรัด */
                windowcontrols button {
                    min-width: 26px;
                    min-height: 26px;
                    padding: 0;
                    margin: 0 2px;
                }

                .toolbar windowcontrols {
                    padding: 4px;
                }
            "#);
            gtk::style_context_add_provider_for_display(
                &gtk::gdk::Display::default().unwrap(),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_USER,
            );

            let window = application.active_window().unwrap_or_else(|| {
                let window = DraftingWindow::new(&*application);
                window.upcast()
            });

            window.present();
        }
    }

    impl GtkApplicationImpl for DraftingApplication {}
    impl AdwApplicationImpl for DraftingApplication {}
}

glib::wrapper! {
    pub struct DraftingApplication(ObjectSubclass<imp::DraftingApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl DraftingApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .property("resource-base-path", "/com/pungpondsalami/drafting")
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        self.add_action_entries([quit_action, about_action]);
    }

    fn show_about(&self) {
        // 1. ใช้ if let แทน unwrap เพื่อความปลอดภัย
        let window = self.active_window();

        // 2. สร้าง Dialog แบบไม่ต้องใส่ไอคอนแอป (ป้องกันหาไฟล์ไม่เจอแล้วเด้ง)
        let about = adw::AboutDialog::builder()
            .application_name("Drafting")
            .developer_name("Supakit Suptorranee")
            .version(VERSION)
            .copyright("© 2026 Supakit Suptorranee")
            .application_icon("com.pungpondsalami.drafting")
            .build();

        // 3. สั่งแสดงผล
        if let Some(parent) = window {
            about.present(Some(&parent));
        } else {
            about.present(Option::<&gtk::Window>::None);
        }
    }
}
