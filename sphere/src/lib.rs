use ray::Ray;
use vec3::Vec3;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub enum ReflType {
    DIFF,
    SPEC,
    REFR
}

pub struct Sphere{
    pub radiance: f64,
    pub position: Vec3,
    pub emission: Vec3,
    pub color: Vec3,
    pub refl_t: ReflType
}

impl Sphere{
    pub fn new(radiance: f64, position: Vec3, emission: Vec3, color: Vec3, refl_t: ReflType) -> Self{
        Sphere{
            radiance: radiance,
            position: position,
            emission: emission,
            color: color,
            refl_t: refl_t
        }
    }
    pub fn intersect(&self, ray: &Ray) -> Option<f64>{
        let op = self.position - ray.origin;
        let mut t: f64;
        let b = op.dot(ray.direction);
        let mut delta = b * b - op.dot(op) + self.radiance * self.radiance;

        if delta < 0.0{
            return None;
        }
        delta = delta.sqrt();
        t = b - delta;
        if t > 1e-4{
            return Some(t);
        }
        else{
            t = b + delta;
            if t > 1e-4{
                return Some(t);
            }
        }
        None
    }
}
