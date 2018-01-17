extern crate cgmath;

use cgmath::Vector3;

use super::{Material, ScatterResult};
use super::super::ray::Ray;
use super::super::hittable::HitRecord;
use super::super::random_in_unit_sphere;

#[derive(Clone, Copy, Debug)]
pub struct Lambertian {
    pub albedo: Vector3<f32>
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let target = rec.point + rec.normal + random_in_unit_sphere();
        Some(ScatterResult {
            scattered: Ray { origin: rec.point, direction: target - rec.point },
            attenuation: self.albedo,
        })
    }
}