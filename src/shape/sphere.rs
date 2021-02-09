use std::{ sync::Arc, ops::Range };
use crate::{ HitRecord, Material, Shape, Ray, Real, Vector };

pub struct Sphere {
    pub cen: Vector,
    pub rad: Real,
    pub mat: Arc< dyn Material + Send + Sync >
}

impl Sphere {
    pub fn new(cen: Vector, rad: Real, mat: Arc< dyn Material + Send + Sync >) -> Self {
        Self { cen, rad, mat }
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: Ray, t_range: Range< Real >) -> Option< HitRecord > {
        let oc = ray.ori - self.cen;
        let a = ray.dir.norm_squared();
        let h = oc.dot(&ray.dir);
        let c = oc.norm_squared() - self.rad * self.rad;
        let d = h * h - a * c;
        let (sm, bm) = ((-h - d.sqrt()) / a, (-h + d.sqrt()) / a);

        if t_range.contains(&sm) {
            Some(HitRecord::new(sm, ray, (ray.at(sm) - self.cen) / self.rad, self.mat.clone()))
        }
        else if t_range.contains(&bm) {
            Some(HitRecord::new(bm, ray, (ray.at(bm) - self.cen) / self.rad, self.mat.clone()))
        }
        else { None }
    }
}