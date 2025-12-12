mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use hittable::HittableList;
use sphere::Sphere;
use std::sync::Arc;

use std::env;

use crate::camera::*;
use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use crate::vec3::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 1600;

    let mut world: HittableList = HittableList::new();

    let lambertian = Arc::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    let metalic_1 = Arc::new(Metalic::new(Colour::new(1.0, 0.0, 0.5), 0.2));
    let metalic_2 = Arc::new(Metalic::new(Colour::new(0.9, 0.2, 0.2), 0.5));
    let glass = Arc::new(Dielectric::new(1.50, Colour::new(0.8, 0.8, 0.9)));
    let bubble = Arc::new(Dielectric::new(1.0 / 1.5, Colour::new(1.0, 1.0, 1.0)));

    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        metalic_1.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        glass.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        bubble.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.5),
        0.5,
        lambertian.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        lambertian.clone(),
    )));

    let center = Point3::new(-3.0, 0.3, -0.5);
    let look_at = Point3::new(0.0, 0.0, -1.7);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    if let Ok(mut cam) = Camera::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        30.0,
        center,
        look_at,
        vup,
        6.0,
        0.6,
        path,
    ) {
        cam.set_samples_per_pixel(200);
        cam.render(&world).unwrap_or_else(|err| {
            eprintln!("Problem Rendering image: {err}");
        });
    };
}
