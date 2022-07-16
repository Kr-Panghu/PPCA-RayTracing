use std::ops::{Add, Sub, Neg, Div, Mul, AddAssign, MulAssign, SubAssign};
//use std::cmp::PartialEq;
use crate::rtweekend;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn ones() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn x(&self) -> f64{
        self.x
    }

    pub fn y(&self) -> f64{
        self.y
    }

    pub fn z(&self) -> f64{
        self.z
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn elemul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn unit(&self) -> Self {
        let l = self.length();
        let self_tmp = Vec3::new(self.x(), self.y(), self.z());
        self_tmp / l
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn get(&self, index: i32) -> f64 {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Index of 'Vector3' is out of bound!"),
        }
    }

    // pub fn write_color(&self) { //for pixel_color
    //     let r = 
    //     print!("{} ", (255.999 * self.x()) as i32);
    //     print!("{} ", (255.999 * self.y()) as i32);
    //     print!("{}\n", (255.999 * self.z()) as i32);
    // }
    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        let tmp_v = Vec3::new(v.x(), v.y(), v.z());
        let tmp_n = Vec3::new(n.x(), n.y(), n.z());
        return tmp_v - tmp_n * Vec3::dot(v, n) * 2.0
    }

    //折射
    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(Vec3::dot(&(-*uv), n), 1.0);
        let r_out_perp = (*uv + (*n) * cos_theta) * etai_over_etat;
        let tmp = f64::abs(1.0 - r_out_perp.squared_length());
        let r_out_parallel = - (*n) * f64::sqrt(tmp);
        return r_out_perp + r_out_parallel
    }

    pub fn cdot(u: &Vec3, v: &Vec3) -> Vec3 {
        let x1 = u.x * v.x;
        let x2 = u.y * v.y;
        let x3 = u.z * v.z;
        Vec3::new(x1,x2,x3)
    }

    pub fn near_zero(&self) -> bool {
        let s: f64 = 0.00000001;
        let x1 = f64::abs(self.x);
        let x2 = f64::abs(self.y);
        let x3 = f64::abs(self.z);
        return x1 < s && x2 < s && x3 < s
    }
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(rtweekend::random_double_2(-1.0, 1.0), rtweekend::random_double_2(-1.0,1.0), 0.0);
            if p.squared_length() < 1.0 {
                return p
            }
        }
    }
}


// impl Clone for Vec3 {
//     fn clone(&self) -> Self {
//         *self
//     }
// }

impl Add for Vec3 { //Add trait, 重载加号
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: 0.0 - self.x,
            y: 0.0 - self.y,
            z: 0.0 - self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x, 
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self {
            x: self.x - other, 
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
            self.x *= other;
            self.y *= other;
            self.z *= other;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        if other == 0.0 {
            panic!("Cannot divide by zero-valued `Rational`!");
        }
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    // fn mul(self, other: Self) -> f64 {
    //     self.x * other.x + self.y * other.y + self.z * other.z
    // }
    fn mul(self, other: Self) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl PartialEq<f64> for Vec3 {
    fn eq(&self, other: &f64) -> bool {
        self.x == *other &&
        self.y == *other &&
        self.z == *other
    }
}






//----------------------------------------------------------





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(3.0, 4.0, 5.0)
        )
    }

    #[test]
    fn test_add_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x += Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(3.0, 4.0, 5.0))
    }

    #[test]
    fn test_add_f64() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) + 233.0,
            Vec3::new(234.0, 233.0, 232.0)
        )
    }

    
    #[test]
    fn test_add_assign_f64() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x += 233.0;
        assert_eq!(x, Vec3::new(234.0, 233.0, 232.0))
    }
    
    #[test]
    fn test_sub() {
        assert_eq!(
            Vec3::new(1.0, 0.0, -1.0) - Vec3::new(2.0, 4.0, 6.0),
            Vec3::new(-1.0, -4.0, -7.0)
        )
    }

    #[test]
    fn test_sub_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x -= Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(x, Vec3::new(-1.0, -4.0, -7.0))
    }

    #[test]
    fn test_sub_f64() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) - 1.0, Vec3::new(0.0, -1.0, -2.0))
    }

    #[test]
    fn test_sub_assign_f64() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x -= 1.0;
        assert_eq!(x, Vec3::new(0.0, -1.0, -2.0))
    }

    #[test]
    fn test_mul() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) * Vec3::ones(), 0.0);
    }

    #[test]
    fn test_mul_assign() {
        let mut x = Vec3::new(1.0, 0.0, -1.0);
        x *= 2.0;
        assert_eq!(x, Vec3::new(2.0, 0.0, -2.0));
    }

    #[test]
    fn test_mul_f64() {
        assert_eq!(Vec3::new(1.0, 0.0, -1.0) * 1.0, Vec3::new(1.0, 0.0, -1.0));
    }

    #[test]
    fn test_div() {
        assert_eq!(Vec3::new(1.0, -2.0, 0.0) / 2.0, Vec3::new(0.5, -1.0, 0.0));
    }

    #[test]
    fn test_elemul() {
        assert_eq!(
            Vec3::elemul(Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 2.0, 3.0)),
            Vec3::new(1.0, 4.0, 9.0)
        );
    }

    #[test]
    fn test_cross() {
        assert_eq!(
            Vec3::cross(Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 3.0, 4.0)),
            Vec3::new(8.0 - 9.0, 6.0 - 4.0, 3.0 - 4.0)
        );
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Vec3::new(1.0, -2.0, 3.0), Vec3::new(-1.0, 2.0, -3.0));
    }
    

    #[test]
    fn test_squared_length() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0).squared_length(), 14.0 as f64);
    }


    #[test]
    fn test_length() {
        assert_eq!(
            Vec3::new(3.0, 4.0, 5.0).length(),
            ((3.0 * 3.0 + 4.0 * 4.0 + 5.0 * 5.0) as f64).sqrt()
        );
    }

    #[test]
    fn test_unit() {
        assert_eq!(Vec3::new(233.0, 0.0, 0.0).unit(), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(
            Vec3::new(-233.0, 0.0, 0.0).unit(),
            Vec3::new(-1.0, 0.0, 0.0)
        );
    }

    #[test]
    #[should_panic]
    fn test_unit_panic() {
        Vec3::new(0.0, 0.0, 0.0).unit();
    }

}
