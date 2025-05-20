use std::{
    future::Future,
    iter::zip,
    pin::Pin,
    sync::{Arc, OnceLock, RwLock},
    task::{Context, Poll},
};

use crossbeam::deque::Injector;

use crate::math::Vector3;

use super::hit::Hit;

pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vector3 {
        &self.origin + &self.direction * t
    }
}

pub(crate) struct RayCast {
    pub(crate) ray: Ray,
    buf_idx: u32,
    samp_idx: usize,
    depth: u32,
    colors: Vec<Vector3>,
    attenuations: Vec<Option<Vector3>>,
    background: Arc<Vector3>,
    result: Arc<RwLock<OnceLock<Vector3>>>,
}

impl RayCast {
    pub(crate) fn new(
        ray: Ray,
        buf_idx: u32,
        samp_idx: usize,
        depth: u32,
        background: Arc<Vector3>,
        result: Arc<RwLock<OnceLock<Vector3>>>,
    ) -> Self {
        Self {
            ray,
            buf_idx,
            samp_idx,
            depth,
            colors: Vec::new(),
            attenuations: Vec::new(),
            background,
            result,
        }
    }

    pub(crate) fn resolve_hit(mut self, opt_hit: Option<Hit>) -> Option<Self> {
        let mut color = None;
        let mut attenuation = None;

        match opt_hit {
            Some(hit) => {
                let material = hit.material;
                let record = hit.record;
                color = Some(material.emit());

                match material.scatter(record) {
                    Some(scattered) => {
                        attenuation = Some(scattered.attenuation);
                        self.ray = scattered.ray;
                        self.depth -= 1;
                    }
                    None => self.depth = 0,
                }
            }
            None => self.depth = 0,
        }

        match color {
            Some(c) => self.colors.push(c),
            None => self.colors.push(self.background.as_ref().clone()),
        }
        self.attenuations.push(attenuation);

        if self.depth == 0 {
            let result = zip(self.colors, self.attenuations).rev().fold(
                Vector3::fill(0.0),
                |acc, (color, attenuation)| match attenuation {
                    Some(attenuation) => (acc + color) * attenuation,
                    None => acc + color,
                },
            );

            if let Ok(guard) = self.result.read() {
                if let Ok(()) = guard.set(result) {
                    return None;
                }
            }

            eprintln!(
                "error setting result for pixel #{} - sample #{}",
                self.buf_idx, self.samp_idx
            );
            return None;
        }

        Some(self)
    }
}

pub(crate) struct RayFut {
    result: Arc<RwLock<OnceLock<Vector3>>>,
}

impl RayFut {
    pub(crate) fn new(
        ray: Ray,
        buf_idx: u32,
        samp_idx: usize,
        depth: u32,
        background: Arc<Vector3>,
        injector: Arc<Injector<RayCast>>,
    ) -> Self {
        let result = Arc::new(RwLock::new(OnceLock::new()));
        let cast = RayCast::new(ray, buf_idx, samp_idx, depth, background, result.clone());
        injector.push(cast);

        Self { result }
    }
}

impl Future for RayFut {
    type Output = Vector3;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if let Ok(mut guard) = this.result.try_write() {
            if let Some(result) = guard.take() {
                return Poll::Ready(result);
            }
        }

        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
