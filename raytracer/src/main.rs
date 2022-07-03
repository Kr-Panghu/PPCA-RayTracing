#![allow(clippy::float_cmp)]
#![feature(box_syntax)]

mod material;
mod scene;
mod vec3;
mod rtweekend;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use rusttype::Font;
//use scene::example_scene;
use std::sync::mpsc::channel;
use std::sync::Arc;
use threadpool::ThreadPool;
use std::rc::Rc;
pub use vec3::Vec3;
//use std::io;
use rand::Rng;

const AUTHOR: &str = "Kr Cen";

const infinity: f64 = std::f64::INFINITY;
const pi: f64 = 3.1415926535897932385;

type point3 = Vec3;
type color = Vec3;

pub struct World {
    pub height: u32,
}

impl World {
    pub fn color(&self, _: u32, y: u32) -> u8 {
        (y * 256 / self.height) as u8
    }
}

fn get_text() -> String {
    // GITHUB_SHA is the associated commit ID
    // only available on GitHub Action
    let github_sha = option_env!("GITHUB_SHA")
        .map(|x| "@".to_owned() + &x[0..6])
        .unwrap_or_default();
    format!("{}{}", AUTHOR, github_sha)
}

fn is_ci() -> bool {
    option_env!("CI").unwrap_or_default() == "true"
}

fn render_text(image: &mut RgbImage, msg: &str) {
    let font_file = if is_ci() {
        "EncodeSans-Regular.ttf"
    } else {
        "/System/Library/Fonts/Helvetica.ttc"
    };
    let font_path = std::env::current_dir().unwrap().join(font_file);
    let data = std::fs::read(&font_path).unwrap();
    let font: Font = Font::try_from_vec(data).unwrap_or_else(|| {
        panic!(format!(
            "error constructing a Font from data at {:?}",
            font_path
        ));
    });

    imageproc::drawing::draw_text_mut(
        image,
        Rgb([255, 255, 255]),
        10,
        10,
        rusttype::Scale::uniform(24.0),
        &font,
        msg,
    );
}

const max_depth: i32 = 50; //限制递归深度

//光线: 渐变色
pub fn ray_color(r: &scene::Ray, world: &dyn scene::hittable, depth: i32) -> Vec3 {
    let mut rec = scene::hit_record::new();
    if depth <= 0 { return Vec3::zero() }
    if world.hit(r, 0.001, infinity, &mut rec) {
        let mut scattered = scene::Ray::new(Vec3::zero(),Vec3::zero());
        let mut attenuation = Vec3::zero();
        //let target = rec.p + rtweekend::random_in_hemisphere(&rec.normal);
        if rec.mat_ptr.scatter(&r, &rec, &mut attenuation, &mut scattered) {
            // print!("Debug atten     : {} {} {}\n", attenuation.x(), attenuation.y(), attenuation.z());
            // print!("Debug scatt.orig: {} {} {}\n", scattered.orig.x(), scattered.orig.y(), scattered.orig.z());
            // print!("Debug scatt.dir : {} {} {}\n", scattered.dir.x(), scattered.dir.y(), scattered.dir.z());
            return Vec3::cdot(&attenuation, &ray_color(&scattered, world, depth - 1))
        }
        //return ray_color(&scene::Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5
        return Vec3::zero()
    }
    let unit_direction = r.direction().unit();
    let t = (unit_direction.y() + 1.0) * 0.5;
    Vec3::ones() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}


// fn main() {
//     // get environment variable CI, which is true for GitHub Action
//     let is_ci = is_ci();

//     // jobs: split image into how many parts
//     // workers: maximum allowed concurrent running threads
//     let (n_jobs, n_workers): (usize, usize) = if is_ci { (32, 2) } else { (16, 2) };

//     println!(
//         "CI: {}, using {} jobs and {} workers",
//         is_ci, n_jobs, n_workers
//     );

//     let height = 512;
//     let width = 1024;

//     // create a channel to send objects between threads
//     let (tx, rx) = channel();
//     let pool = ThreadPool::new(n_workers);

//     let bar = ProgressBar::new(n_jobs as u64);

//     // use Arc to pass one instance of World to multiple threads
//     let world = Arc::new(example_scene());

//     for i in 0..n_jobs {
//         let tx = tx.clone();
//         let world_ptr = world.clone();
//         pool.execute(move || {
//             // here, we render some of the rows of image in one thread
//             let row_begin = height as usize * i / n_jobs;
//             let row_end = height as usize * (i + 1) / n_jobs;
//             let render_height = row_end - row_begin;
//             let mut img: RgbImage = ImageBuffer::new(width, render_height as u32);
//             for x in 0..width {
//                 // img_y is the row in partial rendered image
//                 // y is real position in final image
//                 for (img_y, y) in (row_begin..row_end).enumerate() {
//                     let y = y as u32;
//                     let pixel = img.get_pixel_mut(x, img_y as u32);
//                     let color = world_ptr.color(x, y);
//                     *pixel = Rgb([color, color, color]);
//                 }
//             }
//             // send row range and rendered image to main thread
//             tx.send((row_begin..row_end, img))
//                 .expect("failed to send result");
//         });
//     }

//     let mut result: RgbImage = ImageBuffer::new(width, height);

//     for (rows, data) in rx.iter().take(n_jobs) {
//         // idx is the corrsponding row in partial-rendered image
//         for (idx, row) in rows.enumerate() {
//             for col in 0..width {
//                 let row = row as u32;
//                 let idx = idx as u32;
//                 *result.get_pixel_mut(col, row) = *data.get_pixel(col, idx);
//             }
//         }
//         bar.inc(1);
//     }
//     bar.finish();

