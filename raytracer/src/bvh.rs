//层次包围盒
use crate::scene::*;
use std::sync::Arc;
use crate::Vec3;
use std::cmp::Ordering;
use crate::ray::*;
use crate::rtweekend::*;

type point3 = Vec3;

pub struct bvh_node {
    left: Arc<dyn hittable>,
    right: Arc<dyn hittable>,
    Box: aabb,
}

//归并排序, 函数作为参量
// pub fn merge_sort(nums: &mut Vec<Rc<dyn scene::hittable> >, start: usize, end: usize, cmp: fn(Rc<dyn scene::hittable>,Rc<dyn scene::hittable>)->bool) {

//     fn _merge(nums: &mut Vec<Rc<dyn scene::hittable> >, left: usize, mid: usize, right: usize, cmp: fn(Rc<dyn scene::hittable>,Rc<dyn scene::hittable>)->bool) {
//         let left_part: Vec<Rc<dyn scene::hittable> > = nums[left..mid].iter().cloned().collect();
//         let right_part: Vec<Rc<dyn scene::hittable> > = nums[mid..right].iter().cloned().collect();
//         let (mut left_offset, mut right_offset) = (0usize, 0usize);
//         while left_offset < left_part.len() || right_offset < right_part.len() {
//             if right_offset == right_part.len() || (left_offset < left_part.len() && cmp(left_part[left_offset].clone(),right_part[right_offset].clone())) {
//                 nums[left + left_offset + right_offset] = left_part[left_offset].clone();
//                 left_offset += 1;
//             } else {
//                 nums[left + left_offset + right_offset] = right_part[right_offset].clone();
//                 right_offset += 1;
//             }
//         }
//     }

//     fn _merge_sort(nums: &mut Vec<Rc<dyn scene::hittable> >, left: usize, right: usize, cmp: fn(Rc<dyn scene::hittable>,Rc<dyn scene::hittable>)->bool) {
//         if left+1 < right {
//             let mid = (left + right) / 2;
//             _merge_sort(nums, left, mid, cmp);
//             _merge_sort(nums, mid, right, cmp);
//             _merge(nums, left, mid, right, cmp);
//         }
//     }

//     _merge_sort(nums, start, end+1, cmp)
// }

// pub fn merge_sort(mut nums: &mut Vec<Rc<dyn hittable> >, start: usize, end: usize, cmp: fn(Rc<dyn hittable>,Rc<dyn hittable>)->bool) {
//     if end >= start {return;}
//     let mut tmp_nums:Vec<Rc<dyn hittable> > = Vec::new();
//     let mid = (start+end) / 2;
//     merge_sort(&mut nums, start, mid, cmp);
//     merge_sort(&mut nums, mid+1, end, cmp);
//     let mut k = 0;
//     let mut i = start;
//     let mut j = mid+1;
//     while i <= mid && j <= end {
//         if cmp(Rc::clone(&nums[i]), Rc::clone(&nums[j])) {
//             tmp_nums[k] = Rc::clone(&nums[i]);
//             k += 1;
//             i += 1;
//         }
//         else {
//             tmp_nums[k] = Rc::clone(&nums[j]);
//             k += 1;
//             j += 1;
//         }
//     }
//     while i <= mid {
//         tmp_nums[k] = Rc::clone(&nums[i]);
//         k += 1;
//         i += 1;
//     }
//     while j <= end {
//         tmp_nums[k] = Rc::clone(&nums[j]);
//         k += 1;
//         j += 1;
//     }
//     j = 0;
//     for i in start..=end {
//         nums[i] = Rc::clone(&tmp_nums[j]);
//         j += 1;
//     }
// }

