#![allow(clippy::float_cmp)]
#![feature(box_syntax)]
mod material;
mod scene;
mod vec3;
mod rtweekend;
mod camera;
mod ray;
mod moving_sphere;
mod bvh;
mod texture;
mod perlin;
mod aarect;
mod block;
mod constant_medium;
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
pub fn ray_color(r: &mut ray::Ray, background: &mut color, world: &mut dyn scene::hittable, depth: i32) -> Vec3 {
    let mut rec = scene::hit_record::new();
    if depth <= 0 { return Vec3::zero() }

    if !world.hit(r, 0.001, infinity, &mut rec) {
        return *background     //background.clone()
    }
    
    let mut scattered = ray::Ray::new(Vec3::zero(),Vec3::zero(),0.0);
    let mut attenuation = color::zero();
    let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &mut rec.p);

    if !rec.mat_ptr.scatter(&r, &rec, &mut attenuation, &mut scattered) {
        //print!("{} {} {}\n", emitted.x(), emitted.y(), emitted.z());
        return emitted
    }
    //print!("{:?}", attenuation);
    //print!("{} {} {}      ", emitted.x(), emitted.y(), emitted.z());
    //print!("{} {} {}\n", attenuation.x(), attenuation.y(), attenuation.z());
    return emitted + Vec3::cdot(&attenuation, &ray_color(&mut scattered, background, world, depth - 1));



    // if world.hit(r, 0.001, infinity, &mut rec) {
    //     let mut scattered = ray::Ray::new(Vec3::zero(),Vec3::zero(),0.0);
    //     let mut attenuation = Vec3::zero();
    //     //let target = rec.p + rtweekend::random_in_hemisphere(&rec.normal);
    //     if rec.mat_ptr.scatter(&r, &rec, &mut attenuation, &mut scattered) {
    //         // print!("Debug atten     : {} {} {}\n", attenuation.x(), attenuation.y(), attenuation.z());
    //         // print!("Debug scatt.orig: {} {} {}\n", scattered.orig.x(), scattered.orig.y(), scattered.orig.z());
    //         // print!("Debug scatt.dir : {} {} {}\n", scattered.dir.x(), scattered.dir.y(), scattered.dir.z());
    //         return Vec3::cdot(&attenuation, &ray_color(&scattered, background, world, depth - 1))
    //     }
    //     //return ray_color(&scene::Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5
    //     return Vec3::zero()
    // }
    // let unit_direction = r.direction().unit();
    // let t = (unit_direction.y() + 1.0) * 0.5;
    // Vec3::ones() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
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


/*

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
    let cam = camera::camera::new_with_para(&lookfrom, &lookat, &vup, 20.0, aspect_ratio, aperture, dist_to_focus, 0.0, 0.0);

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

*/







//Version 4: Final_Scene

// fn random_scene() -> scene::hittable_list {
//     let ground_material = Rc::new(material::lambertian::new(&Vec3::new(0.5, 0.5, 0.5)));
//     let mut world = scene::hittable_list::new(Rc::new(scene::Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

//     for a in (-11)..12 {
//         for b in (-11)..12 {
//             let choose_mat = rtweekend::random_double_1();
//             let center = Vec3::new(a as f64 + 0.9 * rtweekend::random_double_1(), 0.2, b as f64 + rtweekend::random_double_1());

