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
        let thc = radius2 - d2;
        let t0 = tca - thc;
        let t1 = t0 + thc * 2.0;
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

    fn secondary_rays(&self, point: &Vec3) -> Vec<Ray> {
        // TODO this is only right for diffuse objects
        vec![]
    }

    fn surface_normal(&self, point: &Vec3) -> Vec3 {
        (*point - self.centre).normalised()
    }

    fn colour(&self, point: &Vec3) -> Colour {
        self.colour
    }
}
