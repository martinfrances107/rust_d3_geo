use super::features_struct::FeaturesStruct;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;
use geo::CoordFloat;
use num_traits::FloatConst;

/// FeatruesCollection - An array of feature objects.
pub struct FeatureCollection<T: CoordFloat>(pub Vec<FeaturesStruct<T>>);

impl<T: CoordFloat + FloatConst> DataObject<T> for FeatureCollection<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        for f in &self.0 {
            for g in f.geometry.clone() {
                processor(&g, stream);
            }
        }
    }
}