//             if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
//                 let sphere_material: Rc<dyn material::Material>;
//                 if choose_mat < 0.8 {
//                     //diffuse
//                     let v1 = Vec3::random_vector_1();
//                     let v2 = Vec3::random_vector_1();
//                     let albedo = Vec3::cdot(&v1, &v2);
//                     sphere_material = Rc::new(material::lambertian::new(&albedo));
//                     world.add(Rc::new(scene::Sphere::new(center, 0.2, sphere_material)));
//                 }
//                 else if choose_mat < 0.95 {
//                     //metal
//                     let albedo = Vec3::random_vector_2(0.5, 1.0);
//                     let fuzz = rtweekend::random_double_2(0.0, 0.5);
//                     sphere_material = Rc::new(material::metal::new(&albedo, fuzz));
//                     world.add(Rc::new(scene::Sphere::new(center, 0.2, sphere_material)));
//                 }
//                 else {
//                     //glass
//                     sphere_material = Rc::new(material::dielectric::new(1.5));
//                     world.add(Rc::new(scene::Sphere::new(center, 0.2, sphere_material)));
//                 }
//             }
//         }
//     } 

//     let material1 = Rc::new(material::dielectric::new(1.5));
//     world.add(Rc::new(scene::Sphere::new(Vec3::new(0.0,1.0,0.0), 1.0, material1)));

//     let material2 = Rc::new(material::lambertian::new(&Vec3::new(0.4,0.2,0.1)));
//     world.add(Rc::new(scene::Sphere::new(Vec3::new(-4.0,1.0,0.0), 1.0, material2)));

//     let material3 = Rc::new(material::metal::new(&Vec3::new(0.7,0.6,0.5), 0.0));
//     world.add(Rc::new(scene::Sphere::new(Vec3::new(4.0,1.0,0.0), 1.0, material3)));

//     return world
// }

fn two_spheres() -> scene::hittable_list {
    let checker = Rc::new(texture::checker_texture::new_with_para(&Vec3::new(0.2,0.3,0.1), &Vec3::new(0.9,0.9,0.9)));
    let mut objects = scene::hittable_list::new(Rc::new(scene::Sphere::new(point3::new(0.0,-10.0,0.0), 10.0, Rc::new(material::lambertian::new_with_ptr(checker.clone())))));
    objects.add(Rc::new(scene::Sphere::new(point3::new(0.0,10.0,0.0), 10.0, Rc::new(material::lambertian::new_with_ptr(checker.clone())))));
    objects
}

fn two_perlin_spheres() -> scene::hittable_list {
    let pertext = Rc::new(texture::noise_texture::new_with_para(4.0));
    let a = Rc::new(material::lambertian::new_with_ptr(pertext.clone()));
    let b = Rc::clone(&a);
    let mut objects = scene::hittable_list::new(Rc::new(scene::Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, a)));
    objects.add(Rc::new(scene::Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, b)));
    objects
}

// fn earth() -> scene::hittable_list {}

fn simple_light() -> scene::hittable_list {
    let pertext = Rc::new(texture::noise_texture::new_with_para(4.0));
    let a = Rc::new(material::lambertian::new_with_ptr(pertext.clone()));
    let b = Rc::clone(&a);
    let mut objects = scene::hittable_list::new(Rc::new(scene::Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, a)));
    objects.add(Rc::new(scene::Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, b)));
    //let difflight = Rc::new(material::diffuse_light::new_with_para(color::new(4.0,4.0,4.0)));
    //objects.add(Rc::new(aarect::xy_rect::new(difflight ,3.0, 5.0, 1.0, 3.0, -2.0)));
    objects
}

