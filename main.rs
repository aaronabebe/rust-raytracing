mod vec;
mod ray;
mod sphere;
mod hit;
mod camera;

use gtk::prelude::*;
use std::io::{stderr, Write};
use gtk::{Application, DrawingArea};
use vec::{Vec3, Color, Point3};
use ray::Ray;
use sphere::Sphere;
use hit::{Hit, World};
use camera::Camera;

fn ray_color(r: &Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(r, 0.0, f64::INFINITY) {
        0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))
    } else {
        // background
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn render(cr: &cairo::Context, width: u64, height: u64, aspect_ratio: f64) {
    let camera = Camera::new(aspect_ratio, 2.0, 1.0);

    let mut world = World::new();
    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -0.0), 100.0)));

    for j in 0..height {
        eprint!("\rScanlines remaining: {:3}", height - j - 1);
        stderr().flush().unwrap();

        // +1 to get rid of weird white line on the edge
        // dont really know why this appears
        for i in 0..(width + 1) {
            let u = (i as f64) / ((width - 1) as f64);
            let v = (j as f64) / ((height - 1) as f64);

            let r = camera.get_ray(u, v);
            let px = ray_color(&r, &world);

            cr.rectangle((width - i) as f64, (height - j) as f64, 1.0, 1.0);
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
    const width: u64 = 720;
    const height: u64 = ((720 as f64) / aspect_ratio) as u64;

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

