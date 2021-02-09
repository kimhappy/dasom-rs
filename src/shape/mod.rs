use std::{ sync::Arc, ops::Range };
use crate::{ Material, Ray, Real, Vector };

#[derive(Clone)]
pub struct HitRecord {
    pub ff    : bool  ,
    pub t     : Real  ,
    pub p     : Vector,
    pub normal: Vector,
    pub mat   : Arc< dyn Material + Send + Sync >
}

impl HitRecord {
    pub fn new(t: Real, ray: Ray, normal: Vector, mat: Arc< dyn Material + Send + Sync >) -> Self {
        let ff = ray.dir.dot(&normal) < 0.0;
        Self { ff, t, p: ray.at(t), normal: if ff { normal.normalize() } else { -normal.normalize() }, mat }
    }
}

pub trait Shape {
    fn hit(&self, ray: Ray, t_range: Range< Real >) -> Option< HitRecord >;
}

pub mod group;
pub mod sphere;

pub use group::*;
pub use sphere::*;