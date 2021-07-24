use std::fmt::Display;
use std::ops::AddAssign;

use crate::stream::Stream;

// use super::feature_geometry::FeatureGeometry;
use super::Streamable;
use geo::MultiPoint;
use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst> Streamable
    for MultiPoint<T>
{
    type T = T;
    // type SD = Self;
    fn to_stream<SD: Stream<SC = Coordinate<T>>>(&self, stream: &mut SD) {
        for p in self.iter() {
            // TODO there must be a better conversion.
            stream.point(&Coordinate { x: p.x(), y: p.y() }, None);
        }
    }
}
