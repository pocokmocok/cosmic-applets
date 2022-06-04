use gtk4::{glib, prelude::*};

mod dbus_service;
mod deref_cell;
mod status_area;
mod status_menu;
mod status_notifier_watcher;

use status_area::StatusArea;

fn main() {
    gtk4::init().unwrap();

    // XXX Implement DBus service somewhere other than applet?
    glib::MainContext::default().spawn_local(status_notifier_watcher::start());

    let status_area = StatusArea::new();
    gtk4::Window::builder()
        .decorated(false)
        .child(&status_area)
        .resizable(false)
        .width_request(1)
        .height_request(1)
        .build()
        .show();

    let main_loop = glib::MainLoop::new(None, false);
    main_loop.run();
}
