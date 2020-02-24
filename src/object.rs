use std::cmp::Ordering;

use crate::geometry::{Ray, Vec3};
use crate::material::{Dielectric, Lambertian, Material, Metal};

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
    objects: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl World {
    pub fn new(objects: Vec<Box<dyn Hittable + Send + Sync>>) -> World {
        World { objects }
    }

    pub fn demo() -> World {
        let sphere = Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Vec3::new(0.1, 0.2, 0.5)),
        );
        let earth = Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(Vec3::new(0.8, 0.8, 0.0)),
        );
        World::new(vec![
            Box::new(sphere),
            Box::new(earth),
            Box::new(Sphere::new(
                Vec3::new(1.0, 0.0, -1.0),
                0.5,
                Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0),
            )),
            Box::new(Sphere::new(
                Vec3::new(-1.0, 0.0, -1.0),
                0.5,
                Dielectric::new(1.5),
            )),
            Box::new(Sphere::new(
                Vec3::new(-1.0, 0.0, -1.0),
                -0.45,
                Dielectric::new(1.5),
            )),
        ])
    }

    pub fn random() -> World {
        use rand::Rng;
        let earth = Sphere::new(
            Vec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Lambertian::new(Vec3::new(0.5, 0.5, 0.5)),
        );
        let mut objects: Vec<Box<dyn Hittable + Send + Sync>> = vec![Box::new(earth)];
        let mut rng = rand::thread_rng();
        for x in (-11)..11 {
            for z in (-11)..11 {
                let center = Vec3::new(
                    (x as f32) + 0.9 * rng.gen::<f32>(),
                    0.2,
                    (z as f32) + 0.9 * rng.gen::<f32>(),
                );
                if (&center - Vec3::new(4.0, 0.2, 0.0)).length() < 0.9 {
                    continue;
                }
                let r: f32 = rng.gen::<f32>();
                if r < 0.8 {
                    objects.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Lambertian::new(Vec3::new(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        )),
                    )));
                } else if r < 0.9 {
                    objects.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                            ),
                            0.5 * rng.gen::<f32>(),
                        ),
                    )));
                } else {
                    objects.push(Box::new(Sphere::new(
                        center.clone(),
                        0.2,
                        Dielectric::new(1.5),
                    )));
                    objects.push(Box::new(Sphere::new(center, -0.195, Dielectric::new(1.5))));
                };
            }
        }
        objects.push(Box::new(Sphere::new(
            Vec3::new(0.0, 1.0, 0.0),
            1.0,
            Dielectric::new(1.5),
        )));
        objects.push(Box::new(Sphere::new(
            Vec3::new(-3.0, 1.0, 0.5),
            1.0,
            Lambertian::new(Vec3::new(0.2, 0.2, 0.6)),
        )));
        objects.push(Box::new(Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Metal::new(Vec3::new(0.8, 0.6, 0.7), 0.0),
        )));
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
