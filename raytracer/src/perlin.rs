use crate::scene;
use std::rc::Rc;
use crate::Vec3;
use crate::ray;
use crate::rtweekend;
type point3 = Vec3;
pub const point_count: usize = 256;

pub struct Perlin { //柏林噪声
    ranfloat: Vec<f64>, //*mut 原生指针
    perm_x: Vec<i64>,
    perm_y: Vec<i64>,
    perm_z: Vec<i64>,
}

pub fn permute(p: &mut Vec<i64>, n: i64) { // mut p
    for i in (1..n).rev() {
        let target: i64 = rtweekend::random_int(0, i);
        let tmp = p[i as usize];
        p[i as usize] = p[target as usize];
        p[target as usize] = tmp;
    }
}

pub fn perlin_generate_perm() -> Vec<i64> {
    let mut p: Vec<i64> = Vec::with_capacity(256);
    for i in 0..256 {
        p.push(i as i64);
    }
    //let p = vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,64,65,66,67,68,69,70,71,72,73,74,75,76,77,78,79,80,81,82,83,84,85,86,87,88,89,90,91,92,93,94,95,96,97,98,99,100,101,102,103,104,105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,120,121,122,123,124,125,126,127,128,129,130,131,132,133,134,135,136,137,138,139,140,141,142,143,144,145,146,147,148,149,150,151,152,153,154,155,156,157,158,159,160,161,162,163,164,165,166,167,168,169,170,171,172,173,174,175,176,177,178,179,180,181,182,183,184,185,186,187,188,189,190,191,192,193,194,195,196,197,198,199,200,201,202,203,204,205,206,207,208,209,210,211,212,213,214,215,216,217,218,219,220,221,222,223,224,225,226,227,228,229,230,231,232,233,234,235,236,237,238,239,240,241,242,243,244,245,246,247,248,249,250,251,252,253,254,255];
    permute(&mut p, 256);
    return p;
}

impl Perlin {
    pub fn new() -> Self {
        let mut _ranfloat: Vec<f64> = Vec::with_capacity(256);
        for i in 0..256 {
            _ranfloat.push(rtweekend::random_double_1());
            //_ranfloat[i as usize] = rtweekend::random_double_1();
        }
        Self {
            ranfloat: _ranfloat.clone(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &point3) -> f64 {
        let i = (4.0*p.x()) as usize & 255;
        let j = (4.0*p.y()) as usize & 255;
        let k = (4.0*p.z()) as usize & 255;
        return self.ranfloat[self.perm_x[i] as usize ^ self.perm_y[j] as usize ^ self.perm_z[k] as usize]
    }
}