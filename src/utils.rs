use rand::Rng;

pub fn rand_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn rand_range_f64(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub mod vec3 {
    use nalgebra::{vector, Vector3};

    pub fn random_in_region(region: [f64; 3]) -> Vector3<f64> {
        loop {
            let v = vector![
                super::rand_range_f64(-1.0, 1.0) * region[0],
                super::rand_range_f64(-1.0, 1.0) * region[1],
                super::rand_range_f64(-1.0, 1.0) * region[2],
            ];
            if v.norm_squared() < 1.0 {
                return v;
            }
        }
    }

    pub fn random_unit() -> Vector3<f64> {
        random_in_region([1.0, 1.0, 1.0]).normalize()
    }

    pub fn random(min: f64, max: f64) -> Vector3<f64> {
        vector![
            super::rand_range_f64(min, max),
            super::rand_range_f64(min, max),
            super::rand_range_f64(min, max),
        ]
    }

    pub fn val_multiply(rhs: Vector3<f64>, lhs: Vector3<f64>) -> Vector3<f64> {
        vector![rhs.x * lhs.x, rhs.y * lhs.y, rhs.y * lhs.y]
    }

    pub fn near_zero(vector: &Vector3<f64>) -> bool {
        let s = 1e-8;
        (vector.x.abs() < s) && (vector.y.abs() < s) && (vector.z.abs() < s)
    }

    pub fn reflect(vector: &Vector3<f64>, normal: &Vector3<f64>) -> Vector3<f64> {
        *vector - 2.0 * vector.dot(normal) * *normal
    }

    pub fn refract(
        vector: &Vector3<f64>,
        normal: &Vector3<f64>,
        eta_over_prime: f64,
        cos_theta: f64,
    ) -> Vector3<f64> {
        let r_perp = eta_over_prime * (*vector + cos_theta * *normal);
        let r_parallel = -(1.0 - r_perp.norm_squared()).abs().sqrt() * *normal;
        r_perp + r_parallel
    }

    pub fn to_color(vector: Vector3<f64>) -> Vec<u8> {
        vec![
            (vector.x.clamp(0.0, 1.0).sqrt() * 255.999) as u8,
            (vector.y.clamp(0.0, 1.0).sqrt() * 255.999) as u8,
            (vector.z.clamp(0.0, 1.0).sqrt() * 255.999) as u8,
        ]
    }
}
