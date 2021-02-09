use crate::{ Color, Ray, HitRecord };

pub trait Material {
    fn scatter(&self, ray: Ray, hr: &HitRecord) -> Option< (Ray, Color) >;
}

pub mod lambertian;
pub mod metal;
pub mod dielectric;

pub use lambertian::*;
pub use metal::*;
pub use dielectric::*;