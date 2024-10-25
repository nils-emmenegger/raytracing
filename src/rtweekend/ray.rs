use super::Vector3;

#[derive(Clone, Debug, Default)]
pub struct Ray {
    orig: Vector3<f64>,
    dir: Vector3<f64>,
}

impl Ray {
    pub fn new(orig: &Vector3<f64>, dir: &Vector3<f64>) -> Self {
        Self {
            orig: orig.clone(),
            dir: dir.clone(),
        }
    }

    pub fn orig(&self) -> &Vector3<f64> {
        &self.orig
    }

    pub fn dir(&self) -> &Vector3<f64> {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Vector3<f64> {
        &self.orig + &(t * &self.dir)
    }
}
