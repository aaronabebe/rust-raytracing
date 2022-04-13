mod vec;
mod ray;

use gtk::prelude::*;
use std::io::{stderr, Write};
use gtk::{Application, DrawingArea};
use vec::{Vec3, Color, Point3};
use ray::Ray;

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 0.5, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}


fn render(cr: &cairo::Context, width: u64, height: u64, aspect_ratio: f64) {
    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0
                          - Vec3::new(0.0, 0.0, focal_length);

    for j in (0..height).rev() {
        eprint!("\rScanlines remaining: {:3}", height - j - 1);
        stderr().flush().unwrap();

        for i in 0..width {
            let u = (i as f64) / ((width - 1) as f64);
            let v = (j as f64) / ((height - 1) as f64);

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin
            );
            let px = ray_color(&r);

            cr.rectangle(i as f64, j as f64, 1.0, 1.0);
            cr.set_source_rgb(px.x(), px.y(), px.z());
            cr.fill().expect("Fill did not work!");
        }
    }
    eprintln!("Done.");
}


fn build_ui(app: &Application) {
    let window = gtk::ApplicationWindow::new(app);
    let drawing_area = Box::new(DrawingArea::new)();

    // Image
    const aspect_ratio: f64 = 16.0 / 9.0;
    const width: u64 = 540;
    const height: u64 = ((540 as f64) / aspect_ratio) as u64;

    window.set_default_size(width as i32, height as i32);

    drawing_area.set_draw_func(move |_, cr, _, _| {
        render(cr, width, height, aspect_ratio);
    });
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

