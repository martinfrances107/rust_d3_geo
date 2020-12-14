// use super::feature_geometry::FeatureGeometry;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;
use geo::Geometry;
use geo::Point;
use num_traits::Float;

/// Point - a single position.
// pub struct Point {
//     pub coordinate: delaunator::Point,
// }

impl<T: Float> DataObject<T> for Point<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        processor(&Geometry::Point(*self), stream);
    }
}
