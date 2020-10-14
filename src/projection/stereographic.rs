use std::rc::Rc;

use delaunator::Point;

use super::projection::Projection;
use super::projection::StreamProcessorValueMaybe;
use super::projection_mutator::ProjectionMutator;
use crate::projection::azimuthal::azimuthal_invert;
use crate::Transform;

#[derive(Clone, Debug)]
pub struct StereographicRaw {}

impl StereographicRaw {
  fn new() -> Box<dyn Transform>
  {
    return Box::new(StereographicRaw {});
  }

  pub fn gen_projection_mutator<'a>() -> ProjectionMutator
    {
    println!("Enter gen_projection_mutator.");
    let s = Rc::new(StereographicRaw::new());
    let mut projection = ProjectionMutator::from_projection_raw(s);
    projection.scale(Some(&250f64));
    let angle = 142f64;
    println!("about to clip angle");
    projection.clip_angle(StreamProcessorValueMaybe::Value(angle));
    println!("Exit gen_projection_mutator.");
    return projection;
  }
}

impl Transform for StereographicRaw
{
  fn transform(&self, p: &Point) -> Point {
    // let x = p.x;
    // let y = p.y;
    // let p = *p.clone();
    let cy = p.y.cos();
    let k = 1f64 + p.x.cos() * cy;
    return Point{x:cy * p.x.sin() / k, y:p.y.sin() / k};
  }

  fn invert(&self, p: &Point) -> Point {
    let f = Box::new(|z: f64| 2f64 * z.atan());
    let g = azimuthal_invert(f);
    return g(p.x, p.y);
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use crate::projection::projection_equal::projection_equal;
  #[test]
  fn test_stereographic_embedded() {
    let mut stereo = StereographicRaw::gen_projection_mutator();
    stereo.translate(Some(&Point{x:0f64, y:0f64}));
    stereo.scale(Some(&1f64));

    assert!(projection_equal(
      &stereo,
      &Point{x:0f64, y:0f64},
      &Point{x:0f64, y:0f64},
      None
    ));
    assert!(projection_equal(
      &stereo,
      &Point{x:-90f64, y:0f64},
      &Point{x:-1f64, y:0f64},
      None
    ));
    assert!(projection_equal(
      &stereo,
      &Point{x:90f64, y:0f64},
      &Point{x:1f64, y:0f64},
      None
    ));
    assert!(projection_equal(
      &stereo,
      &Point{x:0f64, y:-90f64},
      &Point{x:0f64, y:1f64},
      None
    ));
    assert!(projection_equal(
      &stereo,
      &Point{x:0f64, y:90f64},
      &Point{x:0f64, y:-1f64},
      None
    ));
  }
}
