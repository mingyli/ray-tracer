use std::iter;

use rand::Rng;

mod camera;
mod geometry;
mod object;

use camera::Camera;
use geometry::{Ray, Vec3};
use object::{Intersectable, Sphere, World};

fn make_color(ray: &Ray, world: &World) -> Vec3 {
    if let Some(hit) = world.intersect(ray, 0.0, std::f32::MAX) {
        0.5 * Vec3::new(
            hit.normal.x() + 1.0,
            hit.normal.y() + 1.0,
            hit.normal.z() + 1.0,
        )
    } else {
        let direction = ray.direction().normalized();
        let t = 0.5 * (direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx = 600;
    let ny = 300;
    let ns = 10;

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let earth = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    let world = World::new(vec![Box::new(sphere), Box::new(earth)]);

    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
    );

    println!("P3\n{} {}\n255", nx, ny);

    let mut rng = rand::thread_rng();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let color = iter::repeat_with(|| {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                camera.ray(u, v)
            })
            .take(ns)
            .fold(Vec3::default(), |acc, ray| acc + make_color(&ray, &world))
                / ns as f32;

            let ir = (255.99 * color.r()) as i32;
            let ig = (255.99 * color.g()) as i32;
            let ib = (255.99 * color.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
