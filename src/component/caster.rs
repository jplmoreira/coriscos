use rayon::prelude::*;

use crate::math::Vector3;

use super::{camera::Camera, ray::Ray, world::World};

pub struct Caster {
    world: World,
    camera: Camera,
    background: Vector3,
    pixel_samples: u32,
    max_depth: u32,
}

impl Caster {
    pub fn build(pixel_samples: u32, max_depth: u32) -> Self {
        let look_from = Vector3::new(13.0, 2.0, 3.0);
        let look_at = Vector3::new(0.0, 0.0, 0.0);
        let vup = Vector3::new(0.0, 1.0, 0.0);
        let background = Vector3::new(0.0, 0.0, 0.0);

        let aspect_ratio = 16.0 / 9.0;
        let image_width = 1920;
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
            background,
            pixel_samples,
            max_depth,
        }
    }

    fn cast(&self, ray: Ray, depth: u32) -> Vector3 {
        if depth <= 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = self.world.find_hit(&ray) {
            let emission_color = hit.material.emit(&hit);

            if let Some(scattered) = hit.material.scatter(&hit) {
                let scatter_color = scattered
                    .attenuation
                    .mul(&self.cast(scattered.ray, depth - 1));
                return emission_color.add(&scatter_color);
            } else {
                return emission_color;
            }
        }

        self.background.clone()
    }

    fn get_sample(&self, buf_idx: u32) -> Vector3 {
        let ray = self.camera.sample_ray(buf_idx);
        self.cast(ray, self.max_depth)
    }

    fn get_pixel(&self, buf_idx: u32) -> Vec<u8> {
        let pixel = vec![buf_idx; self.pixel_samples as usize]
            .into_par_iter()
            .map(|idx| self.get_sample(idx))
            .reduce(|| Vector3::new(0.0, 0.0, 0.0), |c1, c2| c1.add(&c2));
        let pixel = pixel.reduce(self.pixel_samples as f64);
        pixel.to_color()
    }

    pub fn run(&self, file: &str) {
        let buffer_size = self.camera.get_buffer_size();

        let buffer: Vec<u8> = (0..buffer_size)
            .into_par_iter()
            .map(|idx| self.get_pixel(idx as u32))
            .flatten()
            .collect();

        self.camera.render(buffer, file);
    }
}
