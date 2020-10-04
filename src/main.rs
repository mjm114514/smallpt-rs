use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::f32::INFINITY;
use std::f32::consts::PI;
use rand::Rng;

#[derive(Copy, Clone)]
struct Vec3(f32, f32, f32);

impl Vec3{
    fn dot(&self, other: Vec3) -> f32{
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    fn length(&self) -> f32{
        1.0 / (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
    fn cross(&self, other: Vec3) -> Vec3{
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.1 * other.2 - self.2 * other.1
        )
    }
    fn normalize(mut self) -> Vec3{
        let len = self.length();
        self.0 /= len;
        self.1 /= len;
        self.2 /= len;
        self
    }
}
impl Add<Vec3> for Vec3{
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3{
        Vec3(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2
        )
    }
}
impl Sub<Vec3> for Vec3{
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3{
        Vec3(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2
        )
    }
}
impl Mul<f32> for Vec3{
    type Output = Vec3;
    fn mul(self, k: f32) -> Vec3{
        Vec3(
            self.0 * k,
            self.1 * k,
            self.2 * k
        )
    }
}
impl Mul<Vec3> for f32{
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3{
        Vec3(
            self * other.0,
            self * other.1,
            self * other.2
        )
    }
}
impl Mul<Vec3> for Vec3{
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3{
        Vec3(
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2
        )
    }
}

struct Ray{
    origin: Vec3,
    direction: Vec3
}

impl Ray{
    fn new(origin: &Vec3, direction: &Vec3) -> Self{
        Ray{
            origin: Vec3(origin.0, origin.1, origin.2),
            direction: Vec3(origin.0, origin.1, origin.2)
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
    position: Vec3,
    emission: Vec3,
    color: Vec3,
    refl_t: Refl_t
}

impl Sphere{
    fn new(radiance: f32, position: Vec3, emission: Vec3, color: Vec3, refl_t: Refl_t) -> Self{
        Sphere{
            radiance: radiance,
            position: position,
            emission: emission,
            color: color,
            refl_t: refl_t
        }
    }
    fn intersect(&self, ray: &Ray) -> Option<f32>{
        let op = self.position - ray.origin;
        let mut t: f32;
        let b = op.dot(ray.direction);
        let mut delta = b * b - op.dot(op) + self.radiance * self.radiance;

        if delta > 0.0{
            delta = delta.sqrt();
        }
        t = b - delta;
        if t > 0.0{
            return Some(t);
        }
        else{
            t = b + delta;
            if t > 0.0{
                return Some(t);
            }
        }
        None
    }
}

fn color(scene: &[Sphere], ray: &Ray, depth: i32) -> Vec3{
    let mut t = INFINITY;
    let mut id: Option<usize> = None;
    for i in 0..scene.len(){
        let distance = scene[i].intersect(ray);
        if let Some(distance) = distance{
            if distance < t{
                t = distance;
                id = Some(i);
            }
        }
    }
    if let Some(id) = id{
        let mut rng = rand::thread_rng();
        let hit_pos = ray.origin + (ray.direction * t);
        let hit_normal = (hit_pos - scene[id].position).normalize();
        let f = scene[id].color;
        let nl: Vec3;
        if hit_normal.dot(ray.direction) < 0.0{
            nl = hit_normal;
        }
        else{
            nl = -1.0 * hit_normal;
        }

        match scene[id].refl_t{
            Refl_t::DIFF => {
                let rand1: f32 = rng.gen();
                let rand2: f32 = rng.gen();

                let r1 = 2.0 * PI * rand1;
                let r2 = rand2.sqrt();
                let w = nl;
                let u: Vec3;
                if w.0 > 0.1{
                    u = Vec3(0.0, 1.0, 0.0).cross(w).normalize();
                }
                else{
                    u = Vec3(1.0, 0.0, 0.0).cross(w).normalize();
                }
                let v = w.cross(u);
            },
            Refl_t::REFR => {
                
            },
            Refl_t::SPEC => {

            }
        }
    }
    Vec3(0., 0., 0.) // if miss return black color
}

fn main() {

    let scene = [
        // Left
        Sphere::new(1e5, Vec3(1e5 + 1.0, 40.8, 81.6), Vec3(0., 0., 0.), Vec3(0.75, 0.75, 0.75), Refl_t::DIFF),
        // Right
        Sphere::new(1e5, Vec3(-1e5 + 99.0, 40.8, 81.6), Vec3(0., 0., 0.), Vec3(0.25, 0.25, 0.25), Refl_t::DIFF),
        // Back
        Sphere::new(1e5, Vec3(50.0, 40.8, 1e5), Vec3(0., 0., 0.), Vec3(0.75, 0.75, 0.75), Refl_t::DIFF),
        // Front
        Sphere::new(1e5, Vec3(50.0, 40.8, -1e5 + 170.0), Vec3(0., 0., 0.), Vec3(0., 0., 0.), Refl_t::DIFF),
        // Bottom
        Sphere::new(1e5, Vec3(50.0, 1e5, 81.6), Vec3(0., 0., 0.), Vec3(0.75, 0.75, 0.75), Refl_t::DIFF),
        // Top
        Sphere::new(1e5, Vec3(50.0, -1e5 + 81.6, 81.6), Vec3(0., 0., 0.), Vec3(0.75, 0.75, 0.75), Refl_t::DIFF),
        // Metal ball
        Sphere::new(16.5, Vec3(27.0, 16.5, 47.0), Vec3(0., 0., 0.), Vec3(1., 1., 1.) * 0.999, Refl_t::SPEC),
        // Glass ball
        Sphere::new(16.5, Vec3(73.0, 16.5, 78.0), Vec3(0., 0., 0.), Vec3(1., 1., 1.) * 0.999, Refl_t::REFR),
        // Light
        Sphere::new(600.0, Vec3(50.0, 681.6 - 0.27, 81.6), Vec3(12.0, 12.0, 12.0), Vec3(0., 0., 0.), Refl_t::DIFF),
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
