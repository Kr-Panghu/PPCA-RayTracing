#![allow(clippy::float_cmp)]
#![feature(box_syntax)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut, ))]
#![warn(non_snake_case)]
mod material;
mod scene;
mod vec3;
mod rtweekend;
mod camera;
mod ray;
mod moving_sphere;
mod bvh;
mod aabb;
mod texture;
mod perlin;
mod aarect;
mod block;
mod option;
mod constant_medium;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::{ProgressBar, MultiProgress};
use rusttype::Font;
//use scene::example_scene;
use std::sync::mpsc;
use std::sync::Arc;
use threadpool::ThreadPool;
use std::rc::Rc;
pub use vec3::Vec3;
use std::collections::VecDeque;
use rand::Rng;
use std::thread;
use std::time::Instant;
use console::style;
use rtweekend::*;
use std::{f64::INFINITY, fs::File, process::exit};
use std::fmt::Display;
const AUTHOR: &str = "Kr Cen";

type Point3 = Vec3;
type Color = Vec3;

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

// fn render_text(image: &mut RgbImage, msg: &str) {
//     let font_file = if is_ci() {
//         "EncodeSans-Regular.ttf"
//     } else {
//         "/System/Library/Fonts/Helvetica.ttc"
//     };
//     let font_path = std::env::current_dir().unwrap().join(font_file);
//     let data = std::fs::read(&font_path).unwrap();
//     let font: Font = Font::try_from_vec(data).unwrap_or_else(|| {
//         panic!(format!(
//             "error constructing a Font from data at {:?}",
//             font_path
//         ));
//     });

//     imageproc::drawing::draw_text_mut(
//         image,
//         Rgb([255, 255, 255]),
//         10,
//         10,
//         rusttype::Scale::uniform(24.0),
//         &font,
//         msg,
//     );
// }

const MAX_DEPTH: i32 = 50; //限制递归深度

//光线: 渐变色
// pub fn ray_color(r: &mut ray::Ray, background: &mut Color, world: &mut dyn scene::hittable, depth: i32) -> Vec3 {
//     let mut rec = scene::hit_record::new();
//     if depth <= 0 { return Vec3::zero() }

//     if !world.hit(r, 0.001, infinity, &mut rec) {
//         return *background     //background.clone()
//     }
    
//     let mut scattered = ray::Ray::new(Vec3::zero(),Vec3::zero(),0.0);
//     let mut attenuation = Color::zero();
//     let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &mut rec.p);
    
//     if !rec.mat_ptr.scatter(&r, &rec, &mut attenuation, &mut scattered) {
//         //print!("{} {} {}\n", emitted.x(), emitted.y(), emitted.z());
//         return emitted
//     }
//     //print!("{:?}", attenuation);
//     //print!("{} {} {}      ", emitted.x(), emitted.y(), emitted.z());
//     //print!("{} {} {}\n", attenuation.x(), attenuation.y(), attenuation.z());
//     return emitted + Vec3::cdot(&attenuation, &ray_color(&mut scattered, background, world, depth - 1) );
// }

