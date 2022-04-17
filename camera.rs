use super::vec::{Vec3, Point3};
use super::ray::Ray;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(
        aspect_ratio: f64, 
        viewport_height: f64, 
        focal_length: f64
    ) -> Camera {
        let viewport_width: f64 = aspect_ratio * viewport_height;
        let orig = Point3::new(0.0, 0.0, 0.0);
        let h = Vec3::new(viewport_width, 0.0, 0.0);
        let v = Vec3::new(0.0, viewport_height, 0.0);
        let llc = orig - h / 2.0 - v / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin: orig,
            horizontal: h, 
            vertical: v, 
            lower_left_corner: llc
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin, 
            self.lower_left_corner + u * self.horizontal + 
            v * self.vertical - self.origin)
    }
}
