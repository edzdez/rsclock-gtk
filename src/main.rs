use gtk::prelude::*;
use chrono::Local;
use gtk::{Application, ApplicationWindow, Label, Justification, BaselinePosition, Orientation, Box, DrawingArea};
use gtk::glib;

fn get_current_time() -> String {
    return format!("{}", Local::now().format("%H:%M:%S"));
}

fn get_current_date() -> String {
    return format!("{}", Local::now().format("%Y/%m/%d"));
}

fn main() {
    let app = Application::builder()
        .application_id("com.edzdez.gtkclock")
        .build();

    app.connect_activate(|app| {
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(800)
            .default_height(600)
            .title("Clock")
            .build();

        let time_label = Label::builder()
            .label(&get_current_time())
            .justify(Justification::Fill)
            .build();
        let date_label = Label::builder()
            .label(&get_current_date())
            .justify(Justification::Fill)
            .build();

        let window_box = Box::builder()
            .orientation(Orientation::Vertical)
            .baseline_position(BaselinePosition::Center)
            .build();

        // TODO: use cairo to make the text for date and time scale with the window
        let date_time_box = Box::builder()
            .orientation(Orientation::Vertical)
            .baseline_position(BaselinePosition::Center)
            .build();

        date_time_box.pack_start(&time_label, false, true, 0);
        date_time_box.pack_start(&date_label, false, true, 0);

        window_box.pack_end(&date_time_box, true, false, 0);

        win.add(&window_box);
        win.show_all();

        let tick = move || {
            let time = get_current_time();
            time_label.set_text(&time);

            let date = get_current_date();
            date_label.set_text(&date);
            glib::Continue(true)
        };

        glib::timeout_add_seconds_local(1, tick);
    });

    app.run();
}
