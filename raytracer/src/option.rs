use crate::scene;
use crate::Vec3;
use std::rc::Rc;
use std::sync::Arc;
use crate::texture;
use crate::rtweekend;
use crate::material;
use crate::moving_sphere;
use crate::aarect;
use crate::block;
use crate::bvh;
use crate::camera;
use crate::perlin;
use crate::constant_medium;
type Color = Vec3;
type Point3 = Vec3;

fn default_scene() -> scene::hittable_list {
    let pertext = Arc::new(texture::noise_texture::new_with_para(4.0));
    let a = Arc::new(material::lambertian::new_with_ptr(pertext.clone()));
    let b = Arc::clone(&a);
    let mut objects = scene::hittable_list::new(Arc::new(scene::Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, a)));
    objects.add(Arc::new(scene::Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, b)));
    let difflight = Arc::new(material::diffuse_light::new_with_para(&Color::new(4.0,4.0,4.0)));
    objects.add(Arc::new(aarect::xy_rect::new(difflight ,3.0, 5.0, 1.0, 3.0, -2.0)));
    objects
}

fn two_spheres() -> scene::hittable_list {
    let checker = Arc::new(texture::checker_texture::new_with_para(&Vec3::new(0.2,0.3,0.1), &Vec3::new(0.9,0.9,0.9)));
    let mut objects = scene::hittable_list::new(Arc::new(scene::Sphere::new(Point3::new(0.0,-10.0,0.0), 10.0, Arc::new(material::lambertian::new_with_ptr(checker.clone())))));
    objects.add(Arc::new(scene::Sphere::new(Point3::new(0.0,10.0,0.0), 10.0, Arc::new(material::lambertian::new_with_ptr(checker.clone())))));
    objects
}

fn two_perlin_spheres() -> scene::hittable_list {
    let pertext = Arc::new(texture::noise_texture::new_with_para(4.0));
    let a = Arc::new(material::lambertian::new_with_ptr(pertext.clone()));
    let b = Arc::clone(&a);
    let mut objects = scene::hittable_list::new(Arc::new(scene::Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, a)));
    objects.add(Arc::new(scene::Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, b)));
    objects
}

fn earth() -> scene::hittable_list {
    let earth_texture = Arc::new(texture::ImageTexture::new_with_para("data/earthmap.jpg"));
    //let earth_texture = Arc::new(texture::image_texture::new_with_para("data/doge.jpg"));
    let earth_surface = Arc::new(material::lambertian::new_with_ptr(earth_texture));
    let globe = Arc::new(scene::Sphere::new(Vec3::zero(), 2.0, earth_surface));
    let mut world = scene::hittable_list::new_without_para();
    world.add(globe);
    world
}

fn simple_light() -> scene::hittable_list {
    let pertext = Arc::new(texture::noise_texture::new_with_para(4.0));
    let a = Arc::new(material::lambertian::new_with_ptr(pertext.clone()));
    let b = Arc::clone(&a);
    let mut objects = scene::hittable_list::new(Arc::new(scene::Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, a)));
    objects.add(Arc::new(scene::Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, b)));
    let difflight = Arc::new(material::diffuse_light::new_with_para(&Color::new(4.0,4.0,4.0)));
    objects.add(Arc::new(aarect::xy_rect::new(difflight ,3.0, 5.0, 1.0, 3.0, -2.0)));
    objects
}

