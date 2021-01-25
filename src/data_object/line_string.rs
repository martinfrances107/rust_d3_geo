// use super::feature_geometry::FeatureGeometry;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;
use geo::LineString;
use geo::{CoordFloat, Geometry};
use num_traits::FloatConst;

impl<T: CoordFloat + FloatConst> DataObject<T> for LineString<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        // let g = FeatureGeometry::LineString(
        //     *self
        // );
        processor(&Geometry::LineString(self.clone()), stream);
    }
}
