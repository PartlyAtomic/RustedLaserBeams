extern crate cgmath;

use cgmath::Vector3;

use ray::Ray;
use materials::Material;

pub struct HitRecord<'a> {
    pub t: f32,
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub material: &'a Material,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}