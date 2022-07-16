// #![allow(dead_code)]
// #![allow(clippy::boxed_local)]
use crate::scene;
use crate::Vec3;
use crate::BASIC::rtweekend;
use crate::BASIC::ray;
use crate::texture;
use crate::BASIC::onb;
use crate::pdf::*;
use std::rc::Rc;
use std::sync::Arc;
type Color = Vec3;

struct ScatterRecord {
    specular_ray: ray::Ray,
    is_specular: bool, //specular reflection: "镜面反射"
    attenuation: Color,
    pdf_ptr: Arc<dyn Pdf>
}


pub trait Material: Sync + Send {
    // fn scatter(&self, r_in:&scene::Ray,  rec:&scene::hit_record,  attenuation:&mut Vec3,  scattered:&mut scene::Ray) -> bool{
    //     return true
    // }
    //fn scatter(&self, r_in:&ray::Ray,  rec:&scene::hit_record,  attenuation:&mut Vec3,  scattered:&mut ray::Ray) -> bool;
    fn scatter(&self, r_in:&ray::Ray,  rec:&scene::hit_record,  alb:&mut Vec3,  scattered:&mut ray::Ray, pdf: &mut f64) -> bool {
        false
    }
    fn emitted(&self, r_in: &ray::Ray, rec: &scene::hit_record, u: f64, v: f64, p: &Vec3) -> Color {
        return Vec3::zero() //并不需要让所有材质实现emitted, 默认返回黑色
    }
    fn scattering_pdf(&self, r_in: &ray::Ray, rec: &mut scene::hit_record, scattered: &ray::Ray) -> f64 {
        0.0
    }
}

//朗伯材料
//它既可以始终散射并通过其反射率 R 进行衰减
//也可以不衰减地散射但需要吸收 1 − R 部分的光线
//或者可以混合使用这些策略
pub struct lambertian {
    albedo: Arc<dyn texture::texture>,
}

impl lambertian {
    pub fn new(a: &Vec3) -> Self {
        Self {
            albedo: Arc::new(texture::solid_color::new_with_para(a)),
        }
    }
    pub fn new_with_ptr(a: Arc<dyn texture::texture>) -> Self {
        Self {
            albedo: a,
        }
    }
}

impl Material for lambertian {
    // fn scatter(&self, r_in: &ray::Ray, rec: &scene::hit_record, attenuation: &mut Vec3, mut scattered: &mut ray::Ray, pdf: &mut f64) -> bool {
    //     //let mut scatter_direction = rec.normal + rtweekend::random_unit_vector();        
    //     //if scatter_direction.near_zero() {scatter_direction = rec.normal;}
        
    //     let direction = rtweekend::random_in_hemisphere(&rec.normal);  //随机半球抽样
    //     *scattered = ray::Ray::new(rec.p, direction.unit(), r_in.time());
    //     *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
    //     //*pdf = Vec3::dot( &rec.normal, scattered.direction() ) / rtweekend::pi ;
    //     *pdf = 0.5 / rtweekend::pi ;
    //     return true
    // }
    fn scatter(&self, r_in: &ray::Ray, rec: &scene::hit_record, alb: &mut Vec3, mut scattered: &mut ray::Ray, pdf: &mut f64) -> bool {
        //let mut scatter_direction = rec.normal + rtweekend::random_unit_vector();        
        //if scatter_direction.near_zero() {scatter_direction = rec.normal;}
        let mut uvw = onb::ONB::new();
        uvw.build_from_w(&rec.normal);
        let direction = uvw.local_with_vec(&rtweekend::random_cosine_direction());
        *scattered = ray::Ray::new(rec.p, direction.unit(), r_in.time());
        *alb = self.albedo.value(rec.u, rec.v, &rec.p);
        //*pdf = Vec3::dot( &rec.normal, scattered.direction() ) / rtweekend::pi ;
        *pdf = Vec3::dot(&uvw.w(), scattered.direction()) / rtweekend::pi ;
        return true
    }
    fn scattering_pdf(&self, r_in: &ray::Ray, rec: &mut scene::hit_record, scattered: &ray::Ray) -> f64 {
        let cosine = Vec3::dot(&rec.normal, &scattered.direction().unit());
        if cosine < 0.0 {return 0.0}
        cosine / rtweekend::pi
    }
}


pub struct metal {
    albedo: Vec3,
    fuzz: f64,
}

