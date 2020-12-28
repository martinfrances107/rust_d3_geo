use super::features_struct::FeaturesStruct;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;
use num_traits::Float;

/// FeatruesCollection - An array of feature objects.
pub struct FeatureCollection<T: Float>(pub Vec<FeaturesStruct<T>>);

impl<T: Float> DataObject<T> for FeatureCollection<T> {
    fn to_stream(&self, stream: &mut impl Stream<T>) {
        for f in &self.0 {
            for g in f.geometry.clone() {
                processor(&g, stream);
            }
        }
    }
}
