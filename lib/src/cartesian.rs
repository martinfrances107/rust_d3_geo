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

/// Convert point on sphere to cartesian coordinates.
pub fn cartesian<T: CoordFloat>(spherical: &Coordinate<T>) -> [T; 3] {
    let lambda = spherical.x;
    let phi = spherical.y;
    let cos_phi = phi.cos();
    [cos_phi * lambda.cos(), cos_phi * lambda.sin(), phi.sin()]
}

/// Calculate the dot product.
#[inline]
pub fn dot<T: CoordFloat>(a: &[T; 3], b: &[T; 3]) -> T {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Calculate the cross product.
#[inline]
pub fn cross<T: CoordFloat>(a: &[T; 3], b: &[T; 3]) -> [T; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Add two 3-D coordinates.
#[inline]
pub fn add<T: CoordFloat>(a: [T; 3], b: [T; 3]) -> [T; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// Make a the product of a + b.
pub fn add_in_place<T: CoordFloat>(a: &mut [T; 3], b: &[T; 3]) {
    a[0] = a[0] + b[0];
    a[1] = a[1] + b[1];
    a[2] = a[2] + b[2];
}

/// Scale a 3-D vector by a scalar.
#[inline]
pub fn scale<T: CoordFloat>(vector: &[T; 3], k: T) -> [T; 3] {
    [k * vector[0], k * vector[1], k * vector[2]]
}

/// Set the magnitude of the 3-D vector to one.
pub fn normalize_in_place<T: CoordFloat>(d: &mut [T; 3]) {
    let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    d[0] = d[0] / l;
    d[1] = d[1] / l;
    d[2] = d[2] / l;
}

/// Outputs a 3-D vector with the direction of the input but with a magnitude of one.
pub fn normalize<T: CoordFloat>(d: &[T; 3]) -> [T; 3] {
    let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    [d[0] / l, d[1] / l, d[2] / l]
}
