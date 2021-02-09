use std::ops::Range;
use crate::{ Real, Ray, shape::{ HitRecord, Shape } };

pub type Group = Vec< Box< dyn Shape + Sync + Send > >;

impl Shape for Group {
    fn hit(&self, ray: Ray, t_range: Range< Real >) -> Option< HitRecord > {
        self.iter().fold((None, t_range.end), |(now, end), x| {
            if let Some(hr) = x.hit(ray, t_range.start..end) {
                let t = hr.t;
                (Some(hr), t)
            } else { (now, end) }
        }).0
    }
}