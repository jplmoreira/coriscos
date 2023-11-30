use crate::{
    geometry::{sphere::Sphere, HitRecord, HittableRef},
    material::{
        diffuse_light::DiffuseLight, glass::Glass, lambert::Lambert, metal::Metal, MaterialRef,
    },
    math::{self, Vector3},
};

use super::ray::Ray;

pub struct World {
    objects: Vec<HittableRef>,
}

impl World {
    pub fn build(_file: &str) -> Self {
        let mut objects: Vec<HittableRef> = Vec::new();

        let material_ground: MaterialRef = Lambert::new(Vector3::new(0.5, 0.5, 0.5));
        objects.push(Sphere::new(
            Vector3::new(0.0, -1000.0, 0.0),
            1000.0,
            material_ground,
        ));

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
                        Lambert::new(albedo)
                    } else if choose_mat < 0.95 {
                        let albedo = Vector3::random(0.5, 1.0);
                        let fuzz = math::rand_range_f64(0.0, 0.5);
                        Metal::new(albedo, fuzz)
                    } else {
                        Glass::new(1.5)
                    };
                    objects.push(Sphere::new(center, 0.2, material));
                }
            }
        }

        let material1: MaterialRef = Glass::new(1.5);
        let material2: MaterialRef = Lambert::new(Vector3::new(0.4, 0.2, 0.1));
        let material3: MaterialRef = Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0);
        let light1: MaterialRef = DiffuseLight::new(Vector3::new(4.0, 4.0, 4.0));
        let light2: MaterialRef = DiffuseLight::new(Vector3::new(4.0, 4.0, 4.0));

        objects.push(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, material1));
        objects.push(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, material2));
        objects.push(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, material3));
        objects.push(Sphere::new(Vector3::new(0.0, 1.0, 2.0), 0.5, light1));
        objects.push(Sphere::new(Vector3::new(0.0, 1.0, -2.0), 0.5, light2));

        Self { objects }
    }

    pub fn object(&self, idx: usize) -> &HittableRef {
        &self.objects[idx]
    }

    pub fn find_hit(&self, ray: &Ray) -> Option<HitRecord> {
        // let hit = self
        //     .objects
        //     .iter()
        //     .enumerate()
        //     .filter_map(|(idx, obj)| obj.hit(&ray, idx))
        //     .clone()
        //     .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap_or(Ordering::Equal))?;

        // Some(self.objects[hit.idx].scatter(hit))

        let mut hit = None;
        let mut closest = f64::INFINITY;

        for (idx, obj) in self.objects.iter().enumerate() {
            if let Some(res) = obj.hit(&ray, idx) {
                if res.t < closest {
                    closest = res.t;
                    hit = Some(res);
                }
            }
        }

        hit
    }
}
