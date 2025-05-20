use chrono::Local;
use config::ConfigError;
use futures::{
    executor,
    stream::{self, StreamExt},
    FutureExt,
};
use indicatif::{ProgressBar, ProgressStyle};

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
        let buffer = self.camera.get_buffer();
        let buffer_size = buffer.len();

        let bar = ProgressBar::new(buffer_size as u64).with_style(
            ProgressStyle::default_bar()
                .template("{wide_bar} {percent_precise:>7}%/100%\n{wide_msg} {elapsed_precise:>}")
                .unwrap(),
        );

        let buffer = bar
            .wrap_stream(stream::iter(buffer))
            .map(|buf_idx| {
                stream::iter(vec![buf_idx; self.pixel_samples as usize])
                    .enumerate()
                    .map(|(samp_idx, buf_idx)| (self.camera.sample_ray(buf_idx), buf_idx, samp_idx))
                    .map(|(ray, buf_idx, samp_idx)| {
                        self.scene.cast(ray, buf_idx, samp_idx, self.max_depth)
                    })
                    .buffer_unordered(self.pixel_samples as usize)
                    .fold(Vector3::fill(0.0), |acc, v| async move { acc + v })
                    .map(move |v| (buf_idx, (v / self.pixel_samples).to_color()))
            })
            .buffer_unordered(self.camera.image_width as usize)
            .collect::<Vec<(u32, Vec<u8>)>>();

        println!(
            "Start time - {} | # of pixels - {buffer_size} | Worker threads - {} | Output file - {}",
            Local::now().format("%H:%M:%S"),
            self.scene.thread_count,
            self.output_file,
        );

        let mut buffer = executor::block_on(buffer);
        bar.finish();

        buffer.sort_by(|(a, _), (b, _)| a.cmp(b));
        let buffer = buffer
            .into_iter()
            .map(|v| v.1)
            .flatten()
            .collect::<Vec<u8>>();

        self.camera.render(buffer, &self.output_file);
    }
}
