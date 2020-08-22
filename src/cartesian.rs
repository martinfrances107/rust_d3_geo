use num_traits::cast::FromPrimitive;
use num_traits::Float;

pub fn spherical<F>(cartesian: &[F; 3]) -> [F; 2]
where
  F: Float + FromPrimitive,
{
  return [cartesian[1].atan2(cartesian[0]), cartesian[2].asin()];
}

pub fn cartesian<F>(spherical: &[F; 2]) -> [F; 3]
where
  F: Float,
{
  let lambda = spherical[0];
  let phi = spherical[1];
  let cos_phi = phi.cos();
  return [cos_phi * lambda.cos(), cos_phi * lambda.sin(), phi.sin()];
}

pub fn cartesian_dot<F>(a: &[F; 3], b: &[F; 3]) -> F
where
  F: Float,
{
  return a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
}

pub fn cartesian_cross<F>(a: &[F; 3], b: &[F; 3]) -> [F; 3]
where
  F: Float,
{
  return [
    a[1] * b[2] - a[2] * b[1],
    a[2] * b[0] - a[0] * b[2],
    a[0] * b[1] - a[1] * b[0],
  ];
}

pub fn cartesian_add<F>(a: [F; 3], b: [F; 3]) -> [F; 3]
where
  F: Float,
{
  return [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
}

pub fn cartesian_add_in_place<F>(a: &mut [F; 3], b: &[F; 3])
where
  F: Float,
{
  a[0] = a[0] + b[0];
  a[1] = a[1] + b[1];
  a[2] = a[1] + b[2];
}

pub fn cartesian_scale<F>(vector: &[F; 3], k: F) -> [F; 3]
where
  F: Float,
{
  return [k * vector[0], k * vector[1], k * vector[2]];
}

pub fn cartesian_normalize_in_place<F>(d: &mut [F; 3])
where
  F: Float,
{
  let l = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
  d[0] = d[0] / l;
  d[1] = d[1] / l;
  d[2] = d[2] / l;
}
