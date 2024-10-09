use super::{random_double, random_double_in};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Clone, Debug, Default)]
pub struct Vec3([f64; 3]);

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self([e0, e1, e2])
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(self.0.map(<_>::neg))
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..self.0.len() {
            self.0[i] += rhs[i];
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        for i in 0..self.0.len() {
            self.0[i] *= rhs;
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.0.iter().map(|x| x * x).sum()
    }

    pub fn near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        self.0.iter().all(|&x| x.abs() < S)
    }

    pub fn random() -> Self {
        Self([random_double(), random_double(), random_double()])
    }

    pub fn random_in(min: f64, max: f64) -> Self {
        Self([
            random_double_in(min, max),
            random_double_in(min, max),
            random_double_in(min, max),
        ])
    }
}

pub type Point3 = Vec3;

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        let mut res = [0.0; 3];
        for i in 0..self.0.len() {
            res[i] = self.0[i] + rhs.0[i];
        }
        Vec3(res)
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut res = [0.0; 3];
        for i in 0..self.0.len() {
            res[i] = self.0[i] - rhs.0[i];
        }
        Vec3(res)
    }
}

impl Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = [0.0; 3];
        for i in 0..self.0.len() {
            res[i] = self.0[i] * rhs.0[i];
        }
        Vec3(res)
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        let mut res = [0.0; 3];
        for i in 0..rhs.0.len() {
            res[i] = self * rhs.0[i];
        }
        Vec3(res)
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    let mut ret = 0.0;
    for i in 0..u.0.len() {
        ret += u.0[i] * v.0[i];
    }
    ret
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3([
        u.0[1] * v.0[2] - u.0[2] * v.0[1],
        u.0[2] * v.0[0] - u.0[0] * v.0[2],
        u.0[0] * v.0[1] - u.0[1] * v.0[0],
    ])
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_double_in(-1.0, 1.0),
            random_double_in(-1.0, 1.0),
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_in(-1.0, 1.0);
        let lensq = p.length_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            return &p / lensq.sqrt();
        }
    }
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(&on_unit_sphere, normal) > 0.0 {
        on_unit_sphere
    } else {
        -&on_unit_sphere
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - &(2.0 * dot(v, n) * n)
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * &(uv + &(cos_theta * n));
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;
    &r_out_perp + &r_out_parallel
}
