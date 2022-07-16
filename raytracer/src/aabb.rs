use crate::Vec3;
use crate::ray;
// use std::num;
type Point3 = Vec3;
type Color = Vec3;
//AABB: 轴对齐包围盒
//#[derive(Clone)]
#[derive(Clone, Copy)]
pub struct aabb {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl aabb {
    pub fn new() -> Self {
        Self {
            minimum: Vec3::zero(),
            maximum: Vec3::zero(),
        }
    }
    pub fn new_with_para(a: &Point3, b: &Point3) -> Self {
        Self {
            minimum: *a,
            maximum: *b,
        }
    }
    pub fn min(&self) -> Point3 {
        self.minimum
    }
    pub fn max(&self) -> Point3 {
        self.maximum
    }

    //注意: aabb类有独立的hit函数, 并非impl hittable Trait
    pub fn hit(&self, r: &ray::Ray, mut t_min: f64, mut t_max: f64) -> bool {
        //aabb轴对齐的边界框命中函数
        for index in 0..3 {
            let inv_d = 1.0 / r.direction().get(index);
            let mut t0 = (self.min().get(index) - r.origin().get(index)) * inv_d;
            let mut t1 = (self.max().get(index) - r.origin().get(index)) * inv_d;
            if inv_d < 0.0 {
                let tmp = t0;
                t0 = t1;
                t1 = tmp;
            }
            t_min = f64::max(t0, t_min);
            t_max = f64::min(t1, t_max);
            if t_max <= t_min {
                return false
            }        
        }
        true
    }

    pub fn surrounding_box(box0: aabb, box1: aabb) -> aabb {
        let minimum = Vec3::new(f64::min(box0.min().x(), box1.min().x()),
                                f64::min(box0.min().y(), box1.min().y()),
                                f64::min(box0.min().z(), box1.min().z()));
        let maximum = Vec3::new(f64::max(box0.max().x(), box1.max().x()),
                                f64::max(box0.max().y(), box1.max().y()),
                                f64::max(box0.max().z(), box1.max().z()));
        //return aabb::new_with_para(&small, &big)
        aabb { minimum, maximum }
    }
}


