use crate::geometry::{Ray, Vec3};

#[derive(Debug, Default)]
pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(origin: Vec3, lower_left: Vec3, horizontal: Vec3, vertical: Vec3) -> Camera {
        Camera {
            origin,
            lower_left,
            horizontal,
            vertical,
        }
    }

    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin.clone(),
            &self.lower_left + u * &self.horizontal + v * &self.vertical,
        )
    }
}
