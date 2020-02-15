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

    pub fn from_fov(
        origin: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        vertical_fov: f32,
        aspect: f32,
    ) -> Camera {
        let theta = vertical_fov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (&origin - look_at).normalized();
        let u = (v_up.cross(&w)).normalized();
        let v = w.cross(&u);
        Camera {
            lower_left: &origin - half_width * &u - half_height * &v - &w,
            origin,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
        }
    }

    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin.clone(),
            &self.lower_left + u * &self.horizontal + v * &self.vertical - &self.origin,
        )
    }
}
