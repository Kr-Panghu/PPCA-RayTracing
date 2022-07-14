//康奈尔盒子内的方块 (box)
use crate::aarect;
use crate::scene;
use crate::material;
use crate::ray;
use crate::bvh;
use crate::Vec3;
use std::rc::Rc;
use std::sync::Arc;
use crate::scene::hittable_list;
type point3 = Vec3;

pub struct Block {
    box_min: point3,
    box_max: point3,
    sides: hittable_list,
}

impl Block {
    pub fn new(p0: &point3, p1: &point3, ptr: Arc<dyn material::Material>) -> Self {
        let mut _sides = hittable_list::new(Arc::new(aarect::xy_rect::new(Arc::clone(&ptr), p0.x(), p1.x(), p0.y(), p1.y(), p1.z())));
        _sides.add(Arc::new(aarect::xy_rect::new(Arc::clone(&ptr), p0.x(), p1.x(), p0.y(), p1.y(), p0.z())));

        _sides.add(Arc::new(aarect::xz_rect::new(Arc::clone(&ptr), p0.x(), p1.x(), p0.z(), p1.z(), p1.y())));
        _sides.add(Arc::new(aarect::xz_rect::new(Arc::clone(&ptr), p0.x(), p1.x(), p0.z(), p1.z(), p0.y())));

        _sides.add(Arc::new(aarect::yz_rect::new(Arc::clone(&ptr), p0.y(), p1.y(), p0.z(), p1.z(), p1.x())));
        _sides.add(Arc::new(aarect::yz_rect::new(Arc::clone(&ptr), p0.y(), p1.y(), p0.z(), p1.z(), p0.x())));

        Self {
            box_min: (*p0).clone(),
            box_max: (*p1).clone(),
            sides: _sides.clone(),     //????????
        }
    }
}

impl scene::hittable for Block {
    fn hit(&self, r: &mut ray::Ray, t_min: f64, t_max: f64, rec: &mut scene::hit_record) -> bool {
        return self.sides.hit(r, t_min, t_max, rec);
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut bvh::aabb) -> bool {
        *output_box = bvh::aabb::new_with_para(&self.box_min, &self.box_max);
        return true;
    }
}