use std::ops::*;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {x, y, z}
    }

    pub fn len(&self) -> f32 {
        f32::sqrt(f32::powi(self.x, 2) + f32::powi(self.y, 2) + f32::powi(self.z, 2))
    }

    pub fn normalize(&self) -> Vec3 {
        let length = self.len();
        
        *self * (1.0/length)
    }

    pub fn u8_rgb(&self) -> [u8; 3] {
        [
            (self.x * 255.0) as u8,
            (self.y * 255.0) as u8,
            (self.z * 255.0) as u8
        ]
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Vec3 {
        Vec3::new(
            self.x + rhs.x,
            self.y + rhs.y,
            self.z + rhs.z
        )
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Vec3 {
        Vec3::new(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z
        )
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3::new(
            self.x*rhs,
            self.y*rhs,
            self.z*rhs
        )
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.x*rhs.x,
            self.y*rhs.y,
            self.z*rhs.z
        )
    }
}