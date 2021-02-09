use crate::{ Color, HitRecord, Material, Ray, Real, random_in_unit_vector, reflect };

pub struct Metal {
    albedo: Color,
    fuzz  : Real
}

impl Metal {
    pub fn new(albedo: Color, fuzz: Real) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hr: &HitRecord) -> Option< (Ray, Color) > {
        let reflected = reflect(ray.dir.normalize(), hr.normal);
        if reflected.dot(&hr.normal) > 0.0 { Some((Ray::new(hr.p, reflected + self.fuzz * random_in_unit_vector(-reflected)), self.albedo)) } else { None }
    }
}