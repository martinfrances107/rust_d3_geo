use delaunator::Point;

pub fn azimuthal_raw(scale: Box<dyn Fn(f64) -> f64>) -> Box<dyn Fn(f64, f64) -> Point>
{
  return Box::new(move |x: f64, y: f64| -> Point {
    let cx = x.cos();
    let cy = y.cos();
    let k = scale(cx * cy);
    return match k.is_infinite() {
      true => Point{x:2f64, y:0f64},
      false => Point{x:k * cy * x.sin(), y:k * y.sin()},
    };
  });
}

pub fn azimuthal_invert(angle: Box<dyn Fn(f64) -> f64>) -> Box<dyn Fn(f64, f64) -> Point>
{
  return Box::new(move |x: f64, y: f64| -> Point {
    let z = (x * x + y * y).sqrt();
    let c = angle(z);
    let sc = c.sin();
    let cc = c.cos();

    let ret_x = (x * sc).atan2(z * cc);
    let y_out;
    if z == 0f64 {
      y_out = z;
    } else {
      y_out = y * sc / z;
    }
    let ret_y = y_out.asin();

    return Point{x:ret_x, y:ret_y};
  });
}
