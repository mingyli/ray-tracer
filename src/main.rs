use futures::future;
use std::iter;

use indicatif::ProgressBar;
use itertools::Itertools;
use rand::Rng;

mod camera;
mod geometry;
mod material;
mod object;

use camera::Camera;
use geometry::{Ray, Vec3};
use object::{Hittable, World};

fn bounce(config: &Config, ray: &Ray, world: &World, depth: u32) -> Vec3 {
    if let Some(hit) = world.hit(ray, 0.001, std::f32::MAX) {
        if depth < config.max_depth {
            if let Some((attenuation, scattered)) = hit.material.scatter(&ray, &hit) {
                attenuation * bounce(config, &scattered, world, depth + 1)
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

async fn pixel_color(
    config: &Config,
    camera: &Camera,
    world: &World,
    i: u32,
    j: u32,
) -> (u32, u32, u32) {
    let mut rng = rand::thread_rng();
    let rays = iter::repeat_with(|| {
        let u = (i as f32 + rng.gen::<f32>()) / config.width as f32;
        let v = (j as f32 + rng.gen::<f32>()) / config.height as f32;
        (u, v)
    })
    .map(|(u, v)| camera.ray(u, v))
    .take(config.samples);

    let colors =
        future::join_all(rays.map(|ray| async move { bounce(&config, &ray, &world, 0) })).await;

    let color: Vec3 = colors.into_iter().sum::<Vec3>() / config.samples as f32;
    let color = color.gamma2_corrected();

    const RGB_SCALAR: f32 = 255.99;
    let ir = (RGB_SCALAR * color.r()) as u32;
    let ig = (RGB_SCALAR * color.g()) as u32;
    let ib = (RGB_SCALAR * color.b()) as u32;
    (ir, ig, ib)
}

async fn async_main(config: &Config) {
    eprintln!("Ray tracing...");

    let world = World::random();
    let camera = Camera::from_fov(
        Vec3::new(4.0, 1.5, -3.0),
        Vec3::new(0.0, -0.5, 1.0),
        Vec3::new(0.0, 1.0, 0.0),
        90.0,
        config.width as f32 / config.height as f32,
    );

    let futures = (0..config.height)
        .rev()
        .cartesian_product(0..config.width)
        .map(|(j, i)| pixel_color(&config, &camera, &world, i, j));

    let progress = ProgressBar::new((config.width * config.height).into());
    let colors = future::join_all(futures.map(|future| async {
        let result = future.await;
        progress.inc(1);
        result
    }))
    .await;
    progress.finish_at_current_pos();

    eprintln!("Writing out pixel RGB values...");
    println!("P3\n{} {}\n255", config.width, config.height);
    for (r, g, b) in colors.iter() {
        println!("{} {} {}", r, g, b);
    }
}

struct Config {
    width: u32,
    height: u32,
    samples: usize,
    max_depth: u32,
}

fn main() -> Result<(), clap::Error> {
    use clap::{value_t, App, Arg};
    let matches = App::new("Rust Ray Tracer")
        .arg(
            Arg::with_name("width")
                .help("The width of the output image in pixels.")
                .short("w")
                .long("width")
                .takes_value(true)
                .default_value("200"),
        )
        .arg(
            Arg::with_name("height")
                .help("The height of the output image in pixels.")
                .short("h")
                .long("height")
                .takes_value(true)
                .default_value("150"),
        )
        .arg(
            Arg::with_name("samples")
                .help("The number of samples per pixel.")
                .short("s")
                .long("samples")
                .takes_value(true)
                .default_value("10"),
        )
        .arg(
            Arg::with_name("max_depth")
                .help("The maximum number of bounces per ray.")
                .short("d")
                .long("max_depth")
                .takes_value(true)
                .default_value("50"),
        )
        .get_matches();

    let width = value_t!(matches, "width", u32)?;
    let height = value_t!(matches, "height", u32)?;
    let samples = value_t!(matches, "samples", usize)?;
    let max_depth = value_t!(matches, "max_depth", u32)?;

    let config = Config {
        width,
        height,
        samples,
        max_depth,
    };

    futures::executor::block_on(async_main(&config));
    Ok(())
}
