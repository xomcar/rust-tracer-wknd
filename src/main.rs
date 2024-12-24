#[allow(unused, dead_code)]
use std::{
    error::Error,
    io::{BufWriter, Write},
};

mod ray_tracer;

use ray_tracer::color::{self, Color};
use ray_tracer::geometry;
use ray_tracer::math::{self, Point3, Vec3};
use ray_tracer::ray::{self, Ray};

fn get_ray_color(ray: &ray::Ray, scene: &geometry::Scene) -> Color {
    if let Some(hit) = scene.hit(ray, 0.0, f32::MAX) {
        return 0.5 * Color::new(hit.n().x() + 1.0, hit.n().y() + 1.0, hit.n().z() + 1.0);
    }

    let unit_direction = math::unit_vector(ray.direction());
    let coefficient = 0.5 * (unit_direction.y() + 1.0); // y is limited between -1 and 1
    let white = Color::new(1.0, 1.0, 1.0);
    let blue = Color::new(0.5, 0.7, 1.0);
    (1.0 - coefficient) * white + coefficient * blue
}

fn main() -> Result<(), Box<dyn Error>> {
    use std::fs;
    let file = fs::File::create("output.ppm")?;
    let mut writer = BufWriter::new(file);

    // image settings
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = (image_width as f32 / aspect_ratio) as i32;

    // camera settings
    let focal_length: f32 = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // compute vector along horizontal and vertical references for viewport in global reference
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // compute delta vector from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // compute coordinate of upper left pixel (named Q) from origin
    let viewport_center = Vec3::new(0.0, 0.0, focal_length);
    let viewport_q = camera_center - viewport_center - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00 = viewport_q + 0.5 * (pixel_delta_u + pixel_delta_v);

    // populate scene
    let sphere_center = Point3::new(0.0, 0.0, -1.0);
    let ground_center = Point3::new(0.0, -500.0, 0.0);
    let mut scene = geometry::Scene::new();
    let sphere = Box::new(geometry::Sphere::new(sphere_center, 0.5));
    let ground = Box::new(geometry::Sphere::new(ground_center, 499.0));
    scene.add(sphere);
    scene.add(ground);

    // render to image
    writeln!(writer, "P3\n")?; // ASCII color
    writeln!(writer, "{} {}", image_width, image_height)?; // columns and rows
    writeln!(writer, "{}", 255)?; // max color

    for row in 0..image_height {
        print!("\rProgress: {}/{}", row + 1, image_height);
        for col in 0..image_width {
            let pixel_center =
                pixel00 + (col as f32 * pixel_delta_u) + (row as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let color = get_ray_color(&ray, &scene);
            color::write_color(&mut writer, color);
        }
    }
    Ok(())
}
