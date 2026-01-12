mod bounding;
mod bvh;
mod camera;
mod hittable;
mod material;
mod quad;
mod ray;
mod sphere;
mod texture;
mod vec3;

use hittable::HittableList;
use sphere::Sphere;
use std::sync::Arc;

use std::env;

use crate::bvh::*;
use crate::camera::*;
use crate::hittable::*;
use crate::material::*;
use crate::quad::*;
use crate::ray::*;
use crate::texture::*;
use crate::vec3::*;

fn spheres(world: &mut BvhTree) {
    let earth = Arc::new(Lambertian::new(Arc::new(ImageTexture::new("./earth.jpg"))));
    let wood = Arc::new(Lambertian::new(Arc::new(ImageTexture::new("./wood.jpeg"))));
    let noisy = Arc::new(Lambertian::new(Arc::new(NoiseTexture::new())));

    let lambertian = Arc::new(Lambertian::new(Arc::new(CheckerTexture::new_with_colours(
        0.32,
        Colour::new(0.4, 0.3, 0.2),
        Colour::new(0.9, 0.9, 0.9),
    ))));
    let metalic_1 = Arc::new(Metalic::new(Colour::new(0.8, 0.2, 0.2), 0.3));
    let metalic_2 = Arc::new(Metalic::new(Colour::new(0.9, 0.2, 0.2), 0.5));
    let glass = Arc::new(Dielectric::new(1.50, Colour::new(0.8, 0.8, 0.9)));
    let bubble = Arc::new(Dielectric::new(1.0 / 1.5, Colour::new(1.0, 1.0, 1.0)));

    world.add(Box::new(Sphere::new(
        Ray::new(Point3::new(1.0, 0.5, -1.0), Vec3::new(0.0, 0.0, 0.0), 0.0),
        0.5,
        wood.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Ray::new(Point3::new(-1.0, 0.5, -1.0), Vec3::new(0.0, 0.0, 0.0), 0.0),
        0.5,
        noisy.clone(),
    )));
    // world.add(Box::new(Sphere::new(
    //     Ray::new(Point3::new(-1.0, 0.5, -1.0), Vec3::new(0.0, 0.0, 0.0), 0.0),
    //     0.4,
    //     bubble.clone(),
    // )));
    world.add(Box::new(Sphere::new(
        Ray::new(Point3::new(0.0, 0.5, -1.2), Vec3::new(0.0, 0.0, 0.0), 0.0),
        0.5,
        earth.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Ray::new(
            Point3::new(1.0, -100.0, -1.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.0,
        ),
        100.0,
        lambertian.clone(),
    )));
}

fn quads(world: &mut BvhTree) {
    let lambertian = Arc::new(Lambertian::new(Arc::new(CheckerTexture::new_with_colours(
        0.32,
        Colour::new(0.4, 0.3, 0.2),
        Colour::new(0.9, 0.9, 0.9),
    ))));

    let green = Arc::new(Lambertian::new(Arc::new(SolidColour::new(Colour::new(
        0.1, 1.0, 0.1,
    )))));

    let light = Arc::new(DiffuseLight::from_colour(Colour::new(5.0, 5.0, 5.0)));

    world.add(Box::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        lambertian.clone(),
        |_| {},
    )));

    world.add(Box::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        green.clone(),
        |x| {
            println!("{}", x);
        },
    )));

    world.add(Box::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        lambertian.clone(),
        |_| {},
    )));

    world.add(Box::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        light.clone(),
        |_| {},
    )));
}

