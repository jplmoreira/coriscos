use image::RgbImage;
use rand::Rng;

use crate::{math::Vector3, settings};

use super::ray::Ray;

pub struct Camera {
    look_from: Vector3,
    pixel_upper_left: Vector3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    // u: Vector3,
    // v: Vector3,
    // w: Vector3,
    defocus_disk_u: Vector3,
    defocus_disk_v: Vector3,
    defocus_angle: f64,
    image_width: u32,
    image_height: u32,
}

impl Camera {
    pub fn build(
        settings::Camera {
            image_width,
            aspect_height,
            aspect_width,
            vertical_fov,
            defocus_angle,
            focus_distance,
            look_from,
            look_at,
            vup,
        }: settings::Camera,
    ) -> Self {
        // Image
        let aspect_ratio = aspect_height / aspect_width;
        let image_height = (image_width as f64 / aspect_ratio) as u32; // height calculation from aspect ratio
        let image_height = if image_height > 1 { image_height } else { 1 }; // min height 1

        // Camera
        let look_direction = &look_from - look_at;
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64); // height calculation directly from image, not aspect ratio

        // Basis camera vectors
        let w = look_direction.normal();
        let u = vup.cross(&w).normal();
        let v = w.cross(&u);

        // Viewport vectors
        let viewport_u = &u * viewport_width;
        let viewport_v = -&v * viewport_height;
        let pixel_delta_u = &viewport_u / image_width;
        let pixel_delta_v = &viewport_v / image_height;

        // Upper left pixel
        let viewport_upper_left =
            &look_from - w * focus_distance - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_upper_left = viewport_upper_left + (&pixel_delta_u + &pixel_delta_v) / 2.0;

        // Camera defocus disk
        let defocus_radius = focus_distance * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            look_from,
            pixel_upper_left,
            pixel_delta_u,
            pixel_delta_v,
            // u,
            // v,
            // w,
            defocus_disk_u,
            defocus_disk_v,
            defocus_angle,
            image_width,
            image_height,
        }
    }

    fn pixel_sample_rand(&self) -> Vector3 {
        let mut rng = rand::rng();
        let px = -0.5 + rng.random::<f64>();
        let py = -0.5 + rng.random::<f64>();
        &self.pixel_delta_u * px + &self.pixel_delta_v * py
    }

    fn defocus_disk_sample(&self) -> Vector3 {
        let v = Vector3::random_in_region([1.0, 1.0, 0.0]);
        &self.look_from + &self.defocus_disk_u * v.x + &self.defocus_disk_v * v.y
    }

    pub fn sample_ray(&self, buf_idx: u32) -> Ray {
        let x = buf_idx % self.image_width;
        let y = buf_idx / self.image_width;

        let pixel_center =
            &self.pixel_upper_left + &self.pixel_delta_u * x + &self.pixel_delta_v * y;

        let pixel_sample = pixel_center + self.pixel_sample_rand();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.look_from.clone()
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - &ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    pub fn get_buffer_size(&self) -> u32 {
        self.image_width * self.image_height
    }

    pub fn render(&self, buffer: Vec<u8>, file: &str) {
        RgbImage::from_vec(self.image_width, self.image_height, buffer)
            .unwrap()
            .save(file)
            .unwrap();
    }
}
