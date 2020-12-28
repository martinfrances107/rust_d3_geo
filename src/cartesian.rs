use geo::Coordinate;
use num_traits::Float;

pub fn spherical<T: Float>(cartesian: &[T; 3]) -> Coordinate<T> {
    return Coordinate {
        x: cartesian[1].atan2(cartesian[0]),
        y: cartesian[2].asin(),
    };
}

pub fn cartesian<T: Float>(spherical: &Coordinate<T>) -> [T; 3] {
    let lambda = spherical.x;
    let phi = spherical.y;
    let cos_phi = phi.cos();
    return [cos_phi * lambda.cos(), cos_phi * lambda.sin(), phi.sin()];
}

pub fn cartesian_dot<T: Float>(a: &[T; 3], b: &[T; 3]) -> T {
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

pub fn cartesian_cross<T: Float>(a: &[T; 3], b: &[T; 3]) -> [T; 3] {
    return [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ];
}

pub fn cartesian_add<T: Float>(a: [T; 3], b: [T; 3]) -> [T; 3] {
    return [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
}

pub fn cartesian_add_in_place<T: Float>(a: &mut [T; 3], b: &[T; 3]) {
    a[0] = a[0] + b[0];
    a[1] = a[1] + b[1];
    a[2] = a[1] + b[2];
}

pub fn cartesian_scale<T: Float>(vector: &[T; 3], k: T) -> [T; 3] {
    return [k * vector[0], k * vector[1], k * vector[2]];
}

pub fn cartesian_normalize_in_place<T: Float>(d: &mut [T; 3]) {
    let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    d[0] = d[0] / l;
    d[1] = d[1] / l;
    d[2] = d[2] / l;
}

pub fn cartesian_normalize<T: Float>(d: &[T; 3]) -> [T; 3] {
    let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    return [d[0] / l, d[1] / l, d[2] / l];
}
