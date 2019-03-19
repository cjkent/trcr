use log::trace;

use crate::{Colour, Ray, SceneObject};
use crate::vec3::Vec3;

pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
    pub colour: Colour,
}

impl SceneObject for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        // See here for a diagram labelled with the distances
        //   https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection
        let l = self.centre - ray.source;
        let tca = l.dot(&ray.dir);
        if tca < 0.0 {
            return None;
        }
        let d2 = l.dot(&l) - tca * tca;
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 && t1 < 0.0 {
            None
        } else if t0 < 0.0 && t1 > 0.0 {
            Some(t1)
        } else if t0 > 0.0 && t1 < 0.0 {
            Some(t0)
        } else {
            Some(if t0 <= t1 { t0 } else { t1 })
        }
    }

    fn secondary_rays(&self, _point: &Vec3) -> Vec<Ray> {
        // TODO this is only right for diffuse objects
        vec![]
    }

    fn surface_normal(&self, point: &Vec3) -> Vec3 {
        (*point - self.centre).normalised()
    }

    fn colour(&self, _point: &Vec3) -> Colour {
        self.colour
    }
}

/// A finite plane with edges parallel to the X and Z axes.
pub struct XzPlane {
    pub y: f64,
    pub x_min: f64,
    pub x_max: f64,
    pub z_min: f64,
    pub z_max: f64,
    pub colour: Colour,
    p0: Vec3,
    normal: Vec3,
}

impl XzPlane {

    pub fn new(y: f64, x_min: f64, x_max: f64, z_min: f64, z_max: f64, colour: Colour) -> XzPlane {
        let p0 = Vec3::new(x_min, y, z_min);
        let normal = Vec3::new(0.0, 1.0, 0.0).normalised();
        XzPlane { y, x_min, x_max, z_min, z_max, colour, p0, normal }
    }

    fn in_bounds(&self, pt: Vec3) -> bool {
//        pt.x >= self.x_min && pt.x <= self.x_max && pt.z >= self.z_min && pt.z <= self.z_max
        true
    }
}

impl SceneObject for XzPlane {

    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let denominator = ray.dir.dot(&self.normal);
        // Ray is parallel to the plane (or close to it)
        return if denominator < 1e-6 {
            println!("Dot product of plane normal and ray < 1e-6: {:?}", denominator);
            None
        } else {
            let t = (self.p0 - ray.source).dot(&self.normal) / denominator;
            if t < 0.0 {
                println!("t < 0.0: {:?}", t);
                None
            } else {
                let intersection = ray.source + ray.dir * t;
                if self.in_bounds(intersection) {
                    println!("Ray intersects the plane at {:?}", intersection);
                    Some(t)
                } else {
                    println!("Point is not in bounds: {:?}", intersection);
                    None
                }
            }
        }
    }

    fn secondary_rays(&self, point: &Vec3) -> Vec<Ray> {
        // TODO this is only right for diffuse objects
        vec![]
    }

    fn surface_normal(&self, point: &Vec3) -> Vec3 {
        self.normal
    }

    fn colour(&self, point: &Vec3) -> Colour {
        self.colour
    }
}
