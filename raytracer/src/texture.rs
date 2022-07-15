use crate::Vec3;
use crate::perlin;
use std::rc::Rc;
use crate::scene;
use std::sync::Arc;
use image::*;

type color = Vec3;
type point3 = Vec3;

//纹理特性(纹理基类)
pub trait texture: Sync + Send {
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
        return self.color_value.clone()
    }
}



//格子纹理:checker_texture
//以一种规则的方式交替正弦和余弦的符号,可以创建一个2D棋盘格纹理
//如果在三维空间中乘上三角函数,这个乘积的符号就会形成一个3D棋盘格图案
pub struct checker_texture {
    odd: Arc<dyn texture>,
    even: Arc<dyn texture>,
}

impl checker_texture {
    pub fn new() -> Self {
        Self {
            odd: Arc::new(solid_color::new()),
            even: Arc::new(solid_color::new()),
        }
    }
    pub fn new_with_ptr(_even: Arc<dyn texture>, _odd: Arc<dyn texture>) -> Self {
        Self {
            even: Arc::clone(&_even),
            odd: Arc::clone(&_odd),
        }
    }
    pub fn new_with_para(c1: &color, c2: &color) -> Self {
        Self {
            even: Arc::new(solid_color::new_with_para(c1)),
            odd: Arc::new(solid_color::new_with_para(c2)),
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

//噪音纹理
pub struct noise_texture {
    noise: perlin::Perlin,
    scale: f64,
}

impl noise_texture {
    pub fn new() -> Self {
        Self {
            noise: perlin::Perlin::new(),
            scale: 0.0,
        }
    }
    pub fn new_with_para(sc: f64) -> Self{
        Self {
            noise: perlin::Perlin::new(),
            scale: sc,
        }
    }
}

impl texture for noise_texture {
    fn value(&self, u: f64, v: f64, p: &point3) -> color {
        //return Vec3::ones() * 0.5 * (1.0 + self.noise.noise(&(*p * self.scale)))

        //引入湍流
        //return Vec3::ones() * self.noise.turb(&mut(*p * self.scale));

        //使颜色与正弦函数之类的东西成比例，并使用湍流来调整相位
        //使条纹呈波动状
        return Vec3::ones() * 0.5 * (1.0 + f64::sin(self.scale*p.z() + 10.0 * self.noise.turb(&mut(*p * 1.0))))
    }
}



//-------------------------------------------------------------------------------------------

const bytes_per_pixel: i64 = 3; 

//图像纹理
#[derive(Clone)]
pub struct image_texture {
    data: image::ImageBuffer<image::Rgb<u8>, Vec<u8> >,           //????????
    width: usize,
    height: usize,
    //bytes_per_scanline: i64,
}

impl image_texture {
    pub fn new() -> Self {
        Self {
            data: image::ImageBuffer::new(0, 0),
            width: 0,
            height: 0,
        }
    }
    #[allow(dead_code)]
    pub fn new_with_para(filename: &str) -> Self {
        let data = image::open(filename).unwrap().into_rgb8();
        let width = data.width() as usize;
        let height = data.height() as usize;
        Self {data, width, height}
    }
}

impl texture for image_texture {
    fn value(&self, mut u: f64, mut v: f64, p: &point3) -> Vec3 {
        if self.data.is_empty() {   ///???????
            return Vec3::new(0.0, 1.0, 1.0);
        }
        u = scene::clamp(u, 0.0, 1.0);
        v = 1.0 - scene::clamp(v, 0.0, 1.0);
        let mut i = (u * self.width as f64) as usize;
        let mut j = (v * self.height as f64) as usize;
        
        if i >= self.width  {i = self.width - 1}
        if j >= self.height {j = self.height - 1}

        if i < self.width && j < self.height {
            let pixel = self.data.get_pixel(i.try_into().unwrap(), j.try_into().unwrap()).to_rgb();
            return Vec3::new(pixel[0] as f64 / 255.0,
                             pixel[1] as f64 / 255.0,
                             pixel[2] as f64 / 255.0
                            )
        }
        else {return Vec3::ones()}
    }
}
