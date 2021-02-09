use crate::{ Color, HitRecord, Material, Ray, random_unit_vector };

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: Ray, hr: &HitRecord) -> Option< (Ray, Color) > {
        Some((Ray::new(hr.p, hr.normal + random_unit_vector(-hr.normal)), self.albedo))
    }
}