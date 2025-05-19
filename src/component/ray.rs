use std::{
    future::Future,
    iter::zip,
    pin::Pin,
    task::{Context, Poll},
};

use crossbeam::channel::{unbounded, Receiver, Sender};

use crate::math::Vector3;

use super::hit::{HitReceiver, HitSender};

pub struct Ray {
    pub buf_idx: u32,
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(buf_idx: u32, origin: Vector3, direction: Vector3) -> Self {
        Self {
            buf_idx,
            origin,
            direction,
        }
    }

    pub fn at(&self, t: f64) -> Vector3 {
        &self.origin + &self.direction * t
    }
}

pub(crate) type RaySender = Sender<(Ray, HitSender)>;
pub(crate) type RayReceiver = Receiver<(Ray, HitSender)>;

pub(crate) struct RayCast {
    buf_idx: u32,
    depth: u32,
    colors: Vec<Vector3>,
    attenuations: Vec<Option<Vector3>>,
    background: Vector3,
    ray_sender: RaySender,
    hit_receiver: HitReceiver,
    hit_sender: HitSender,
}

impl RayCast {
    pub(crate) fn new(ray: Ray, depth: u32, background: Vector3, sender: RaySender) -> Self {
        let buf_idx = ray.buf_idx;

        let (s, r) = unbounded();
        sender.send((ray, s.clone())).unwrap();

        Self {
            buf_idx,
            depth,
            colors: Vec::new(),
            attenuations: Vec::new(),
            background,
            ray_sender: sender,
            hit_receiver: r,
            hit_sender: s,
        }
    }

    fn resolve(&self) -> Vector3 {
        zip(&self.colors, &self.attenuations).rev().fold(
            Vector3::fill(0.0),
            |acc, (color, attenuation)| match attenuation {
                Some(attenuation) => (acc + color) * attenuation,
                None => acc + color,
            },
        )
    }
}

impl Future for RayCast {
    type Output = Vector3;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.depth == 0 {
            return Poll::Ready(this.resolve());
        }
        let buf_idx = this.buf_idx;

        let message = match this.hit_receiver.try_recv() {
            Ok(record) => Some(record),
            Err(crossbeam::channel::TryRecvError::Empty) => None,
            Err(e) => {
                eprintln!("error casting a ray #{buf_idx}: {e}");
                this.depth = 0;
                None
            }
        };

        if let Some(record) = message {
            match record {
                Some(hit) => {
                    let material = hit.material;
                    let record = hit.record;

                    this.colors.push(material.emit());
                    let mut attenuation = None;
                    match material.scatter(record) {
                        Some(scattered) => {
                            this.depth -= 1;
                            attenuation = Some(scattered.attenuation);

                            if let Err(e) = this
                                .ray_sender
                                .send((scattered.ray, this.hit_sender.clone()))
                            {
                                eprintln!("error casting a ray #{buf_idx}: {e}");
                                this.depth = 0;
                            }
                        }
                        None => this.depth = 0,
                    }
                    this.attenuations.push(attenuation);
                }
                None => {
                    this.colors.push(this.background.clone());
                    this.attenuations.push(None);
                    this.depth = 0;
                }
            }
        }

        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
