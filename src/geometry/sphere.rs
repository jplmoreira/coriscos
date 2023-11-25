use crate::{
    component::ray::Ray,
    material::{MaterialRef, ScatterResult},
    math::Vector3,
};

use super::{HitRecord, Hittable, HittableRef};

pub struct Sphere {
    center: Vector3,
    radius: f64,
    material: MaterialRef,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64, material: MaterialRef) -> HittableRef {
        Box::new(Self {
            center,
            radius,
            material,
        })
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, idx: usize) -> Option<HitRecord> {
        let t_min = 0.001;
        let t_max = f64::INFINITY;
        let oc = ray.origin() - self.center;
        let a = ray.direction().quadrance();
        let half_b = oc.dot(&ray.direction());
        let c = oc.quadrance() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root <= t_min || root >= t_max {
            root = (-half_b + sqrtd) / a;
            if root <= t_min || root >= t_max {
                return None;
            }
        }
        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;
        let front = ray.direction().dot(&normal) < 0.0;
        Some(HitRecord {
            point,
            normal: if front { normal } else { -normal },
            direction: ray.direction(),
            t: root,
            front,
            idx,
        })
    }

    fn scatter(&self, record: HitRecord) -> ScatterResult {
        self.material.scatter(record)
    }
}

unsafe impl Send for Sphere {}
unsafe impl Sync for Sphere {}
