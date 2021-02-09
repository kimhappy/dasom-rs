use crate::{ Vector, Real, Ray, random_in_unit_disk };

pub struct Camera {
    pub ratio : Real,
    pub vxsize: Real,
    pub vysize: Real,
    pub lrad  : Real,
        ori   : Vector,
        hor   : Vector,
        ver   : Vector,
        lol   : Vector,
        u     : Vector,
        v     : Vector
}

impl Camera {
    pub fn new(ratio: Real, vfov: Real, lrad: Real, fdist: Real, lookfrom: Vector, lookat: Vector, vup: Vector) -> Self {
        let h = (vfov / 2.0).tan();
        let (vxsize, vysize) = (2.0 * h * ratio, 2.0 * h);

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let ori = lookfrom;
        let hor = vxsize * u * fdist;
        let ver = vysize * v * fdist;
        let lol = ori - hor / 2.0 - ver / 2.0 - fdist * w;

        Self { ratio, vxsize, vysize, lrad, ori, hor, ver, lol, u, v }
    }

    pub fn get_ray(&self, (s, t): (Real, Real)) -> Ray {
        let rd = self.lrad * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.ori + offset, self.lol + s * self.hor + t * self.ver - self.ori - offset)
    }
}