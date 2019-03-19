use ::{Colour, Scene};
use ::{Intensity, Light};
use objects::Sphere;
use objects::XzPlane;
use vec3::Vec3;

pub fn one_sphere() -> Scene {
    let sphere = Sphere {
        centre: Vec3::new(0.0, 0.0, -3.0),
        radius: 1.0,
        colour: Colour::from_24bit_int(0xA0F0A0)
    };
    let plane = XzPlane::new(-1.0, -1.0, 1.0, -4.0, -2.0, Colour::from_24bit_int(0xFFFFFF));
    let intensity = Intensity::new(1.0, 1.0, 1.0);
    let light = Light::Distant {
        dir: Vec3::new(-2.0, -5.0, -2.0).normalised(),
        intensity
    };
    Scene {
        objects: vec![Box::new(sphere), Box::new(plane)],
        lights: vec![light]
    }
}

pub fn one_sphere_two_lights() -> Scene {
    let sphere = Sphere {
        centre: Vec3::new(0.0, 0.0, -2.0),
        radius: 1.0,
        colour: Colour::from_24bit_int(0xFFFFFF)
    };
    let light1 = Light::Distant {
        dir: Vec3::new(-1.0, -5.0, -1.0).normalised(),
        intensity: Intensity::new(1.0, 0.1, 0.1)
    };
    let light2 = Light::Distant {
        dir: Vec3::new(2.0, -2.0, -2.0).normalised(),
        intensity: Intensity::new(1.0, 1.0, 1.0)
    };
    Scene {
        objects: vec![Box::new(sphere)],
        lights: vec![light1, light2]
    }
}
