use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use rand::Rng;

pub fn rand_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn rand_range_f64(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

#[derive(Clone, Copy)]
pub struct Vector3 {
    elems: [f64; 3],
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { elems: [x, y, z] }
    }

    pub fn random(min: f64, max: f64) -> Self {
        Self::new(
            rand_range_f64(min, max),
            rand_range_f64(min, max),
            rand_range_f64(min, max),
        )
    }

    pub fn random_in_region(region: [f64; 3]) -> Self {
        loop {
            let v = Self::new(
                rand_range_f64(-1.0, 1.0) * region[0],
                rand_range_f64(-1.0, 1.0) * region[1],
                rand_range_f64(-1.0, 1.0) * region[2],
            );
            if v.quadrance() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_unit() -> Self {
        Self::random_in_region([1.0, 1.0, 1.0]).normalize()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.elems[0] * rhs.elems[0] + self.elems[1] * rhs.elems[1] + self.elems[2] * rhs.elems[2]
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            elems: [
                self.elems[1] * rhs.elems[2] - self.elems[2] * rhs.elems[1],
                self.elems[2] * rhs.elems[0] - self.elems[0] * rhs.elems[2],
                self.elems[0] * rhs.elems[1] - self.elems[1] * rhs.elems[0],
            ],
        }
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - 2.0 * self.dot(normal) * *normal
    }

    pub fn refract(&self, normal: &Vector3, eta_over_prime: f64, cos_theta: f64) -> Self {
        let r_perp = eta_over_prime * (*self + cos_theta * *normal);
        let r_parallel = -(1.0 - r_perp.quadrance()).abs().sqrt() * *normal;
        r_perp + r_parallel
    }

    pub fn x(&self) -> f64 {
        self.elems[0]
    }

    pub fn y(&self) -> f64 {
        self.elems[1]
    }

    pub fn z(&self) -> f64 {
        self.elems[2]
    }

    pub fn quadrance(&self) -> f64 {
        self.dot(self)
    }

    pub fn len(&self) -> f64 {
        self.quadrance().sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.len()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.elems[0].abs() < s) && (self.elems[1].abs() < s) && (self.elems[2].abs() < s)
    }

    pub fn to_color(self) -> Vec<u8> {
        vec![
            (self.x().clamp(0.0, 1.0).sqrt() * 255.999) as u8,
            (self.y().clamp(0.0, 1.0).sqrt() * 255.999) as u8,
            (self.z().clamp(0.0, 1.0).sqrt() * 255.999) as u8,
        ]
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Vector3) -> Self::Output {
        Self {
            elems: [
                self.elems[0] + rhs.elems[0],
                self.elems[1] + rhs.elems[1],
                self.elems[2] + rhs.elems[2],
            ],
        }
    }
}

impl Add<f64> for Vector3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            elems: [
                self.elems[0] + rhs,
                self.elems[1] + rhs,
                self.elems[2] + rhs,
            ],
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            elems: [
                self.elems[0] - rhs.elems[0],
                self.elems[1] - rhs.elems[1],
                self.elems[2] - rhs.elems[2],
            ],
        }
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            elems: [-self.elems[0], -self.elems[1], -self.elems[2]],
        }
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            elems: [
                self.elems[0] * rhs.x(),
                self.elems[1] * rhs.y(),
                self.elems[2] * rhs.z(),
            ],
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            elems: [
                self.elems[0] * rhs,
                self.elems[1] * rhs,
                self.elems[2] * rhs,
            ],
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            elems: [
                self.elems[0] / rhs,
                self.elems[1] / rhs,
                self.elems[2] / rhs,
            ],
        }
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self * rhs
    }
}
