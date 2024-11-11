use crate::{material::Material, rtweekend::*};

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vector3<f64>,
    pub front_face: bool,
    pub normal: Vector3<f64>,
    pub mat: Arc<dyn Material + Send + Sync>,
}

impl HitRecord {
    /// The normal in the returned HitRecord will always point against the incident Ray (it will flip outward_normal if needed)
    pub fn new(
        time_of_intersection: f64,
        intersection_point: Vector3<f64>,
        incident_ray: &Ray,
        outward_normal: Vector3<f64>,
        mat: Arc<dyn Material + Send + Sync>,
    ) -> Self {
        let front_face = incident_ray.dir().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            t: time_of_intersection,
            p: intersection_point,
            front_face,
            normal,
            mat,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
