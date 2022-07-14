use crate::Vec3;
use crate::rtweekend;
type point3 = Vec3;
pub const point_count: usize = 256;

pub struct Perlin { //柏林噪声
    //ranfloat: Vec<f64>, //*mut 原生指针
    perm_x: Vec<i64>,
    perm_y: Vec<i64>,
    perm_z: Vec<i64>,
    ranvec: Vec<Vec3>,
    //把随机浮点数更改为随机向量
    //这些向量是任何合理的不规则方向集 
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
    permute(&mut p, 256);
    return p;
}

impl Perlin {
    pub fn new() -> Self {
        let mut _ranvec: Vec<Vec3> = Vec::with_capacity(256);
        for i in 0..256 {
            _ranvec.push(Vec3::random_vector_2(-1.0,1.0).unit());
            //_ranfloat[i as usize] = rtweekend::random_double_1();
        }
        Self {
            ranvec: _ranvec.clone(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &point3) -> f64 {
        let u = p.x() - f64::floor(p.x());
        let v = p.y() - f64::floor(p.y());
        let w = p.z() - f64::floor(p.z());

        // let i = (4.0*p.x()) as usize & 255;
        // let j = (4.0*p.y()) as usize & 255;
        // let k = (4.0*p.z()) as usize & 255;
        let i = f64::floor(p.x()) as i32;
        let j = f64::floor(p.y()) as i32;
        let k = f64::floor(p.z()) as i32;
        // let mut arr: [[i32; 4]; 4] = [[2, 4, 6, 8],
        // [1, 2, 3, 5],
        // [4, 5, 8, 3],
        // [5, 8, 9, 2]];

        let mut c: [[[Vec3; 2];2];2] = [ [[Vec3::zero(),Vec3::zero()],[Vec3::zero(),Vec3::zero()]],
                                        [[Vec3::zero(),Vec3::zero()],[Vec3::zero(),Vec3::zero()]] ];

        for di in 0..2 {     //三线性插值:使平滑
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(
                        self.perm_x[((i+di as i32) & 255) as usize] ^ 
                        self.perm_y[((j+dj as i32) & 255) as usize] ^ 
                        self.perm_z[((k+dk as i32) & 255) as usize]
                    ) as usize];
                }
            }
        }
        //return trilinear_interp(c, u, v, w)
        return perlin_interp(c.clone(), u, v, w)
        //return self.ranfloat[self.perm_x[i] as usize ^ self.perm_y[j] as usize ^ self.perm_z[k] as usize]
    }

    //湍流:使用具有多个求和频率的复合噪声
    //重复调用噪声的总和
    pub fn turb(&self, p: &mut point3) -> f64 {
        let depth = 7;
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for i in 0..depth {
            accum += weight*self.noise(temp_p);
            weight *= 0.5;
            *temp_p *= 2.0;
        }

        return f64::abs(accum)
    }
}

// pub fn trilinear_interp(c: [[[f64; 2];2];2], u: f64, v:f64, w: f64) -> f64 {
//     let mut accum: f64 = 0.0;
//     for i in 0..2 {
//         for j in 0..2 {
//             for k in 0..2 {
//                 accum += (i as f64*u + (1.0-i as f64) * (1.0-u))
//                         *(j as f64*v + (1.0-j as f64) * (1.0-v))
//                         *(k as f64*w + (1.0-k as f64) * (1.0-w))
//                         *c[i][j][k];
//             }
//         }
//     }
//     return accum
// }

pub fn perlin_interp(c: [[[Vec3; 2];2];2], u: f64, v: f64, w: f64) -> f64 {
    //Hermitian厄米平滑改进
    let uu = u*u*(3.0-2.0*u);
    let vv = v*v*(3.0-2.0*v);
    let ww = w*w*(3.0-2.0*w);
    let mut accum: f64 = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let weight_v = Vec3::new(u-i as f64,v-j as f64,w-k as f64);
                accum += (i as f64*uu + (1-i)as f64*(1.0-uu))
                        *(j as f64*vv + (1-j)as f64*(1.0-vv))
                        *(k as f64*ww + (1-k)as f64*(1.0-ww))
                        *Vec3::dot(&c[i][j][k].clone(), &weight_v);
            }
        }
    }
    return accum
}