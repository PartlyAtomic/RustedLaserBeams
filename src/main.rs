extern crate cgmath;

use cgmath::Vector3;
use cgmath::prelude::*;
use cgmath::vec3;

use std::default::Default;

mod camera;

use camera::Camera;

mod hittable;
mod sphere;
mod hittable_list;

use hittable::Hittable;
use sphere::Sphere;
use hittable_list::HittableList;

mod ray;

use ray::Ray;

fn get_background(r: &Ray) -> Vector3<f32> {
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vector3::from_value(1.0).lerp(vec3(0.5, 0.7, 1.0), t)
}

fn color(r: &Ray, hittable: &Hittable) -> Vector3<f32> {
    match hittable.hit(r, 0.0, 1e10) {
        Some(hit_record) => 0.5 * hit_record.normal.add_element_wise(1.0),
        None => get_background(r)
    }
}

fn main() {
    let nx = 800;
    let ny = 400;
    let mut pixels = Vec::new();

    let world = HittableList {
        list: vec![
            Box::new(Sphere { center: vec3(0.0, 0.0, -1.0), radius: 0.5 }),
            Box::new(Sphere { center: vec3(0.0, -100.5, -1.0), radius: 100.0 }),
        ]
    };

    let camera: Camera = Default::default();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = camera.get_ray(u, v);

            let color = color(&r, &world) * 255.99;

            pixels.push(color.x as u8);
            pixels.push(color.y as u8);
            pixels.push(color.z as u8);
        }
    }

    write_png("test", &pixels, (nx, ny))
        .expect("error writing PNG file");
}

extern crate image;

/* Programming Rust p. 34 */
fn write_png(filename: &str, pixels: &[u8], bounds: (usize, usize))
             -> Result<(), std::io::Error>
{
    use std::fs::File;
    use image::png::PNGEncoder;
    use image::ColorType;

    let output = File::create(filename.to_owned() + ".png")?;

    let encoder = PNGEncoder::new(output);

    encoder.encode(&pixels,
                   bounds.0 as u32, bounds.1 as u32,
                   ColorType::RGB(8))?;

    Ok(())
}