use super::{
    color::{self, Color},
    geometry,
    math::{self, dot, Interval, Point3, Vec3},
};
use std::{
    error::Error,
    fs,
    io::{BufWriter, Write},
};

pub trait Hittable {
    fn is_hit_by(&self, ray: &Ray, t_interval: Interval) -> Option<Hit>;
}

pub struct Hit {
    p: Point3,
    n: Vec3,
    t: f32,
    is_front_facing: bool,
}

impl Hit {
    pub fn new(hit_point: Point3, outward_normal: Vec3, t: f32, ray: &Ray) -> Hit {
        // normal will always point against the ray
        let is_front_facing = dot(ray.direction(), outward_normal) < 0.0;
        let normal: Vec3 = if is_front_facing {
            // ray is outside surface
            outward_normal
        } else {
            // ray is inside surface
            -outward_normal
        };
        Hit {
            p: hit_point,
            n: normal,
            is_front_facing,
            t,
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

pub struct Camera {
    aspect_ratio: f32,
    image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    sample_per_pixel: i32,
    sample_per_pixel_scale: f32, // each surrounding pixel contributes a fraction of 1.0 to color of that pixel
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: i32, sample_per_pixel: i32) -> Camera {
        // image settings
        let image_height = (image_width as f32 / aspect_ratio) as i32;

        // camera settings
        let focal_length: f32 = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
        let center = Point3::new(0.0, 0.0, 0.0);

        // compute vector along horizontal and vertical references for viewport in global reference
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // compute delta vector from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // compute coordinate of upper left pixel (named Q) from origin
        let viewport_center = Vec3::new(0.0, 0.0, focal_length);
        let viewport_q = center - viewport_center - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00 = viewport_q + 0.5 * (pixel_delta_u + pixel_delta_v);
        let sample_per_pixel_scale = 1.0 / sample_per_pixel as f32;

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00,
            pixel_delta_u,
            pixel_delta_v,
            sample_per_pixel,
            sample_per_pixel_scale,
        }
    }

    pub fn get_random_ray(&self, idx_u: i32, idx_v: i32) -> Ray {
        fn sample_square() -> Vec3 {
            Vec3::new(
                rand::random::<f32>() - 0.5,
                rand::random::<f32>() - 0.5,
                0.0,
            )
        }

        // build camera ray from origin directed to surrounding pixels
        let offset = sample_square();
        let pixel_sample = self.pixel00
            + (idx_u as f32 + offset.x()) * self.pixel_delta_u
            + (idx_v as f32 + offset.y()) * self.pixel_delta_v;

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    pub fn render_to_file(
        &self,
        scene: &geometry::Scene,
        file_path: &str,
    ) -> Result<(), Box<dyn Error>> {
        let file = fs::File::create(file_path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "P3\n")?; // ASCII color
        writeln!(writer, "{} {}", self.image_width, self.image_height)?; // columns and rows
        writeln!(writer, "{}", 255)?; // max color

        for row in 0..self.image_height {
            print!("\rProgress: {}/{}", row + 1, self.image_height);
            for col in 0..self.image_width {
                let mut pixel_color = color::Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.sample_per_pixel {
                    let ray = self.get_random_ray(col, row);
                    pixel_color += get_ray_color(&ray, scene);
                }
                color::write_color(&mut writer, self.sample_per_pixel_scale * pixel_color);
            }
        }
        println!("\nOutput saved to: {}", file_path);
        Ok(())
    }
}

fn get_ray_color(ray: &Ray, scene: &geometry::Scene) -> Color {
    if let Some(hit) = scene.hit(ray, Interval::new(0.0, f32::INFINITY)) {
        return 0.5 * Color::new(hit.n().x() + 1.0, hit.n().y() + 1.0, hit.n().z() + 1.0);
    }

    // no hit, so render gradient in current pixel
    let unit_direction = math::unit_vector(ray.direction());
    let coefficient = 0.5 * (unit_direction.y() + 1.0); // y is limited between -1 and 1
    let white = Color::new(1.0, 1.0, 1.0);
    let blue = Color::new(0.5, 0.7, 1.0);
    (1.0 - coefficient) * white + coefficient * blue
}
