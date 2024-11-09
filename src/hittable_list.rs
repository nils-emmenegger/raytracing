use crate::hittable::{HitRecord, Hittable};
use crate::rtweekend::*;

#[derive(Clone, Default)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new(object: Rc<dyn Hittable>) -> Self {
        let mut list: HittableList = Default::default();
        list.add(object);
        list
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
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
