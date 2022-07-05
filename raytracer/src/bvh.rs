//层次包围盒
use crate::scene;
use std::rc::Rc;
use crate::Vec3;
use crate::ray;

type point3 = Vec3;

pub struct bvh_node {
    left: Rc<dyn scene::hittable>,
    right: Rc<dyn scene::hittable>,
    //Box: aabb,
}

// impl bvh_node {
//     pub fn new(list: &scene::hittable_list, time0: f64, time1: f64) -> Self {
//         Self {
//             list: hittable_list,
//         }
//     }    
// }

//AABB: 轴对齐包围盒
#[derive(Clone)]
pub struct aabb {
    pub minimum: point3,
    pub maximum: point3,
}

impl aabb {
    pub fn new() -> Self {
        Self {
            minimum: Vec3::new(0.0,0.0,0.0),
            maximum: Vec3::new(0.0,0.0,0.0),
        }
    }
    pub fn new_with_para(a: &point3, b: &point3) -> Self {
        Self {
            minimum: *a,
            maximum: *b,
        }
    }
    pub fn min(&self) -> point3 {
        self.minimum
    }
    pub fn max(&self) -> point3 {
        self.maximum
    }
    pub fn hit(&self, r: &ray::Ray, mut t_min: f64, mut t_max: f64) -> bool {
        //aabb轴对齐的边界框命中函数
        {
            let invD = 1.0 / r.direction().x();
            let mut t0 = (self.min().x() - r.origin().x()) * invD;
            let mut t1 = (self.max().x() - r.origin().x()) * invD;
            if invD < 0.0 {
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
        {
            let invD = 1.0 / r.direction().y();
            let mut t0 = (self.min().y() - r.origin().y()) * invD;
            let mut t1 = (self.max().y() - r.origin().y()) * invD;
            if invD < 0.0 {
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
        {
            let invD = 1.0 / r.direction().z();
            let mut t0 = (self.min().z() - r.origin().z()) * invD;
            let mut t1 = (self.max().z() - r.origin().z()) * invD;
            if invD < 0.0 {
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
        return true
    }

    pub fn surrounding_box(box0: aabb, box1: aabb) -> aabb {
        let small = Vec3::new(f64::min(box0.min().x(), box1.min().x()),
                              f64::min(box0.min().y(), box1.min().y()),
                              f64::min(box0.min().z(), box1.min().z()));
        let big = Vec3::new(f64::max(box0.max().x(), box1.max().x()),
                            f64::max(box0.max().y(), box1.max().y()),
                            f64::max(box0.max().z(), box1.max().z()));
        return aabb::new_with_para(&small, &big)
    }
}