use num_traits::Float;
use num_traits::FloatConst;

pub fn point_equal<F>(a: (F,F,bool), b : (F,F,bool)) -> bool
where F: Float {
  return ((a.0 - b.0).abs() < Float::epsilon()) &&
         ((a.1 - b.1).abs() < Float::epsilon());
}
