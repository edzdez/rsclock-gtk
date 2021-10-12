use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, Justification, BaselinePosition, Orientation, Box};
use gtk::glib;
use gtk::pango;
use chrono::Local;

fn get_current_time() -> String {
    return format!("{}", Local::now().format("%H:%M:%S"));
}

fn get_current_date() -> String {
    return format!("{}", Local::now().format("%Y/%m/%d"));
}

fn build_ui(app: &Application) {
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

        let date_time_box = Box::builder()
            .orientation(Orientation::Vertical)
            .baseline_position(BaselinePosition::Center)
            .build();

        // TODO: find out how to change the scale dynamically
        // maybe use gdk::EventConfigure and pango::AttrList::change
        let time_attr_list = pango::AttrList::new();
        let mut attr = pango::Attribute::new_scale(10.);
        attr.set_start_index(0);
        time_attr_list.insert(attr);
        use pango::Weight;
        let mut attr = pango::Attribute::new_weight(Weight::Bold);
        attr.set_start_index(0);
        time_attr_list.insert(attr);
        
        time_label.set_attributes(Some(&time_attr_list));

        let date_attr_list = pango::AttrList::new();
        let mut attr = pango::Attribute::new_scale(2.);
        attr.set_start_index(0);
        date_attr_list.insert(attr);

        date_label.set_attributes(Some(&date_attr_list));

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
}

fn main() {
    let app = Application::builder()
        .application_id("com.edzdez.gtkclock")
        .build();

    app.connect_activate(build_ui);

    app.run();
}
