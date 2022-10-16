use rand::prelude::*;


pub static pi: f64 = 3.1415926535897932385;


pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    return min + (max - min) * random_f64();
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * pi / 180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min
    }
    if x > max {
        return max
    }
    return x
}