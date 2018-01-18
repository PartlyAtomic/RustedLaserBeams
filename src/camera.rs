extern crate cgmath;
extern crate rand;

use cgmath::prelude::*;
use cgmath::Vector3;
use cgmath::vec3;
use ray::Ray;

pub struct Camera {
    pub lower_left_corner: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub origin: Vector3<f32>,
    pub u: Vector3<f32>,
    pub v: Vector3<f32>,
    pub w: Vector3<f32>,
    pub lens_radius: f32,
}

fn random_in_unit_disk() -> Vector3<f32> {
    use rand::distributions::{IndependentSample, Range};

    let mut rng = rand::thread_rng();
    let range = Range::new(-1.0f32, 1.0f32);

    // Vector out of range for now
    let mut p = Vector3::from_value(2.0);

    while p.dot(p) >= 1.0 {
        p = vec3(range.ind_sample(&mut rng), range.ind_sample(&mut rng), 0.0);
    }
    p
}

// TODO: Initialize with ray and up vector
impl Camera {
    pub fn new(origin: &Vector3<f32>, target: &Vector3<f32>, up: &Vector3<f32>, vertical_fov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Self {
        let theta = vertical_fov * ::std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let w = (origin - target).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

//        let lower_left_corner = origin - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let lower_left_corner = origin - focus_dist * (half_width * u + half_height * v + w);
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin: *origin,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        let origin = self.origin + offset;
        let point = self.lower_left_corner + s * self.horizontal + t * self.vertical;

        Ray {
            origin,
            direction: point - origin,
        }
    }
}