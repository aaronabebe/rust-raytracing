use gtk::prelude::*;
use gtk::{Application, DrawingArea};


fn draw_gradient(cr: &cairo::Context, width: u32, height: u32) {
    for j in (0..height).rev() {
        for i in 0..width {
            let r = (i as f64) / ((width - 1) as f64);
            let g = (j as f64) / ((height - 1) as f64);
            let b = 0.25;

            cr.rectangle(i as f64, j as f64, 1.0, 1.0);
            cr.set_source_rgb(r, g, b);
            cr.fill().expect("Fill did not work!");
        }
    }
}


fn build_ui(app: &Application) {
    let window = gtk::ApplicationWindow::new(app);
    let drawing_area = Box::new(DrawingArea::new)();
    let width = 500;
    let height = 500;

    drawing_area.set_draw_func(move |_, cr, _, _| {
        draw_gradient(cr, width, height);
    });
    window.set_default_size(width as i32, height as i32);
    window.set_child(Some(&drawing_area));
    window.present();
}

fn main() {
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();
    app.connect_activate(build_ui);
    app.run();
}

