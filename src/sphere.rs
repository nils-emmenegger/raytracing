use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    rtweekend::*,
};
use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Derive from (C - P) * (C - P) = r^2
        // where C = (Cx, Cy, Cz) and P = Q + d t
        // i.e. C is the sphere center and P is ray at origin Q with direction d
        // It is a quadratic equation that we further simplify with b = -2h
        let oc = self.center - r.orig();
        let a = r.dir().norm_squared();
        let h = r.dir().dot(&oc);
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // find closest root in range
        let mut root = (h - sqrtd) / a;
        if !ray_t.contains(&root) {
            root = (h + sqrtd) / a;
            if !ray_t.contains(&root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(Rc::clone(&self.mat));

        true
    }
}
