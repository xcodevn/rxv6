#![crate_type="staticlib"]
#![allow(unused_variable)]

#[no_mangle]
pub fn floorf(num: f32) -> f32 {
    1.0* (num as f32)
}

#[no_mangle]
pub fn floor(num: f64) -> f64 {
    1.0* (num as f64)
}

#[no_mangle]
pub fn ceilf(num: f32) -> f32 {
    let fl = floorf(num);
    if fl < num { fl + 1.0 } else { fl }
}

#[no_mangle]
pub fn ceil(num: f64) -> f64 {
    let fl = floor(num);
    if fl < num { fl + 1.0 } else { fl }
}

#[no_mangle]
pub fn roundf(num: f32) -> f32 {
    let fl = floorf(num);
    let ce = ceilf(num);
    if num - fl <= ce - num { fl } else { ce }
}

#[no_mangle]
pub fn round(num: f64) -> f64 {
    let fl = floor(num);
    let ce = ceil(num);
    if num - fl <= ce - num { fl } else { ce }
}

fn absf(num: f32) -> f32 {
    if num < 0.0 { -num } else { num }
}

fn abs(num: f64) -> f64 {
    if num < 0.0 { -num } else { num }
}

#[no_mangle]
pub fn truncf(num: f32) -> f32 {
    let fl = floorf(num);
    let ce = ceilf(num);
    if absf(fl) < absf(ce) { fl } else { ce }
}

#[no_mangle]
pub fn trunc(num: f64) -> f64 {
    let fl = floor(num);
    let ce = ceil(num);
    if abs(fl) < abs(ce) { fl } else { ce }
}


#[no_mangle]
pub fn fmaf(a: f32, b: f32, c: f32) -> f32 {
    a * b + c
}

#[no_mangle]
pub fn powf(a: f32, b: f32) -> f32 {
    0f32
}

