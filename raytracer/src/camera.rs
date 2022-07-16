use crate::Vec3;
use crate::rtweekend;
use crate::ray;

type Point3 = Vec3;

pub fn degrees_to_radians(degrees: f64) -> f64{ //度数到弧度
    degrees * rtweekend::pi / 180.0
}

//摄像机类
#[derive(Clone)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3, v: Vec3, w: Vec3,
    lens_radious: f64,
    time0: f64, time1: f64, // shutter open/close times, 默认为0
}

impl Camera {
    // pub fn new() -> Self {
    //     let aspect_ratio: f64 = 16.0 / 9.0; //纵横比
    //     let viewport_height = 2.0;
    //     let viewport_width = aspect_ratio * viewport_height;
    //     let focal_length = 1.0;  //焦距
    //     Self {
    //         origin: point3::zero(),
    //         horizontal: Vec3::new(viewport_width, 0.0, 0.0),
    //         vertical: Vec3::new(0.0, viewport_height, 0.0),
    //         lower_left_corner: point3::zero() - Vec3::new(viewport_width, 0.0, 0.0) / 2.0 - Vec3::new(0.0, viewport_height, 0.0) / 2.0 - Vec3::new(0.0, 0.0, focal_length),
    //     }
    // }

    pub fn new_with_para(lookfrom: &Point3, 
                         lookat: &Point3, 
                         vup: &Vec3,        //view_up
                         vfov: f64,         //垂直视野(度)
                         aspect_ratio: f64, //纵横比
                         aperture: f64,     //光圈
                         focus_dist: f64,   //焦点距离
                         _time0: f64,       //相机打开时间
                         _time1: f64,       //相机关闭时间
                        ) -> Self {
        //可定位相机
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta/2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let ww = (*lookfrom - *lookat).unit();
        let uu = Vec3::cross(*vup, ww).unit();
        let vv = Vec3::cross(ww, uu);

        //散焦模糊,
        //从以lookfrom点为中心的磁盘内部生成随机场景光线
        //半径越大,离焦模糊越大
        
        Self {
            w: ww,
            u: uu,
            v: vv,
            origin: *lookfrom,
            horizontal: uu * viewport_width * focus_dist,
            vertical: vv * viewport_height * focus_dist,
            lower_left_corner: *lookfrom - uu * viewport_width * focus_dist / 2.0 - vv * viewport_height * focus_dist / 2.0 - ww * focus_dist,
            lens_radious: aperture / 2.0,
            time0: _time0,
            time1: _time1,
        }
    }

    // pub fn get_ray(&self, u: f64, v: f64) -> Ray {
    //     Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
    // }

    pub fn get_ray(&self, s: f64, t: f64) -> ray::Ray {
        let rd: Vec3 = Vec3::random_in_unit_disk() * self.lens_radious;
        let offset: Vec3 = self.u * rd.x() + self.v * rd.y();

        return ray::Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset, rtweekend::random_double_2(self.time0, self.time1))
    }

}