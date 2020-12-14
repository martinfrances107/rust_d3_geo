use super::feature_geometry::FeatureGeometry;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;
use geo::Geometry;
use geo::MultiLineString;
use num_traits::Float;
/// MultiLineString - an array of arrays of positions forming several lines.

impl<T: Float> DataObject<T> for MultiLineString<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        processor(&Geometry::MultiLineString(self.clone()), stream);
    }
}
