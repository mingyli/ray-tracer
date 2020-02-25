use crate::geometry::Vec3;

pub trait Texture: Sync + Send {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3;
}

#[derive(Default)]
pub struct Uniform {
    color: Vec3,
}

impl Uniform {
    pub fn new(color: Vec3) -> Uniform {
        Uniform { color }
    }
}

impl Texture for Uniform {
    fn value(&self, _u: f32, _v: f32, _p: &Vec3) -> Vec3 {
        self.color.clone()
    }
}

pub struct Checkered {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
}

impl Checkered {
    pub fn new(odd: Box<dyn Texture>, even: Box<dyn Texture>) -> Checkered {
        Checkered { odd, even }
    }
}

impl Texture for Checkered {
    fn value(&self, u: f32, v: f32, p: &Vec3) -> Vec3 {
        let s = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();
        if s < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
