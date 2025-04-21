use config::{Config, ConfigError, File};
use serde::Deserialize;

use crate::math::Vector3;

#[derive(Debug, Deserialize)]
pub(crate) struct Image {
    pub(crate) output: String,
    pub(crate) pixel_samples: u32,
    pub(crate) max_depth: u32,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Camera {
    pub(crate) image_width: u32,
    pub(crate) aspect_height: f64,
    pub(crate) aspect_width: f64,
    pub(crate) vertical_fov: f64,
    pub(crate) defocus_angle: f64,
    pub(crate) focus_distance: f64,
    pub(crate) look_from: Vector3,
    pub(crate) look_at: Vector3,
    pub(crate) vup: Vector3,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Scene {
    pub(crate) input: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    pub(crate) image: Image,
    pub(crate) camera: Camera,
    pub(crate) scene: Option<Scene>,
}

impl Settings {
    pub(crate) fn new() -> Result<Self, ConfigError> {
        let config_file = std::env::var("CORISCOS_CONFIG").unwrap_or("coriscos.toml".into());

        let s = Config::builder()
            .add_source(File::with_name(&config_file))
            .build()?;

        s.try_deserialize()
    }
}
