use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    rtweekend::*,
};

#[derive(Clone)]
pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    mat: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, mat: Arc<dyn Material + Send + Sync>) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
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
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // find closest root in range
        let mut root = (h - sqrtd) / a;
        if !ray_t.contains(&root) {
            root = (h + sqrtd) / a;
            if !ray_t.contains(&root) {
                return None;
            }
        }

        let intersection_point = r.at(root);
        let outward_normal = (intersection_point - self.center) / self.radius;
        Some(HitRecord::new(
            root,
            intersection_point,
            r,
            outward_normal,
            Arc::clone(&self.mat),
        ))
    }
}
