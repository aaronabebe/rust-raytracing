mod vec;
mod ray;
mod sphere;
mod hit;
mod camera;
mod material;

use gtk::prelude::*;
use std::io::{stderr, Write};
use gtk::{Application, DrawingArea};
use vec::{Color, Point3};
use material::{Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use hit::{Hit, World};
use camera::Camera;
use rand::Rng;
use std::sync::Arc;
use rayon::prelude::*;

fn ray_color(r: &Ray, world: &World, depth: u64) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat.scatter(r, &rec) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        // background
        let unit_direction = r.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn render(
    cr: &cairo::Context, 
    width: u64, 
    height: u64, 
    aspect_ratio: f64, 
    samples: u64,
    max_depth: u64
) {
    let camera = Camera::new(aspect_ratio, 2.0, 1.0);

    let mut world = World::new();
    let mat_ground = Arc::new(Lambertian::new(Color::new(0.4, 0.1, 0.1)));
    let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -0.0), 100.0, mat_ground);
    world.push(Box::new(sphere_ground));

    let mat_center = Arc::new(Lambertian::new(Color::new(1.0, 0.6, 0.3)));
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    world.push(Box::new(sphere_center));

    let mat_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.2));
    let sphere_left = Sphere::new(Point3::new(0.8, 0.1, -0.8), 0.2, mat_left);
    world.push(Box::new(sphere_left));

    let mat_right = Arc::new(Metal::new(Color::new(0.8, 0.8, 1.0), 0.8));
    let sphere_right = Sphere::new(Point3::new(-0.8, 0.1, -0.8), 0.2, mat_right);
    world.push(Box::new(sphere_right));

    for j in 0..height {
        eprint!("\rScanlines remaining: {:3}", height - j + 1);
        stderr().flush().unwrap();

        // +1 to get rid of weird white line on the edge
        // dont really know why this appears
        let scanline: Vec<Color> = (0..(width + 1)).into_par_iter().map(|i| {
            let mut px = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let mut rng = rand::thread_rng();
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((width - 1) as f64);
                let v = ((j as f64) + random_v) / ((height - 1) as f64);

                let r = camera.get_ray(u, v);
                px += ray_color(&r, &world, max_depth);
            }

            px
        }).collect();

        for (i, px) in scanline.iter().enumerate() {
            cr.rectangle((width - i as u64) as f64, (height - j) as f64, 1.0, 1.0);
            let ir = (px.x() / (samples as f64)).sqrt().clamp(0.0, 0.999);
            let ig = (px.y() / (samples as f64)).sqrt().clamp(0.0, 0.999);
            let ib = (px.z() / (samples as f64)).sqrt().clamp(0.0, 0.999);
            cr.set_source_rgb(ir, ig, ib);
            cr.fill().expect("Fill did not work!");
        }
    }
    stderr().flush().unwrap();
    eprintln!("Done.");
}


fn build_ui(app: &Application) {
    let window = gtk::ApplicationWindow::new(app);
    let drawing_area = Box::new(DrawingArea::new)();

    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u64 = 720;
    const HEIGHT: u64 = ((720 as f64) / ASPECT_RATIO) as u64;
    const SAMPLES: u64 = 200;
    const MAX_DEPTH: u64 = 5;

    window.set_default_size(WIDTH as i32, HEIGHT as i32);

    drawing_area.set_draw_func(move |_, cr, _, _| {
        render(cr, WIDTH, HEIGHT, ASPECT_RATIO, SAMPLES, MAX_DEPTH);
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

