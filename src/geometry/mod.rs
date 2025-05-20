use crate::component::{hit::Hit, ray::Ray};

pub mod sphere;

pub trait Hittable: Send + Sync + 'static {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}

pub type HittableRef = Box<dyn Hittable>;
