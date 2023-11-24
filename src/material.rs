use std::sync::Arc;

use crate::{
    component::{ray::Ray, world::HitRecord},
    math::Vector3,
};

pub mod glass;
pub mod lambert;
pub mod metal;

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Vector3)>;
}

pub type MaterialRef = Arc<dyn Material + Send + Sync>;
