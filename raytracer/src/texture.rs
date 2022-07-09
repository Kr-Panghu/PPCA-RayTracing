use crate::Vec3;
use crate::material;
use crate::scene;
use crate::rtweekend;
use crate::ray;
use std::rc::Rc;

type color = Vec3;
type point3 = Vec3;

//纹理特性(纹理基类)
pub trait texture {
    fn value(&self, u: f64, v: f64, p: &point3) -> color {
        return Vec3::zero()
    }
}

//常量颜色纹理
pub struct solid_color {
    color_value: color,
}

impl solid_color {
    pub fn new() -> Self {
        Self {color_value: Vec3::zero(),}
    }
    pub fn new_with_para(c: &color) -> Self {
        Self {color_value: *c,}
    }
    pub fn new_with_3para(r: f64, g: f64, b: f64) -> Self {
        Self {color_value: Vec3::new(r,g,b)}
    }
}

impl texture for solid_color {
    fn value(&self, u: f64, v: f64, p: &point3) -> color {
        return self.color_value
    }
}



//格子纹理:checker_texture
//以一种规则的方式交替正弦和余弦的符号,可以创建一个2D棋盘格纹理
//如果在三维空间中乘上三角函数,这个乘积的符号就会形成一个3D棋盘格图案
pub struct checker_texture {
    odd: Rc<dyn texture>,
    even: Rc<dyn texture>,
}

impl checker_texture {
    pub fn new() -> Self {
        Self {
            odd: Rc::new(solid_color::new()),
            even: Rc::new(solid_color::new()),
        }
    }
    pub fn new_with_ptr(_even: Rc<dyn texture>, _odd: Rc<dyn texture>) -> Self {
        Self {
            even: _even.clone(),
            odd: _odd.clone(),
        }
    }
    pub fn new_with_para(c1: &color, c2: &color) -> Self {
        Self {
            even: Rc::new(solid_color::new_with_para(c1)),
            odd: Rc::new(solid_color::new_with_para(c2)),
        }
    }
}

impl texture for checker_texture {
    fn value(&self, u: f64, v: f64, p: &point3) -> color {
        let sines = f64::sin(10.0*p.x()) * f64::sin(10.0*p.y()) * f64::sin(10.0*p.z());
        if sines < 0.0 {
            return self.odd.value(u,v,p);
        }
        else {
            return self.even.value(u,v,p);
        }
    }
}
