#[allow(dead_code)]
pub const DEG_TO_RAD: f32 = 0.01745329251;
#[allow(dead_code)]
pub const RAD_TO_DEG: f32 = 57.2957795131;

#[allow(dead_code)]
pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}
