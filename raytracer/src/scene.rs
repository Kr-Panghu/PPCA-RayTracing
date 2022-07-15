#![allow(dead_code)]
// You SHOULD remove above line in your code.

use crate::Vec3;
use crate::World;
use crate::ray;
use std::rc::Rc;
use crate::material;
use crate::bvh;
use std::sync::Arc;
use crate::camera;
// use std::num;
use raytracer_codegen::make_spheres_impl;
type point3 = Vec3;
type color = Vec3;
use image::RgbImage;
const infinity: f64 = std::f64::INFINITY;
const pi: f64 = 3.1415926535897932385;

// fn random_double() -> f64 {
//     random_double()
// }

// Call the procedural macro, which will become `make_spheres` function.
//make_spheres_impl! {}

// These three structs are just written here to make it compile.
// You should `use` your own structs in this file.
// e.g. replace next two lines with
// `use crate::materials::{DiffuseLight, ConstantTexture}

//use crate::materials::{DiffuseLight, ConstantTexture};
// pub struct ConstantTexture(Vec3);         //纹理
// pub struct DiffuseLight(ConstantTexture); //漫反射光

// impl ConstantTexture {
//     pub fn new() -> Self {
//         Self(Vec3::ones())
//     }
// }

// impl DiffuseLight {
//     pub fn new() -> Self{
//         Self(ConstantTexture::new())
//     }
// }

pub struct Sphere {
    center: Vec3,            //中心位置
    radius: f64,             //半径
    //material: DiffuseLight,  //材质
    material: Arc<dyn material::Material>,
}

impl Sphere {
    pub fn new_without_para() -> Self {
        Self {
            center: Vec3::zero(),
            radius: 0.0,
            material: Arc::new(material::lambertian::new(&Vec3::ones()))
        }
    }

    pub fn new(center: point3, radius: f64, material: Arc<dyn material::Material>) -> Self{
        Self {center, radius, material}
    }

    pub fn get_sphere_uv(p: &point3, u: &mut f64, v: &mut f64) {
            // p: a given point on the sphere of radius one, centered at the origin.
            // u: returned value [0,1] of angle around the Y axis from X=-1.
            // v: returned value [0,1] of angle from Y=-1 to Y=+1.
            //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
            //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
            //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
            let theta = f64::acos(-p.y());
            let phi = f64::atan2(-p.z(), p.x()) + pi;
            *u = phi/(2.0*pi);
            *v = theta/pi;
    }
    pub fn get_sphere_u(p: &point3) -> f64 {
        let theta = f64::acos(-p.y());
        let phi = f64::atan2(-p.z(), p.x()) + pi;
        return phi/(2.0*pi);
    }
    pub fn get_sphere_v(p: &point3) -> f64 {
        let theta = f64::acos(-p.y());
        let phi = f64::atan2(-p.z(), p.x()) + pi;
        return theta / pi;
    }
}

// pub fn example_scene() -> World {
//     let mut spheres: Vec<Box<Sphere>> = make_spheres(); // Now `spheres` stores two spheres.
//     let mut hittable_list = vec![];
//     // You can now add spheres to your own world
//     hittable_list.append(&mut spheres);

//     hittable_list.clear();
//     World { height: 512 }
// }

pub fn hit_sphere(center: Vec3, radius: f64, r: &ray::Ray) -> f64{
    let oc: Vec3 = *r.origin() - center;
    let a: f64 = Vec3::dot(r.direction(), r.direction());
    //let b: f64 = 2.0 * Vec3::dot(r.direction(), &oc);
    let half_b: f64 = Vec3::dot(r.direction(), &oc);
    let c: f64 = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant: f64 = half_b * half_b - a * c;
    if discriminant < 0.0 { return -1.0 }
    else {
        let t1: f64 = (-half_b - f64::sqrt(discriminant)) / a;
        if t1 > 0.0 {return t1}
        else {
            let t2: f64 = (-half_b + f64::sqrt(discriminant)) / a;
            if t2 > 0.0 {return t2}
            else {return -1.0}
        }
    }
}

// pub fn ray_color(r: &Ray, world: &hittable) -> Vec3 {
//     let mut rec = hit_record::new();
//     if world.hit(r, 0.0, infinity, &mut rec) {
//         let target = rec.œp + rec.normal + random_in_unit_sphere();
//         return (rec.normal + Vec3::ones()) * 0.5
//     }
//     let unit_direction: Vec3 = r.direction().unit();
//     let t: f64 = (unit_direction.y() + 1.0) * 0.5;
//     Vec3::ones() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
// }

