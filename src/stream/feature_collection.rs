use std::ops::AddAssign;

use super::Streamable;
// use crate::data_object::FeaturesStruct;
use crate::data_object::FeatureCollection;

use crate::stream::Stream;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;

impl<T: AddAssign + CoordFloat + Default + FloatConst> Streamable<T> for FeatureCollection<T> {
    type SC = Coordinate<T>;
    fn to_stream(&self, stream: &mut impl Stream<T, C = Self::SC>) {
        for f in &self.0 {
            for g in &f.geometry {
                g.to_stream(stream);
            }
        }
    }
}
