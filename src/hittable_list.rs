use crate::hittable::{HitRecord, Hittable};
use crate::rtweekend::*;

#[derive(Clone, Default)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, mut ray_t: Interval) -> Option<HitRecord> {
        let mut closest_hit_record = None;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(r, ray_t.clone()) {
                ray_t.end = rec.t;
                closest_hit_record = Some(rec);
            }
        }

        closest_hit_record
    }
}
