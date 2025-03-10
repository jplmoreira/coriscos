use image::RgbImage;
use rand::Rng;

use crate::math::Vector3;

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
        look_from: Vector3,
        look_at: Vector3,
        vup: Vector3,
        aspect_ratio: f64,
        image_width: u32,
        vfov: f64,
        defocus_angle: f64,
        focus_distance: f64,
    ) -> Self {
        // Image
        let image_height = (image_width as f64 / aspect_ratio) as u32; // height calculation from aspect ratio
        let image_height = if image_height > 1 { image_height } else { 1 }; // min height 1

        // Camera
        let look_direction = look_from.sub(&look_at);
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64); // height calculation directly from image, not aspect ratio

        // Basis camera vectors
        let w = look_direction.normal();
        let u = vup.cross(&w).normal();
        let v = w.cross(&u);

        // Viewport vectors
        let viewport_u = u.extend(viewport_width);
        let viewport_v = v.neg().extend(viewport_height);
        let pixel_delta_u = viewport_u.reduce(image_width as f64);
        let pixel_delta_v = viewport_v.reduce(image_height as f64);

        // Upper left pixel
        let viewport_upper_left = look_from
            .sub(&w.extend(focus_distance))
            .sub(&viewport_u.reduce(2.0))
            .sub(&viewport_v.reduce(2.0));
        let pixel_upper_left =
            viewport_upper_left.add(&pixel_delta_u.add(&pixel_delta_v).reduce(2.0));

        // Camera defocus disk
        let defocus_radius = focus_distance * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u.extend(defocus_radius);
        let defocus_disk_v = v.extend(defocus_radius);

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
        self.pixel_delta_u
            .extend(px)
            .add(&self.pixel_delta_v.extend(py))
    }

    fn defocus_disk_sample(&self) -> Vector3 {
        let v = Vector3::random_in_region([1.0, 1.0, 0.0]);
        self.look_from
            .add(&self.defocus_disk_u.extend(v.x()))
            .add(&self.defocus_disk_v.extend(v.y()))
    }

    pub fn sample_ray(&self, buf_idx: u32) -> Ray {
        let x = buf_idx % self.image_width;
        let y = buf_idx / self.image_width;

        let pixel_center = self
            .pixel_upper_left
            .add(&self.pixel_delta_u.extend(x as f64))
            .add(&self.pixel_delta_v.extend(y as f64));

        let pixel_sample = pixel_center.add(&self.pixel_sample_rand());

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.look_from.clone()
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample.sub(&ray_origin);

        Ray::new(ray_origin, ray_direction)
    }

    pub fn create_buffer(&self) -> Vec<Vec<u8>> {
        let size = (self.image_width * self.image_height) as usize;
        vec![vec![0; 3]; size]
    }

    pub fn render(&self, buffer: Vec<u8>, file: &str) {
        RgbImage::from_vec(self.image_width, self.image_height, buffer)
            .unwrap()
            .save(file)
            .unwrap();
    }
}
