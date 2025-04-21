use config::ConfigError;
use rayon::prelude::*;

use crate::{math::Vector3, settings::Settings};

use super::{camera::Camera, ray::Ray, world::World};

pub struct Caster {
    world: World,
    camera: Camera,
    background: Vector3,
    pixel_samples: u32,
    max_depth: u32,
    output_file: String,
}

impl Caster {
    pub fn build() -> Result<Self, ConfigError> {
        let Settings {
            image,
            camera,
            scene,
        } = Settings::new()?;

        let background = Vector3::new(0.0, 0.0, 0.0);

        let camera = Camera::build(camera);

        let world = World::build(scene);

        Ok(Self {
            world,
            camera,
            background,
            pixel_samples: image.pixel_samples,
            max_depth: image.max_depth,
            output_file: image.output,
        })
    }

    fn cast(&self, ray: Ray, depth: u32) -> Vector3 {
        if depth <= 0 {
            return Vector3::new(0.0, 0.0, 0.0);
        }

        if let Some(hit) = self.world.find_hit(&ray) {
            let emission_color = hit.material.emit(&hit);

            if let Some(scattered) = hit.material.scatter(&hit) {
                let scatter_color = scattered.attenuation * self.cast(scattered.ray, depth - 1);
                return emission_color + scatter_color;
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
            .reduce(|| Vector3::new(0.0, 0.0, 0.0), |c1, c2| c1 + c2);
        let pixel = pixel / self.pixel_samples;
        pixel.to_color()
    }

    pub fn run(self) {
        let buffer_size = self.camera.get_buffer_size();

        let buffer: Vec<u8> = (0..buffer_size)
            .into_par_iter()
            .map(|idx| self.get_pixel(idx as u32))
            .flatten()
            .collect();

        self.camera.render(buffer, &self.output_file);
    }
}
