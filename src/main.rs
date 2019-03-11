#[macro_use]
extern crate bmp;

use bmp::{Image, Pixel};

use vec3::Vec3;

use crate::sphere::Sphere;

mod sphere;
mod vec3;

const BACKGROUND_COLOUR: Colour = Colour { r: 0xD0, g: 0xD0, b: 0xFF };

fn main() {
    let sphere = Sphere {
        centre: Vec3::new(0.0, 0.0, -4.0),
        radius: 1.0,
        colour: Colour::from_24bit_int(0xF4AE22)
    };
    let camera = Camera::fixed();
    let scene = Scene {
        objects: vec![Box::new(sphere)],
    };
    let pixel_colours = render(&scene, &camera);
    let mut img = Image::new(camera.px_per_row, camera.row_count);
    let mut idx = 0;
    for y in 0..camera.row_count {
        for x in 0..camera.px_per_row {
            let colour = pixel_colours[idx];
            img.set_pixel(x, y, colour.pixel());
        }
    }
    let _ = img.save("/Users/chris/tmp/trcr.bmp");
}

fn render(scene: &Scene, camera: &Camera) -> Vec<Colour> {
    let mut pixel_colours: Vec<Colour> = vec![];
    for y in 0..camera.row_count {
        for x in 0..camera.px_per_row {
            let ray = camera.primary_ray(x, y);
            let pixel_colour = trace(&ray, &scene.objects);
            pixel_colours.push(pixel_colour);
        }
    }
    pixel_colours
}

fn trace(ray: &Ray, objects: &Vec<Box<dyn SceneObject>>) -> Colour {
    // The closest point found so far where the ray hits an object
    let mut intersect: Option<RayIntersection> = None;
    for object in objects.iter() {
        if let Some(distance) = object.intersect(&ray) {
            if let Some(RayIntersection { object: _, distance: min_distance }) = intersect {
                // An intersection point has been found before, check whether this one is closer
                if distance < min_distance {
                    // This point is the closest found so far, keep it
                    intersect = Some(RayIntersection { object, distance })
                }
            } else {
                // This is the first point found, keep it
                intersect = Some(RayIntersection { object, distance })
            }
        }
    }
    if let Some(RayIntersection { object, distance }) = intersect {
        let intersect_point = ray.source + ray.dir * distance;
        object.colour(&intersect_point)
    } else {
        BACKGROUND_COLOUR
    }
}

struct RayIntersection<'a> {
    object: &'a Box<dyn SceneObject>,
    distance: f64,
}

struct Ray {
    source: Vec3,
    dir: Vec3,
}

// TODO should there be a Material trait for some of these? how would that interact with this trait?
//   intersect() and surface_normal() both belong to the object geometry
//   secondary_rays() is trickier as it seems to be both. or is it on material with the normal passed in?
//   and if the object and material are separated, what holds them?
//   or does a SceneObject struct hold a Material and ObjectGeometry?
//   colour() belongs to the material, but in the case of textures the object geometry has an effect too
//   should the geometry have a method to map from a point to texture coordinates?
trait SceneObject {
    /// Returns the intersection point as a distance along the ray from its source.
    fn intersect(&self, ray: &Ray) -> Option<f64>;

    fn secondary_rays(&self, point: &Vec3) -> Vec<Ray>;

    fn surface_normal(&self, point: &Vec3) -> Vec3;

    fn colour(&self, point: &Vec3) -> Colour;
}

struct Scene {
    objects: Vec<Box<dyn SceneObject>>,
}

/// The camera and its viewport.
///
/// The location and direction refer to the camera itself.
/// The depth is the distance between the camera and the viewport.
/// The viewport origin is the top left.
/// The direction vector is normalised.
struct Camera {
    loc: Vec3,
    dir: Vec3,
    depth: f64,
    viewport_width: f64,
    px_per_row: u32,
    row_count: u32,
    px_size: f64,
    origin_pixel: Vec3,
}

impl Camera {
    /// Returns a camera:
    ///   * Located at the origin
    ///   * Looking down the z-axis
    ///   * 1 unit gap between the camera and viewport
    ///   * Viewport ratio 1/1
    ///   * Viewport width 2 units
    ///   * Viewport width 200 pixels
    fn fixed() -> Camera {
        let viewport_width = 2.0;
        let px_per_row = 200;
        let row_count = 200;
        let viewport_origin = Vec3::new(-1.0, 1.0, -1.0);
        let px_size = viewport_width / (px_per_row as f64);
        let origin_pixel = viewport_origin + (Vec3::new(px_size, -px_size, 0.0) / 2.0);
        Camera {
            loc: Vec3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(0.0, 0.0, -1.0),
            depth: 1.0,
            viewport_width,
            px_per_row,
            row_count,
            px_size,
            origin_pixel,
        }
    }

    fn primary_ray(&self, x_idx: u32, y_idx: u32) -> Ray {
        let px_offset = Vec3::new(self.px_size * (x_idx as f64), -self.px_size * (y_idx as f64), 0.0);
        let px_loc = self.origin_pixel + px_offset;
        Ray {
            source: self.loc,
            dir: px_loc - self.loc,
        }
    }
}

/// A 24-bit RGB colour.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Colour {
pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8) -> Colour {
        Colour { r, g, b }
    }

    pub fn from_24bit_int(colour: u32) -> Colour {
        let red = (colour & 0xff0000) >> 16;
        let green = (colour & 0x00ff00) >> 8;
        let blue = colour & 0x0000ff;
        Colour::new(red as u8, green as u8, blue as u8)
    }

    pub fn pixel(&self) -> Pixel {
        px!(self.r, self.g, self.b)
    }
}

