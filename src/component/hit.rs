use std::sync::Arc;

use crate::{material::Material, math::Vector3};

pub(crate) struct Hit {
    pub(crate) record: HitRecord,
    pub(crate) material: Arc<dyn Material>,
}

pub(crate) struct HitRecord {
    pub(crate) point: Vector3,
    pub(crate) normal: Vector3,
    pub(crate) direction: Vector3,
    pub(crate) t: f64,
    pub(crate) front: bool,
}
