use geo::{CoordFloat, Coordinate};

/// Converts 3D Cartesian to spherical coordinates (degrees).
#[inline]
pub fn spherical<T: CoordFloat>(cartesian: &[T; 3]) -> Coordinate<T> {
    Coordinate {
        x: cartesian[1].atan2(cartesian[0]).to_degrees(),
        y: cartesian[2].asin().to_degrees(),
    }
}

/// Converts 3D Cartesian to spherical coordinates (radians).
#[inline]
pub fn spherical_r<T: CoordFloat>(cartesian: &[T; 3]) -> Coordinate<T> {
    Coordinate {
        x: cartesian[1].atan2(cartesian[0]),
        y: cartesian[2].asin(),
    }
}

pub fn cartesian<T: CoordFloat>(spherical: &Coordinate<T>) -> [T; 3] {
    let lambda = spherical.x;
    let phi = spherical.y;
    let cos_phi = phi.cos();
    return [cos_phi * lambda.cos(), cos_phi * lambda.sin(), phi.sin()];
}

#[inline]
pub fn cartesian_dot<T: CoordFloat>(a: &[T; 3], b: &[T; 3]) -> T {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[inline]
pub fn cartesian_cross<T: CoordFloat>(a: &[T; 3], b: &[T; 3]) -> [T; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[inline]
pub fn cartesian_add<T: CoordFloat>(a: [T; 3], b: [T; 3]) -> [T; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

pub fn cartesian_add_in_place<T: CoordFloat>(a: &mut [T; 3], b: &[T; 3]) {
    a[0] = a[0] + b[0];
    a[1] = a[1] + b[1];
    a[2] = a[1] + b[2];
}

#[inline]
pub fn cartesian_scale<T: CoordFloat>(vector: &[T; 3], k: T) -> [T; 3] {
    [k * vector[0], k * vector[1], k * vector[2]]
}

pub fn cartesian_normalize_in_place<T: CoordFloat>(d: &mut [T; 3]) {
    let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    d[0] = d[0] / l;
    d[1] = d[1] / l;
    d[2] = d[2] / l;
}

pub fn cartesian_normalize<T: CoordFloat>(d: &[T; 3]) -> [T; 3] {
    let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    return [d[0] / l, d[1] / l, d[2] / l];
}