fn random_scene() -> scene::hittable_list {
    //let ground_material = Rc::new(material::lambertian::new(&Vec3::new(0.5, 0.5, 0.5)));
    //let mut world = scene::hittable_list::new(Rc::new(scene::Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    let checker = Rc::new(texture::checker_texture::new_with_para(&color::new(0.2,0.3,0.1), &color::new(0.9,0.9,0.9)));
    let mut world = scene::hittable_list::new(Rc::new(scene::Sphere::new(point3::new(0.0,-1000.0,0.0), 1000.0, Rc::new(material::lambertian::new_with_ptr(checker)))));

    for a in (-11)..12 {
        for b in (-11)..12 {
            let choose_mat = rtweekend::random_double_1();
            let center = Vec3::new(a as f64 + 0.9 * rtweekend::random_double_1(), 0.2, b as f64 + 0.9 * rtweekend::random_double_1());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn material::Material>;
                if choose_mat < 0.8 {
                    //diffuse
                    let v1 = Vec3::random_vector_1();
                    let v2 = Vec3::random_vector_1();
                    let albedo = Vec3::cdot(&v1, &v2);
                    sphere_material = Rc::new(material::lambertian::new(&albedo));
                    let center2 = center + Vec3::new(0.0, rtweekend::random_double_2(0.0, 0.5), 0.0);
                    world.add(Rc::new(moving_sphere::moving_sphere::new(center, center2, 0.0, 1.0, 0.2, sphere_material)));
                }
                else if choose_mat < 0.95 {
                    //metal
                    let albedo = Vec3::random_vector_2(0.5, 1.0);
                    let fuzz = rtweekend::random_double_2(0.0, 0.5);
                    sphere_material = Rc::new(material::metal::new(&albedo, fuzz));
                    world.add(Rc::new(scene::Sphere::new(center, 0.2, sphere_material)));
                }
                else {
                    //glass
                    sphere_material = Rc::new(material::dielectric::new(1.5));
                    world.add(Rc::new(scene::Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(material::dielectric::new(1.5));
    world.add(Rc::new(scene::Sphere::new(Vec3::new(0.0,1.0,0.0), 1.0, material1)));

    let material2 = Rc::new(material::lambertian::new(&Vec3::new(0.4,0.2,0.1)));
    world.add(Rc::new(scene::Sphere::new(Vec3::new(-4.0,1.0,0.0), 1.0, material2)));

    let material3 = Rc::new(material::metal::new(&Vec3::new(0.7,0.6,0.5), 0.0));
    world.add(Rc::new(scene::Sphere::new(Vec3::new(4.0,1.0,0.0), 1.0, material3)));

    return world
}


fn empty_cornell_box() -> scene::hittable_list {
    let red = Rc::new(material::lambertian::new(&Vec3::new(0.65,0.05,0.05)));
    let white = Rc::new(material::lambertian::new(&Vec3::new(0.73,0.73,0.73)));
    let green = Rc::new(material::lambertian::new(&Vec3::new(0.12,0.45,0.15)));
    let light = Rc::new(material::diffuse_light::new_with_para(&Vec3::new(15.0,15.0,15.0)));

    let mut objects = scene::hittable_list::new_without_para();

    objects.add(Rc::new(aarect::yz_rect::new(green.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));
    objects.add(Rc::new(aarect::yz_rect::new(red.clone(), 0.0, 555.0, 0.0, 555.0, 0.0)));
    objects.add(Rc::new(aarect::xz_rect::new(light.clone(), 213.0, 343.0, 227.0, 332.0, 554.0)));
    objects.add(Rc::new(aarect::xz_rect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 0.0)));
    objects.add(Rc::new(aarect::xz_rect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));
    objects.add(Rc::new(aarect::xy_rect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));

    objects.add(Rc::new(block::Block::new(&point3::new(130.0, 0.0, 65.0), &point3::new(295.0, 165.0, 230.0), white.clone())));
    objects.add(Rc::new(block::Block::new(&point3::new(265.0, 0.0, 295.0), &point3::new(430.0, 330.0, 460.0), white.clone())));

    objects
}

fn standard_cornell_box() -> scene::hittable_list {
    let red = Rc::new(material::lambertian::new(&Vec3::new(0.65,0.05,0.05)));
    let white = Rc::new(material::lambertian::new(&Vec3::new(0.73,0.73,0.73)));
    let green = Rc::new(material::lambertian::new(&Vec3::new(0.12,0.45,0.15)));
    let light = Rc::new(material::diffuse_light::new_with_para(&Vec3::new(15.0,15.0,15.0)));

    let mut objects = scene::hittable_list::new_without_para();

    objects.add(Rc::new(aarect::yz_rect::new(green.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));
    objects.add(Rc::new(aarect::yz_rect::new(red.clone(), 0.0, 555.0, 0.0, 555.0, 0.0)));
    objects.add(Rc::new(aarect::xz_rect::new(light.clone(), 213.0, 343.0, 227.0, 332.0, 554.0)));
    objects.add(Rc::new(aarect::xz_rect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 0.0)));
    objects.add(Rc::new(aarect::xz_rect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));
    objects.add(Rc::new(aarect::xy_rect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));

    let block1_1 = Rc::new(block::Block::new(&Vec3::zero(), &Vec3::new(165.0, 330.0, 165.0), white.clone()));
    let block1_2 = Rc::new(scene::rotate_y::new(block1_1, 15.0));
    let block1 = Rc::new(scene::translate::new(block1_2, &Vec3::new(265.0, 0.0, 295.0)));
    objects.add(block1);

    let block2_1 = Rc::new(block::Block::new(&Vec3::zero(), &Vec3::new(165.0, 165.0, 165.0), white.clone()));
    let block2_2 = Rc::new(scene::rotate_y::new(block2_1, -18.0));
    let block2 = Rc::new(scene::translate::new(block2_2, &Vec3::new(130.0, 0.0, 65.0)));
    objects.add(block2);

    objects
}


fn cornell_smoke() -> scene::hittable_list {
    let red = Rc::new(material::lambertian::new(&Vec3::new(0.65,0.05,0.05)));
    let white = Rc::new(material::lambertian::new(&Vec3::new(0.73,0.73,0.73)));
    let green = Rc::new(material::lambertian::new(&Vec3::new(0.12,0.45,0.15)));
    let light = Rc::new(material::diffuse_light::new_with_para(&Vec3::new(7.0,7.0,7.0)));

    let mut objects = scene::hittable_list::new_without_para();

    objects.add(Rc::new(aarect::yz_rect::new(green.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));
    objects.add(Rc::new(aarect::yz_rect::new(red.clone(), 0.0, 555.0, 0.0, 555.0, 0.0)));
    objects.add(Rc::new(aarect::xz_rect::new(light.clone(), 113.0, 443.0, 127.0, 432.0, 554.0)));
    objects.add(Rc::new(aarect::xz_rect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));
    objects.add(Rc::new(aarect::xz_rect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 0.0)));
    objects.add(Rc::new(aarect::xy_rect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));

    // let block1_1 = Rc::new(block::Block::new(&Vec3::zero(), &Vec3::new(165.0, 330.0, 165.0), white.clone()));
    // let block1_2 = Rc::new(scene::rotate_y::new(block1_1, 15.0));
    // let block1 = Rc::new(scene::translate::new(block1_2, &Vec3::new(265.0, 0.0, 295.0)));
    // objects.add(block1);

    let mut block1: Rc<dyn scene::hittable> = Rc::new(block::Block::new(&Vec3::zero(), &Vec3::new(165.0, 330.0, 165.0), white.clone()));
    block1 = Rc::new(scene::rotate_y::new(block1, 15.0));
    block1 = Rc::new(scene::translate::new(block1, &Vec3::new(265.0, 0.0, 295.0)));
    // objects.add(block1);

    let mut block2: Rc<dyn scene::hittable> = Rc::new(block::Block::new(&Vec3::zero(), &Vec3::new(165.0, 165.0, 165.0), white.clone()));
    block2 = Rc::new(scene::rotate_y::new(block2, -18.0));
    block2 = Rc::new(scene::translate::new(block2, &Vec3::new(130.0, 0.0, 65.0)));
    // objects.add(block2);

    objects.add(Rc::new(constant_medium::constant_medium::new_with_para(block1, 0.01, Vec3::zero())));
    objects.add(Rc::new(constant_medium::constant_medium::new_with_para(block2, 0.01, Vec3::ones())));

    objects
}


fn final_scene() -> scene::hittable_list {
    let mut boxes1 = scene::hittable_list::new_without_para();
    let ground = Rc::new(material::lambertian::new(&Vec3::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rtweekend::random_double_2(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Rc::new(block::Block::new(&Vec3::new(x0,y0,z0), &Vec3::new(x1,y1,z1), ground.clone())));
        }
    }

    let mut objects = scene::hittable_list::new_without_para();
    objects.add(Rc::new(bvh::bvh_node::new_with_3para(&mut boxes1, 0.0, 1.0)));

    let light = Rc::new(material::diffuse_light::new_with_para(&Vec3::new(7.0, 7.0, 7.0)));
    objects.add(Rc::new(aarect::xz_rect::new(light.clone(), 123.0, 423.0, 147.0, 412.0, 554.0)));

    let center1 = point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Rc::new(material::lambertian::new(&Vec3::new(0.7, 0.3, 0.1)));
    objects.add(Rc::new(moving_sphere::moving_sphere::new(center1, center2, 0.0, 1.0, 50.0, moving_sphere_material)));

    objects.add(Rc::new(scene::Sphere::new(point3::new(260.0, 150.0, 45.0), 50.0, Rc::new(material::dielectric::new(1.5)))));
    objects.add(Rc::new(scene::Sphere::new(point3::new(0.0, 150.0, 145.0), 50.0, Rc::new(material::metal::new(&Vec3::new(0.8,0.8,0.9), 1.0)))));

    let boundary = Rc::new(scene::Sphere::new(point3::new(360.0,150.0,145.0), 70.0, Rc::new(material::dielectric::new(1.5))));
    objects.add(boundary.clone());
    objects.add(Rc::new(constant_medium::constant_medium::new_with_para(boundary, 0.2, Vec3::new(0.2, 0.4, 0.9))));
    let boundary = Rc::new(scene::Sphere::new(point3::zero(), 5000.0, Rc::new(material::dielectric::new(1.5))));
    objects.add(Rc::new(constant_medium::constant_medium::new_with_para(boundary, 0.0001, Vec3::ones())));

    //let emat = Rc::new(material::lambertian::new_with_ptr(Rc::new(texture::)))
    //

    let pertext = Rc::new(texture::noise_texture::new_with_para(0.1));
    objects.add(Rc::new(scene::Sphere::new(point3::new(220.0,280.0,300.0), 80.0, Rc::new(material::lambertian::new_with_ptr(pertext)))));

    let mut boxes2 = scene::hittable_list::new_without_para();
    let white = Rc::new(material::lambertian::new(&Vec3::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for j in 0..ns {
        boxes2.add(Rc::new(scene::Sphere::new(Vec3::random_vector_2(0.0, 165.0), 10.0, white.clone())));
    }

    objects.add(Rc::new(scene::translate::new(
                    Rc::new(scene::rotate_y::new
                        (Rc::new(bvh::bvh_node::new_with_3para(&mut boxes2, 0.0, 1.0)), 15.0)), 
                        &Vec3::new(-100.0,270.0,395.0)
                )
        )
    );
    
    return objects
}



fn main() {
    //Image
    // Book1: Final Scene
    // let aspect_ratio: f64 = 3.0 / 2.0; //纵横比
    // let image_width: i32 = 1200;
    // let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;
    // let samples_per_pixel: i32 = 200;

    let mut aspect_ratio: f64 = 16.0 / 9.0; //纵横比
    let mut image_width: i32 = 400;
    let mut samples_per_pixel: i32 = 50;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let mut vfov = 40.0;
    let mut aperture = 0.0;
    let mut background = color::zero();

    let mut world = scene::hittable_list::new_without_para();

    let option = 9;    //option: 场景选择
    let mut lookfrom = Vec3::zero();
    let mut lookat = Vec3::zero();

    if option == 1 {
        world = random_scene();
        background = color::new(0.7, 0.8, 1.0);
        lookfrom = Vec3::new(13.0, 2.0, 3.0);
        lookat = Vec3::new(0.0, 0.0, 0.0);
        vfov = 20.0;
        aperture = 0.1;
    }
    if option == 2 {
        world = two_spheres();
        background = color::new(0.7, 0.8, 1.0);
        lookfrom = Vec3::new(13.0,2.0,3.0);
        lookat = Vec3::zero();
        vfov = 20.0;
    }
    if option == 3 {
        world = two_perlin_spheres();
        background = color::new(0.7, 0.8, 1.0);
        lookfrom = Vec3::new(13.0,2.0,3.0);
        lookat = Vec3::zero();
        vfov = 20.0;
    }
    // if option == 4 {
    //     world = earth();
    //     background = color::new(0.7, 0.8, 1.0);
    //     lookfrom = Vec3::new(13.0,100.0,3.0);
    //     lookat = Vec3::zero();
    //     vfov = 20.0;
    // }
    if option == 5 {
        world = simple_light();
        samples_per_pixel = 400;
        background = color::zero();
        lookfrom = point3::new(26.0, 3.0, 6.0);
        lookat = point3::new(0.0, 2.0, 0.0);
        vfov = 20.0;
    }
    if option == 6 || option == 7 {
        if option == 6 {world = empty_cornell_box();} //option = 6
        else {world = standard_cornell_box();}        //option = 7
        aspect_ratio = 1.0;
        image_width = 600;
        samples_per_pixel = 200;
        background = color::new(0.0, 0.0, 0.0);
        lookfrom = point3::new(278.0, 278.0, -800.0);
        lookat = point3::new(278.0, 278.0, 0.0);
        vfov = 40.0;
    }
    if option == 8 {
        world = cornell_smoke();
        aspect_ratio = 1.0;
        image_width = 600;
        samples_per_pixel = 200;
        lookfrom = point3::new(278.0, 278.0, -800.0);
        lookat = point3::new(278.0, 278.0, 0.0);
        vfov = 40.0;
    }
    if option == 9 {
        world = final_scene();
        aspect_ratio = 1.0;
        // image_width = 800;
        // samples_per_pixel = 10000;
        image_width = 400;
        samples_per_pixel = 50;
        background = color::zero();
        lookfrom = point3::new(478.0, 278.0, -600.0);
        lookat = point3::new(278.0, 278.0, 0.0);
        vfov = 40.0;
    }

    let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;
    let cam = camera::camera::new_with_para(&lookfrom, &lookat, &vup, vfov, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0);

    //视口左下角的坐标
    //let lower_left_corner: Vec3 = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
    let msg = get_text();
    //Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev(){
        for i in 0..image_width {
            //print!("{} {} :", j, i);
            let mut pixel_color = Vec3::zero();
            for s in 0..samples_per_pixel {
                let u: f64 = (i as f64 + rtweekend::random_double_1()) / (image_width as f64 - 1.0);
                let v: f64 = (j as f64 + rtweekend::random_double_1()) / (image_height as f64 - 1.0);
                let mut r = cam.get_ray(u, v);
                pixel_color += ray_color(&mut r, &mut background, &mut world, max_depth);
            }
            scene::write_color(pixel_color, samples_per_pixel);
        }
    }
    let mut result: RgbImage = ImageBuffer::new(image_width as u32, image_height as u32);
    render_text(&mut result, msg.as_str());
    result.save("output/test.png").unwrap();
}





//测试


// fn main() {
//     // for i in 1..256 {
//     //     println!("{} = {}", i, rtweekend::random_int(0, i))
//     // }
//     // for i in 0..256 {
//     //     println!("{}", rtweekend::random_double_1());
//     // }
//     if !true {
//         println!("hahaha")
//     }
//     if true {
//         println!("damn")
//     }
// }