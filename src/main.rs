/* main.rs
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

mod application;
mod config;
mod window;
mod engine;
mod spatial;

use self::application::DraftingApplication;
use self::window::DraftingWindow;

#[cfg(not(windows))]
use config::{GETTEXT_PACKAGE, LOCALEDIR};
use gtk::{gio, glib};
use gtk::prelude::*;
#[cfg(not(windows))]
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};

fn main() -> glib::ExitCode {
    // Set up gettext translations (Linux/Mac only)
    #[cfg(not(windows))]
    {
        bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
        bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8")
            .expect("Unable to set the text domain encoding");
        textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");
    }

    // Load resources
    let res_bytes = include_bytes!(concat!(env!("OUT_DIR"), "/drafting.gresource"));
    let resource = gio::Resource::from_data(&glib::Bytes::from(&res_bytes[..]))
        .expect("Failed to load gresource from data");
    gio::resources_register(&resource);
    // Create a new GtkApplication. The application manages our main loop,
    // application windows, integration with the window manager/compositor, and
    // desktop features such as file opening and single-instance applications.

    // Run the application. This function will block until the application
    // exits. Upon return, we have our exit code to return to the shell. (This
    // is the code you see when you do `echo $?` after running a command in a
    // terminal.
    {
        let mut test_engine = crate::engine::Engine::new();
        test_engine.camera.target_scale = 2.0;
        println!("--- Engine Logic Check ---");
        test_engine.step(100_000); 
        println!("Scale after 0.1s: {:.4}", test_engine.camera.scale);
        println!("--- Logic OK, Launching UI... ---\n");
    }

    let app = DraftingApplication::new("com.pungpondsalami.drafting", &gio::ApplicationFlags::empty());
    app.run()
}