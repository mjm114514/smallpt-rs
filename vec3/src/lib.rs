use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3{
    pub fn dot(&self, other: Vec3) -> f64{
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
    pub fn length(&self) -> f64{
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
    pub fn cross(&self, other: Vec3) -> Vec3{
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0
        )
    }
    pub fn normalize(mut self) -> Vec3{
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