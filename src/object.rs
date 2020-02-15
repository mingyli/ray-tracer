use std::cmp::Ordering;

use crate::geometry::{Ray, Vec3};
use crate::material::Material;

pub struct Hit<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub struct Sphere<M: Material> {
    center: Vec3,
    radius: f32,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3, radius: f32, material: M) -> Sphere<M> {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.origin() - &self.center;
        let a = ray.direction().dot(ray.direction());
        let b = oc.dot(ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / a;
            let t2 = (-b + discriminant.sqrt()) / a;

            if t_min < t1 && t1 < t_max {
                Some(Hit {
                    t: t1,
                    p: ray.at_time(t1),
                    normal: (ray.at_time(t1) - &self.center) / self.radius,
                    material: &self.material,
                })
            } else if t_min < t2 && t2 < t_max {
                Some(Hit {
                    t: t2,
                    p: ray.at_time(t2),
                    normal: (ray.at_time(t2) - &self.center) / self.radius,
                    material: &self.material,
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Default)]
pub struct World {
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> World {
        World { objects }
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.objects
            .iter()
            .filter_map(|obj| obj.hit(ray, t_min, t_max))
            .min_by(|hit1, hit2| hit1.t.partial_cmp(&hit2.t).unwrap_or(Ordering::Less))
    }
}
