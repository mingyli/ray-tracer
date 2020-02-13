use crate::geometry::{Ray, Vec3};

pub struct Hit {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

#[derive(Debug, Default)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
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
                })
            } else if t_min < t2 && t2 < t_max {
                Some(Hit {
                    t: t2,
                    p: ray.at_time(t2),
                    normal: (ray.at_time(t2) - &self.center) / self.radius,
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
    objects: Vec<Box<dyn Intersectable>>,
}

impl World {
    pub fn new(objects: Vec<Box<dyn Intersectable>>) -> World {
        World { objects }
    }
}

use std::cmp::Ordering;
impl Intersectable for World {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        self.objects
            .iter()
            .filter_map(|obj| obj.intersect(ray, t_min, t_max))
            .min_by(|hit1, hit2| hit1.t.partial_cmp(&hit2.t).unwrap_or(Ordering::Less))
    }
}
