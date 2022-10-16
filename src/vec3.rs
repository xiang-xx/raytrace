use std::{ops::{Add, Neg, Sub, AddAssign, Mul, MulAssign, Div, DivAssign, Index}};
use crate::util::{self, random_f64_range, random_f64};

#[derive(Copy, Clone, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: f64) -> Self::Output {
        Vec3(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(
            self.0 * rhs,
            self.1 * rhs,
            self.2 * rhs,
        )
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0/rhs, self.1/rhs, self.2/rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl Index<isize> for Vec3 {
    type Output = f64;
    fn index(&self, index: isize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _default => &self.0,
        }
    }
}

impl Vec3 {
    pub fn random() -> Self {
        Vec3(util::random_f64(), util::random_f64(), util::random_f64())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3(random_f64_range(min, max),random_f64_range(min, max),random_f64_range(min, max))
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1., 1.);
            if p.length_squared() >= 1. {
                continue;
            }
            return p
        }
    }

    pub fn random_unit_vector() -> Self {
        return Self::random_in_unit_sphere().unit_vector();
    }

    pub fn random_in_hemisphere(normal: Self) -> Self {
        let in_unit_sphere = Self::random_in_unit_sphere();
        if Self::dot(in_unit_sphere, normal) > 0. {
            return in_unit_sphere
        } else {
            return -in_unit_sphere
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3(random_f64_range(-1., 1.), random_f64_range(-1.,1.), 0.);
            if p.length_squared() >= 1. {
                continue;
            }
            return p;
        }
    }

    pub fn dot(u: Self, v: Self) -> f64 {
        u.0*v.0 + u.1*v.1 + u.2*v.2
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v -  n * (Self::dot(v, n) * 2.);
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(Self::dot(-uv, n), 1.0);
        let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
        let r_out_parallel = n * -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared()));
        return r_out_parallel + r_out_perp;
    }

    pub fn cross(u: Self, v: Self) -> Self {
        Vec3(
            u.1*v.2 - u.2*v.1,
            u.2*v.0 - u.0*v.2,
            u.0*v.1 - u.1*v.0,
        )
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn unit_vector(self) -> Self {
        let l = self.length();
        self / l
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return f64::abs(self.0) < s && f64::abs(self.1) < s && f64::abs(self.2) < s;
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;