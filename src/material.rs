use crate::{ray::{Ray, HitRecord}, vec3::{Color, Vec3}};
use crate::util;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_dir = rec.normal + Vec3::random_unit_vector();

        if scatter_dir.near_zero() {
            scatter_dir = rec.normal;
        }

        let s = Ray::new(rec.p, scatter_dir);
        scattered.dri = s.dri;
        scattered.orig = s.orig;
        attenuation.0 = self.albedo.0;
        attenuation.1 = self.albedo.1;
        attenuation.2 = self.albedo.2;
        return true;
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(r_in.dri.unit_vector(), rec.normal);
        let s = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        scattered.dri = s.dri;
        scattered.orig = s.orig;
        attenuation.0 = self.albedo.0;
        attenuation.1 = self.albedo.1;
        attenuation.2 = self.albedo.2;
        return Vec3::dot(scattered.dri, rec.normal) > 0.;
    }
}

impl Metal {
    pub fn new(color: Vec3, f: f64) -> Self {
        return Metal { albedo: color, fuzz: if f < 1. {f} else {1.} }
    }
}


pub struct Dielectric {
    pub ir: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        attenuation.0 = 1.;
        attenuation.1 = 1.;
        attenuation.2 = 1.;
        let refraction_ratio = if rec.front_face {1.0/self.ir} else {self.ir};
        let unit_direction = r_in.dri.unit_vector();

        let cos_theta = f64::min(Vec3::dot(-unit_direction, rec.normal), 1.);
        let sin_theta = f64::sqrt(1. - cos_theta*cos_theta);
        
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3 = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > util::random_f64() {
            Vec3::reflect(unit_direction, rec.normal)
        } else {
            Vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };

        scattered.orig = rec.p;
        scattered.dri = direction;
        return true;
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 = r0 * r0;
        return r0 + (1.-r0)*f64::powi(1.-cosine, 5);
    }
}