fn light(world: &mut BvhTree) {
    let earth = Arc::new(Lambertian::new(Arc::new(ImageTexture::new("./earth.jpg"))));
    let wood = Arc::new(Lambertian::new(Arc::new(ImageTexture::new("./wood.jpeg"))));
    let noisy = Arc::new(Lambertian::new(Arc::new(NoiseTexture::new())));

    let lambertian = Arc::new(Lambertian::new(Arc::new(CheckerTexture::new_with_colours(
        0.32,
        Colour::new(0.4, 0.3, 0.2),
        Colour::new(0.9, 0.9, 0.9),
    ))));
    let metalic_1 = Arc::new(Metalic::new(Colour::new(0.8, 0.2, 0.2), 0.3));
    let metalic_2 = Arc::new(Metalic::new(Colour::new(0.9, 0.2, 0.2), 0.5));
    let glass = Arc::new(Dielectric::new(1.50, Colour::new(0.8, 0.8, 0.9)));
    let bubble = Arc::new(Dielectric::new(1.0 / 1.5, Colour::new(1.0, 1.0, 1.0)));

    let light = Arc::new(DiffuseLight::from_colour(Colour::new(5.0, 5.0, 5.0)));

    world.add(Box::new(Quad::new(
        Point3::new(-2.0, 2.0, -5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        light.clone(),
        |_| {},
    )));

    world.add(Box::new(Sphere::new(
        Ray::new(Point3::new(1.0, 0.5, -1.0), Vec3::new(0.0, 0.0, 0.0), 0.0),
        0.5,
        wood.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Ray::new(Point3::new(-1.0, 0.5, -1.0), Vec3::new(0.0, 0.0, 0.0), 0.0),
        0.5,
        noisy.clone(),
    )));
    // world.add(Box::new(Sphere::new(
    //     Ray::new(Point3::new(-1.0, 0.5, -1.0), Vec3::new(0.0, 0.0, 0.0), 0.0),
    //     0.4,
    //     bubble.clone(),
    // )));
    world.add(Box::new(Sphere::new(
        Ray::new(Point3::new(0.0, 0.5, -1.2), Vec3::new(0.0, 0.0, 0.0), 0.0),
        0.5,
        earth.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Ray::new(
            Point3::new(1.0, -100.0, -1.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.0,
        ),
        100.0,
        lambertian.clone(),
    )));
}

fn boxes(world: &mut BvhTree) {
    let red = Lambertian::as_arc(SolidColour::as_arc_from_rgb(0.65, 0.05, 0.05));
    let white = Lambertian::as_arc(SolidColour::as_arc_from_rgb(0.73, 0.73, 0.73));
    let green = Lambertian::as_arc(SolidColour::as_arc_from_rgb(0.12, 0.45, 0.15));
    let light = DiffuseLight::as_arc_from_colour(Colour::new(15.0, 15.0, 15.0));

    world.add(Quad::boxed(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
        |_| {},
    ));
    world.add(Quad::boxed(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
        |_| {},
    ));
    world.add(Quad::boxed(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
        |_| {},
    ));
    world.add(Quad::boxed(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
        |_| {},
    ));
    world.add(Quad::boxed(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
        |_| {},
    ));
    world.add(Quad::boxed(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
        |_| {},
    ));

    let cube1 = Cube::boxed(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let rotate1 = RotateY::boxed(cube1, 15.0);
    world.add(Translate::boxed(rotate1, &Vec3::new(265.0, 0.0, 295.0)));

    let cube2 = Cube::boxed(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let rotate2 = RotateY::boxed(cube2, -18.0);
    world.add(Translate::boxed(rotate2, &Vec3::new(130.0, 0.0, 65.0)));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 800;

    let mut world: BvhTree = BvhTree::new();

    boxes(&mut world);

    let center = Point3::new(278.0, 278.0, -800.0);
    let look_at = Point3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    if let Ok(mut cam) = Camera::new(
        ASPECT_RATIO,
        IMAGE_WIDTH,
        40.0,
        center,
        look_at,
        vup,
        3.5,
        0.0,
        path,
    ) {
        cam.set_samples_per_pixel(2000);
        cam.set_max_depth(50);
        cam.render(&world).unwrap_or_else(|err| {
            eprintln!("Problem Rendering image: {err}");
        });
    };
}