impl metal {
    pub fn new(a: &Vec3, f: f64) -> Self {
        Self {
            albedo: Vec3::new(a.x(),a.y(),a.z()),
            fuzz: f64::min(1.0, f),
        }
    }
}

impl Material for metal {
    fn scatter(&self, r_in: &ray::Ray, rec: &scene::hit_record, attenuation: &mut Vec3, scattered: &mut ray::Ray, pdf: &mut f64) -> bool{
        let reflected = Vec3::reflect(&r_in.direction().unit(), &rec.normal);
        // scattered = &mut scene::Ray::new(rec.p, reflected);
        //模糊反射 Fuzzy Reflection
        scattered.orig = rec.p;
        scattered.dir = reflected + rtweekend::random_in_unit_sphere() * self.fuzz;
        scattered.tm = r_in.time();
        *attenuation = self.albedo.clone();
        // attenuation.x = self.albedo.x();
        // attenuation.y = self.albedo.y();
        // attenuation.z = self.albedo.z();
        if Vec3::dot(scattered.direction(), &rec.normal) > 0.0 { return true }
        else { return false }
    }
}

//dielectric: 电介质,绝缘体
//水、玻璃和钻石等透明材料是电介质
//当光线射到它们上时，它分裂为反射射线和折射（透射）射线
//通过在反射和折射之间随机选择，并且每次交互仅生成一条散射射线
pub struct dielectric {
    ir: f64, //index of refraction
}

impl dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {ir: index_of_refraction}
    }

    pub fn reflectence(&self, cosine: f64, ref_idx: f64) -> f64 {
        //使用Schlick近似法计算反射率
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        let tmp = 1.0 - cosine;
        return r0 + (1.0 - r0) * tmp*tmp*tmp*tmp*tmp;
    }
}

impl Material for dielectric {
    fn scatter(&self, r_in: &ray::Ray, rec: &scene::hit_record, attenuation: &mut Vec3, scattered: &mut ray::Ray, pdf: &mut f64) -> bool {
        //attenuation = &mut Vec3::ones();
        attenuation.x = 1.0;
        attenuation.y = 1.0;
        attenuation.z = 1.0;
        let refraction_ratio: f64;
        if rec.front_face {refraction_ratio = 1.0 / self.ir;}
        else {refraction_ratio = self.ir;}

        let unit_direction = r_in.direction().unit();
        let cos_theta = f64::min(Vec3::dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract {direction = Vec3::reflect(&unit_direction, &rec.normal);}
        else {direction = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);}

        scattered.orig = rec.p;
        scattered.dir = direction;
        scattered.tm = r_in.time();
        return true
    }
}



//发光材质
//这里将灯光也当做了一种材质,这种材质可以发光
//但是对光线没有反射,折射等交互作用
pub struct diffuse_light {
    emit: Arc<dyn texture::texture>
}

impl diffuse_light {
    pub fn new_with_ptr(a: Arc<dyn texture::texture>) -> Self {
        Self {emit: a}    ////////////
    }
    pub fn new_with_para(c: &Color) -> Self {
        Self {
            emit: Arc::new(texture::solid_color::new_with_para(&c.clone()))
        }
    }
}

impl Material for diffuse_light {
    fn scatter(&self, r_in: &ray::Ray, rec: &scene::hit_record, attenuation: &mut Vec3, scattered: &mut ray::Ray, pdf: &mut f64) -> bool {
        false //光源并不散射光线
    }
    fn emitted(&self, r_in: &ray::Ray, rec: &scene::hit_record, u: f64, v: f64, p: &Vec3) -> Color {
        if rec.front_face {return self.emit.value(u, v, p)}
        Vec3::zero()
    }
}




//各向同性
pub struct isotropic {
    albedo: Arc<dyn texture::texture>,
}

impl isotropic {
    pub fn new_with_para(c: &Color) -> Self {
        Self {
            albedo: Arc::new(texture::solid_color::new_with_para(c)),
        }
    }
    pub fn new_with_ptr(a: Arc<dyn texture::texture>) -> Self {
        Self {
            albedo: a,
        }
    }
}

impl Material for isotropic {
    fn scatter(&self, r_in: &ray::Ray, rec: &scene::hit_record, attenuation: &mut Vec3, scattered: &mut ray::Ray, pdf: &mut f64) -> bool {
        *scattered = ray::Ray::new(rec.p, rtweekend::random_in_unit_sphere(), r_in.time());
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        return true
    }
}