pub fn ray_color(r: &mut ray::Ray, background: &mut Color, world: &Arc<bvh::bvh_node>, depth: i32) -> Vec3 {
    let mut rec = scene::hit_record::new();
    if depth <= 0 { return Vec3::zero() }

    if !world.hit(r, 0.001, infinity, &mut rec) {
        return *background     //background.clone()
    }
    
    let mut scattered = ray::Ray::new(Vec3::zero(),Vec3::zero(),0.0);
    let mut attenuation = Color::zero();
    let emitted = rec.mat_ptr.emitted(rec.u, rec.v, &mut rec.p);
    
    if !rec.mat_ptr.scatter(&r, &rec, &mut attenuation, &mut scattered) {
        //print!("{} {} {}\n", emitted.x(), emitted.y(), emitted.z());
        return emitted
    }
    //print!("{:?}", attenuation);
    //print!("{} {} {}      ", emitted.x(), emitted.y(), emitted.z());
    //print!("{} {} {}\n", attenuation.x(), attenuation.y(), attenuation.z());
    return emitted + Vec3::cdot(&attenuation, &ray_color(&mut scattered, background, world, depth - 1) );
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




fn main() {

    //===============================PART I=================================


    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("--------------------\n🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥");

    println!("WELCOME! THIS IS A {} SIMULATOR", style("RAY TRACING").yellow());
    println!("YOU CAN CHOOSE DIFFERENT SCENES TO EXPERIRENCE THE EFFECT OF COLORING");
    println!("BUT REMEMBER THE ADDRESS YOU INPUT MUST BE {}", style("VALID").yellow());
    println!("📝 Author: {}\n🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥🔥", style(AUTHOR).bold().blue());
    println!("--------------------");

    println!(
        "{} 🚀 {}",
        style("[1/5]").bold().dim(),
        style("Part I: INITIALIZE AND READ SCENARIOS...").green()
    );
    let n_jobs: usize = 16;   //Number of threads

    println!("--------------------");
    println!(
        "{}\n{}",
        "PLEASE ENTER THE ADDRESS YOU WANT TO SAVE THE IMAGE",
        style("Eg: output/test.png").yellow()
    );

    let mut index = String::new();
    use std::io;
    io::stdin().read_line(&mut index).expect("not a string");
    let output = index.trim();

    const QUALITY: u8 = 100;

    println!("--------------------");

    //===============================PART II=================================

    
    println!(
        "{} 🔭 {} {}{}",
        style("[2/5]").bold().dim(),
        style("Part II: THE NUMBER OF THREADS USED IS").green(),
        style(n_jobs.to_string()).yellow(),
        style("...").green()
    );

    let mut aspect_ratio: f64 = 16.0 / 9.0; //纵横比
    let mut image_width: usize = 400;
    let mut samples_per_pixel: usize = 100;
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let mut vfov = 40.0;
    let mut aperture = 0.0;
    let mut background = Color::zero();
    let mut lookfrom = Vec3::zero();
    let mut lookat = Vec3::zero();
    let mut option: usize; //option: 场景选择

    println!("PLEASE ENTER THE SCENE YOU WANT TO CHOOSE");
    println!("{}", style("1: RANDOM SCENE WITH BOUNCING SPHERES").yellow());
    println!("{}", style("2: TWO SPHERES WITH CHECKER_TEXTURE").yellow());
    println!("{}", style("3: TWO SPHERES WITH PERLIN NOISE").yellow());
    println!("{}", style("4: THE EARTH MAP").yellow());
    println!("{}", style("5: SCENE WITH RENTANGLE LIGHT SOURCE").yellow());
    println!("{}", style("6: EMPTY CORNELL BOX").yellow());
    println!("{}", style("7: STANDARD CORNELL BOX").yellow());
    println!("{}", style("8: CORNELL SMOKE").yellow());
    println!("{}", style("9: FINAL SCENE WITH ALL FEATURES").yellow());
    //println!("{}", style("OTHER: DEFAULT SCENE").yellow());

    println!("--------------------");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("not a num");
    let index = index.trim();
    match index.parse::<usize>() {
        Ok(i) => option = i,
        Err(..) => {
            println!("THIS WAS NOT AN INTEGER: {}", index);
            println!("{} {} ❌\n--------------------\n", "THE PROGRAM ENDED", style("UNEXPECTEDLY").red());
            exit(1);
        }
        //Err(..) => option = 0,
    }

    println!("--------------------");
    let now = Instant::now();
    
    //let option = 10;
    let world = option::get_world(option, &mut aspect_ratio, &mut image_width, &mut samples_per_pixel, &mut background, &mut lookfrom, &mut lookat, &mut vfov, &mut aperture);
    let world = Arc::new(bvh::bvh_node::new_with_5para(&mut world.objects.clone(), 0, world.objects.len(), 0.0, 1.0));
    // let world = Test_for_bvh();
    // background = Vec3::new(0.7, 0.8, 1.0);
    // lookfrom = Vec3::new(13.0, 2.0, 3.0);
    // lookat = Vec3::new(0.0, 0.0, 0.0);
    // vfov = 20.0;
    // aperture = 0.1;

    let image_height: usize = ((image_width as f64) / aspect_ratio) as usize;
    let cam = camera::Camera::new_with_para(&lookfrom, &lookat, &vup, vfov, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0);
    let section_line_num: usize = image_height as usize / n_jobs;
    let mut output_pixel_color = Vec::<Vec3>::new();
    //let mut thread_pool = VecDeque::<_>::new();
    //let mut thread_pool: std::collections::VecDeque<(std::thread::JoinHandle<()>, std::sync::mpsc::Receiver<std::vec::Vec<vec3::Vec3>>)> = VecDeque::new();
    let mut thread_pool = VecDeque::<_>::new();

    let mut img: RgbImage = ImageBuffer::new(
        image_width.try_into().unwrap(),
        image_height.try_into().unwrap(),
    );

    //print!("P3\n{} {}\n255\n", image_width, image_height);

    

    for thread_id in 0..n_jobs {
        let line_beg = section_line_num * thread_id;
        let line_end = if line_beg + section_line_num > image_height || (thread_id == n_jobs - 1 && line_beg + section_line_num < image_height) {
            image_height
        } 
        else {
            line_beg + section_line_num
        };

        let (tx, rx) = mpsc::channel();
        let camera_clone = cam.clone();
        let mut world_clone = world.clone();
        thread_pool.push_back((
            thread::spawn(move || {
                let channel_send = tx.clone();

                let mut section_pixel_color = Vec::<Vec3>::new();

                for j in line_beg..line_end {
                    for i in 0..image_width {
                        let mut pixel_color = Vec3::zero();
                        // take samples_per_pixel samples and average them
                        for _s in 0..samples_per_pixel {
                            let u = (i as f64 + rtweekend::random_double_1()) / (image_width as f64);
                            let v = (j as f64 + rtweekend::random_double_1()) / (image_height as f64);
                            let mut r = camera_clone.get_ray(u, v);
                            pixel_color += ray_color(&mut r, &mut background, &world_clone, MAX_DEPTH);
                        }
                        section_pixel_color.push(pixel_color);
                    }
                    // progress += 1;
                    // progress_bar.set_position(progress);
                }
                channel_send.send(section_pixel_color).unwrap();
                // progress_bar.finish_with_message("Finished.");
            }),
            rx,
        ));
    }


    //===============================PART III=================================

    
    println!(
        "{} 🦀 {}",
        style("[3/5]").bold().dim(),
        style("PART III: PLEASE WAITING...TASK IN PROGRESS...").green(),
    );

    //let collecting_progress_bar = MultiProgress::with_draw_target();
    let bar = ProgressBar::new(n_jobs as u64);

    for thread_id in 0..n_jobs {
        let thread = thread_pool.pop_front().unwrap();
        match thread.0.join() {
            Ok(_) => {
                let mut received = thread.1.recv().unwrap();
                output_pixel_color.append(&mut received);
                bar.inc(1);    //
            }
            Err(_) => {
                println!(
                    "{} {}{}",
                    style("FAILED TO JOIN THE").red(),
                    style(thread_id.to_string()).yellow(),
                    style("th THREAD").red(),
                );
            }
        }
    }
    bar.finish();


    //===============================PART IV=================================


    println!("--------------------");
    println!(
        "{} 🎨 {}",
        style("[4/5]").bold().dim(),
        style("PART IV: IMAGE COLORING...").green()
    );

    let mut pixel_id = 0;
    for j in 0..image_height {
        for i in 0..image_width {
            scene::write_to_img(
                output_pixel_color[pixel_id],
                samples_per_pixel,
                &mut img,
                i,
                image_height - j - 1,
            );
            pixel_id += 1;
        }
    }


    //===============================PART V=================================


    println!("--------------------");
    println!(
        "{} ✨ {}",
        style("[5/5]").bold().dim(),
        style("PART V: OUTPUT THE IMAGE...").green()
    );
    println!("--------------------");
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(output).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(QUALITY)) {    //specified quality
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Unfortunately, you failed to load image.").red()),
    }
    println!("\n🎉 {} {} \"{}\"", style("Congratulations!").bold().green(), style("You got the output file in").green(), style(output).yellow());
    println!("🕒 {} {} {}\n", 
        style("Execution Time: Done in").blue(), 
        style(now.elapsed().as_millis() / 1000).bold().yellow(), 
        style("seconds").blue(),
    );

}


