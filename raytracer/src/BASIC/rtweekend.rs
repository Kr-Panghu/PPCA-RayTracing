//存放一些常量以及随机函数
use crate::Vec3;
use rand::Rng;
use rand::prelude::ThreadRng;
pub const infinity: f64 = std::f64::INFINITY;
pub const pi: f64 = 3.1415926535897932385;
pub const e: f64 = std::f64::consts::E;
pub fn random_double_1() -> f64 {
    // let secret_number = rand::thread_rng().gen_range(1..10000);
    // return (secret_number as f64) / 10001.0
    rand::random::<f64>()
}

pub fn random_double_2(min: f64, max: f64) -> f64 {
    return min + (max - min) * random_double_1()
}

// pub fn random_unit_vector() -> Vec3 {
//     let a = random_double_2(0.0, 2.0 * pi);
//     let z = random_double_2(-1.0, 1.0);
//     let r = f64::sqrt(1.0 - z * z);
//     return Vec3::new(r * f64::cos(a), r * f64::sin(a), z)
// }

pub fn random_unit_vector() -> Vec3 {
    return random_in_unit_sphere().unit()
}

pub fn random_in_unit_sphere() -> Vec3 { //"拒绝算法"在球中产生随机点
    loop {
        let p = Vec3::random_vector_2(-1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p
        }
    }
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 { //半球
    let in_unit_sphere = random_in_unit_sphere();
    
    if Vec3::dot(&in_unit_sphere, normal) > 0.0 
        { return in_unit_sphere }
    return -in_unit_sphere
}

impl Vec3 {
    pub fn random_vector_1() -> Vec3 {
        return Vec3::new(random_double_1(),random_double_1(),random_double_1());
    }
    pub fn random_vector_2(min: f64, max: f64) -> Vec3 {
        return Vec3::new(random_double_2(min, max),random_double_2(min, max),random_double_2(min, max));
    }
}

pub fn random_int(min: i64, max: i64) -> i64{
    //返回[min, max]中的整数
    //round函数返回距离最近的整数
    // let mut random: ThreadRng = rand::thread_rng();
    // return random.gen_range(min..=max);
    // return random_double_2(min as f64 - 0.5, max as f64 + 0.4999).round() as i64;
    random_double_2(min as f64 , max as f64 + 1.0) as i64
}

pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_double_1();
    let r2 = random_double_1();
    let z = (1.0 - r2).sqrt();
    
    let phi = 2.0 * pi * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3::new(x, y, z)
}