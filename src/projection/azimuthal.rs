use num_traits::Float;

pub fn azimuthal_raw<F>(scale: Box<dyn Fn(F) -> F>) -> Box<dyn Fn(F, F) -> [F; 2]>
where
  F: Float + 'static,
{
  return Box::new(move |x: F, y: F| -> [F; 2] {
    let cx = x.cos();
    let cy = y.cos();
    let k = scale(cx * cy);
    return match k.is_infinite() {
      true => [F::from(2u8).unwrap(), F::zero()],
      false => [k * cy * x.sin(), k * y.sin()],
    };
  });
}

pub fn azimuthal_invert<F>(angle: Box<dyn Fn(F) -> F>) -> Box<dyn Fn(F, F) -> [F; 2]>
where
  F: Float + 'static,
{
  return Box::new(move |x: F, y: F| -> [F; 2] {
    let z = (x * x + y * y).sqrt();
    let c = angle(z);
    let sc = c.sin();
    let cc = c.cos();

    let ret_x = (x * sc).atan2(z * cc);
    let ret_y;
    if z.is_zero() {
      ret_y = z;
    } else {
      ret_y = y * sc / z;
    }

    return [ret_x, ret_y];
  });
}
