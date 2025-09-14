use rand::{self, Rng};

pub const PI: f64 = 3.1415926535897932385;
pub const INFINITY: f64 = f64::INFINITY;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    (degrees * PI) / 180.
}

pub fn get_random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.r#gen()
}

pub fn get_random_f64_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
