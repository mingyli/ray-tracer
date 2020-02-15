use crate::geometry::{Ray, Vec3};
use crate::object::Hit;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let target = &hit.p + &hit.normal + Vec3::sample_from_unit_sphere();
        let scattered = Ray::new(hit.p.clone(), target - &hit.p);
        let attenuation = self.albedo.clone();
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let reflected = Vec3::reflect(&ray.direction().normalized(), &hit.normal);
        let scattered = Ray::new(
            hit.p.clone(),
            reflected + self.fuzz * Vec3::sample_from_unit_sphere(),
        );
        let attenuation = self.albedo.clone();
        if scattered.direction().dot(&hit.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
