extern crate cgmath;

use cgmath::Vector3;

use materials::Material;

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: Box<Material>,
}

use hittable::Hittable;
use hittable::HitRecord;
use ray::Ray;

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        use cgmath::dot;

        let oc = ray.origin - self.center;
        let a = dot(ray.direction, ray.direction);
        let b = dot(oc, ray.direction);
        let c = dot(oc, oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        use std::ops::Deref;
        if discriminant >= 0.0 {
            let solution1 = (-b - discriminant.sqrt()) / a;
            if solution1 > t_min && solution1 < t_max {
                let point = ray.point_at_parameter(solution1);
                return Some(HitRecord {
                    t: solution1,
                    point,
                    normal: (point - self.center) / self.radius,
                    material: self.material.deref(),
                });
            }

            let solution2 = (-b + discriminant.sqrt()) / a;
            if solution2 > t_min && solution2 < t_max {
                let point = ray.point_at_parameter(solution2);
                return Some(HitRecord {
                    t: solution2,
                    point,
                    normal: (point - self.center) / self.radius,
                    material: self.material.deref(),
                });
            }
        }

        None
    }
}

