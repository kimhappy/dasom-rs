use crate::{ Real, Vector };

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub ori: Vector,
    pub dir: Vector
}

impl Ray {
    pub fn new(ori: Vector, dir: Vector) -> Self {
        Self { ori, dir }
    }

    pub fn at(&self, t: Real) -> Vector {
        self.ori + t * self.dir
    }
}