use std::sync::Arc;

use crate::{
    component::{
        hit::{Hit, HitRecord},
        ray::Ray,
    },
    material::Material,
    math::Vector3,
};

use super::Hittable;

pub struct Sphere<M: Material> {
    center: Vector3,
    radius: f64,
    material: Arc<M>,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vector3, radius: f64, material: M) -> Box<Self> {
        Box::new(Self {
            center,
            radius,
            material: Arc::new(material),
        })
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let t_min = 0.001;
        let t_max = f64::INFINITY;

        let oc = &ray.origin - &self.center;
        let a = ray.direction.quadrance();
        let half_b = oc.dot(&ray.direction);
        let c = oc.quadrance() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let sqrtd = discriminant.sqrt();
            let mut root = (-half_b - sqrtd) / a;
            if root <= t_min || root >= t_max {
                root = (-half_b + sqrtd) / a;
                if root <= t_min || root >= t_max {
                    return None;
                }
            }
            let point = ray.at(root);
            let normal = (&point - &self.center) / self.radius;
            let front = ray.direction.dot(&normal) < 0.0;

            let record = HitRecord {
                point,
                normal: if front { normal } else { -normal },
                direction: ray.direction.clone(),
                t: root,
                front,
            };
            return Some(Hit {
                record,
                material: self.material.clone(),
            });
        }

        None
    }
}
