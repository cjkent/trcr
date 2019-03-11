use crate::{Colour, Ray, SceneObject};
use crate::vec3::Vec3;

pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
}

impl SceneObject for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        unimplemented!()
    }

    fn secondary_rays(&self, point: &Vec3) -> Vec<Ray> {
        // TODO this is only right for diffuse objects
        vec![]
    }

    fn surface_normal(&self, point: &Vec3) -> Vec3 {
        unimplemented!()
    }

    fn colour(&self, point: &Vec3) -> Colour {
        unimplemented!()
    }
}
