use std::sync::Arc;

use crossbeam::channel::{Receiver, Sender};

use crate::{material::Material, math::Vector3};

pub(crate) struct Hit {
    pub(crate) record: HitRecord,
    pub(crate) material: Arc<dyn Material>,
}

pub(crate) struct HitRecord {
    pub(crate) buf_idx: u32,
    pub(crate) point: Vector3,
    pub(crate) normal: Vector3,
    pub(crate) direction: Vector3,
    pub(crate) t: f64,
    pub(crate) front: bool,
}

pub(crate) type HitReceiver = Receiver<Option<Hit>>;
pub(crate) type HitSender = Sender<Option<Hit>>;
