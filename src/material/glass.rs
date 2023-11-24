use rand::Rng;

use crate::{
    component::{ray::Ray, world::HitRecord},
    math::Vector3,
};

use super::Material;

pub struct Glass {
    refraction_index: f64,
}

impl Glass {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Glass {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Vector3)> {
        let refraction_ratio = if record.front {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray.direction().normalize();
        let cos_theta = (-unit_direction).dot(&record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let mut rng = rand::thread_rng();
        let direction = if refraction_ratio * sin_theta > 1.0
            || Self::reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>()
        {
            unit_direction.reflect(&record.normal)
        } else {
            unit_direction.refract(&record.normal, refraction_ratio, cos_theta)
        };
        Some((
            Ray::new(record.point, direction),
            Vector3::new(1.0, 1.0, 1.0),
        ))
    }
}