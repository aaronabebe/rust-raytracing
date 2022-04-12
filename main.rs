use gtk::prelude::*;
use gtk::{Application, DrawingArea};


fn build_ui(app: &Application) {
    let window = gtk::ApplicationWindow::new(app);
    let drawing_area = Box::new(DrawingArea::new)();
    let width = 500;
    let height = 500;

    drawing_area.set_draw_func(move |_, cr, _, _| {
        for j in (0..height).rev() {
            for i in 0..width {
                let r = (i as f64) / ((width - 1) as f64);
                let g = (j as f64) / ((height - 1) as f64);
                let b = 0.25;

                //println!("[{}|{}]: [{}, {}, {}]", i, j, r, g, b);
                cr.rectangle(i as f64, j as f64, 1.0, 1.0);
                cr.set_source_rgb(r, g, b);
                cr.fill()
                    .expect("Fill did not work!");
            }
        }
        //cr.set_source_rgb(0.5, 0.5, 1.0);
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


/*
fn main() -> () {
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = 256;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let g = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let b = 0.25;

            let ir = (255.999 * r) as u64;
            let ig = (255.999 * g) as u64;
            let ib = (255.999 * b) as u64;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
*/
