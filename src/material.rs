use crate::{component::ray::Ray, geometry::HitRecord, math::Vector3};

pub mod diffuse_light;
pub mod glass;
pub mod lambert;
pub mod metal;

pub struct ScatterResult {
    pub t: f64,
    pub ray: Ray,
    pub attenuation: Vector3,
}

pub trait Material {
    fn scatter(&self, record: &HitRecord) -> Option<ScatterResult>;
    fn emit(&self, _record: &HitRecord) -> Vector3 {
        return Vector3::new(0.0, 0.0, 0.0);
    }
}

pub type MaterialRef = Box<dyn Material>;
