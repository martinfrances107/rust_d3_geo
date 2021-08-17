use std::fmt::Display;
use std::ops::AddAssign;

use geo::CoordFloat;
use num_traits::AsPrimitive;
use num_traits::FloatConst;

use crate::data_object::FeatureCollection;
use crate::stream::Stream;

use super::Streamable;

impl<T> Streamable for FeatureCollection<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
{
    // type SD = Self;
    type T = T;
    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        for f in &self.0 {
            for g in &f.geometry {
                g.to_stream(stream);
            }
        }
    }
}
