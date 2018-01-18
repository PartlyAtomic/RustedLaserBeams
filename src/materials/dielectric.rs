extern crate cgmath;

use cgmath::Vector3;
use cgmath::vec3;

extern crate rand;

use rand::Rand;

use super::{Material, ScatterResult};
use super::super::ray::Ray;
use super::super::hittable::HitRecord;
use super::metal::reflect;

use cgmath::InnerSpace;

#[derive(Clone, Copy, Debug)]
pub struct Dielectric {
    pub refraction_index: f32
}

fn refract(v: &Vector3<f32>, normal: &Vector3<f32>, ni_over_nt: f32) -> Option<Vector3<f32>> {
    let uv = v.normalize();
    let dt = uv.dot(*normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - normal * dt) - normal * discriminant.sqrt();
        Some(refracted)
    } else {
        None
    }
}

fn schlick(cosine: f32, refraction_index: f32) -> f32 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}


impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let outward_normal;
        let ni_over_nt;
        let cosine;
        if r_in.direction.dot(rec.normal) > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.refraction_index;
            cosine = self.refraction_index * r_in.direction.dot(rec.normal) / r_in.direction.magnitude();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.refraction_index;
            cosine = -r_in.direction.dot(rec.normal) / r_in.direction.magnitude();
        }

        let reflected = reflect(&r_in.direction, &rec.normal);
        let mut rng = rand::thread_rng();
        let is_reflected = f32::rand(&mut rng) < schlick(cosine, self.refraction_index);

        match refract(&r_in.direction, &outward_normal, ni_over_nt) {
            // refracted
            Some(refracted) if !is_reflected => Some(ScatterResult {
                attenuation: vec3(1.0, 1.0, 1.0),
                scattered: Ray { origin: rec.point, direction: refracted },
            }),

            // reflected
            _ => Some(ScatterResult {
                attenuation: vec3(1.0, 1.0, 1.0),
                scattered: Ray { origin: rec.point, direction: reflected },
            }),
        }
    }
}
