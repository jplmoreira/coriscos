use rayon::prelude::*;

use crate::math::Vector3;

use super::{camera::Camera, ray::Ray, world::World};

pub struct Caster {
    world: World,
    camera: Camera,
    pixel_samples: u32,
    max_depth: u32,
}

impl Caster {
    pub fn build(pixel_samples: u32, max_depth: u32) -> Self {
        let look_from = Vector3::new(13.0, 2.0, 3.0);
        let look_at = Vector3::new(0.0, 0.0, 0.0);
        let vup = Vector3::new(0.0, 1.0, 0.0);

        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400;
        let vfov = 20.0;
        let defocus_angle = 0.6;
        let focus_distance = 10.0;

        let camera = Camera::build(
            look_from,
            look_at,
            vup,
            aspect_ratio,
            image_width,
            vfov,
            defocus_angle,
            focus_distance,
        );

        let world = World::build("");

        Self {
            world,
            camera,
            pixel_samples,
            max_depth,
        }
    }

    fn cast(&self, ray: Ray, depth: u32) -> Vector3 {
        if depth <= 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        if let Some(scatter) = self.world.find_hit(&ray) {
            return scatter.attenuation * self.cast(scatter.ray, depth - 1);
        }

        let unit_dir = ray.direction().normalize();
        let a = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - a) * Vector3::new(1.0, 1.0, 1.0) + a * Vector3::new(0.5, 0.7, 1.0)
    }

    fn get_sample(&self, buf_idx: u32) -> Vector3 {
        let ray = self.camera.sample_ray(buf_idx);
        self.cast(ray, self.max_depth)
    }

    fn get_pixel(&self, buf_idx: u32) -> Vec<u8> {
        let pixel = (0..self.pixel_samples)
            .into_par_iter()
            .map(|_| self.get_sample(buf_idx))
            .reduce(|| Vector3::new(0.0, 0.0, 0.0), |c1, c2| c1 + c2);
        let pixel = pixel / self.pixel_samples as f64;
        pixel.to_color()
    }

    pub fn run(&self, file: &str) {
        let buffer = self.camera.create_buffer();

        let buffer: Vec<u8> = buffer
            .par_iter()
            .enumerate()
            .map(|(idx, _)| self.get_pixel(idx as u32))
            .flatten()
            .collect();

        self.camera.render(buffer, file);
    }
}
