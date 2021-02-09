use std::ops;
use crate::Real;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: Real,
    pub g: Real,
    pub b: Real
}

impl Color {
    pub fn new(r: Real, g: Real, b: Real) -> Self {       
        Self { r, g, b }
    }

    pub fn from_24(c: u32) -> Self {
        let map = 255.0;
        let (ri, gi, bi) = (c >> 16, (c >> 8) & 0xFF, c & 0xFF);
        Self { r: (ri as Real / map).powi(2), g: (gi as Real / map).powi(2), b: (bi as Real / map).powi(2) }
    }

    pub fn mix(colors: impl Iterator< Item = Self >) -> Self {
        let len = colors.size_hint().1.unwrap() as Real;
        colors.fold(Color::new(0.0, 0.0, 0.0), ops::Add::add) / len
    }
}

overload::overload!((lhs: ?Color) + (rhs: ?Color) -> Color {
    Color::new(lhs.r + rhs.r, lhs.g + rhs.g, lhs.b + rhs.b)
});

overload::overload!((lhs: ?Color) - (rhs: ?Color) -> Color {
    Color::new(lhs.r - rhs.r, lhs.g - rhs.g, lhs.b - rhs.b)
});

overload::overload!((lhs: ?Color) * (rhs: ?Color) -> Color {
    Color::new(lhs.r * rhs.r, lhs.g * rhs.g, lhs.b * rhs.b)
});

overload::overload!((lhs: ?Real) * (rhs: ?Color) -> Color {
    Color::new(lhs * rhs.r, lhs * rhs.g, lhs * rhs.b)
});

overload::overload!((lhs: ?Color) * (rhs: ?Real) -> Color {
    Color::new(lhs.r * rhs, lhs.g * rhs, lhs.b * rhs)
});

overload::overload!((lhs: ?Color) / (rhs: ?Real) -> Color {
    Color::new(lhs.r / rhs, lhs.g / rhs, lhs.b / rhs)
});