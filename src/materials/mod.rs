pub mod lambertian;
pub mod metal;

extern crate cgmath;

use cgmath::Vector3;
use ray::Ray;
use hittable::HitRecord;

pub struct ScatterResult {
    pub attenuation: Vector3<f32>,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult>;
}