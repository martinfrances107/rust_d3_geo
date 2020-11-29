use super::features_struct::FeaturesStruct;
use super::DataObject;
use crate::stream::geometry_processor::processor;
use crate::stream::Stream;

/// FeatruesCollection - An array of feature objects.
pub struct FeatureCollection {
    pub features: Vec<FeaturesStruct>,
}

impl DataObject for FeatureCollection {
    fn to_stream(&self, stream: &mut impl Stream) {
        for f in &self.features {
            for g in f.geometry.clone() {
                processor(&g, stream);
            }
        }
    }
}
