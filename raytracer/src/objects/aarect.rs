//rectangle
use crate::scene;
use crate::material;
use crate::BASIC::ray;
use crate::BVH::bvh;
use crate::Vec3;
use crate::BVH::aabb::*;
use std::rc::Rc;
use crate::BASIC::rtweekend::*;
use std::sync::Arc;
type Point3 = Vec3;

pub struct xy_rect {
    mp: Arc<dyn material::Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k:  f64,
}

impl xy_rect {
    pub fn new(mat: Arc<dyn material::Material>, _x0: f64, _x1: f64, _y0: f64, _y1: f64, _k: f64) -> Self {
        Self {
            mp: mat,
            x0: _x0,
            x1: _x1,
            y0: _y0,
            y1: _y1,
            k: _k,
        }
    }
}

impl scene::hittable for xy_rect {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut scene::hit_record) -> bool {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {return false}
        
        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {return false}

        rec.u = (x-self.x0) / (self.x1 - self.x0);
        rec.v = (y-self.y0) / (self.y1 - self.y0);
        rec.t = t;

        let mut outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, &mut outward_normal);
        rec.mat_ptr = Arc::clone(&self.mp);
        rec.p = r.at(t);

        return true;
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut aabb) -> bool {
        *output_box = aabb::new_with_para(&Point3::new(self.x0, self.y0, self.k - 0.0001), 
                                          &Point3::new(self.x1, self.y1, self.k + 0.0001));
        return true
    }
}


pub struct xz_rect {
    mp: Arc<dyn material::Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k:  f64,
}

impl xz_rect {
    pub fn new(mat: Arc<dyn material::Material>, _x0: f64, _x1: f64, _z0: f64, _z1: f64, _k: f64) -> Self {
        Self {
            mp: mat,
            x0: _x0, x1: _x1, z0: _z0, z1: _z1, k: _k,
        }
    }
}

impl scene::hittable for xz_rect {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut scene::hit_record) -> bool {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {return false}
        
        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {return false}

        rec.u = (x-self.x0) / (self.x1 - self.x0);
        rec.v = (z-self.z0) / (self.z1 - self.z0);
        rec.t = t;

        let mut outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r, &mut outward_normal);
        rec.mat_ptr = Arc::clone(&self.mp);    ///////////
        rec.p = r.at(t);

        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut aabb) -> bool {
        *output_box = aabb::new_with_para(&Point3::new(self.x0, self.k-0.0001, self.z0), 
                                          &Point3::new(self.x1, self.k-0.0001, self.z1));
        true
    }
    fn pdf_value(&self, origin: &Vec3, v: &Vec3) -> f64 {
        let mut rec = scene::hit_record::new();
        if !self.hit(&ray::Ray::new(origin.clone(), v.clone(), 0.0), 0.001, infinity, &mut rec) {
            return 0.0
        }
        let area = (self.x1-self.x0)*(self.z1-self.z0);
        let distance_squared = rec.t * rec.t * v.squared_length();
        let cosine = (Vec3::dot(v, &rec.normal) / v.length()).abs();

        distance_squared / (cosine * area)
    }
    fn random(&self, origin: &Vec3) -> Vec3 {
        let random_point = Vec3::new(random_double_2(self.x0, self.x1), self.k, random_double_2(self.z0, self.z1));
        random_point - origin.clone()
    }
}


pub struct yz_rect {
    mp: Arc<dyn material::Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k:  f64,
}

impl yz_rect {
    pub fn new(mat: Arc<dyn material::Material>, _y0: f64, _y1: f64, _z0: f64, _z1: f64, _k: f64) -> Self {
        Self {
            mp: mat,
            y0: _y0, y1: _y1, z0: _z0, z1: _z1, k: _k,
        }
    }
}

impl scene::hittable for yz_rect {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut scene::hit_record) -> bool {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {return false}
        
        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {return false}

        rec.u = (y-self.y0) / (self.y1 - self.y0);
        rec.v = (z-self.z0) / (self.z1 - self.z0);
        rec.t = t;

        let mut outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r, &mut outward_normal);
        rec.mat_ptr = Arc::clone(&self.mp);    ///////////
        rec.p = r.at(t);

        return true;
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut aabb) -> bool {
        *output_box = aabb::new_with_para(&Point3::new(self.k-0.0001, self.y0, self.z0), 
                                          &Point3::new(self.k+0.0001, self.y1, self.z1));
        true
    }
}


