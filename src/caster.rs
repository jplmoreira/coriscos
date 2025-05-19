use config::ConfigError;
use futures::{
    executor,
    stream::{self, StreamExt},
    FutureExt,
};

use crate::{
    component::{camera::Camera, scene::Scene},
    math::Vector3,
    settings::Settings,
};

pub(crate) struct Caster {
    scene: Scene,
    camera: Camera,
    pixel_samples: u32,
    max_depth: u32,
    output_file: String,
}

impl Caster {
    pub(crate) fn build() -> Result<Self, ConfigError> {
        let Settings {
            image,
            camera,
            scene,
        } = Settings::new()?;

        let camera = Camera::build(camera);

        let scene = Scene::build(scene);

        Ok(Self {
            scene,
            camera,
            pixel_samples: image.pixel_samples,
            max_depth: image.max_depth,
            output_file: image.output,
        })
    }

    pub(crate) fn run(self) {
        let buffer_size = self.camera.get_buffer_size();
        println!("creating a buffer of size {buffer_size}");

        let buffer = stream::iter(0..buffer_size)
            .map(|buf_idx| {
                stream::iter(vec![buf_idx; self.pixel_samples as usize])
                    .map(|buf_idx| self.camera.sample_ray(buf_idx))
                    .then(|ray| self.scene.cast(ray, self.max_depth))
                    .fold(Vector3::fill(0.0), |acc, v| async move { acc + v })
                    .then(move |v| async move {
                        println!("finished pixel #{buf_idx}");
                        return v;
                    })
            })
            .then(|v| async move { v.await / self.pixel_samples })
            .map(|p| stream::iter(p.to_color()))
            .flatten()
            .collect::<Vec<u8>>();
        let buffer = executor::block_on(buffer);

        println!("finished building the buffer, now rendering it...");

        self.camera.render(buffer, &self.output_file);
    }
}