//==================守护Rust最好的螃蟹====================//
/*
                          \\/
     ▒▒          ▒▒▒▒▒▒▒▒      ▒▒▒▒▒▒▒▒          ▒▒
   ▒▒▒▒  ▒▒    ▒▒        ▒▒  ▒▒        ▒▒    ▒▒  ▒▒▒▒
   ▒▒▒▒  ▒▒  ▒▒            ▒▒            ▒▒  ▒▒  ▒▒▒▒
 ░░▒▒▒▒░░▒▒  ▒▒            ▒▒            ▒▒  ▒▒░░▒▒▒▒
   ▓▓▓▓▓▓▓▓  ▓▓      ▓▓██  ▓▓  ▓▓██      ▓▓  ▓▓▓▓▓▓▓▓
     ▒▒▒▒    ▒▒      ████  ▒▒  ████      ▒▒░░  ▒▒▒▒
       ▒▒  ▒▒▒▒▒▒        ▒▒▒▒▒▒        ▒▒▒▒▒▒  ▒▒
         ▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▓▓▒▒▓▓▒▒▒▒▒▒▒▒
           ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
             ▒▒▒▒▒▒▒▒▒▒██▒▒▒▒▒▒██▒▒▒▒▒▒▒▒▒▒
           ▒▒  ▒▒▒▒▒▒▒▒▒▒██████▒▒▒▒▒▒▒▒▒▒  ▒▒
         ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒
       ▒▒    ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒    ▒▒
       ▒▒  ▒▒    ▒▒                  ▒▒    ▒▒  ▒▒
           ▒▒  ▒▒                      ▒▒  ▒▒
*/
//======================================================//


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