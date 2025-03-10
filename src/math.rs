use rand::Rng;

pub fn rand_f64() -> f64 {
    let mut rng = rand::rng();
    rng.random()
}

pub fn rand_range_f64(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}

#[derive(Clone)]
pub struct Vector3 {
    elems: [f64; 3],
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { elems: [x, y, z] }
    }

    pub fn add(&self, rhs: &Self) -> Self {
        Self {
            elems: [
                self.elems[0] + rhs.elems[0],
                self.elems[1] + rhs.elems[1],
                self.elems[2] + rhs.elems[2],
            ],
        }
    }

    pub fn _to(&self, rhs: f64) -> Self {
        Self {
            elems: [
                self.elems[0] + rhs,
                self.elems[1] + rhs,
                self.elems[2] + rhs,
            ],
        }
    }

    pub fn sub(&self, rhs: &Self) -> Self {
        Self {
            elems: [
                self.elems[0] - rhs.elems[0],
                self.elems[1] - rhs.elems[1],
                self.elems[2] - rhs.elems[2],
            ],
        }
    }

    pub fn neg(&self) -> Self {
        Self {
            elems: [-self.elems[0], -self.elems[1], -self.elems[2]],
        }
    }

    pub fn mul(&self, rhs: &Self) -> Self {
        Self {
            elems: [
                self.elems[0] * rhs.x(),
                self.elems[1] * rhs.y(),
                self.elems[2] * rhs.z(),
            ],
        }
    }

    pub fn extend(&self, rhs: f64) -> Self {
        Self {
            elems: [
                self.elems[0] * rhs,
                self.elems[1] * rhs,
                self.elems[2] * rhs,
            ],
        }
    }

    pub fn reduce(&self, rhs: f64) -> Self {
        Self {
            elems: [
                self.elems[0] / rhs,
                self.elems[1] / rhs,
                self.elems[2] / rhs,
            ],
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
        self.sub(&normal.extend(2.0 * self.dot(normal)))
    }

    pub fn refract(&self, normal: &Vector3, eta_over_prime: f64, cos_theta: f64) -> Self {
        let r_perp = self.add(&normal.extend(cos_theta)).extend(eta_over_prime);
        let r_parallel = normal.extend(-(1.0 - r_perp.quadrance()).abs().sqrt());
        r_perp.add(&r_parallel)
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

    pub fn normal(&self) -> Self {
        self.reduce(self.len())
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
