// #![allow(dead_code)]
// #![allow(clippy::boxed_local)]
use crate::scene;
use crate::Vec3;
use crate::rtweekend;
// You SHOULD remove above line in your code.

// This file shows necessary examples of how to complete Track 4 and 5.

// pub trait Texture {

// }
pub trait Material {
    // fn scatter(&self, r_in:&scene::Ray,  rec:&scene::hit_record,  attenuation:&mut Vec3,  scattered:&mut scene::Ray) -> bool{
    //     return true
    // }
    fn scatter(&self, r_in:&scene::Ray,  rec:&scene::hit_record,  attenuation:&mut Vec3,  scattered:&mut scene::Ray) -> bool;
}

/// `Lambertian` now takes a generic parameter `T`.
/// This reduces the overhead of using `Box<dyn Texture>`
// #[derive(Clone)]
// pub struct Lambertian<T: Texture> {
//     pub albedo: T,
// }

// impl<T: Texture> Lambertian<T> {
//     pub fn new(albedo: T) -> Self {
//         Self { albedo }
//     }
// }

// impl<T: Texture> Material for Lambertian<T> {}

// pub trait Hitable {}
// pub struct AABB;

/// This BVHNode should be constructed statically.
/// You should use procedural macro to generate code like this:
/// ```
/// let bvh = BVHNode::construct(
///     box BVHNode::construct(
///         box Sphere { .. }
///         box Sphere { .. }
///     ),
///     box BVHNode::construct(
///         box Sphere { .. }
///         box Sphere { .. }
///     )
/// )
/// ```
/// 
/// And you can put that `bvh` into your `HittableList`.
// pub struct BVHNode<L: Hitable, R: Hitable> {
//     left: Box<L>,
//     right: Box<R>,
//     bounding_box: AABB,
// }

// impl<L: Hitable, R: Hitable> BVHNode<L, R> {
//     pub fn construct(_left: Box<L>, _right: Box<R>) -> Self {
//         unimplemented!()
//     }
// }

//struct hit_record;


//朗伯材料
//它既可以始终散射并通过其反射率 R 进行衰减
//也可以不衰减地散射但需要吸收 1 − R 部分的光线
//或者可以混合使用这些策略
pub struct lambertian {
    albedo: Vec3,
}

impl lambertian {
    pub fn new(a: &Vec3) -> Self {
        Self {
            albedo: Vec3::new(a.x(),a.y(),a.z()),
        }
    }
}

impl Material for lambertian {
    fn scatter(&self, r_in: &scene::Ray, rec: &scene::hit_record, attenuation: &mut Vec3, mut scattered: &mut scene::Ray) -> bool {
        let mut scatter_direction = rec.normal + rtweekend::random_unit_vector();
        //scattered = &mut scene::Ray::new(rec.p, scatter_direction);
        if scatter_direction.near_zero() {scatter_direction = rec.normal;}
        scattered.orig = rec.p;
        scattered.dir = scatter_direction;
        //attenuation = &mut self.albedo;
        *attenuation = self.albedo.clone();
        // attenuation.x = self.albedo.x();
        // attenuation.y = self.albedo.y();
        // attenuation.z = self.albedo.z();
        return true
    }
}


pub struct metal {
    albedo: Vec3,
    //fuzz: f64,
}

impl metal {
    pub fn new(a: &Vec3) -> Self {
        Self {
            albedo: Vec3::new(a.x(),a.y(),a.z()),
            //fuzz: f64::max(1.0, f),
        }
    }
}

impl Material for metal {
    fn scatter(&self, r_in: &scene::Ray, rec: &scene::hit_record, attenuation: &mut Vec3, scattered: &mut scene::Ray) -> bool{
        let reflected = Vec3::reflect(&r_in.direction().unit(), &rec.normal);
        // scattered = &mut scene::Ray::new(rec.p, reflected);
        scattered.orig = rec.p;
        scattered.dir = reflected;
        *attenuation = self.albedo.clone();
        // attenuation.x = self.albedo.x();
        // attenuation.y = self.albedo.y();
        // attenuation.z = self.albedo.z();
        if Vec3::dot(scattered.direction(), &rec.normal) > 0.0 { return true }
        else { return false }
    }
}

pub struct dielectric {
    ref_idx: f64,
}

impl dielectric {
    pub fn new(ri: f64) -> Self {
        Self {ref_idx: ri}
    }
}

impl Material for dielectric {
    fn scatter(&self, r_in: &scene::Ray, rec: &scene::hit_record, attenuation: &mut Vec3, scattered: &mut scene::Ray) -> bool {
        //attenuation = &mut Vec3::ones();
        attenuation.x = 1.0;
        attenuation.y = 1.0;
        attenuation.z = 1.0;
        let etai_over_etat: f64;
        if rec.front_face {etai_over_etat = 1.0 / self.ref_idx;}
        else {etai_over_etat = self.ref_idx;}
        let unit_direction = r_in.direction().unit();
        let refracted = Vec3::refract(&unit_direction, &rec.normal, etai_over_etat);
        //scattered = &mut scene::Ray::new(rec.p, refracted);
        scattered.orig = rec.p;
        scattered.dir = refracted;
        return true
    }
}