pub fn box_compare(a: &Arc<dyn hittable>, b: &Arc<dyn hittable>, axis: i32) -> Ordering {
    let mut box_a = aabb::new();
    let mut box_b = aabb::new();
    if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
        print!("No bounding box in bvh_node constructor.\n");
    }
    // if axis == 0 {return box_a.min().x() < box_b.min().x()}
    // else if axis == 1 {return box_a.min().y() < box_b.min().y()}
    // return box_a.min().z() < box_b.min().z()
    box_a.min().get(axis).total_cmp(&box_b.min().get(axis))
}

pub fn box_x_compare(a: &Arc<dyn hittable>, b: &Arc<dyn hittable>) -> Ordering {
    box_compare(a, b, 0)
}
pub fn box_y_compare(a: &Arc<dyn hittable>, b: &Arc<dyn hittable>) -> Ordering {
    box_compare(a, b, 1)
}
pub fn box_z_compare(a: &Arc<dyn hittable>, b: &Arc<dyn hittable>) -> Ordering {
    box_compare(a, b, 2)
}

impl bvh_node {
    pub fn new_with_5para(src_objects: &mut Vec< Arc<dyn hittable> >, start: usize, end: usize, time_0: f64, time_1: f64) -> Self {
        let mut objects = src_objects.clone();
        let axis = random_int(0, 2); //创建源场景对象的可修改数组
        let comparator: fn(a: &Arc<dyn hittable>, b: &Arc<dyn hittable>) -> Ordering;

        //判定随机值bvh_node
        if axis == 0 {comparator = box_x_compare;}
        else if axis == 1 {comparator = box_y_compare;}
        else {comparator = box_z_compare;}

        let object_span = end - start;

        //let mut _right: std::rc::Rc<dyn scene::hittable> = std::rc::Rc::new(scene::Sphere::new_without_para());
        let mut _right: Arc<dyn hittable>;
        //let mut _left: std::rc::Rc<dyn scene::hittable> = std::rc::Rc::new(scene::Sphere::new_without_para());
        let mut _left: Arc<dyn hittable>;
        if object_span == 1 {
            _right = objects[start].clone(); //vector下标只能为usize
            _left = _right.clone();
        }
        else if object_span == 2 {
            if comparator(&objects[start], &objects[start+1]) == Ordering::Less {
                _left = objects[start].clone();
                _right = objects[start+1].clone();
            }
            else {
                _left = objects[start+1].clone();
                _right = objects[start].clone();
            }
        }
        else { //object_span == 3
            //merge_sort(src_objects, start, end, comparator);
            src_objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            _left = Arc::new(bvh_node::new_with_5para(&mut objects, start, mid, time_0, time_1));
            _right = Arc::new(bvh_node::new_with_5para(&mut objects, mid, end, time_0, time_1));
        }

        let mut box_left = aabb::new();
        let mut box_right = aabb::new();

        if !_left.bounding_box(time_0, time_1, &mut box_left)
            || !_right.bounding_box(time_0, time_1, &mut box_right)
        {print!("No bounding box in bvh_node constructor.\n");}
        
        Self {
            left: Arc::clone(&_left), //clone() ?
            right: Arc::clone(&_right),
            Box: aabb::surrounding_box(box_left, box_right),
        }
    }
    pub fn new_with_3para(list: &mut hittable_list, time0: f64, time1: f64) -> Self {
        let l = list.objects.len();
        bvh_node::new_with_5para(&mut list.objects, 0, l, time0, time1)
    }
    pub fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut aabb) -> bool {
        *output_box = self.Box.clone();
        true
    }
}

impl hittable for bvh_node {
    fn hit(&self, r: &mut Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
        if !self.Box.hit(r, t_min, t_max) {
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
        true
    }
}




//AABB: 轴对齐包围盒
//#[derive(Clone)]
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
    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        //aabb轴对齐的边界框命中函数
        for a in 0..3 {
            let invD = 1.0 / r.direction().get(a);
            let mut t0 = (self.min().get(a) - r.origin().get(a)) * invD;
            let mut t1 = (self.max().get(a) - r.origin().get(a)) * invD;
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