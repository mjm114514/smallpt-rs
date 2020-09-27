use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;


struct Vector(f32, f32, f32);

impl Vector{
    fn dot(&self, other: &Vector) -> f32{
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn length(&self) -> f32{
        1.0 / (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    fn cross(&self, other: &Vector) -> Vector{
        Vector{
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.z - self.z * other.y
        }
    }
}
impl Add<&Vector> for Vector{
    type Output = Vector;
    fn add(self, other: &Vector) -> Vector{
        Vector{
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}
impl Sub<&Vector> for Vector{
    type Output = Vector;
    fn sub(self, other: &Vector) -> Vector{
        Vector{
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}
impl Mul<f32> for Vector{
    type Output = Vector;
    fn mul(self, k: f32) -> Vector{
        Vector{
            x: self.x * k,
            y: self.y * k,
            z: self.z * k
        }
    }
}
impl Mul<&Vector> for Vector{
    type Output = Vector;
    fn mul(self, other: &Vector) -> Vector{
        Vector{
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

struct Ray{
    origin: Vector,
    direction: Vector
}

impl Ray{
    fn new(origin: &Vector, direction: &Vector) -> Self{
        Ray{
            origin: Vector::new(origin.x, origin.y, origin.z),
            direction: Vector::new(direction.x, direction.y, direction.z)
        }
    }
}

enum Refl_t {
    DIFF,
    SPEC,
    REFR
}

struct Sphere{
    radiance: f32,
    position: Vector,
    emission: Vector,
    color: Vector,
    refl_t: Refl_t
}

impl Sphere{
    fn new(radiance: f32, position: Vector, emission: Vector, color: Vector, refl_t: Refl_t) -> Self{
        Sphere{
            radiance: radiance,
            position: position,
            emission: emission,
            color: color,
            refl_t: refl_t
        }
    }
    fn intersect(self, ray: &Ray) -> f32{
        let op = self.position - &ray.origin;
        let mut t: f32;
        let eps = 1e-4;
        let b = op.dot(&ray.direction);
        let mut delta = b * b - op.dot(&op) + self.radiance * self.radiance;

        if delta < 0f32{
            return 0f32;
        }
        else{
            delta = delta.sqrt();
        }
        t = b - delta;
        if t > eps{
            return t;
        }
        else{
            t = b + delta;
            if t > eps{
                return t;
            }
        }
        0f32
    }
}

fn main() {

    let scene = [
        Sphere::new(1e5, Vector::new(1e5+1, 40.8, 81.6), Vector::new(0, 0, 0), Vector::new(0.75, 0.75, 0.75), Refl_t::DIFF)),
        Sphere::new(1e5, Vector::new(-1e5+99, 40.8, 81.6), Vector::new(0, 0, 0), Vector::new(0.25, 0.25, 0.25), Refl_t::DIFF)
    ];

    let row = 256;
    let col = 256;
    let level = 255;
    let mut f = match File::create(&Path::new("/home/jiaming/test.ppm")){
        Err(why) => panic!("Couldn't open {}: {}", "test.ppm", why),
        Ok(file) => file
    };

    f.write("P3\n".as_bytes()).expect("Failed to write");
    write!(f, "{}\n{}\n{}\n", row, col, level).expect("Failed to write");

    for _i in 0..row{
        for _j in 0..col{
            f.write("0 255 255\n".as_bytes()).expect("Failed to write");
        }
    }
}
