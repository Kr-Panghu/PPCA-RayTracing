use crate::Vec3;
use crate::scene;
use crate::material;
use crate::ray;
use crate::aabb::aabb; 
use crate::bvh;
use std::sync::Arc;

type Point3 = Vec3;

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f64, time1: f64,
    radius: f64,
    mat_ptr: Arc<dyn material:: Material>
}

impl MovingSphere {
    pub fn new(center0: Point3, center1: Point3, time0: f64, time1: f64, radius: f64, mat_ptr: Arc<dyn material:: Material>) -> Self {
        Self {
            center0, center1,
            time0, time1,
            radius, mat_ptr
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        return self.center0 + (self.center1 - self.center0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl scene::hittable for MovingSphere {
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
        let mut outward_normal: Vec3 = (rec.p - self.center(r.time())) / self.radius;
        rec.set_face_normal(r, &mut outward_normal);
        rec.mat_ptr = Arc::clone(&self.mat_ptr);
        return true
    }

    //移动球体的包围盒
    fn bounding_box(&self, _time0: f64, _time1: f64, mut output_box: &mut aabb) -> bool {
        let box0 = aabb::new_with_para(&(self.center(_time0) - Vec3::new(self.radius, self.radius, self.radius)), 
                                  &(self.center(_time0) + Vec3::new(self.radius, self.radius, self.radius)));
        let box1 = aabb::new_with_para(&(self.center(_time1) - Vec3::new(self.radius, self.radius, self.radius)),
                                  &(self.center(_time1) - Vec3::new(self.radius, self.radius, self.radius)));
        // let tmp = aabb::surrounding_box(box0, box1);
        // output_box.minimum = tmp.minimum.clone();
        // output_box.maximum = tmp.maximum.clone(); //????
        *output_box = aabb::surrounding_box(box0, box1);
        return true
    }
}