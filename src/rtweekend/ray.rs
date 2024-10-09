use super::{Point3, Vec3};

#[derive(Clone, Debug, Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: &Point3, dir: &Vec3) -> Self {
        Self {
            orig: orig.clone(),
            dir: dir.clone(),
        }
    }

    pub fn orig(&self) -> &Point3 {
        &self.orig
    }

    pub fn dir(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        &self.orig + &(t * &self.dir)
    }
}
