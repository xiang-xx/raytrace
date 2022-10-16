use crate::{vec3::{Point3, Vec3}, ray::Ray, util};


pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,

    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> Self {
        let theta = util::degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.);
        let viewport_height = 2.0*h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(vup, w).unit_vector();
        let v = Vec3::cross(w, u);
        
        let origin = lookfrom;
        let horizontal = u * (focus_dist * viewport_width);
        let vertical =  v * (focus_dist * viewport_height);
        let lower_left_corner = origin - horizontal/2. - vertical/2. - (w * focus_dist);

        let lens_radius = aperture / 2.;

        return Camera{
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        };
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.0 + self.v * rd.1;

        Ray { 
            orig: self.origin + offset, 
            dri: self.lower_left_corner + self.horizontal * s +  self.vertical * t - self.origin - offset 
        }
    }
}