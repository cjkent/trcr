use ::{Colour, Scene};
use ::{Intensity, Light};
use sphere::Sphere;
use vec3::Vec3;

pub fn one_sphere() -> Scene {
    let sphere = Sphere {
        centre: Vec3::new(0.0, 0.0, -2.0),
        radius: 1.0,
        colour: Colour::from_24bit_int(0xA0F0A0)
    };
    let intensity = Intensity::new(1.0, 1.0, 1.0);
    let light = Light::Distant {
        dir: Vec3::new(-1.0, -5.0, -1.0).normalised(),
        intensity
    };
    Scene {
        objects: vec![Box::new(sphere)],
        lights: vec![light]
    }
}
