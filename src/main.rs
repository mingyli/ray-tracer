use std::iter;

use rand::Rng;

mod camera;
mod geometry;
mod material;
mod object;

use camera::Camera;
use geometry::{Ray, Vec3};
use material::{Dielectric, Lambertian, Metal};
use object::{Hittable, Sphere, World};

fn bounce(ray: &Ray, world: &World, depth: i32) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.001, std::f32::MAX) {
        if depth < 50 {
            if let Some((attenuation, scattered)) = hit.material.scatter(&ray, &hit) {
                attenuation * bounce(&scattered, world, depth + 1)
            } else {
                Vec3::default()
            }
        } else {
            Vec3::default()
        }
    } else {
        let direction = ray.direction().normalized();
        let t = 0.5 * (direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx = 400;
    let ny = 200;
    let ns = 10;

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
    let world = World::new(vec![
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
    ]);

    let camera = Camera::from_fov(
        Vec3::new(-2.0, 2.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        nx as f32 / ny as f32,
    );

    println!("P3\n{} {}\n255", nx, ny);

    let mut rng = rand::thread_rng();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = iter::repeat_with(|| {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                camera.ray(u, v)
            })
            .take(ns)
            .fold(Vec3::default(), |acc, ray| acc + bounce(&ray, &world, 0));
            color /= ns as f32;
            color = Vec3::new(color.r().sqrt(), color.g().sqrt(), color.b().sqrt());

            let ir = (255.99 * color.r()) as i32;
            let ig = (255.99 * color.g()) as i32;
            let ib = (255.99 * color.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
