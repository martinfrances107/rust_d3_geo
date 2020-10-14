use delaunator::Point;

pub fn spherical(cartesian: &[f64; 3]) -> Point
{
  return Point{x:cartesian[1].atan2(cartesian[0]), y:cartesian[2].asin()};
}

pub fn cartesian(spherical: &Point) -> [f64; 3]
{
  let lambda = spherical.x;
  let phi = spherical.y;
  let cos_phi = phi.cos();
  return [cos_phi * lambda.cos(), cos_phi * lambda.sin(), phi.sin()];
}

pub fn cartesian_dot(a: &[f64; 3], b: &[f64; 3]) -> f64
{
  return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

pub fn cartesian_cross(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3]
{
  return [
    a[1] * b[2] - a[2] * b[1],
    a[2] * b[0] - a[0] * b[2],
    a[0] * b[1] - a[1] * b[0],
  ];
}

pub fn cartesian_add(a: [f64; 3], b: [f64; 3]) -> [f64; 3]
{
  return [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
}

pub fn cartesian_add_in_place(a: &mut [f64; 3], b: &[f64; 3])
{
  a[0] = a[0] + b[0];
  a[1] = a[1] + b[1];
  a[2] = a[1] + b[2];
}

pub fn cartesian_scale(vector: &[f64; 3], k: f64) -> [f64; 3]
{
  return [k * vector[0], k * vector[1], k * vector[2]];
}

pub fn cartesian_normalize_in_place(d: &mut [f64; 3])
{
  let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
  d[0] = d[0] / l;
  d[1] = d[1] / l;
  d[2] = d[2] / l;
}

pub fn cartesian_normalize(d: &[f64; 3])-> [f64;3]
{
  let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
  return  [d[0] / l, d[1] / l, d[2] / l];
}
