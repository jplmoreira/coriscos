use std::sync::Arc;

use crate::{
    geometry::sphere::Sphere,
    material::{glass::Glass, lambert::Lambert, metal::Metal, MaterialRef},
    math::{self, Vector3},
};

use super::ray::Ray;

pub struct HitRecord {
    pub point: Vector3,
    pub normal: Vector3,
    pub t: f64,
    pub front: bool,
    pub material: MaterialRef,
}

impl HitRecord {
    pub fn new(
        point: Vector3,
        normal: Vector3,
        t: f64,
        front: bool,
        material: MaterialRef,
    ) -> Self {
        Self {
            point,
            normal,
            t,
            front,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub type HittableRef = Box<dyn Hittable + Send + Sync>;

pub struct World {
    objects: Vec<HittableRef>,
}

impl World {
    pub fn build(_file: &str) -> Self {
        let mut objects: Vec<HittableRef> = Vec::new();

        let material_ground: MaterialRef = Arc::new(Lambert::new(Vector3::new(0.5, 0.5, 0.5)));
        objects.push(Box::new(Sphere::new(
            Vector3::new(0.0, -1000.0, 0.0),
            1000.0,
            material_ground,
        )));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat: f64 = math::rand_f64();
                let center = Vector3::new(
                    a as f64 + 0.9 * math::rand_f64(),
                    0.2,
                    b as f64 + 0.9 * math::rand_f64(),
                );

                if (center - Vector3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                    let material: MaterialRef = if choose_mat < 0.8 {
                        let albedo = Vector3::random(0.0, 1.0) * Vector3::random(0.0, 1.0);
                        Arc::new(Lambert::new(albedo))
                    } else if choose_mat < 0.95 {
                        let albedo = Vector3::random(0.5, 1.0);
                        let fuzz = math::rand_range_f64(0.0, 0.5);
                        Arc::new(Metal::new(albedo, fuzz))
                    } else {
                        Arc::new(Glass::new(1.5))
                    };
                    objects.push(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }

        let material1: MaterialRef = Arc::new(Glass::new(1.5));
        let material2: MaterialRef = Arc::new(Lambert::new(Vector3::new(0.4, 0.2, 0.1)));
        let material3: MaterialRef = Arc::new(Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0));

        objects.push(Box::new(Sphere::new(
            Vector3::new(0.0, 1.0, 0.0),
            1.0,
            material1,
        )));
        objects.push(Box::new(Sphere::new(
            Vector3::new(-4.0, 1.0, 0.0),
            1.0,
            material2,
        )));
        objects.push(Box::new(Sphere::new(
            Vector3::new(4.0, 1.0, 0.0),
            1.0,
            material3,
        )));

        Self { objects }
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // let mut res = self
        //     .objects
        //     .par_iter()
        //     .map(|obj| obj.hit(ray))
        //     .filter(|opt| opt.is_some())
        //     .map(|h| h.unwrap())
        //     .collect::<Vec<HitRecord>>();
        // res.par_sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        // if res.len() > 0 {
        //     Some(res.remove(0))
        // } else {
        //     None
        // }
        let mut res = None;
        let mut closest = t_max;

        for obj in self.objects.iter() {
            if let Some(hit) = obj.hit(ray, t_min, closest) {
                if hit.t < closest {
                    closest = hit.t;
                    res = Some(hit);
                }
            }
        }
        res
    }
}
