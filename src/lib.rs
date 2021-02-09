#![allow(dead_code)]

pub mod color;
pub mod ray_tracer;
pub mod ray;
pub mod real;
pub mod vector;
pub mod shape;
pub mod camera;
pub mod material;
pub mod utility;

pub use color::*;
pub use ray_tracer::*;
pub use ray::*;
pub use real::*;
pub use vector::*;
pub use camera::*;
pub use material::Material;
pub use shape::{ HitRecord, Shape };
pub use utility::*;