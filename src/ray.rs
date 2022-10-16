use std::{rc::Rc, sync::Arc};

use crate::{vec3::{Point3, Vec3, self}, material::Material};

pub struct Ray {
    pub orig: Point3,
    pub dri: Point3,
}

impl Ray {
    pub fn new(orig: Point3, dri: Point3) -> Ray {
        Ray { orig, dri }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dri * t
    }
}

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(mat: Arc<dyn Material>) -> Self {
        return HitRecord {
            p: Vec3(0.,0.,0.),
            normal: Vec3(0.,0.,0.),
            t: 0.,
            front_face: false,
            mat: mat,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(r.dri, outward_normal) < 0.;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub trait Hittable : Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new(rec.mat.clone());
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                // rec.clone_from(&temp_rec)
                // todo material 实现 clone
                rec.p = temp_rec.p;
                rec.t = temp_rec.t;
                rec.front_face = temp_rec.front_face;
                rec.normal = temp_rec.normal;
                rec.mat = temp_rec.mat.clone();
                // rec = temp_rec;
            }
        }
        return hit_anything;
    }
}

impl HittableList {
    pub fn new() -> Self {
        return HittableList {
            objects: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}


