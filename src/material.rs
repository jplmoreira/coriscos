use crate::{component::ray::Ray, geometry::HitRecord, math::Vector3};

pub mod glass;
pub mod lambert;
pub mod metal;

pub struct ScatterResult {
    pub t: f64,
    pub ray: Ray,
    pub attenuation: Vector3,
}

pub trait Material {
    fn scatter(&self, record: HitRecord) -> ScatterResult;
}

pub type MaterialRef = Box<dyn Material>;
