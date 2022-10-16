use std::rc::Rc;
use std::sync::Arc;

use crate::material::Material;
use crate::vec3::{Point3, Vec3};
use crate::ray::{Hittable, HitRecord, Ray};

pub struct Sphere {
    pub center: Point3,
    pub r: f64,
    pub mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, r: f64, mat: Arc<dyn Material>) -> Sphere {
        return Sphere { center, r, mat }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.orig - self.center;
        let a = r.dri.length_squared();
        let half_b  = Vec3::dot(oc, r.dri);
        let c = oc.length_squared() - self.r * self.r;
        let discriminant = half_b * half_b  -  a * c;
        if discriminant < 0. {
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) /a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.r;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();

        return true
    }
}


