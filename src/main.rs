use vec3::Vec3;

use crate::sphere::Sphere;

mod sphere;
mod vec3;

fn main() {
    let sphere = Sphere {
        centre: Vec3::new(0.0, 0.0, -4.0),
        radius: 1.0,
    };
    let camera = Camera::fixed();
    let scene = Scene {
        objects: vec![sphere],
    };

}

struct Ray {
    source: Vec3,
    dir: Vec3,
}

// TODO should there be a Material trait for some of these? how would that interact with this trait?
//   intersect() and surface_normal() both belong to the object geometry
//   colour() belongs to the material
//   secondary_rays() is trickier as it seems to be both. or is it on material with the normal passed in?
//   and if the object and material are separated, what holds them?
//   or does a SceneObject struct hold a Material and ObjectGeometry?
trait SceneObject {
    /// Returns the intersection point as a distance along the ray from its source.
    fn intersect(&self, ray: Ray) -> Option<f64>;

    fn secondary_rays(&self, point: Vec3) -> Vec<Ray>;

    fn surface_normal(&self, point: Vec3) -> Vec3;

    fn colour(&self, point: Vec3) -> Vec3;
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
    viewport_ratio: f64,
    px_per_row: u32,
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
        let viewport_origin = Vec3::new(-1.0, 1.0, -1.0);
        let px_size = viewport_width / (px_per_row as f64);
        let origin_pixel = viewport_origin + (Vec3::new(px_size, -px_size, 0.0) / 2);
        Camera {
            loc: Vec3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(0.0, 0.0, -1.0),
            depth: 1.0,
            viewport_width,
            viewport_ratio: 4.0 / 3.0,
            px_per_row,
            px_size,
            origin_pixel,
        }
    }

    fn pixel_count(&self) -> u32 {
        let rows = (self.px_per_row as f64 / self.viewport_ratio) as u32;
        self.px_per_row * rows
    }

    fn primary_ray(&self, x_idx: u32, y_idx: u32) -> Ray {
        let px_offset = Vec3::new(self.px_size * x_idx, -self.px_size * y_idx, 0.0);
        let px_loc = self.origin_pixel + px_offset;
        Ray {
            source: self.loc,
            dir: px_loc - self.loc,
        }
    }

    fn render(scene: Scene) -> Vec<u32> {
        vec![]
    }
}

/// A 24-bit RGB colour.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Colour {
    r: u8,
    g: u8,
    b: u8,
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

    pub fn to_24bit_int(&self) -> u32 {
        // TODO
        0x000000
    }
}

