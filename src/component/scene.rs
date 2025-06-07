use std::{
    iter,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use crossbeam::deque::{Injector, Stealer, Worker};

use crate::{
    geometry::{sphere::Sphere, HittableRef},
    material::{diffuse_light::DiffuseLight, glass::Glass, lambert::Lambert, metal::Metal},
    math::{self, Vector3},
    settings,
};

use super::ray::{Ray, RayCast, RayFut};

#[allow(dead_code)]
pub(crate) struct Scene {
    objects: Arc<Vec<HittableRef>>,
    background: Arc<Vector3>,
    pub(crate) thread_count: usize,
    injector: Arc<Injector<RayCast>>,
    is_running: Arc<AtomicBool>,
    handlers: Vec<JoinHandle<()>>,
}

impl Scene {
    pub fn build(settings: Option<settings::Scene>) -> Self {
        let _input_file = settings.map(|s| s.input).unwrap_or("".into());

        let objects = random_scene();
        let objects = Arc::new(objects);
        let background = Arc::new(Vector3::fill(0.0));

        let thread_count = thread::available_parallelism().unwrap().get();

        let injector = Arc::new(Injector::new());
        let is_running = Arc::new(AtomicBool::new(true));

        let mut workers = Vec::with_capacity(thread_count);
        let mut stealers = Vec::with_capacity(thread_count);

        for _ in 0..thread_count {
            let worker: Worker<RayCast> = Worker::new_fifo();
            stealers.push(worker.stealer());
            workers.push(worker);
        }

        let mut handlers = Vec::with_capacity(thread_count);

        let batch_limit = thread_count / 2;
        for (idx, worker) in workers.into_iter().enumerate() {
            let injector = injector.clone();
            let mut stealers = stealers.clone();
            stealers.remove(idx);
            let is_running = is_running.clone();
            let objects = objects.clone();

            handlers.push(thread::spawn(move || {
                while is_running.load(Ordering::Relaxed) {
                    let mut work = find_work(&worker, &injector, &stealers, batch_limit);

                    while let Some(cast) = work {
                        let mut hit = None;
                        let mut closest = f64::INFINITY;

                        for obj in objects.iter() {
                            if let Some(res) = obj.hit(&cast.ray) {
                                if res.record.t < closest {
                                    closest = res.record.t;
                                    hit = Some(res);
                                }
                            }
                        }

                        work = cast.resolve_hit(hit);
                    }
                }
            }));
        }

        Self {
            objects,
            background,
            thread_count,
            injector,
            is_running,
            handlers,
        }
    }

    pub fn cast(&self, ray: Ray, buf_idx: u32, samp_idx: usize, max_depth: u32) -> RayFut {
        RayFut::new(
            ray,
            buf_idx,
            samp_idx,
            max_depth,
            self.background.clone(),
            self.injector.clone(),
        )
    }
}

impl Drop for Scene {
    fn drop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
        for handler in self.handlers.drain(..) {
            let _ = handler.join();
        }
    }
}

fn random_scene() -> Vec<HittableRef> {
    let mut objects = Vec::<HittableRef>::new();

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

fn find_work<T>(
    local: &Worker<T>,
    global: &Injector<T>,
    stealers: &[Stealer<T>],
    batch_limit: usize,
) -> Option<T> {
    // Pop a task from the local queue, if not empty.
    local.pop().or_else(|| {
        // Otherwise, we need to look for a task elsewhere.
        iter::repeat_with(|| {
            // Try stealing a batch of tasks from the global queue.
            global
                .steal_batch_with_limit_and_pop(local, batch_limit)
                // Or try stealing a task from one of the other threads.
                .or_else(|| stealers.iter().map(|s| s.steal()).collect())
        })
        // Loop while no task was stolen and any steal operation needs to be retried.
        .find(|s| !s.is_retry())
        // Extract the stolen task, if there is one.
        .and_then(|s| s.success())
    })
}
