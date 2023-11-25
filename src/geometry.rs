use crate::{component::ray::Ray, material::ScatterResult, math::Vector3};

pub mod sphere;

pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub direction: Vector3,
    pub t: f64,
    pub front: bool,
    pub idx: usize,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, idx: usize) -> Option<HitRecord>;
    fn scatter(&self, record: HitRecord) -> ScatterResult;
}

pub type HittableRef = Box<dyn Hittable + Send + Sync>;
