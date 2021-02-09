use crate::{ Real, Vector };

fn near_zero(v: Vector) -> bool {
    let criterion = 1e-8;
    v.x.abs() < criterion && v.y.abs() < criterion && v.z.abs() < criterion
}

pub fn random_in_unit_disk() -> Vector {
    std::iter::repeat_with(|| Vector::new(rand::random::< Real >() * 2.0 - 1.0, rand::random::< Real >() * 2.0 - 1.0, 0.0)).find(|v| v.norm() < 1.0).unwrap()
}

pub fn random_in_unit_vector(except: Vector) -> Vector {
    std::iter::repeat_with(|| Vector::new(rand::random::< Real >() * 2.0 - 1.0, rand::random::< Real >() * 2.0 - 1.0, rand::random::< Real >() * 2.0 - 1.0)).find(|v| v.norm() < 1.0 && !near_zero(v - except)).unwrap()
}

pub fn random_unit_vector(except: Vector) -> Vector {
    std::iter::repeat_with(|| Vector::new(rand::random::< Real >() * 2.0 - 1.0, rand::random::< Real >() * 2.0 - 1.0, rand::random::< Real >() * 2.0 - 1.0).normalize()).find(|v| !near_zero(v - except)).unwrap()
}

pub fn reflect(v: Vector, normal: Vector) -> Vector {
    v - 2.0 * v.dot(&normal) * normal
}

pub fn refract(v: Vector, normal: Vector, nab: Real) -> Vector {
    let ct = 1f64.min(-v.dot(&normal));
    let rp1 = nab * (v + ct * normal);
    let rp2 = -(1.0 - rp1.norm_squared()).abs().sqrt() * normal;
    rp1 + rp2
}

pub fn schlick_approximation(ct: Real, ri: Real) -> Real {
    let r0 = ((1.0 - ri) / (1.0 + ri)).powi(2);
    r0 + (1.0 - r0) * (1.0 - ct).powi(5)
}