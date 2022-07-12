use crate::material;
use crate::scene;
use crate::Vec3;

type point3 = Vec3;
type color = Vec3;

#[derive(Debug)]
pub struct Ray {
    pub orig: point3,
    pub dir: color,
    pub tm: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Self {
        Self {
            orig: origin,
            dir: direction,
            tm: time,    //时空光线追踪
        }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn time(&self) -> f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}