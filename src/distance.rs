use num_traits::Float;

use crate::data_object::DataObject;
use crate::length::LengthStream;

pub fn distance<F>(a: &[F; 2], b: &[F; 2]) -> F
where
  F: Float,
{
  // TODO consider making object static outside of distance.
  // It does not need to be created each time.
  let object = DataObject::LineString {
    coordinates: vec![*a, *b],
  };

  return LengthStream::calc(object);
}
