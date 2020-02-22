use futures::future;
use std::iter;

use itertools::Itertools;
use rand::Rng;

mod camera;
mod geometry;
mod material;
mod object;

use camera::Camera;
use geometry::{Ray, Vec3};
use object::{Hittable, World};

const NUM_COLS: u32 = 200;
const NUM_ROWS: u32 = 150;
const NUM_SAMPLES: usize = 10;

fn bounce(ray: &Ray, world: &World, depth: u32) -> Vec3 {
    const MAX_DEPTH: u32 = 50;
    if let Some(hit) = world.hit(ray, 0.001, std::f32::MAX) {
        if depth < MAX_DEPTH {
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

async fn pixel_color(camera: &Camera, world: &World, i: u32, j: u32) -> (u32, u32, u32) {
    let mut rng = rand::thread_rng();
    let mut color = iter::repeat_with(|| {
        let u = (i as f32 + rng.gen::<f32>()) / NUM_COLS as f32;
        let v = (j as f32 + rng.gen::<f32>()) / NUM_ROWS as f32;
        camera.ray(u, v)
    })
    .take(NUM_SAMPLES)
    .fold(Vec3::default(), |acc, ray| acc + bounce(&ray, &world, 0));
    color /= NUM_SAMPLES as f32;
    color = Vec3::new(color.r().sqrt(), color.g().sqrt(), color.b().sqrt());

    const RGB_SCALAR: f32 = 255.99;
    let ir = (RGB_SCALAR * color.r()) as u32;
    let ig = (RGB_SCALAR * color.g()) as u32;
    let ib = (RGB_SCALAR * color.b()) as u32;
    (ir, ig, ib)
}

async fn async_main() {
    eprintln!("Ray tracing...");

    let world = World::random();
    let camera = Camera::from_fov(
        Vec3::new(4.0, 1.5, -3.0),
        Vec3::new(0.0, -0.5, 1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        NUM_COLS as f32 / NUM_ROWS as f32,
    );

    let futures = (0..NUM_ROWS)
        .rev()
        .cartesian_product(0..NUM_COLS)
        .map(|(j, i)| pixel_color(&camera, &world, i, j));
    let colors = future::join_all(futures).await;

    eprintln!("Writing out pixel RGB values...");
    println!("P3\n{} {}\n255", NUM_COLS, NUM_ROWS);
    for (r, g, b) in colors.iter() {
        println!("{} {} {}", r, g, b);
    }
}

fn main() {
    futures::executor::block_on(async_main());
}
