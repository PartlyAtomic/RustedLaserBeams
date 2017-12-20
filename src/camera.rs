extern crate cgmath;

use std::default::Default;

use cgmath::Vector3;
use cgmath::vec3;
use ray::Ray;

pub struct Camera {
    pub lower_left_corner: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub origin: Vector3<f32>,
}

impl Camera {
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