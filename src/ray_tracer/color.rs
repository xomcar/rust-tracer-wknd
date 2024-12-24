use super::math;
use std::io::Write;

pub type Color = math::Vec3;

pub fn write_color<W>(out: &mut W, pixel_color: Color)
where
    W: Write,
{
    let r = (255.999 * pixel_color.x()) as u32;
    let g = (255.999 * pixel_color.y()) as u32;
    let b = (255.999 * pixel_color.z()) as u32;
    writeln!(out, "{} {} {}", r, g, b).expect("error in color writing");
}
