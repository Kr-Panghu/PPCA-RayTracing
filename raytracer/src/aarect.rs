//rectangle
use crate::rtweekend;
use crate::scene;
use crate::material;
use crate::ray;
use crate::bvh;
use crate::Vec3;
use std::rc::Rc;
type point3 = Vec3;

pub struct xy_rect {
    mp: Rc<dyn material::Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k:  f64,
}

impl xy_rect {
    pub fn new(mat: Rc<dyn material::Material>, _x0: f64, _x1: f64, _y0: f64, _y1: f64, _k: f64) -> Self {
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
    fn hit(&self, r: &mut ray::Ray, t_min: f64, t_max: f64, rec: &mut scene::hit_record) -> bool {
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
        rec.mat_ptr = Rc::clone(&self.mp);    ///////////
        rec.p = r.at(t);

        return true;
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut bvh::aabb) -> bool {
        *output_box = bvh::aabb::new_with_para(&point3::new(self.x0, self.y0, self.k - 0.0001), &point3::new(self.x1, self.y1, self.k + 0.0001));
        return true
    }
}


pub struct xz_rect {
    mp: Rc<dyn material::Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k:  f64,
}

impl xz_rect {
    pub fn new(mat: Rc<dyn material::Material>, _x0: f64, _x1: f64, _z0: f64, _z1: f64, _k: f64) -> Self {
        Self {
            mp: mat,
            x0: _x0, x1: _x1, z0: _z0, z1: _z1, k: _k,
        }
    }
}

impl scene::hittable for xz_rect {
    fn hit(&self, r: &mut ray::Ray, t_min: f64, t_max: f64, rec: &mut scene::hit_record) -> bool {
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
        rec.mat_ptr = Rc::clone(&self.mp);    ///////////
        rec.p = r.at(t);

        return true;
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut bvh::aabb) -> bool {
        *output_box = bvh::aabb::new_with_para(&point3::new(self.x0, self.k-0.0001, self.z0), &point3::new(self.x1, self.k-0.0001, self.z1));
        return true
    }
}


pub struct yz_rect {
    mp: Rc<dyn material::Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k:  f64,
}

impl yz_rect {
    pub fn new(mat: Rc<dyn material::Material>, _y0: f64, _y1: f64, _z0: f64, _z1: f64, _k: f64) -> Self {
        Self {
            mp: mat,
            y0: _y0, y1: _y1, z0: _z0, z1: _z1, k: _k,
        }
    }
}

impl scene::hittable for yz_rect {
    fn hit(&self, r: &mut ray::Ray, t_min: f64, t_max: f64, rec: &mut scene::hit_record) -> bool {
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
        rec.mat_ptr = Rc::clone(&self.mp);    ///////////
        rec.p = r.at(t);

        return true;
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut bvh::aabb) -> bool {
        *output_box = bvh::aabb::new_with_para(&point3::new(self.k-0.0001, self.y0, self.z0), &point3::new(self.k+0.0001, self.y1, self.z1));
        return true
    }
}

