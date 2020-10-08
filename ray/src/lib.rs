use vec3::Vec3;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct Ray{
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray{
    pub fn new(origin: &Vec3, direction: &Vec3) -> Self{
        Ray{
            origin: Vec3(origin.0, origin.1, origin.2),
            direction: Vec3(direction.0, direction.1, direction.2)
        }
    }
}