#[derive(Clone)] 
pub struct hit_record {
    pub p: point3,        //交点
    pub normal: Vec3,     //法向量
    pub t: f64,           //距离
    pub u: f64,           //u坐标
    pub v: f64,           //v坐标
    pub front_face: bool, //始终使得法线的方向与射线的方向相反
    pub mat_ptr: Arc<dyn material::Material>,
}

impl hit_record {
    // pub fn print(&self) {
    //     println!("{:?}\n{:?}\n{} {} {} {}", self.p, self.normal, self.t, self.u, self.v, self.front_face);
    // }
    pub fn new() -> Self { //默认构造函数
        Self {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: false,
            mat_ptr: Arc::new(material::lambertian::new(&Vec3::ones())),
        }
    }

    pub fn set_face_normal(&mut self, r: &ray::Ray, outward_normal: &mut Vec3) {
        self.front_face = Vec3::dot(r.direction(), &(*outward_normal).clone()) < 0.0;
        if self.front_face {self.normal = *outward_normal;}  //outward_normal.clone()
        else {self.normal = -*outward_normal}
    }//////////////////
}

//设计一个hittable的trait,并限定t的范围
//当t_min<t<t_max时才认为有交点
pub trait hittable: Sync + Send {
    fn hit(&self, r: &mut ray::Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut bvh::aabb) -> bool;
}

impl hittable for Sphere {
    fn hit(&self, r: &mut ray::Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool{
        let oc = *r.origin() - self.center;
        let a = r.direction().squared_length();
        let half_b = Vec3::dot(r.direction(), &oc);
        let c = oc.squared_length() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {return false}
        let sqrtd = f64::sqrt(discriminant);

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let mut outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &mut outward_normal);
        //Sphere::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.u = Sphere::get_sphere_u(&outward_normal);
        rec.v = Sphere::get_sphere_v(&outward_normal);
        rec.mat_ptr = Arc::clone(&self.material);
        return true
    }
    //球体的包围盒
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut bvh::aabb) -> bool {
        // (*output_box).minimum = self.center - Vec3::new(self.radius, self.radius, self.radius);
        // (*output_box).maximum = self.center + Vec3::new(self.radius, self.radius, self.radius);
        *output_box = bvh::aabb::new_with_para( &(self.center - Vec3::new(self.radius, self.radius, self.radius))
                                               ,&(self.center + Vec3::new(self.radius, self.radius, self.radius)) );
        return true;
    }
}


//可命中对象列表
#[derive(Clone)]
pub struct hittable_list {
    pub objects: Vec< Arc<dyn hittable> >,
}

impl hittable_list {
    pub fn new_without_para() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn new(object: Arc<dyn hittable>) -> Self {
        Self {
            objects: vec![object]  //使用宏
        }
    }
    pub fn add(&mut self, object: Arc<dyn hittable>) {
        self.objects.push(object);
    }
    // pub fn clear(&mut self) {
    //     self.objects.clear();
    // }
}

