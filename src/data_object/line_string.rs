use super::feature_geometry::FeatureGeometry;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;
use geo::Geometry;
use geo::LineString;
use num_traits::Float;

///  LineString - an array of positions forming a continuous line.
// pub struct LineString {
//     pub coordinates: Vec<Point<T>>,
// }

impl<T: Float> DataObject<T> for LineString<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        // let g = FeatureGeometry::LineString(
        //     *self
        // );
        processor(&Geometry::LineString(self.clone()), stream);
    }
}
