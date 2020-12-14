use crate::stream::Stream;

pub mod feature_collection;
pub mod feature_geometry;
pub mod feature_property;
pub mod feature_struct;
pub mod features_struct;
pub mod geometry_collection;
pub mod line_string;
pub mod multi_line_string;
pub mod multi_point;
pub mod multi_polygon;
pub mod point;
pub mod polygon;
use num_traits::Float;
pub trait DataObject<T: Float> {
    fn to_stream(&self, stream: &mut impl Stream<T>);
}
