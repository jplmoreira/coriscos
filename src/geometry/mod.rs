use crate::{component::ray::Ray, material::Material, math::Vector3};

pub mod sphere;

pub struct HitRecord<'a> {
    pub point: Vector3,
    pub normal: Vector3,
    pub direction: Vector3,
    pub t: f64,
    pub front: bool,
    pub material: &'a dyn Material,
}

pub trait Hittable: Send + Sync + 'static {
    fn hit(&self, ray: &Ray) -> Option<HitRecord>;
}

pub type HittableRef = Box<dyn Hittable>;
