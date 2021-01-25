// use super::feature_geometry::FeatureGeometry;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;
use geo::Point;
use geo::{CoordFloat, Geometry};
use num_traits::FloatConst;

impl<T: CoordFloat + FloatConst> DataObject<T> for Point<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        processor(&Geometry::Point(*self), stream);
    }
}
