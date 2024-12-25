use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

pub type Point3 = Vec3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn random() -> Vec3 {
        Vec3::new(random(), random(), random())
    }

    pub fn random_between(min: f32, max: f32) -> Vec3 {
        Vec3::new(
            random_between(min, max),
            random_between(min, max),
            random_between(min, max),
        )
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() + v.x(), self.y() + v.y(), self.z() + v.z())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() - v.x(), self.y() - v.y(), self.z() - v.z())
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x() * v.x(), self.y() * v.y(), self.z() * v.z())
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self * v.x(), self * v.y(), self * v.z())
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Vec3 {
        Vec3::new(self.x() * t, self.y() * t, self.z() * t)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Vec3 {
        Vec3::new(self.x() / t, self.y() / t, self.z() / t)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        *self = *self * t;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        *self = *self / t;
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f32 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

#[inline]
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

#[inline]
pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_between(-1.0, 1.0);
        let length_squared = p.length_squared();
        if 1e-44 < length_squared && length_squared <= 1.0 {
            return p / f32::sqrt(length_squared);
        }
    }
}

#[inline]
pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if dot(on_unit_sphere, normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

#[derive(Clone, Copy)]
pub struct Interval {
    min: f32,
    max: f32,
}

impl Interval {
    pub const fn new(min: f32, max: f32) -> Interval {
        Interval { min, max }
    }

    pub fn empty() -> Interval {
        Interval {
            min: f32::INFINITY,
            max: f32::NEG_INFINITY,
        }
    }

    pub const fn max(&self) -> f32 {
        self.max
    }

    pub fn universe() -> Interval {
        Interval {
            min: f32::NEG_INFINITY,
            max: f32::INFINITY,
        }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, element: f32) -> bool {
        element >= self.min && element <= self.max
    }

    pub fn surrounds(&self, element: f32) -> bool {
        element > self.min && element < self.max
    }

    pub fn clamp(&self, element: f32) -> f32 {
        if element < self.min {
            return self.min;
        };
        if element > self.max {
            return self.max;
        };
        element
    }
}

pub fn random() -> f32 {
    rand::random::<f32>()
}

pub fn random_between(min: f32, max: f32) -> f32 {
    min + (max - min) * rand::random::<f32>()
}
