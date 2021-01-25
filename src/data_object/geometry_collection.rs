// use super::feature_geometry::FeatureGeometry;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;
use geo::{CoordFloat, GeometryCollection};

/// GeometryCollection - an array of geometry objects.
// pub struct GeometryCollection<T: Float> {
//     pub geometries: Vec<FeatureGeometry<T>>,
// }

impl<T: CoordFloat + num_traits::FloatConst> DataObject<T> for GeometryCollection<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        for g in self {
            processor(g, stream);
        }
    }
}
