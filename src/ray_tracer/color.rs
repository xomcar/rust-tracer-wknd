use crate::ray_tracer::math::Interval;

use super::math;
use std::io::Write;

pub type Color = math::Vec3;

pub fn write_color<W>(out: &mut W, pixel_color: Color)
where
    W: Write,
{
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    static INTENSITY: Interval = Interval::new(0.0, 0.999);
    let r = (256.0 * INTENSITY.clamp(r)) as i32;
    let g = (256.0 * INTENSITY.clamp(g)) as i32;
    let b = (256.0 * INTENSITY.clamp(b)) as i32;
    writeln!(out, "{} {} {}", r, g, b).expect("error in color writing");
}
