use crate::{
    component::{hit::HitRecord, ray::Ray},
    math::Vector3,
};

pub mod diffuse_light;
pub mod glass;
pub mod lambert;
pub mod metal;

pub struct ScatterResult {
    pub _t: f64,
    pub ray: Ray,
    pub attenuation: Vector3,
}

pub trait Material: Send + Sync + 'static {
    fn scatter(&self, _record: HitRecord) -> Option<ScatterResult> {
        None
    }
    fn emit(&self) -> Vector3 {
        Vector3::fill(0.0)
    }
}
