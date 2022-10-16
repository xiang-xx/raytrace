use raytrace::material::{Lambertian, Metal, Material, Dielectric};
use raytrace::util::{random_f64, random_f64_range};
use raytrace::{util, camera};
use raytrace::vec3::{Vec3, Color, Point3};
use raytrace::ray::{Ray, Hittable, HitRecord, HittableList};
use raytrace::sphere::Sphere;
use std::process::id;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread::{self, current};
use std::time::{SystemTime};
use rayon::prelude::*;

fn write_color(p: &mut Color, sample_per_pix: f64) {
    let scale = 1.0 / sample_per_pix;
    // gamma correct for gamma=2.0
    p.0 = f64::sqrt(p.0 * scale);
    p.1 = f64::sqrt(p.1 * scale);
    p.2 = f64::sqrt(p.2 * scale);

    println!(
        "{} {} {}",
        ((256. * util::clamp(p.0, 0., 0.999)) as i32),
        ((256. * util::clamp(p.1, 0., 0.999)) as i32),
        ((256. * util::clamp(p.2, 0., 0.999)) as i32),
    )
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Vec3(0.,0.,0.);
    }
    let mut rec = HitRecord::new(Arc::new(Metal{albedo: Vec3(0., 0., 0.), fuzz: 1.}));
    if world.hit(r, 0.001, f64::MAX, &mut rec) {
        let mut scattered = Ray { orig: Vec3(0., 0., 0.), dri: Vec3(0., 0., 0.) };
        let mut attenuation = Vec3(0., 0., 0.);
        if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Vec3(0.,0.,0.);
    }
    
    let unit_driection = r.dri.unit_vector();
    let t = 0.5 * (unit_driection.1 + 1.0);
    Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let material_ground = Arc::new(Lambertian{
        albedo: Vec3(0.5,0.5,0.5),
    });
    world.add(Box::new(Sphere::new(Vec3(0., -1000., -1.), 1000., material_ground)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = Vec3(a as f64 + 0.9 * random_f64(), 0.2, b as f64 + 0.9 * random_f64());
            if (center - Vec3(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    let mat = Arc::new(Lambertian{albedo: albedo});
                    world.add(Box::new(Sphere::new(center, 0.2, mat)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random_range(0.5, 1.);
                    let fuzz = random_f64_range(0., 0.5);
                    let mat = Arc::new(Metal{albedo, fuzz});
                    world.add(Box::new(Sphere::new(center, 0.2, mat)))
                } else {
                    // glass
                    let mat = Arc::new(Dielectric{ir: 1.5});
                    world.add(Box::new(Sphere::new(center, 0.2, mat)))
                }
            }
        }
    }

    let mat1 = Arc::new(Dielectric{ir: 1.5});
    world.add(Box::new(Sphere::new(Vec3(0., 1., 0.), 1., mat1)));

    let mat2 = Arc::new(Lambertian{albedo: Vec3(0.4, 0.2, 0.1)});
    world.add(Box::new(Sphere::new(Vec3(-4., 1., 0.), 1., mat2)));
    
    let mat3 = Arc::new(Metal{albedo: Vec3(0.7, 0.6, 0.5), fuzz: 0.0});
    world.add(Box::new(Sphere::new(Vec3(4., 1., 0.), 1., mat3)));
    

    return world;
}

fn main() {
    // image size
    let aspect_ratio = 3./2.;
    let image_width = 1200;
    let image_height =  ((image_width as f64) / aspect_ratio) as i32;
    let sample_per_pix = 500;
    let sample_per_pix_f = sample_per_pix as f64;
    let max_depth = 50;


    // world
    let world = random_scene();
    
    let lookfrom = Vec3(13.0, 2.0, 3.0);
    let lookat = Vec3(0., 0., 0.);
    let cam = camera::Camera::new(
        lookfrom, 
        lookat, 
        Vec3(0., 1., 0.), 
        20.,
        aspect_ratio,
        0.1,
        10., 
    );


    println!("P3\n{image_width} {image_height}\n255");

    let mut arr = Vec::<Vec3>::new();
    arr.resize((image_height * image_width) as usize, Vec3(0., 0., 0.));
    let mut result = Arc::new(Mutex::new(arr));

    let start = SystemTime::now();
    // render
    for j in 0..image_height { // 800px
        // eprintln!("\nScanlines remaining: {}", image_height - j);
        let is = 0..image_width;
        is.into_par_iter().for_each(|i| {
            let jj = image_height - j - 1;
            let mut pixel_color = Vec3(0.,0.,0.);
            for _s in 0..sample_per_pix {
                let u = ((i as f64) + random_f64()) / (image_width as f64 - 1.);
                let v = ((jj as f64) + random_f64()) / (image_height as f64 - 1.);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }

            let idx = (j * image_width + i) as usize;
            result.lock().unwrap()[idx] = pixel_color;
            // result.[idx] = pixel_color;
        })
    }

    for v in result.lock().unwrap().iter() {
        let mut p = v.clone();
        write_color(&mut p, sample_per_pix_f);
    }

    eprintln!("\nDone");

    let end = SystemTime::now();
    let since = end.duration_since(start).expect("Time went backwards");
    eprintln!("{} sec", since.as_secs())
}
