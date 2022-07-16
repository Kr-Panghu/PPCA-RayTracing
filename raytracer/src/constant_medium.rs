//恒定密度媒介
use crate::rtweekend;
use crate::scene;
use crate::material;
use crate::texture;
use crate::Vec3;
use crate::bvh;
use crate::aabb::aabb;
use crate::ray;
use std::rc::Rc;
use std::sync::Arc;

//对于恒定的体积
//只需要密度 C 和边界
//使用另一个hit表作为边界

//常量介质
pub struct ConstantMedium {
    boundary: Arc<dyn scene::hittable>,
    phase_function: Arc<dyn material::Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new_with_ptr(b: Arc<dyn scene::hittable>, d: f64, a: Arc<dyn texture::texture>) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0/d,
            phase_function: Arc::new(material::isotropic::new_with_ptr(a)), /////
        }
    }
    pub fn new_with_para(b: Arc<dyn scene::hittable>, d: f64, c: Vec3) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0/d,
            phase_function: Arc::new(material::isotropic::new_with_para(&c)),
        }
    }
}

impl scene::hittable for ConstantMedium {
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut scene::hit_record) -> bool {
        // let enableDebug = false;
        // let debugging = enableDebug && rtweekend::random_double_1() < 0.00001;
        
        let mut rec1 = scene::hit_record::new();
        let mut rec2 = scene::hit_record::new();

        if !self.boundary.hit(r, -rtweekend::infinity, rtweekend::infinity, &mut rec1) {return false;}
        if !self.boundary.hit(r, rec1.t+0.0001, rtweekend::infinity, &mut rec2) {return false;}

        //if debugging {print!("\nt_min={} , t_max={}\n", rec1.t, rec2.t)};

        if rec1.t < t_min {rec1.t = t_min}
        if rec2.t > t_max {rec2.t = t_max}

        if rec1.t >= rec2.t {return false}

        if rec1.t < 0.0 {rec1.t = 0.0}

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rtweekend::random_double_1().log(rtweekend::e);

        if hit_distance > distance_inside_boundary {return false}

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        // if debugging {
        //     print!("hit_distance = {}\n", hit_distance);
        //     print!("rec.t = {}\n", rec.t);
        //     print!("rec.p = {:?}\n", rec.p);
        // }

        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat_ptr = Arc::clone(&self.phase_function);

        return true
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut aabb) -> bool {
        return self.boundary.bounding_box(time0, time1, output_box);
    }
}