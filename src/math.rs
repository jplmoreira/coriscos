use std::ops;

use rand::Rng;
use serde::Deserialize;

pub fn rand_f64() -> f64 {
    let mut rng = rand::rng();
    rng.random()
}

pub fn rand_range_f64(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}

#[derive(Clone, Debug, Deserialize)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn fill(val: f64) -> Self {
        Self {
            x: val,
            y: val,
            z: val,
        }
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
        Self::random_in_region([1.0, 1.0, 1.0]).normal()
    }

    #[inline]
    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    pub fn cross(&self, rhs: &Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    #[inline]
    pub fn reflect(&self, normal: &Self) -> Self {
        self - normal * (2.0 * self.dot(normal))
    }

    #[inline]
    pub fn refract(&self, normal: &Self, eta_over_prime: f64, cos_theta: f64) -> Self {
        let r_perp = (self + (normal * cos_theta)) * eta_over_prime;
        let r_parallel = normal * -(1.0 - r_perp.quadrance()).abs().sqrt();
        r_perp + r_parallel
    }

    #[inline]
    pub fn quadrance(&self) -> f64 {
        self.dot(self)
    }

    #[inline]
    pub fn len(&self) -> f64 {
        self.quadrance().sqrt()
    }

    #[inline]
    pub fn normal(&self) -> Self {
        self / self.len()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn to_color(self) -> Vec<u8> {
        vec![
            (self.x.clamp(0.0, 1.0).sqrt() * 255.999) as u8,
            (self.y.clamp(0.0, 1.0).sqrt() * 255.999) as u8,
            (self.z.clamp(0.0, 1.0).sqrt() * 255.999) as u8,
        ]
    }
}

macro_rules! impl_math_vec3 {
    (impl $tr:ident as $f:ident & $op:tt) => {
        impl_math_generic!(impl $tr for Vector3 as $f & $op use |a, b| {
            Vector3::new(a.x $op b.x,a.y $op b.y, a.z $op b.z)
        });
        impl_math_into_generic!(impl $tr for Vector3 as $f & $op use |a, b| {
            a $op Vector3::fill(b)
        } in f64);
    };
}

macro_rules! impl_math_generic {
    (impl $tr:ident for $name:ident as $f:ident & $op:tt use |$lhs:tt, $rhs:tt| { $ex:expr }) => {
        impl ops::$tr for &$name {
            type Output = $name;
            #[inline]
            fn $f(self, $rhs: Self) -> Self::Output {
                let $lhs = self;
                $ex
            }
        }
        impl ops::$tr<$name> for &$name {
            type Output = $name;
            #[inline]
            fn $f(self, $rhs: $name) -> Self::Output {
                self $op &$rhs
            }
        }
        impl ops::$tr<&$name> for $name {
            type Output = Self;
            #[inline]
            fn $f(self, $rhs: &$name) -> Self::Output {
                &self $op $rhs
            }
        }
        impl ops::$tr for $name {
            type Output = Self;
            #[inline]
            fn $f(self, $rhs: Self) -> Self::Output {
                &self $op &$rhs
            }
        }
    };
}

macro_rules! impl_math_into_generic {
    (impl $tr:ident for $name:ident as $f:ident & $op:tt use |$lhs:tt, $rhs:tt| { $ex:expr } in $into:ty) => {
        impl<T: Into<$into>> ops::$tr<T> for &$name {
            type Output = $name;
            #[inline]
            fn $f(self, $rhs: T) -> Self::Output {
                let $lhs = self;
                let $rhs = $rhs.into();
                $ex
            }
        }
        impl<T: Into<$into>> ops::$tr<T> for $name {
            type Output = Self;
            #[inline]
            fn $f(self, $rhs: T) -> Self::Output {
                &self $op $rhs
            }
        }
    };
}

impl_math_vec3!(impl Add as add & +);
impl_math_vec3!(impl Sub as sub & -);
impl_math_vec3!(impl Mul as mul & *);
impl_math_vec3!(impl Div as div & /);

impl ops::Neg for &Vector3 {
    type Output = Vector3;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Neg for Vector3 {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self::Output {
        -&self
    }
}
