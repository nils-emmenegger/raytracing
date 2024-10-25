use crate::{material::Material, rtweekend::*};
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub mat: Option<Rc<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3<f64>) {
        // outward_normal is assumed to have unit length
        self.front_face = r.dir().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal.clone()
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
