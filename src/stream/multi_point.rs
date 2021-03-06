use std::fmt::Display;
use std::ops::AddAssign;

use crate::stream::Stream;

// use super::feature_geometry::FeatureGeometry;
use super::Streamable;
use geo::MultiPoint;
use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Streamable<T>
    for MultiPoint<T>
{
    type SC = Coordinate<T>;
    fn to_stream(&self, stream: &mut impl Stream<T, C = Self::SC>) {
        for p in self.iter() {
            // TODO there must be a better conversion.
            stream.point(&Coordinate { x: p.x(), y: p.y() }, None);
        }
    }
}
