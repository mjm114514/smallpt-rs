use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::f64::INFINITY;
use std::f64::consts::PI;
use rand::Rng;

#[derive(Copy, Clone)]
struct Vec3(f64, f64, f64);

impl Vec3{
    fn dot(&self, other: Vec3) -> f64{
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    fn length(&self) -> f64{
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
impl Mul<f64> for Vec3{
    type Output = Vec3;
    fn mul(self, k: f64) -> Vec3{
        Vec3(
            self.0 * k,
            self.1 * k,
            self.2 * k
        )
    }
}
impl Mul<Vec3> for f64{
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
            direction: Vec3(direction.0, direction.1, direction.2)
        }
    }
}

enum Refl_t {
    DIFF,
    SPEC,
    REFR
}

struct Sphere{
    radiance: f64,
    position: Vec3,
    emission: Vec3,
    color: Vec3,
    refl_t: Refl_t
}

impl Sphere{
    fn new(radiance: f64, position: Vec3, emission: Vec3, color: Vec3, refl_t: Refl_t) -> Self{
        Sphere{
            radiance: radiance,
            position: position,
            emission: emission,
            color: color,
            refl_t: refl_t
        }
    }
    fn intersect(&self, ray: &Ray) -> Option<f64>{
        let op = self.position - ray.origin;
        let mut t: f64;
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
            Refl_t::DIFF => {  // Ideal DIFFUSE reflection
                let rand1: f64 = rng.gen();
                let rand2: f64 = rng.gen();

                let r1 = 2.0 * PI * rand1;
                let r2s = rand2.sqrt();
                let w = nl;
                let u: Vec3;
                if w.0 > 0.1{
                    u = Vec3(0.0, 1.0, 0.0).cross(w).normalize();
                }
                else{
                    u = Vec3(1.0, 0.0, 0.0).cross(w).normalize();
                }
                let v = w.cross(u);

                let dir = (r1.cos() * r2s * u + r1.sin() * r2s * v + (1.0 - rand2).sqrt() * w).normalize();

                return scene[id].emission + f * color(scene, &Ray::new(&hit_pos, &dir), depth + 1);
            },
            Refl_t::REFR => {
                let refl_dir = ray.direction - 2.0 * hit_normal.dot(ray.direction) * hit_normal;
                let refl_ray = Ray::new(&hit_pos, &refl_dir);
                let into = hit_normal.dot(nl) > 0.0;
                let nc = 1.0;
                let nt = 1.5;
                let nnt: f64;
                if into{
                    nnt = nc / nt;
                }
                else{
                    nnt = nt / nc;
                }
                let ddotn = ray.direction.dot(nl);
                let cos2t = 1.0 - nnt * nnt * (1.0 - ddotn * ddotn);
                if cos2t < 0.0{
                    // Total Internal reflection
                    return scene[id].emission + f * color(scene, &refl_ray, depth + 1);
                }
                let refr_dir: Vec3;
                let a = nt - nc;
                let b = nt + nc;
                let r0 = a * a / (b * b);
                let c;
                if into{
                    refr_dir = (ray.direction * nnt - hit_normal * (ddotn * nnt + cos2t.sqrt())).normalize();
                    c = 1.0 + ddotn;
                }
                else{
                    refr_dir = (ray.direction * nnt - hit_normal * -1.0 * (ddotn * nnt * cos2t.sqrt())).normalize();
                    c = 1.0 - (refr_dir.dot(hit_normal));
                }

                let re = r0 + (1.0 - r0) * c * c * c * c * c;
                let tr = 1.0 - re;
                let p = 0.25 + 0.5 * re;
                let rp = re / p;
                let tp = tr / (1.0 - p);

                let r: f64 = rng.gen();


                if depth > 2{
                    if r < p{
                        return scene[id].emission + rp * f * color(scene, &refl_ray, depth + 1);
                    }
                    else{
                        return scene[id].emission + tp * f * color(scene, &Ray::new(&hit_pos, &refr_dir), depth + 1);
                    }
                }
                else{
                    return scene[id].emission +
                        re * color(scene, &refl_ray, depth + 1) +
                        tr * color(scene, &Ray::new(&hit_pos, &refr_dir), depth + 1);
                }
            },
            Refl_t::SPEC => { // Ideal SPECULAR reflection
                let dir = ray.direction - 2.0 * hit_normal.dot(ray.direction) * hit_normal;
                return scene[id].emission + f * color(scene, &Ray::new(&hit_pos, &dir), depth + 1);
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
