use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};
use gtk4 as gtk;

fn main() {
    let app = Application::builder()
        .application_id("com.github.zapkube.todo")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Click me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .width_request(120)
        .height_request(80)
        .build();
    button.connect_clicked(|btn| {
        btn.set_label("Click again!");
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(320)
        .default_height(200)
        .title("Hello, World!")
        .child(&button)
        .build();
    window.present();
}
