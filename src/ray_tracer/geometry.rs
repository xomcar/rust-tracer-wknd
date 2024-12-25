use super::camera::{Hit, Hittable, Ray};
use super::math::{dot, Interval, Point3, Vec3};

pub struct Scene {
    entities: Vec<Entity>,
}

pub enum Entity {
    Sphere { center: Point3, radius: f32 },
}

impl Entity {
    pub fn new_sphere(center: Point3, radius: f32) -> Entity {
        Entity::Sphere { center, radius }
    }
}

impl Hittable for Entity {
    fn is_hit_by(&self, ray: &Ray, t_interval: Interval) -> Option<Hit> {
        match self {
            Entity::Sphere { center, radius } => {
                let oc = *center - ray.origin();
                let a = ray.direction().length_squared();
                let h = dot(ray.direction(), oc);
                let c = oc.length_squared() - radius * radius;
                let discriminant = h * h - a * c;

                if discriminant < 0.0 {
                    None // ignore sphere behind the camera in addition to no hit
                } else {
                    let root = (h - f32::sqrt(discriminant)) / a;
                    // if collision is too close to camera or too far, skip
                    if !t_interval.surrounds(root) {
                        let root = (h + f32::sqrt(discriminant)) / a;
                        if !t_interval.surrounds(root) {
                            return None;
                        }
                    }
                    let t = root;
                    let p = ray.at(t);
                    let n = (p - *center) / *radius;
                    Some(Hit::new(p, n, t, &ray))
                }
            }
        }
    }
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            entities: Vec::new(),
        }
    }

    pub fn add(&mut self, element: Entity) {
        self.entities.push(element);
    }

    pub fn hit(&self, ray: &Ray, t_interval: Interval) -> Option<Hit> {
        let mut closest_t = t_interval.max();
        let mut best_hit = None;
        for entity in &self.entities {
            let maybe_hit = entity.is_hit_by(ray, t_interval);
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
