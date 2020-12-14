use super::feature_geometry::FeatureGeometry;
use super::feature_property::FeatureProperty;
use num_traits::Float;
// Signular veriosn of the struct.
#[derive(Clone, Debug)]
pub struct FeatureStruct<T: Float> {
    pub properties: Vec<FeatureProperty>,
    pub geometry: FeatureGeometry<T>,
}
