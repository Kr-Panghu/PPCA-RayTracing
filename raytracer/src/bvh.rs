//层次包围盒
use crate::scene;
use std::rc::Rc;
use crate::Vec3;
use crate::ray;
use crate::rtweekend;

type point3 = Vec3;

pub struct bvh_node {
    left: Rc<dyn scene::hittable>,
    right: Rc<dyn scene::hittable>,
    Box: aabb,
}

//归并排序, 函数作为参量
pub fn merge_sort(nums: &mut Vec<Rc<dyn scene::hittable> >, start: usize, end: usize, cmp: fn(Rc<dyn scene::hittable>,Rc<dyn scene::hittable>)->bool) {

    fn _merge(nums: &mut Vec<Rc<dyn scene::hittable> >, left: usize, mid: usize, right: usize, cmp: fn(Rc<dyn scene::hittable>,Rc<dyn scene::hittable>)->bool) {
        let left_part: Vec<Rc<dyn scene::hittable> > = nums[left..mid].iter().cloned().collect();
        let right_part: Vec<Rc<dyn scene::hittable> > = nums[mid..right].iter().cloned().collect();
        let (mut left_offset, mut right_offset) = (0usize, 0usize);
        while left_offset < left_part.len() || right_offset < right_part.len() {
            if right_offset == right_part.len() || (left_offset < left_part.len() && cmp(left_part[left_offset].clone(),right_part[right_offset].clone())) {
                nums[left + left_offset + right_offset] = left_part[left_offset].clone();
                left_offset += 1;
            } else {
                nums[left + left_offset + right_offset] = right_part[right_offset].clone();
                right_offset += 1;
            }
        }
    }

    fn _merge_sort(nums: &mut Vec<Rc<dyn scene::hittable> >, left: usize, right: usize, cmp: fn(Rc<dyn scene::hittable>,Rc<dyn scene::hittable>)->bool) {
        if left+1 < right {
            let mid = (left + right) / 2;
            _merge_sort(nums, left, mid, cmp);
            _merge_sort(nums, mid, right, cmp);
            _merge(nums, left, mid, right, cmp);
        }
    }

    _merge_sort(nums, start, start+end, cmp)
}

pub fn box_compare(a: Rc<dyn scene::hittable>, b: Rc<dyn scene::hittable>, axis: usize) -> bool {
    let mut box_a = aabb::new();
    let mut box_b = aabb::new();
    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        print!("No bounding box in bvh_node constructor.\n");
    }
    if axis == 0 {return box_a.min().x() < box_b.min().x()}
    else if axis == 1 {return box_a.min().y() < box_b.min().y()}
    return box_a.min().z() < box_b.min().z()
}

pub fn box_x_compare(a: Rc<dyn scene::hittable>, b: Rc<dyn scene::hittable>) -> bool {
    return box_compare(a,b,0)
}
pub fn box_y_compare(a: Rc<dyn scene::hittable>, b: Rc<dyn scene::hittable>) -> bool {
    return box_compare(a,b,1)
}
pub fn box_z_compare(a: Rc<dyn scene::hittable>, b: Rc<dyn scene::hittable>) -> bool {
    return box_compare(a,b,2)
}

impl bvh_node {
    pub fn new_with_5para(src_objects: &mut Vec< Rc<dyn scene::hittable> >, start: usize, end: usize, time_0: f64, time_1: f64) -> Self {
        let mut objects = src_objects.clone();
        let axis = rtweekend::random_int(0, 2); //创建源场景对象的可修改数组
        let comparator: fn(a: Rc<dyn scene::hittable>, b: Rc<dyn scene::hittable>) -> bool;

        //判定随机值bvh_node
        if axis == 0 {comparator = box_x_compare;}
        else if axis == 1 {comparator = box_y_compare;}
        else {comparator = box_z_compare;}

        let object_span = end - start;

        let mut _right: std::rc::Rc<dyn scene::hittable> = std::rc::Rc::new(scene::Sphere::new_without_para());
        //let _right = Rc::new(material::lambertian::new(&Vec3::ones()))
        let mut _left: std::rc::Rc<dyn scene::hittable> = std::rc::Rc::new(scene::Sphere::new_without_para());
        if object_span == 1 {
            _right = objects[start].clone(); //vector下标只能为usize
            _left = _right.clone();
        }
        else if object_span == 2 {
            if comparator(objects[start].clone(), objects[start+1].clone()) {
                _left = objects[start].clone();
                _right = objects[start+1].clone();
            }
            else {
                _left = objects[start+1].clone();
                _right = objects[start].clone();
            }
        }
        else { //object_span == 3
            merge_sort(src_objects, start, end, comparator);
            let mid = start + object_span / 2;
            _left = Rc::new(bvh_node::new_with_5para(&mut objects, start, mid, time_0, time_1));
            _right = Rc::new(bvh_node::new_with_5para(&mut objects, mid, end, time_0, time_1));
        }

        let mut box_left = aabb::new();
        let mut box_right = aabb::new();

        if !_left.bounding_box(time_0, time_1, &mut box_left)
            || !_right.bounding_box(time_0, time_1, &mut box_right)
        {print!("No bounding box in bvh_node constructor.\n");}
        
        Self {
            left: Rc::clone(&_left), //clone() ?
            right: Rc::clone(&_right),
            Box: aabb::surrounding_box(box_left, box_right),
        }
    }
    pub fn new_with_3para(mut list: &mut scene::hittable_list, time0: f64, time1: f64) -> Self {
        let l = list.objects.clone().len();
        bvh_node::new_with_5para(&mut list.objects, 0, l as usize, time0, time1)
    }
    pub fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut aabb) -> bool {
        *output_box = self.Box.clone();
        return true
    }
}

impl scene::hittable for bvh_node {
    fn hit(&self, r: &mut ray::Ray, t_min: f64, t_max: f64, rec: &mut scene::hit_record) -> bool {
        if self.Box.hit(r, t_min, t_max) {
            return false
        }
        
        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right: bool;
        if hit_left {hit_right = self.right.hit(r, t_min, rec.t, rec)}
        else {hit_right = self.right.hit(r, t_min, t_max, rec)}

        return hit_left || hit_right
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut aabb) -> bool {
        *output_box = self.Box.clone();
        return true
    }
}




//AABB: 轴对齐包围盒
#[derive(Clone)]
pub struct aabb {
    pub minimum: point3,
    pub maximum: point3,
}

impl aabb {
    pub fn new() -> Self {
        Self {
            minimum: Vec3::zero(),
            maximum: Vec3::zero(),
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

    //注意: aabb类有独立的hit函数, 并非impl hittable Trait
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







// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_merge_sort() {
//         let mut a: Vec<usize> = vec![10,2,5,8];
//         merge_sort(&mut a);
//         assert_eq!(a[0], 2);
//     }
// }