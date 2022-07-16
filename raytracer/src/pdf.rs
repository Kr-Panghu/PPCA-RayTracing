use crate::scene::*;
use std::sync::Arc;
use crate::Vec3;
use std::cmp::Ordering;
use crate::BASIC::ray::*;
use crate::BASIC::rtweekend::*;
use crate::BVH::aabb::*;
use crate::BASIC::onb::ONB;

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    uvw: ONB
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self{
        let mut uvw = ONB::new();
        uvw.build_from_w(w);
        Self{uvw}
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = Vec3::dot(&direction.unit(), &self.uvw.w());
        if cosine <= 0.0 {return 0.0}
        cosine / pi
    }
    fn generate(&self) -> Vec3 {
        self.uvw.local_with_vec(&random_cosine_direction())
    }
}


pub struct HittablePdf {
    o: Vec3,
    ptr: Arc<dyn hittable>
}

impl HittablePdf {
    pub fn new(p: Arc<dyn hittable>, origin: &Vec3) -> Self {
        Self {ptr: p, o: *origin}
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }
}


pub struct mixture_pdf {
    p: Vec<Arc<dyn Pdf> >
}

impl mixture_pdf {
    pub fn new(p0: Arc<dyn Pdf>, p1: Arc<dyn Pdf>) -> Self {
        Self{
            p: vec![p0, p1]
        }
    }
}

impl Pdf for mixture_pdf {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }
    fn generate(&self) -> Vec3 {
        if random_double_1() < 0.5 {
            return self.p[0].generate()
        }
        self.p[1].generate()
    }
}