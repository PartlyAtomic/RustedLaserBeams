extern crate cgmath;

use std::default::Default;

use cgmath::InnerSpace;
use cgmath::Vector3;
use cgmath::vec3;
use ray::Ray;

pub struct Camera {
    pub lower_left_corner: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub origin: Vector3<f32>,
}

// TODO: Initialize with ray and up vector
impl Camera {
    pub fn new(origin: &Vector3<f32>, target: &Vector3<f32>, up: &Vector3<f32>, vertical_fov: f32, aspect_ratio: f32) -> Self {
        let theta = vertical_fov * ::std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect_ratio * half_height;

        let w = (origin - target).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        let lower_left_corner = origin - half_width * u - half_height * v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;

        Camera { lower_left_corner, horizontal, vertical, origin: *origin }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            lower_left_corner: vec3(-2.0, -1.0, -1.0),
            horizontal: vec3(4.0, 0.0, 0.0),
            vertical: vec3(0.0, 2.0, 0.0),
            origin: vec3(0.0, 0.0, 0.0),
        }
    }
}