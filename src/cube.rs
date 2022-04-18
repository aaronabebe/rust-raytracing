use super::vec::{Point3};
use super::ray::Ray;
use super::hit::{Hit, HitRecord};
use std::cmp::{min, max};

pub struct Cube {
    x_min: Point3,
    x_max: Point3,
    y_min: Point3,
    y_max: Point3,
}

impl Cube {
    pub fn new(x_min: Point3, x_max: Point3, y_min: Point3, y_max: Point3) -> Cube {
        Cube {
            x_min: x_min, 
            x_max: x_max, 
            y_min: y_min, 
            y_max: y_max
        }
    }
}

impl Hit for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if r.normal().x() != 0.0 {
            let tx1 = (self.x_min.x() - r.origin().x()) / r.normal().x();
            let tx2 = (self.x_max.x() - r.origin().x()) / r.normal().x();

            t_min = max(t_min, min(tx1, tx2));
            t_max = min(t_max, max(tx1, tx2));
        }
        if r.normal().y() != 0.0 {
            let ty1 = (self.y_min.y() - r.origin().y()) / r.normal().y();
            let ty2 = (self.y_max.y() - r.origin().y()) / r.normal().y();

            t_min = max(t_min, min(ty1, ty2));
            t_max = min(t_max, max(ty1, ty2));
        }

        if t_max >= t_min {
            let mut rec = HitRecord {
                t: self.t_min,
                p: r.at(t_min),
                normal: Vec3::new(0.0, 0.0, 0.0),
                front_face: false
            }
            Some(rec)
        } else {
            None
        }
    }
}



