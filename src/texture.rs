use std::path::Path;
use std::sync::Arc;

use crate::vec3::*;
use image::{open, ImageBuffer, RgbImage};
use noise::{NoiseFn, Perlin, Turbulence};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: Point3) -> Colour;
}

pub struct SolidColour {
    colour: Colour,
}

impl SolidColour {
    pub fn new(a: Colour) -> SolidColour {
        SolidColour { colour: a }
    }

    pub fn as_arc(a: Colour) -> Arc<SolidColour> {
        Arc::new(SolidColour { colour: a })
    }

    pub fn as_arc_from_rgb(r: f64, g: f64, b: f64) -> Arc<SolidColour> {
        Arc::new(SolidColour {
            colour: Colour::new(r, g, b),
        })
    }
}

impl Texture for SolidColour {
    fn value(&self, u: f64, v: f64, p: Point3) -> Colour {
        return self.colour;
    }
}

pub struct CheckerTexture {
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
    scale: f64,
}

impl CheckerTexture {
    pub fn new_with_colours(scale: f64, a: Colour, b: Colour) -> CheckerTexture {
        CheckerTexture {
            even: Box::new(SolidColour::new(a)),
            odd: Box::new(SolidColour::new(b)),
            scale: 1.0 / scale,
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Colour {
        let xint = f64::floor(p.axis(0) * self.scale) as i32;
        let yint = f64::floor(p.axis(1) * self.scale) as i32;
        let zint = f64::floor(p.axis(2) * self.scale) as i32;

        let is_even = (xint + yint + zint).rem_euclid(2) == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

pub struct ImageTexture {
    image: RgbImage,
}

impl ImageTexture {
    pub fn new<P>(path: P) -> ImageTexture
    where
        P: AsRef<Path>,
    {
        let image = open(path).expect("Image couldn't be opened").into_rgb8();
        ImageTexture { image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Colour {
        let (image_width, image_height) = self.image.dimensions();

        let uclamp = f64::clamp(u, 0.0, 1.0);
        let vclamp = 1.0 - f64::clamp(v, 0.0, 1.0);

        let x = uclamp * image_width as f64;
        let y = vclamp * image_height as f64;

        let pixel = self.image.get_pixel(x as u32, y as u32);

        let colour_scale = 1.0 / 255.0;

        let r = pixel.0[0] as f64 * colour_scale;
        let g = pixel.0[1] as f64 * colour_scale;
        let b = pixel.0[2] as f64 * colour_scale;

        Colour::new(r, g, b)
    }
}

pub struct NoiseTexture {
    noise: Turbulence<Perlin, Perlin>,
}

impl NoiseTexture {
    pub fn new() -> NoiseTexture {
        let mut noise = Turbulence::new(Perlin::new(1));
        noise = noise.set_frequency(150.0);
        NoiseTexture { noise }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Colour {
        let point = [u, v];

        let noise = self.noise.get(point);

        Colour::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + noise)
    }
}
