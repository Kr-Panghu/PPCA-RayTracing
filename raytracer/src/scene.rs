#![allow(dead_code)]
// You SHOULD remove above line in your code.

use crate::Vec3;
use crate::World;
use std::rc::Rc;
use crate::material;
// use std::num;
use raytracer_codegen::make_spheres_impl;
type point3 = Vec3;
type color = Vec3;

const infinity: f64 = std::f64::INFINITY;
const pi: f64 = 3.1415926535897932385;

fn random_double() -> f64 {
    random_double()
}

fn degrees_to_radians(degrees: f64) -> f64{ //度数到弧度
    degrees * pi / 180.0
}

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
    material: Rc<dyn material::Material>,
}

impl Sphere {
    pub fn new_without_para() -> Self {
        Self {
            center: Vec3::zero(),
            radius: 0.0,
            material: Rc::new(material::lambertian::new(&Vec3::ones()))
        }
    }

    pub fn new(center: point3, radius: f64, material: Rc<dyn material::Material>) -> Self{
        Self {center, radius, material}
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

pub struct Ray {
    pub orig: point3,
    pub dir: color,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}

// fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> bool{
//     let oc: Vec3 = *r.origin() - center;
//     let a: f64 = Vec3::dot(r.direction(), r.direction());
//     let b: f64 = 2.0 * Vec3::dot(r.direction(), &oc);
//     let c: f64 = Vec3::dot(&oc, &oc) - radius * radius;
//     let discriminant: f64 = b * b - 4.0 * a * c;
//     return discriminant > 0.0
// }

pub fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> f64{
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
//         let target = rec.p + rec.normal + random_in_unit_sphere();
//         return (rec.normal + Vec3::ones()) * 0.5
//     }
//     let unit_direction: Vec3 = r.direction().unit();
//     let t: f64 = (unit_direction.y() + 1.0) * 0.5;
//     Vec3::ones() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
// }

pub struct hit_record {
    pub p: point3,        //交点
    pub normal: Vec3,     //法向量
    pub t: f64,           //距离
    pub front_face: bool, //始终使得法线的方向与射线的方向相反
    pub mat_ptr: Rc<dyn material::Material>,
}

impl hit_record {
    pub fn new() -> Self { //默认构造函数
        Self {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: true,
            mat_ptr: Rc::new(material::lambertian::new(&Vec3::ones())),
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = Vec3::dot(r.direction(), outward_normal) < 0.0;
        if self.front_face {self.normal = *outward_normal;}
        else {self.normal = -(*outward_normal)}
    }
}

//设计一个hittable的trait,并限定t的范围
//当t_min<t<t_max时才认为有交点
pub trait hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool{
        true
    }
}

impl hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool{
        let oc: Vec3 = *r.origin() - self.center;
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
            // rec.t = t;
            // rec.p = r.at(t);
            // let outward_normal = (rec.p - self.center) / self.radius;
            // rec.set_face_normal(r, &outward_normal);
            // rec.mat_ptr = Rc::clone(&self.material);  //????????
            // return true
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal: Vec3 = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = Rc::clone(&self.material);
        return true
    }
}


//可命中对象列表
pub struct hittable_list {
    objects: Vec< Rc<dyn hittable> >,
}

impl hittable_list {
    pub fn new_without_para() -> Self {
        Self {
            objects: vec![],
        }
    }
    pub fn new(object: Rc<dyn hittable>) -> Self {
        Self {
            objects: vec![object]  //使用宏
        }
    }
    pub fn add(&mut self, object: Rc<dyn hittable>) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl hittable for hittable_list {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, mut rec: &mut hit_record) -> bool{
        //let mut temp_rec = hit_record::new();
        let temp_rec = &mut hit_record::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            //hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut hit_record)
            if (*object).hit(r, t_min, closest_so_far, temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                //rec = temp_rec;
                rec.front_face = (&temp_rec).front_face;
                rec.t = (&temp_rec).t;
                rec.normal = Vec3::new((&temp_rec).normal.x,(&temp_rec).normal.y,(&temp_rec).normal.z);
                rec.p = Vec3::new((&temp_rec).p.x,(&temp_rec).p.y,(&temp_rec).p.z);
            }
        }
        return hit_anything;
    }
}

//摄像机类
pub struct camera {
    origin: point3,
    lower_left_corner: point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl camera {
    pub fn new() -> Self {
        let aspect_ratio: f64 = 16.0 / 9.0; //纵横比
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;  //焦距
        Self {
            origin: point3::zero(),
            horizontal: Vec3::new(viewport_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, viewport_height, 0.0),
            lower_left_corner: point3::zero() - Vec3::new(viewport_width, 0.0, 0.0) / 2.0 - Vec3::new(0.0, viewport_height, 0.0) / 2.0 - Vec3::new(0.0, 0.0, focal_length),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
    }
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {return min};
    if x > max {return max};
    return x;
}

pub fn write_color(pixel_color: Vec3, samples_per_pixel: i32) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
    //根据样本数对颜色取平均值
    let scale = 1.0 / (samples_per_pixel as f64);
    //取根号, Gamma校正
    r = f64::sqrt(r * scale);
    g = f64::sqrt(g * scale);
    b = f64::sqrt(b * scale);
    print!("{} ", (256.0 * clamp(r, 0.0, 0.999)) as i32);
    print!("{} ", (256.0 * clamp(g, 0.0, 0.999)) as i32);
    print!("{}\n", (256.0 * clamp(b, 0.0, 0.999)) as i32);
}