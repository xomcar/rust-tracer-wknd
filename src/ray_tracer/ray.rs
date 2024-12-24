use super::math::{dot, Point3, Vec3};

pub trait Hittable {
    fn is_hit_by(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<Hit>;
}

pub struct Hit {
    p: Point3,
    n: Vec3,
    t: f32,
    front_face: bool,
}

impl Hit {
    pub fn new(hit_point: Point3, outward_normal: Vec3, t: f32, ray: &Ray) -> Hit {
        // normal will always point against the ray
        let front_face = dot(ray.direction(), outward_normal) < 0.0;
        let n = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Hit {
            p: hit_point,
            n,
            t,
            front_face,
        }
    }

    pub fn t(&self) -> f32 {
        self.t
    }

    pub fn n(&self) -> Vec3 {
        self.n
    }

    pub fn p(&self) -> Point3 {
        self.p
    }
}

#[derive(Clone, Debug)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}
