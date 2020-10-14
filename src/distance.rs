use delaunator::Point;

use crate::data_object::DataObject;
use crate::length::LengthStream;

pub fn distance(a: &Point, b: &Point) -> f64
{
  // TODO consider making object static outside of distance.
  // It does not need to be created each time.
  let object = DataObject::LineString {
    coordinates: vec![(*a).clone(), (*b).clone()],
  };

  return LengthStream::calc(object);
}
