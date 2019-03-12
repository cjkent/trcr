use Ray;
use vec3::Vec3;

/// The camera and its viewport.
///
/// The location and direction refer to the camera itself.
/// The depth is the distance between the camera and the viewport.
/// The viewport origin is the top left.
/// The direction vector is normalised.
#[derive(Debug)]
pub struct Camera {
    pub px_per_row: u32,
    pub row_count: u32,
    loc: Vec3,
    dir: Vec3,
    depth: f64,
    viewport_width: f64,
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
    pub fn fixed() -> Camera {
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

    pub fn primary_ray(&self, x_idx: u32, y_idx: u32) -> Ray {
        let px_offset = Vec3::new(
            self.px_size * (x_idx as f64),
            -self.px_size * (y_idx as f64),
            0.0
        );
        let px_loc = self.origin_pixel + px_offset;
        Ray::new(self.loc, px_loc - self.loc)
    }
}
