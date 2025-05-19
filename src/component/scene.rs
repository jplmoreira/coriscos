use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use crossbeam::channel::{unbounded, Receiver, Sender};

use crate::{
    geometry::{sphere::Sphere, HittableRef},
    material::{diffuse_light::DiffuseLight, glass::Glass, lambert::Lambert, metal::Metal},
    math::{self, Vector3},
    settings,
};

use super::ray::{Ray, RayCast, RayReceiver, RaySender};

#[allow(dead_code)]
pub(crate) struct Scene {
    objects: Arc<Vec<HittableRef>>,
    senders: Vec<RaySender>,
    handlers: Vec<JoinHandle<()>>,
    background: Vector3,
    current_sender: AtomicUsize,
}

impl Scene {
    pub fn build(settings: Option<settings::Scene>) -> Self {
        let _input_file = settings.map(|s| s.input).unwrap_or("".into());

        let objects = random_scene();
        let objects = Arc::new(objects);

        let background = Vector3::fill(0.0);

        let thread_count = thread::available_parallelism().unwrap().get();
        println!("Creating a thread pool of size {thread_count}");
        let mut handlers = Vec::with_capacity(thread_count);
        let mut senders = Vec::with_capacity(thread_count);

        // Create a channel per thread
        for id in 0..thread_count {
            let (sender, receiver) = unbounded();
            let objects = objects.clone();
            senders.push(sender);
            handlers.push(thread::spawn(move || receive_rays(receiver, objects, id)));
        }

        Self {
            objects,
            senders,
            handlers,
            background,
            current_sender: AtomicUsize::new(0),
        }
    }

    pub fn cast(&self, ray: Ray, max_depth: u32) -> RayCast {
        // Round-robin through senders to distribute work
        let idx = self.current_sender.fetch_add(1, Ordering::Relaxed) % self.senders.len();
        let sender = &self.senders[idx];
        
        RayCast::new(ray, max_depth, self.background.clone(), sender.clone())
    }
}

fn receive_rays(receiver: RayReceiver, objects: Arc<Vec<HittableRef>>, thread_id: usize) {
    println!("starting thread #{thread_id}");
    // let mut last_idx = None;
    while let Ok((ray, sender)) = receiver.recv() {
        // let buf_idx = ray.buf_idx;
        // if last_idx.unwrap_or(buf_idx + 1) != buf_idx {
        //     println!("thread #{thread_id} now working on ray #{buf_idx}");
        //     last_idx = Some(buf_idx);
        // }

        let mut hit = None;
        let mut closest = f64::INFINITY;

        for obj in objects.iter() {
            if let Some(res) = obj.hit(&ray) {
                if res.record.t < closest {
                    closest = res.record.t;
                    hit = Some(res);
                }
            }
        }

        let _ = sender.send(hit);
        // if let Err(e) = sender.send(hit) {
        //     eprintln!("error sending hit on ray #{buf_idx}: {e}");
        // }
    }

    println!("thread #{thread_id} stopping...");
}

fn random_scene() -> Vec<HittableRef> {
    let mut objects = Vec::new();

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

            if (&center - Vector3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Vector3::random(0.0, 1.0) * Vector3::random(0.0, 1.0);
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

    objects
}
