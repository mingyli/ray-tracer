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
        let target = &hit.p + &hit.normal + Vec3::sample_in_unit_sphere();
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
            reflected + self.fuzz * Vec3::sample_in_unit_sphere(),
        );
        let attenuation = self.albedo.clone();
        if scattered.direction().dot(&hit.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Dielectric {
        Dielectric { refractive_index }
    }

    fn schlick(cosine: f32, refractive_index: f32) -> f32 {
        let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        r0 * r0 + (1.0 - r0 * r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Vec3, Ray)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let reflected = Vec3::reflect(&ray.direction(), &hit.normal);

        let (outward_normal, ni, nt, cosine) = if ray.direction().dot(&hit.normal) > 0.0 {
            let cosine =
                self.refractive_index * ray.direction().dot(&hit.normal) / ray.direction().length();
            (-hit.normal.clone(), self.refractive_index, 1.0, cosine)
        } else {
            let cosine = -ray.direction().dot(&hit.normal) / ray.direction().length();
            (hit.normal.clone(), 1.0, self.refractive_index, cosine)
        };

        let scattered =
            if let Some(refracted) = Vec3::refract(&ray.direction(), &outward_normal, ni, nt) {
                let reflection_probability = Dielectric::schlick(cosine, self.refractive_index);
                if rand::random::<f32>() < reflection_probability {
                    Ray::new(hit.p.clone(), reflected)
                } else {
                    Ray::new(hit.p.clone(), refracted)
                }
            } else {
                Ray::new(hit.p.clone(), reflected)
            };
        Some((attenuation, scattered))
    }
}
