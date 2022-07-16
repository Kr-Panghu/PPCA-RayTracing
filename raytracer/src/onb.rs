use crate::scene;
use crate::Vec3;
use crate::rtweekend;
use crate::ray;
use crate::texture;
use std::rc::Rc;
use std::sync::Arc;

pub struct ONB {
    axis: Vec<Vec3>
}

//"orthonormal basis"
impl ONB {
    pub fn new() -> Self {
        Self {axis: vec![Vec3::zero(), Vec3::zero(), Vec3::zero()]}
    }
    pub fn get(&self, i: usize) -> Vec3 {
        self.axis[i]
    }
    pub fn u(&self) -> Vec3 {self.axis[0]}
    pub fn v(&self) -> Vec3 {self.axis[1]}
    pub fn w(&self) -> Vec3 {self.axis[2]}

    pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
        self.u() * a + self.v() * b + self.w() * c
    }

    pub fn local_with_vec(&self, a: &Vec3) -> Vec3 {
        self.u() * a.x() + self.v() * a.y() + self.w() * a.z()
    }

    pub fn build_from_w(&mut self, n: &Vec3) {
        self.axis[2] = n.unit();
        let a = if f64::abs(self.w().x()) > 0.9 {Vec3::new(0.0, 1.0, 0.0)}
                else {Vec3::new(1.0, 0.0, 0.0)};
        self.axis[1] = Vec3::cross(self.w().clone(), a).unit();
        self.axis[0] = Vec3::cross(self.w().clone(), self.v().clone());
    }
}