//     // render commit ID and author name on image
//     let msg = get_text();
//     println!("Extra Info: {}", msg);

//     render_text(&mut result, msg.as_str());

//     result.save("output/test.png").unwrap();
// }




//-------------------------------------------------------------




//Version 3
fn main() {
    //Image
    let aspect_ratio: f64 = 16.0 / 9.0; //纵横比
    let image_width: i32 = 400;
    let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel: i32 = 100;

    // let new_sphere_1 = scene::Sphere::new(Vec3::new(0.0,0.0,-1.0), 0.5, Rc::new(material::lambertian::new(&Vec3::ones())));
    // let ptr_1 = Rc::new(new_sphere_1);
    // let new_sphere_2 = scene::Sphere::new(Vec3::new(0.0,-100.5,-1.0), 100.0, Rc::new(material::lambertian::new(&Vec3::ones())));
    // let ptr_2 = Rc::new(new_sphere_2);
    // let mut world = scene::hittable_list::new(ptr_1);
    // world.add(ptr_2);

    //World
    // image_12
    // let material_ground = Rc::new(material::lambertian::new(&Vec3::new(0.8,0.8,0.0)));
    // let material_center = Rc::new(material::lambertian::new(&Vec3::new(0.7,0.3,0.3)));
    // let material_left = Rc::new(material::metal::new(&Vec3::new(0.8,0.8,0.8), 0.3));
    // let material_right = Rc::new(material::metal::new(&Vec3::new(0.8,0.6,0.2), 1.0));

    // image_14
    // let material_ground = Rc::new(material::lambertian::new(&Vec3::new(0.8,0.8,0.0)));
    // let material_center = Rc::new(material::dielectric::new(1.5));
    // let material_left = Rc::new(material::dielectric::new(1.5));
    // let material_right = Rc::new(material::metal::new(&Vec3::new(0.8,0.6,0.2), 1.0));

    // image_15
    // let material_ground = Rc::new(material::lambertian::new(&Vec3::new(0.8,0.8,0.0)));
    // let material_center = Rc::new(material::lambertian::new(&Vec3::new(0.1,0.2,0.5)));
    // let material_left = Rc::new(material::dielectric::new(1.5));
    // let material_right = Rc::new(material::metal::new(&Vec3::new(0.8,0.6,0.2), 0.0));
    // let material_left_clone = Rc::clone(&material_left);
    // let mut world = scene::hittable_list::new(Rc::new(scene::Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    // world.add(Rc::new(scene::Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    // world.add(Rc::new(scene::Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    // world.add(Rc::new(scene::Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, material_left_clone)));
    // world.add(Rc::new(scene::Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));


    // image_17
    // let R = f64::cos(pi / 4.0);
    // let material_left = Rc::new(material::lambertian::new(&Vec3::new(0.0,0.0,1.0)));
    // let material_right = Rc::new(material::lambertian::new(&Vec3::new(1.0,0.0,0.0)));
    // let mut world = scene::hittable_list::new(Rc::new(scene::Sphere::new(Vec3::new(-R, 0.0, -1.0), R, material_left)));
    // world.add(Rc::new(scene::Sphere::new(Vec3::new(R, 0.0, -1.0), R, material_right)));

    // image_18
    let material_ground = Rc::new(material::lambertian::new(&Vec3::new(0.8,0.8,0.0)));
    let material_center = Rc::new(material::lambertian::new(&Vec3::new(0.1,0.2,0.5)));
    let material_left = Rc::new(material::dielectric::new(1.5));
    let material_right = Rc::new(material::metal::new(&Vec3::new(0.8,0.6,0.2), 0.0));
    let material_left_clone = Rc::clone(&material_left);
    let mut world = scene::hittable_list::new(Rc::new(scene::Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Rc::new(scene::Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(Rc::new(scene::Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Rc::new(scene::Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, material_left_clone)));
    world.add(Rc::new(scene::Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    //Camera
    // image_16
    // let cam = scene::camera::new();

    // image_17
    // let cam = scene::camera::new_with_para(&Vec3::new(-2.0,2.0,1.0), &Vec3::new(0.0,0.0,-1.0) ,&Vec3::new(0.0,1.0,0.0), 90.0, aspect_ratio);

    // image_19
    //let cam = scene::camera::new_with_para(&Vec3::new(-2.0,2.0,1.0), &Vec3::new(0.0,0.0,-1.0) ,&Vec3::new(0.0,1.0,0.0), 20.0, aspect_ratio);

    // image_20
    let lookfrom = Vec3::new(3.0, 3.0, 2.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 2.0;
    let cam = scene::camera::new_with_para(&lookfrom, &lookat, &vup, 20.0, aspect_ratio, aperture, dist_to_focus);

    //视口左下角的坐标
    //let lower_left_corner: Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    let msg = get_text();
    //Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev(){
        for i in 0..image_width {
            let mut pixel_color = Vec3::zero();
            for s in 0..samples_per_pixel {
                let u: f64 = (i as f64 + rtweekend::random_double_1()) / (image_width as f64 - 1.0);
                let v: f64 = (j as f64 + rtweekend::random_double_1()) / (image_height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            scene::write_color(pixel_color, samples_per_pixel);
            // let u: f64 = (i as f64) / (image_width - 1) as f64;
            // let v: f64 = (j as f64) / (image_height - 1) as f64;
            // let r = scene::Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            // let pixel_color: Vec3 = scene::ray_color(&r, &world);
            // pixel_color.write_color();
        }
    }
    let mut result: RgbImage = ImageBuffer::new(image_width as u32, image_height as u32);
    render_text(&mut result, msg.as_str());
    //result.save("output/test.png").unwrap();
}