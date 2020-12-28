//Clamps a given float
pub fn clamp_float(num: f32, min: f32, max: f32) -> f32 {
    if num > max {
        max
    }
    else if num < min {
        min
    }
    else {
        num
    }
}

//Finds the max of a given float
pub fn max_float(num1: f32, num2: f32) -> f32 {
    if num1 > num2 {
        num1
    }
    else {
        num2
    }
}

//Finds the max of a given float
pub fn min_float(num1: f32, num2: f32) -> f32 {
    if num1 < num2 {
        num1
    }
    else {
        num2
    }
}