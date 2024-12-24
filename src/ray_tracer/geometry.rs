use super::math::{dot, Point3, Vec3};
use super::ray::{Hit, Hittable, Ray};

pub struct Scene {
    entities: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            entities: Vec::new(),
        }
    }

    pub fn add(&mut self, element: Box<dyn Hittable>) {
        self.entities.push(element);
    }

    pub fn hit(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<Hit> {
        let mut closest_t = f32::MAX;
        let mut best_hit = None;
        for entity in &self.entities {
            let maybe_hit = entity.is_hit_by(ray, tmin, tmax);
            if let Some(hit) = maybe_hit {
                if hit.t() < closest_t {
                    closest_t = hit.t();
                    best_hit = Some(hit);
                }
            }
        }

        return best_hit;
    }
}

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Sphere {
        assert_eq!(
            true,
            radius > 0.0,
            "sphere radius shall be greater than 0.0"
        );
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn is_hit_by(&self, ray: &Ray, tmin: f32, tmax: f32) -> Option<Hit> {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = dot(ray.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            None // ignore sphere behind the camera in addition to no hit
        } else {
            let disc_root = f32::sqrt(discriminant);
            let root = (h - disc_root) / a;
            // if collision is too close to camera or too far, skip
            if root <= tmin || root >= tmax {
                return None;
            }
            let t = root;
            let p = ray.at(t);
            let n = (p - self.center) / self.radius;
            Some(Hit::new(p, n, t, &ray))
        }
    }
}
