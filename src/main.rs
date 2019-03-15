#[macro_use]
extern crate bmp;
extern crate log;

use std::ops::{Add, Mul};

use bmp::{Image, Pixel};
use log::trace;

use vec3::Vec3;

use crate::camera::Camera;
use crate::sphere::Sphere;

mod camera;
mod sphere;
mod vec3;

static BACKGROUND_COLOUR: Colour = Colour { r: 0.4, g: 0.4, b: 1.0 };
static BLACK: Colour = Colour { r: 0.0, g: 0.0, b: 0.0 };

fn main() {
    let sphere = Sphere {
        centre: Vec3::new(0.0, 0.0, -2.0),
        radius: 1.0,
        colour: Colour::from_24bit_int(0xF4AE22)
    };
    let camera = Camera::fixed();
    let scene = Scene {
        objects: vec![Box::new(sphere)],
        lights: vec![Light::Distant { dir: Vec3::new(-1.0, -5.0, -1.0).normalised() }]
    };
    let pixels = render(&scene, &camera);
    // TODO normalise the intensities and populate this
    let colours: Vec<Colour> = vec![];
    let mut img = Image::new(camera.px_per_row, camera.row_count);
    let mut idx = 0;
    for y in 0..camera.row_count {
        for x in 0..camera.px_per_row {
            let colour = colours[idx];
            img.set_pixel(x, y, colour.pixel());
            idx += 1;
        };
    };
    if let Err(err) = img.save("/Users/chris/tmp/trcr.bmp") {
        println!("Error saving image {}", err);
    }
}

fn render(scene: &Scene, camera: &Camera) -> Vec<Intensity> {
    let mut pixels: Vec<Intensity> = vec![];
    for y in 0..camera.row_count {
        for x in 0..camera.px_per_row {
            let ray = camera.primary_ray(x, y);
            let pixel_intensity = trace(&ray, scene);
            pixels.push(pixel_intensity);
        }
    }
    pixels
}

fn trace(ray: &Ray, scene: &Scene) -> Intensity {
    // The closest point found so far where the ray hits an object
    let mut intersect: Option<RayIntersection> = None;
    for object in scene.objects.iter() {
        if let Some(distance) = object.intersect(&ray) {
            if let Some(RayIntersection { object: _, distance: min_distance }) = intersect {
                // An intersection point has been found before, check whether this one is closer
                if distance < min_distance {
                    // This point is the closest found so far, keep it
                    trace!("closer point found {:?}", distance);
                    intersect = Some(RayIntersection { object, distance })
                }
            } else {
                // This is the first point found, keep it
                trace!("new point found {:?}", distance);
                intersect = Some(RayIntersection { object, distance })
            }
        }
    }
    if let Some(RayIntersection { object, distance }) = intersect {
        let intersect_point = ray.source + ray.dir * distance;
        // TODO
        //   * reflection rays
        //   * refraction rays
        // TODO this logic is wrong - a point can be in shadow for one light and not for another
        //   the light from all the non-shadow lights need to be summed
        if shadow_rays(&intersect_point, scene).is_empty() {
            Intensity::new(0.0, 0.0, 0.0)
        } else {
//            let normal = object.surface_normal(&intersect_point);
//            normal.dot()
            Intensity::new(1.0, 1.0, 1.0) * object.colour(&intersect_point)
        }
    } else {
        Intensity::new(1.0, 1.0, 1.0) * BACKGROUND_COLOUR
    }
}

/// Returns the shadow rays that illuminates the point
/// The vector is empty if the point is in shadow.
fn shadow_rays(point: &Vec3, scene: &Scene) -> Vec<Ray> {
    // for each light
    //   calculate ray from point to light
    //   check for intersections with objects
    //   for each intersection
    //     check distance is +ve (i.e. object is between point and light)
    //     check intersection is closer to point than the light is
    //     if both true
    //       point is in shadow
    let mut rays: Vec<Ray> = vec![];
    'light: for light in scene.lights.iter() {
        // The direction the light shines on the point
        let light_dir = light.direction(point);
        let shadow_ray = Ray::new(*point, -light_dir);
        for object in scene.objects.iter() {
            if let Some(distance) = object.intersect(&shadow_ray) {
                let shadow_point = shadow_ray.source + shadow_ray.dir * distance;
                if distance >= 0.0 && distance < light.distance(&shadow_point) {
                    continue 'light;
                }
            }
        }
        rays.push(shadow_ray.clone());
    }
    rays
}

struct RayIntersection<'a> {
    object: &'a Box<dyn SceneObject>,
    distance: f64,
}

#[derive(Debug, Clone)]
pub struct Ray {
    pub source: Vec3,
    pub dir: Vec3,
}

impl Ray {
    /// Creates a new ray whose origin is `source` and whose direction is `dir`.
    ///
    /// The ray direction is normalised.
    fn new(source: Vec3, dir: Vec3) -> Ray {
        Ray {
            source,
            dir: dir.normalised()
        }
    }
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
    lights: Vec<Light>,
}

/// An RGB colour; each component has a value between 0 and 1 inclusive.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Colour {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour { r, g, b }
    }

    pub fn from_24bit_int(colour: u32) -> Colour {
        let red = (colour & 0xff0000) >> 16;
        let green = (colour & 0x00ff00) >> 8;
        let blue = colour & 0x0000ff;
        Colour::new((red as f64) / 255.0, (green as f64) / 255.0, (blue as f64) / 255.0)
    }

    pub fn pixel(&self) -> Pixel {
        px!((self.r * 255.0) as u8, (self.g * 255.0) as u8, (self.b * 255.0) as u8)
    }
}

/// The intensity of light at a point; components can be zero or greater
#[derive(Debug, Clone, Copy)]
struct Intensity {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Intensity {
    fn new(r: f64, g: f64, b: f64) -> Intensity {
        Intensity { r, b, g}
    }
}

impl Add for Intensity {
    type Output = Intensity;

    fn add(self, other: Intensity) -> Intensity {
        Intensity {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Mul<Colour> for Intensity {
    type Output = Intensity;

    fn mul(self, colour: Colour) -> Intensity {
        Intensity {
            r: self.r * colour.r,
            g: self.g * colour.g,
            b: self.b * colour.b,
        }
    }
}

impl Mul<f64> for Intensity {
    type Output = Intensity;

    fn mul(self, factor: f64) -> Intensity {
        Intensity {
            r: self.r * factor,
            g: self.g * factor,
            b: self.b * factor,
        }
    }
}

// TODO colour and intensity
enum Light {
    Point { loc: Vec3 },
    Distant { dir: Vec3 },
}

impl Light {
    fn distance(&self, loc: &Vec3) -> f64 {
        match self {
            Light::Point { loc: light_loc } => (*light_loc - *loc).mag(),
            Light::Distant { dir: _ } => std::f64::INFINITY,
        }
    }

    /// The direction of the light falling on the point, orientated from the light to the point.
    fn direction(&self, point: &Vec3) -> Vec3 {
        match self {
            Light::Point { loc } => *point - *loc,
            Light::Distant { dir } => *dir,
        }
    }
}
