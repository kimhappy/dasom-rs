use crate::{ Color, HitRecord, Material, Ray, Real, reflect, refract, schlick_approximation };

pub struct Dielectric {
    ir: Real
}

impl Dielectric {
    pub fn new(ir: Real) -> Self {
        Self { ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, hr: &HitRecord) -> Option< (Ray, Color) > {
        let ratio = if hr.ff { 1.0 / self.ir } else { self.ir };
        let ud = ray.dir.normalize();
        let ct = 1f64.min(-ud.dot(&hr.normal));
        let st = (1.0 - ct * ct).sqrt();
        let lr = ratio * st > 1.0 || schlick_approximation(ct, ratio) > rand::random();
        let direction = if lr {
            reflect(ud, hr.normal)
        }
        else {
            refract(ud, hr.normal, ratio)
        };
        Some((Ray::new(hr.p, direction), Color::new(1.0, 1.0, 1.0)))
    }
}