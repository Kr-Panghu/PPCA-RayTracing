//The document was an attempt to achieve triangle,
//However...

use super::*;
use crate::Vec3;
use std::sync::Arc;
use crate::scene::hittable_list;
use crate::material::*;
use crate::BASIC::*;
use std::f64::{INFINITY, NEG_INFINITY};
use rand::{prelude::ThreadRng, Rng};
use crate::scene::{hit_record, hittable};

pub struct Triangle {
    ver: [Vec3; 3],   // 3 vertices of triangle
    normal: Vec3,
    center: Vec3,
    area: f64,
    v: Vec3,
    w: Vec3,
    ab: Vec3,
    ac: Vec3
}

impl Triangle {
    pub fn new(ver: [Vec3; 3]) -> Self {
        let normal = (Vec3::cross(ver[0], ver[1])
                   + Vec3::cross(ver[1], ver[2])
                   + Vec3::cross(ver[2], ver[0]))
                   .unit();
        let center = (ver[0]+ver[1]+ver[2]) / 3.0;
        
        let l0 = (ver[0].x() - ver[1].x()).powi(2)
               + (ver[0].y() - ver[1].y()).powi(2)
               + (ver[0].z() - ver[1].z()).powi(2)
                .sqrt();
        let l1 = (ver[1].x() - ver[2].x()).powi(2)
               + (ver[1].y() - ver[2].y()).powi(2)
               + (ver[1].z() - ver[2].z()).powi(2)
                .sqrt();
        let l2 = (ver[2].x() - ver[0].x()).powi(2)
               + (ver[2].y() - ver[0].y()).powi(2)
               + (ver[2].z() - ver[0].z()).powi(2)
                .sqrt();

        let p = (l0+l1+l2) / 2.0;
        let area = (p * (p-l0) * (p-l1) * (p-l2)).sqrt();
        let mut v = Vec3::cross(normal.clone(), ver[1]-ver[0]);
        v = v / Vec3::dot(&(ver[2]-ver[0]), &v);
        let mut w = Vec3::cross(normal.clone(), ver[2]-ver[0]);
        w = w / Vec3::dot(&(ver[1]-ver[0]), &w);

        Self {
            ver, normal, area, center, v, w, 
            ab: ver[1] - ver[0],
            ac: ver[2] - ver[0]
        }
    }
}

// impl hittable for Triangle {
//     fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
//         let orig = r.origin();
//         let dir = r.direction();
//         let n = self.normal;
//         let cen = self.center;
//         let t = ((cen.x - orig.x) * n.x + (cen.y - orig.y) * n.y + (cen.z - orig.z) * n.z)
//             / (dir.x * n.x + dir.y * n.y + dir.z * n.z);
//         if t.is_nan() || t < t_min || t > t_max { return false }

//         let ap = (*orig + *dir * t) - self.ver[0];
//         let gamma = Vec3::dot(&ap, &self.v);
//         if gamma.is_sign_positive() && gamma < 1.0 {
//             let beta = Vec3::dot(&ap, &self.w);
//             if beta.is_sign_positive() && beta < 1.0 {
//                 let alpha = 1.0 - gamma - beta;
//                 if alpha.is_sign_positive() && alpha < 1.0 {
                    
//                 }
//             }
//         }

//         false
//     }
// }