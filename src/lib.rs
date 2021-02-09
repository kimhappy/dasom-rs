#![allow(dead_code)]

//! # dasom-rs
//! A toy ray tracing engine based on [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) in Rust.
//!
//!You can easily render images and add your own shapes or materials easily.
//!
//![Examples](https://github.com/POMMI3R/dasom-rs/tree/master/examples) will be very helpful.

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