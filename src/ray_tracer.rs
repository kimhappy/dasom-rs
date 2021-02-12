use crate::{ Camera, Color, Ray, Real, Shape, shape::Group };
use rayon::iter::{ IntoParallelIterator, ParallelIterator };

pub struct RayTracer {
    pub camera : Camera,
    pub ixsize : u32,
    pub iysize : u32,
    pub spp    : u32,
    pub rdepth : u32
}

impl RayTracer {
    pub fn new(camera: Camera, ixsize: u32, spp: u32, rdepth: u32) -> Self {
        let iysize = (ixsize as Real / camera.ratio) as u32;
        Self { camera, ixsize, iysize, spp, rdepth }
    }

    fn get_st(&self, (x, y): (Real, Real)) -> (Real, Real) {
        (x / (self.ixsize - 1) as Real, (self.iysize as Real - y - 1.0) / (self.iysize - 1) as Real)
    }

    fn sample_vars(&self) -> impl Iterator< Item = (Real, Real) > {
        (0..self.spp).map(|_| (rand::random(), rand::random()))
    }

    fn ray_color(world: &Group, ray: Ray, depth: u32) -> Color {
        match (0..depth).try_fold(
            (Color::new(1.0, 1.0, 1.0), ray),
            |(color, ray), _| world.hit(ray, 0.001..Real::INFINITY)
                .ok_or_else(|| {
                    let t = 0.5 * (1.0 + ray.dir.normalize().y);
                    color * ((1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.86, 0.92, 1.0)) })
                .and_then(|hr| hr.mat.scatter(ray, &hr)
                    .map(|(scattered, attenuation)| (color * attenuation, scattered))
                    .ok_or_else(|| Color::new(0.0, 0.0, 0.0)))) {
            Ok(_) => Color::new(0.0, 0.0, 0.0),
            Err(color) => color,
        }
    }

    pub fn draw(&self, world: &Group) -> Vec< Color > {
        let pbar = indicatif::ProgressBar::new(self.iysize as u64);
        let sv = self.sample_vars().collect::< Vec< _ > >();

        (0..self.iysize).into_par_iter().map(|y| {
            let ret = (0..self.ixsize).map(|x|
                Color::mix(sv.iter().map(|(xp, yp)| {
                    let (xx, yy) = (x as Real + xp, y as Real + yp);
                    let (s, t) = self.get_st((xx, yy));
                    let ray = self.camera.get_ray((s, t));
                    Self::ray_color(&world, ray, self.rdepth)
                }))
            ).collect::< Vec< _ > >();
            pbar.inc(1);
            ret
        }).flatten().collect::< Vec< _ > >()
    }
}