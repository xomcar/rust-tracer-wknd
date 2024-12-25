#[allow(unused, dead_code)]
use std::{
    error::Error,
    io::{BufWriter, Write},
};

use ray_tracer::{camera::Camera, geometry::Entity, geometry::Scene, math::Point3};
mod ray_tracer;

fn main() -> Result<(), Box<dyn Error>> {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 500;

    let camera = Camera::new(aspect_ratio, image_width);

    // populate scene
    let sphere_center = Point3::new(0.0, 0.0, -1.0);
    let ground_center = Point3::new(0.0, -500.0, 0.0);
    let mut scene = Scene::new();
    let sphere = Entity::new_sphere(sphere_center, 0.5);
    let ground: Entity = Entity::new_sphere(ground_center, 499.0);
    scene.add(sphere);
    scene.add(ground);

    camera.render_to_file(&scene, "output.ppm")?;

    Ok(())
}
