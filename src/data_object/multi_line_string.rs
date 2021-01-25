// use super::feature_geometry::FeatureGeometry;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;
use geo::MultiLineString;
use geo::{CoordFloat, Geometry};
use num_traits::FloatConst;

/// MultiLineString - an array of arrays of positions forming several lines.

impl<T: CoordFloat + FloatConst> DataObject<T> for MultiLineString<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        processor(&Geometry::MultiLineString(self.clone()), stream);
    }
}
