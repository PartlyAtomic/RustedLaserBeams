extern crate cgmath;

use cgmath::Vector3;
use cgmath::prelude::*;
use cgmath::vec3;

extern crate rand;

pub mod materials;

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

fn random_in_unit_sphere() -> Vector3<f32> {
    use rand::distributions::{IndependentSample, Range};

    let mut rng = rand::thread_rng();
    let range = Range::new(-1.0f32, 1.0f32);

    // Vector out of range for now
    let mut p = Vector3::from_value(2.0);

    while p.magnitude2() >= 1.0 {
        p = vec3(range.ind_sample(&mut rng), range.ind_sample(&mut rng), range.ind_sample(&mut rng));
    }
    p
}

fn get_background(r: &Ray) -> Vector3<f32> {
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    Vector3::from_value(1.0).lerp(vec3(0.5, 0.7, 1.0), t)
}

fn color(r: &Ray, hittable: &Hittable, depth: i32) -> Vector3<f32> {
    match hittable.hit(r, 0.001, 1e10) {
        Some(hit_record) => {
            match hit_record.material.scatter(&r, &hit_record) {
                Some(ref scatter) if depth <= 50 => {
                    use cgmath::ElementWise;
                    scatter.attenuation.mul_element_wise(color(&scatter.scattered, hittable, depth + 1))
                }
                _ => vec3(0.0, 0.0, 0.0)
            }
        }
        None => get_background(r)
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    let mut pixels = Vec::new();

    use materials::lambertian::Lambertian;
    use materials::metal::Metal;
    use materials::dielectric::Dielectric;

//    let world = HittableList {
//        list: vec![
//            Box::new(Sphere { center: vec3(0.0, 0.0, -1.0), radius: 0.5, material: Box::new(Lambertian { albedo: vec3(0.5, 0.0, 0.1) }) }),
//            Box::new(Sphere { center: vec3(0.0, -100.5, -1.0), radius: 100.0, material: Box::new(Lambertian { albedo: vec3(0.1, 0.0, 0.5) }) }),
//        ]
//    };

//    let r = (std::f32::consts::PI / 4.0).cos();
    let world = HittableList {
        list: vec![
            Box::new(Sphere { center: vec3(0.0, 0.0, -1.0), radius: 0.5, material: Box::new(Lambertian { albedo: vec3(0.1, 0.2, 0.5) }) }),
            Box::new(Sphere { center: vec3(0.0, -100.5, -1.0), radius: 100.0, material: Box::new(Lambertian { albedo: vec3(0.8, 0.8, 0.0) }) }),
            Box::new(Sphere { center: vec3(1.0, 0.0, -1.0), radius: 0.5, material: Box::new(Metal { albedo: vec3(0.8, 0.6, 0.2), fuzz: 0.0 }) }),
            Box::new(Sphere { center: vec3(-1.0, 0.0, -1.0), radius: 0.5, material: Box::new(Dielectric { refraction_index: 1.5 }) }),
            Box::new(Sphere { center: vec3(-1.0, 0.0, -1.0), radius: -0.45, material: Box::new(Dielectric { refraction_index: 1.5 }) })
//Box::new(Sphere { center: vec3(-r, 0.0, -1.0), radius: r, material: Box::new(Lambertian { albedo: vec3(0.0, 0.0, 1.0) }) }),
//Box::new(Sphere { center: vec3(r, 0.0, -1.0), radius: r, material: Box::new(Lambertian { albedo: vec3(1.0, 0.0, 0.0) }) })
        ]
    };

    let camera = Camera::new(
        &vec3(-2.0, 2.0, 1.0),
        &vec3(0.0, 0.0, -1.0),
        &vec3(0.0, 1.0, 0.0),
        90.0,
        nx as f32 / ny as f32,
    );

    use rand::Rand;
    let mut rng = rand::thread_rng();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vector3::from_value(0.0);
            for _ in 0..ns {
                let u = (i as f32 + f32::rand(&mut rng)) / nx as f32;
                let v = (j as f32 + f32::rand(&mut rng)) / ny as f32;

                let r = camera.get_ray(u, v);

                col += color(&r, &world, 0);
            }

            col /= ns as f32;
            // Apply gamma correction
            col.x = col.x.sqrt();
            col.y = col.y.sqrt();
            col.z = col.z.sqrt();

            col *= 255.99;

            pixels.push(col.x as u8);
            pixels.push(col.y as u8);
            pixels.push(col.z as u8);
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