impl hittable for hittable_list {
    fn hit(&self, r: &mut ray::Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool{
        //let mut temp_rec = hit_record::new();
        let temp_rec = &mut hit_record::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        //for object in &self.objects {
        for object in self.objects.iter() {
            //hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut hit_record)
            if object.hit(r, t_min, closest_so_far, temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        return hit_anything;
    }

    fn bounding_box(&self, time0: f64, time1: f64, mut output_box: &mut bvh::aabb) -> bool {
        if self.objects.is_empty() {return false}

        let temp_box = &mut bvh::aabb::new();
        let mut first_box = true;

        for object in &self.objects {
            let damn = temp_box.clone();
            if !object.bounding_box(time0, time1, temp_box) {return false}
            if first_box {
                *output_box = damn;
            }
            else {
                // let damn_bro = output_box.clone();
                // let damn_it = temp_box.clone();
                // let damnit = bvh::aabb::surrounding_box(damn_bro, damn_it);
                // output_box.minimum = damnit.min();
                // output_box.maximum = damnit.max();
                *output_box = bvh::aabb::surrounding_box(output_box.clone(), temp_box.clone());
            }
            first_box = false;
        }

        return true
    }
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {return min};
    if x > max {return max};
    return x;
}

pub fn write_color(pixel_color: Vec3, samples_per_pixel: usize, img: &mut RgbImage, i: usize, j: usize,) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
    //根据样本数对颜色取平均值
    let scale = 1.0 / (samples_per_pixel as f64);
    //取根号, Gamma校正
    r = f64::sqrt(r * scale);
    g = f64::sqrt(g * scale);
    b = f64::sqrt(b * scale);
    let pixel = img.get_pixel_mut(i.try_into().unwrap(), j.try_into().unwrap());
    *pixel = image::Rgb([
    // print!("{} ", (256.0 * clamp(r, 0.0, 0.999)) as i32);
    // print!("{} ", (256.0 * clamp(g, 0.0, 0.999)) as i32);
    // print!("{}\n", (256.0 * clamp(b, 0.0, 0.999)) as i32);
        (256.0 * clamp(r, 0.0, 0.999)).floor() as u8,
        (256.0 * clamp(g, 0.0, 0.999)).floor() as u8,
        (256.0 * clamp(b, 0.0, 0.999)).floor() as u8,
    ])
}



//hittable的变换类 (实例的移动/坐标偏移)
pub struct translate {
    offset: Vec3,
    ptr: Arc<dyn hittable>,
}

impl translate {
    pub fn new(p: Arc<dyn hittable>, displacement: &Vec3) -> Self {
        Self{
            ptr: p,
            offset: *displacement,
        }
    }
}

impl hittable for translate {
    fn hit(&self, r: &mut ray::Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
        let mut moved_r = ray::Ray::new(*r.origin()-self.offset, *r.direction(), r.time());
        if !self.ptr.hit(&mut moved_r, t_min, t_max, rec) {
            return false;
        }

        rec.p += self.offset;
        rec.set_face_normal(&moved_r, &mut rec.normal.clone());

        return true;
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut bvh::aabb) -> bool {
        if !self.ptr.bounding_box(time0, time1, output_box) {
            return false;
        }

        *output_box = bvh::aabb::new_with_para(
            &(output_box.min() + self.offset),
            &(output_box.max() + self.offset)
        );

        return true;
    }
}


pub struct rotate_y {
    ptr: Arc<dyn hittable>,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: bvh::aabb,
}

impl rotate_y {
    pub fn new(p: Arc<dyn hittable>, angle: f64) -> Self {
        let radians = camera::degrees_to_radians(angle);
        let _sin_theta = f64::sin(radians);
        let _cos_theta = f64::cos(radians);
        let mut _bbox = bvh::aabb::new();
        let _hasbox = p.bounding_box(0.0, 1.0, &mut _bbox);

        let mut min = point3::new(infinity, infinity, infinity);
        let mut max = point3::new(-infinity, -infinity, -infinity);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * _bbox.max().x() + (1-i) as f64 * _bbox.min().x();
                    let y = j as f64 * _bbox.max().y() + (1-j) as f64 * _bbox.min().y();
                    let z = k as f64 * _bbox.max().z() + (1-k) as f64 * _bbox.min().z();

                    let newx =  _cos_theta * x + _sin_theta * z;
                    let newz = -_sin_theta * x + _cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    min.x = f64::min(min.x(), tester.x());
                    min.y = f64::min(min.y(), tester.y());
                    min.z = f64::min(min.z(), tester.z());
                    max.x = f64::max(max.x(), tester.x());
                    max.y = f64::max(max.y(), tester.y());
                    max.z = f64::max(max.z(), tester.z());
                }
            }
        }

        Self {
            ptr: p,
            sin_theta: _sin_theta,
            cos_theta: _cos_theta,
            hasbox: _hasbox,
            bbox: _bbox,
        }
    }
}

impl hittable for rotate_y {
    fn hit(&self, r: &mut ray::Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
        let mut origin = *r.origin();
        let mut direction = *r.direction();

        origin.x = self.cos_theta * r.origin().x() - self.sin_theta * r.origin().z();
        origin.z = self.sin_theta * r.origin().x() + self.cos_theta * r.origin().z();

        direction.x = self.cos_theta * r.direction().x() - self.sin_theta * r.direction().z();
        direction.z = self.sin_theta * r.direction().x() + self.cos_theta * r.direction().z();

        let mut rotated_r = ray::Ray::new(origin, direction, r.time());

        if !self.ptr.hit(&mut rotated_r, t_min, t_max, rec) {return false}

        let mut p = rec.p.clone();
        let mut normal = rec.normal.clone();

        p.x =  self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z();
        p.z = -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z();

        normal.x =  self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z();
        normal.z = -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z();

        rec.p = p;
        rec.set_face_normal(&rotated_r, &mut normal);

        return true;
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut bvh::aabb) -> bool {
        *output_box = self.bbox.clone();
        return self.hasbox;
    }
}