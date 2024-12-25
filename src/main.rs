#[allow(unused, dead_code)]
use std::{
    env,
    error::Error,
    io::{BufWriter, Write},
};

use ray_tracer::{camera::Camera, geometry::Entity, geometry::Scene, math::Point3};
mod ray_tracer;

fn main() -> Result<(), Box<dyn Error>> {
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 1000;

    let camera = Camera::new(aspect_ratio, image_width, 10, 50);

    let mut output_file = "output.ppm";
    let args: Vec<String> = env::args().collect();
    if let Some(filename) = args.get(1) {
        output_file = filename
    }

    let sphere_center = Point3::new(0.0, 0.0, -1.0);
    let ground_center = Point3::new(0.0, -500.0, 0.0);
    let mut scene = Scene::new();
    let sphere = Entity::new_sphere(sphere_center, 0.5);
    let ground: Entity = Entity::new_sphere(ground_center, 499.5);
    scene.add(sphere);
    scene.add(ground);

    let start = std::time::Instant::now();
    camera.render_to_file(&scene, output_file)?;
    let took = std::time::Instant::now() - start;
    let took_ms = took.as_millis();

    println!("Took {} ms - {:0.3} fps", took_ms, 1000.0 / took_ms as f32);

    Ok(())
}
