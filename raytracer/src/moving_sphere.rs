use crate::Vec3;
use crate::scene;
use crate::material;
use crate::ray;
use std::rc::Rc;

type point3 = Vec3;

pub struct moving_sphere {
    center0: point3,
    center1: point3,
    time0: f64, time1: f64,
    radius: f64,
    mat_ptr: Rc<dyn material:: Material>
}

impl moving_sphere {
    pub fn new(cen0: point3, cen1: point3, _time0: f64, _time1: f64, r: f64, m: Rc<dyn material:: Material>) -> Self {
        Self {
            center0: cen0,
            center1: cen1,
            time0: _time0,
            time1: _time1,
            radius: r,
            mat_ptr: m,
        }
    }

    pub fn center(&self, time: f64) -> point3 {
        return self.center0 + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl scene::hittable for moving_sphere {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut scene::hit_record) -> bool {
        let oc: Vec3 = *r.origin() - self.center(r.time());
        let a: f64 = r.direction().squared_length();
        let half_b: f64 = Vec3::dot(r.direction(), &oc);
        let c = oc.squared_length() - self.radius * self.radius;

        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant < 0.0 {return false}
        let sqrtd: f64 = f64::sqrt(discriminant);

        let mut root: f64 = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center(r.time())) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = Rc::clone(&self.mat_ptr);
        return true
    }
}