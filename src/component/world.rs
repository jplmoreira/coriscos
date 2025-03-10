use crate::{
    geometry::{sphere::Sphere, HitRecord, HittableRef},
    material::{diffuse_light::DiffuseLight, glass::Glass, lambert::Lambert, metal::Metal},
    math::{self, Vector3},
};

use super::ray::Ray;

pub struct World {
    objects: Vec<HittableRef>,
}

impl World {
    pub fn build(_file: &str) -> Self {
        let mut objects: Vec<HittableRef> = Vec::new();

        let material_ground = Lambert::new(Vector3::new(0.5, 0.5, 0.5));
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

                if (center.sub(&Vector3::new(4.0, 0.2, 0.0))).len() > 0.9 {
                    if choose_mat < 0.8 {
                        let albedo = Vector3::random(0.0, 1.0).mul(&Vector3::random(0.0, 1.0));
                        objects.push(Sphere::new(center, 0.2, Lambert::new(albedo)));
                    } else if choose_mat < 0.95 {
                        let albedo = Vector3::random(0.5, 1.0);
                        let fuzz = math::rand_range_f64(0.0, 0.5);
                        objects.push(Sphere::new(center, 0.2, Metal::new(albedo, fuzz)));
                    } else {
                        objects.push(Sphere::new(center, 0.2, Glass::new(1.5)));
                    }
                }
            }
        }

        let material1 = Glass::new(1.5);
        let material2 = Lambert::new(Vector3::new(0.4, 0.2, 0.1));
        let material3 = Metal::new(Vector3::new(0.7, 0.6, 0.5), 0.0);
        let light1 = DiffuseLight::new(Vector3::new(4.0, 4.0, 4.0));
        let light2 = DiffuseLight::new(Vector3::new(4.0, 4.0, 4.0));

        objects.push(Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0, material1));
        objects.push(Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0, material2));
        objects.push(Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0, material3));
        objects.push(Sphere::new(Vector3::new(0.0, 1.0, 2.0), 0.5, light1));
        objects.push(Sphere::new(Vector3::new(0.0, 1.0, -2.0), 0.5, light2));

        Self { objects }
    }

    pub fn find_hit(&self, ray: &Ray) -> Option<HitRecord> {
        // let hit = self
        //     .objects
        //     .iter()
        //     .filter_map(|obj| obj.hit(&ray))
        //     .min_by(|a, b| a.t.total_cmp(&b.t));

        let mut hit = None;
        let mut closest = f64::INFINITY;

        for obj in self.objects.iter() {
            if let Some(res) = obj.hit(&ray) {
                if res.t < closest {
                    closest = res.t;
                    hit = Some(res);
                }
            }
        }

        hit
    }
}