fn random_scene() -> scene::hittable_list {
    // let ground_material = Arc::new(material::lambertian::new(&Vec3::new(0.5, 0.5, 0.5)));
    // let mut world = scene::hittable_list::new(Arc::new(scene::Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    let checker = Arc::new(texture::checker_texture::new_with_para(&Color::new(0.2,0.3,0.1), &Color::new(0.9,0.9,0.9)));
    let mut world = scene::hittable_list::new(Arc::new(scene::Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, Arc::new(material::lambertian::new_with_ptr(checker)))));

    for a in (-11)..12 {
        for b in (-11)..12 {
            let choose_mat = rtweekend::random_double_1();
            let center = Vec3::new(a as f64 + 0.9 * rtweekend::random_double_1(), 0.2, b as f64 + 0.9 * rtweekend::random_double_1());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn material::Material>;
                if choose_mat < 0.8 {
                    //diffuse
                    let v1 = Vec3::random_vector_1();
                    let v2 = Vec3::random_vector_1();
                    let albedo = Vec3::cdot(&v1, &v2);
                    sphere_material = Arc::new(material::lambertian::new(&albedo));
                    let center2 = center + Vec3::new(0.0, rtweekend::random_double_2(0.0, 0.5), 0.0);
                    world.add(Arc::new(moving_sphere::MovingSphere::new(center, center2, 0.0, 1.0, 0.2, sphere_material)));
                }
                else if choose_mat < 0.95 {
                    //metal
                    let albedo = Vec3::random_vector_2(0.5, 1.0);
                    let fuzz = rtweekend::random_double_2(0.0, 0.5);
                    sphere_material = Arc::new(material::metal::new(&albedo, fuzz));
                    world.add(Arc::new(scene::Sphere::new(center, 0.2, sphere_material)));
                }
                else {
                    //glass
                    sphere_material = Arc::new(material::dielectric::new(1.5));
                    world.add(Arc::new(scene::Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(material::dielectric::new(1.5));
    world.add(Arc::new(scene::Sphere::new(Vec3::new(0.0,1.0,0.0), 1.0, material1)));

    let material2 = Arc::new(material::lambertian::new(&Vec3::new(0.4,0.2,0.1)));
    world.add(Arc::new(scene::Sphere::new(Vec3::new(-4.0,1.0,0.0), 1.0, material2)));

    let material3 = Arc::new(material::metal::new(&Vec3::new(0.7,0.6,0.5), 0.0));
    world.add(Arc::new(scene::Sphere::new(Vec3::new(4.0,1.0,0.0), 1.0, material3)));

    return world
}


fn empty_cornell_box() -> scene::hittable_list {
    let red = Arc::new(material::lambertian::new(&Vec3::new(0.65,0.05,0.05)));
    let white = Arc::new(material::lambertian::new(&Vec3::new(0.73,0.73,0.73)));
    let green = Arc::new(material::lambertian::new(&Vec3::new(0.12,0.45,0.15)));
    let light = Arc::new(material::diffuse_light::new_with_para(&Vec3::new(15.0,15.0,15.0)));

    let mut objects = scene::hittable_list::new_without_para();

    objects.add(Arc::new(aarect::yz_rect::new(green.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));
    objects.add(Arc::new(aarect::yz_rect::new(red.clone(), 0.0, 555.0, 0.0, 555.0, 0.0)));
    objects.add(Arc::new(aarect::xz_rect::new(light.clone(), 213.0, 343.0, 227.0, 332.0, 554.0)));
    objects.add(Arc::new(aarect::xz_rect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 0.0)));
    objects.add(Arc::new(aarect::xz_rect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));
    objects.add(Arc::new(aarect::xy_rect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));

    // objects.add(Arc::new(block::Block::new(&point3::new(130.0, 0.0, 65.0), &point3::new(295.0, 165.0, 230.0), white.clone())));
    // objects.add(Arc::new(block::Block::new(&point3::new(265.0, 0.0, 295.0), &point3::new(430.0, 330.0, 460.0), white.clone())));

    objects
}

fn standard_cornell_box() -> scene::hittable_list {
    let red = Arc::new(material::lambertian::new(&Vec3::new(0.65,0.05,0.05)));
    let white = Arc::new(material::lambertian::new(&Vec3::new(0.73,0.73,0.73)));
    let green = Arc::new(material::lambertian::new(&Vec3::new(0.12,0.45,0.15)));
    let light = Arc::new(material::diffuse_light::new_with_para(&Vec3::new(15.0,15.0,15.0)));

    let mut objects = scene::hittable_list::new_without_para();

    objects.add(Arc::new(aarect::yz_rect::new(green.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));
    objects.add(Arc::new(aarect::yz_rect::new(red.clone(), 0.0, 555.0, 0.0, 555.0, 0.0)));
    objects.add(Arc::new(aarect::xz_rect::new(light.clone(), 213.0, 343.0, 227.0, 332.0, 554.0)));
    objects.add(Arc::new(aarect::xz_rect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 0.0)));
    objects.add(Arc::new(aarect::xz_rect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));
    objects.add(Arc::new(aarect::xy_rect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));

    let block1_1 = Arc::new(block::Block::new(&Vec3::zero(), &Vec3::new(165.0, 330.0, 165.0), white.clone()));
    let block1_2 = Arc::new(scene::rotate_y::new(block1_1, 15.0));
    let block1 = Arc::new(scene::translate::new(block1_2, &Vec3::new(265.0, 0.0, 295.0)));
    objects.add(block1);

    let block2_1 = Arc::new(block::Block::new(&Vec3::zero(), &Vec3::new(165.0, 165.0, 165.0), white.clone()));
    let block2_2 = Arc::new(scene::rotate_y::new(block2_1, -18.0));
    let block2 = Arc::new(scene::translate::new(block2_2, &Vec3::new(130.0, 0.0, 65.0)));
    objects.add(block2);

    objects
}


fn cornell_smoke() -> scene::hittable_list {
    let red = Arc::new(material::lambertian::new(&Vec3::new(0.65,0.05,0.05)));
    let white = Arc::new(material::lambertian::new(&Vec3::new(0.73,0.73,0.73)));
    let green = Arc::new(material::lambertian::new(&Vec3::new(0.12,0.45,0.15)));
    let light = Arc::new(material::diffuse_light::new_with_para(&Vec3::new(7.0,7.0,7.0)));

    let mut objects = scene::hittable_list::new_without_para();

    objects.add(Arc::new(aarect::yz_rect::new(green.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));
    objects.add(Arc::new(aarect::yz_rect::new(red.clone(), 0.0, 555.0, 0.0, 555.0, 0.0)));
    objects.add(Arc::new(aarect::xz_rect::new(light.clone(), 113.0, 443.0, 127.0, 432.0, 554.0)));
    objects.add(Arc::new(aarect::xz_rect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));
    objects.add(Arc::new(aarect::xz_rect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 0.0)));
    objects.add(Arc::new(aarect::xy_rect::new(white.clone(), 0.0, 555.0, 0.0, 555.0, 555.0)));

    // let block1_1 = Rc::new(block::Block::new(&Vec3::zero(), &Vec3::new(165.0, 330.0, 165.0), white.clone()));
    // let block1_2 = Rc::new(scene::rotate_y::new(block1_1, 15.0));
    // let block1 = Rc::new(scene::translate::new(block1_2, &Vec3::new(265.0, 0.0, 295.0)));
    // objects.add(block1);

    let mut block1: Arc<dyn scene::hittable> = Arc::new(block::Block::new(&Vec3::zero(), &Vec3::new(165.0, 330.0, 165.0), white.clone()));
    block1 = Arc::new(scene::rotate_y::new(block1, 15.0));
    block1 = Arc::new(scene::translate::new(block1, &Vec3::new(265.0, 0.0, 295.0)));
    // objects.add(block1);

    let mut block2: Arc<dyn scene::hittable> = Arc::new(block::Block::new(&Vec3::zero(), &Vec3::new(165.0, 165.0, 165.0), white.clone()));
    block2 = Arc::new(scene::rotate_y::new(block2, -18.0));
    block2 = Arc::new(scene::translate::new(block2, &Vec3::new(130.0, 0.0, 65.0)));
    // objects.add(block2);

    objects.add(Arc::new(constant_medium::ConstantMedium::new_with_para(block1, 0.01, Vec3::zero())));
    objects.add(Arc::new(constant_medium::ConstantMedium::new_with_para(block2, 0.01, Vec3::ones())));

    objects
}


fn final_scene() -> scene::hittable_list {
    let mut boxes1 = scene::hittable_list::new_without_para();
    let ground = Arc::new(material::lambertian::new(&Vec3::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64 * w);
            let z0 = -1000.0 + (j as f64 * w);
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rtweekend::random_double_2(1.0, 101.0);
            let z1 = z0 + w;
            let _ground = Arc::clone(&ground);
            //let _ground = ground.clone();
            boxes1.add(Arc::new(block::Block::new(&Vec3::new(x0,y0,z0), &Vec3::new(x1,y1,z1), _ground )));
        }
    }

    let mut objects = scene::hittable_list::new_without_para();
    objects.add(Arc::new(bvh::bvh_node::new_with_3para(&mut boxes1, 0.0, 1.0)));

    let light = Arc::new(material::diffuse_light::new_with_para(&Vec3::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(aarect::xz_rect::new(light.clone(), 123.0, 423.0, 147.0, 412.0, 554.0)));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(material::lambertian::new(&Vec3::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(moving_sphere::MovingSphere::new(center1, center2, 0.0, 1.0, 50.0, moving_sphere_material)));

    objects.add(Arc::new(scene::Sphere::new(Point3::new(260.0, 150.0, 45.0), 50.0, Arc::new(material::dielectric::new(1.5)))));
    objects.add(Arc::new(scene::Sphere::new(Point3::new(0.0, 150.0, 145.0), 50.0, Arc::new(material::metal::new(&Vec3::new(0.8,0.8,0.9), 1.0)))));

    let mut boundary = Arc::new(scene::Sphere::new(Point3::new(360.0,150.0,145.0), 70.0, Arc::new(material::dielectric::new(1.5))));
    objects.add(boundary.clone());
    let boundary = Arc::new(scene::Sphere::new(Point3::new(360.0,150.0,145.0), 70.0, Arc::new(material::dielectric::new(1.5))));   ///////////boundary
    objects.add(Arc::new(constant_medium::ConstantMedium::new_with_para(boundary, 0.2, Vec3::new(0.2, 0.4, 0.9))));
    let boundary = Arc::new(scene::Sphere::new(Point3::zero(), 5000.0, Arc::new(material::dielectric::new(1.5))));
    objects.add(Arc::new(constant_medium::ConstantMedium::new_with_para(boundary, 0.0001, Vec3::ones())));

    let emat = Arc::new(material::lambertian::new_with_ptr(Arc::new(texture::ImageTexture::new_with_para("data/earthmap.jpg"))));
    objects.add(Arc::new(scene::Sphere::new(Vec3::new(400.0, 200.0, 400.0), 100.0, emat)));

    let pertext = Arc::new(texture::noise_texture::new_with_para(0.1));
    objects.add(Arc::new(scene::Sphere::new(Point3::new(220.0,280.0,300.0), 80.0, Arc::new(material::lambertian::new_with_ptr(pertext)))));

    let mut boxes2 = scene::hittable_list::new_without_para();
    let white = Arc::new(material::lambertian::new(&Vec3::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for j in 0..ns {
        boxes2.add(Arc::new(scene::Sphere::new(Vec3::random_vector_2(0.0, 165.0), 10.0, white.clone())));
    }

    objects.add(Arc::new(scene::translate::new(
                    Arc::new(scene::rotate_y::new
                        (Arc::new(bvh::bvh_node::new_with_3para(&mut boxes2, 0.0, 1.0)), 15.0)), 
                        &Vec3::new(-100.0,270.0,395.0)
                )
        )
    );
    
    return objects
}


pub fn get_world(op: usize, 
                 aspect_ratio: &mut f64,
                 image_width: &mut usize,
                 samples_per_pixel: &mut usize,
                 background: &mut Vec3,
                 lookfrom: &mut Vec3,
                 lookat: &mut Vec3,
                 vfov: &mut f64,
                 aperture: &mut f64) -> scene::hittable_list {
    if op == 1 {
        *background = Vec3::new(0.7, 0.8, 1.0);
        *lookfrom = Vec3::new(13.0, 2.0, 3.0);
        *lookat = Vec3::new(0.0, 0.0, 0.0);
        *vfov = 20.0;
        *aperture = 0.1;
        return random_scene();
    }
    else if op == 2 {
        *background = Color::new(0.7, 0.8, 1.0);
        *lookfrom = Vec3::new(13.0,2.0,3.0);
        *lookat = Vec3::zero();
        *vfov = 20.0;
        return two_spheres();
    }
    else if op == 3 {
        *background = Color::new(0.7, 0.8, 1.0);
        *lookfrom = Vec3::new(13.0,2.0,3.0);
        *lookat = Vec3::zero();
        *vfov = 20.0;
        return two_perlin_spheres();
    }
    else if op == 4 {
        *background = Color::new(0.7, 0.8, 1.0);
        *lookfrom = Vec3::new(13.0,2.0,3.0);
        *lookat = Vec3::zero();
        *vfov = 20.0;
        return earth();   
    }
    else if op == 5 {
        *samples_per_pixel = 400;
        *background = Color::zero();
        *lookfrom = Point3::new(26.0, 3.0, 6.0);
        *lookat = Point3::new(0.0, 2.0, 0.0);
        *vfov = 20.0;
        return simple_light();
    }
    else if op == 6 || op == 7 {
        *aspect_ratio = 1.0;
        *image_width = 600;
        *samples_per_pixel = 200;
        *background = Color::new(0.0, 0.0, 0.0);
        *lookfrom = Point3::new(278.0, 278.0, -800.0);
        *lookat = Point3::new(278.0, 278.0, 0.0);
        *vfov = 40.0;
        if op == 6 {
            return empty_cornell_box();
        }
        else {
            return standard_cornell_box();
        }
    }
    else if op == 8 {
        *aspect_ratio = 1.0;
        *image_width = 600;
        *samples_per_pixel = 200;
        *lookfrom = Point3::new(278.0, 278.0, -800.0);
        *lookat = Point3::new(278.0, 278.0, 0.0);
        *vfov = 40.0;
        return cornell_smoke();
    }
    else if op == 9 {
        *aspect_ratio = 1.0;
        *image_width = 800;
        *samples_per_pixel = 10000;
        // *image_width = 400;
        // *samples_per_pixel = 100;
        *background = Color::zero();
        *lookfrom = Point3::new(478.0, 278.0, -600.0);
        *lookat = Point3::new(278.0, 278.0, 0.0);
        *vfov = 40.0;
        return final_scene();
    }
    default_scene()
}