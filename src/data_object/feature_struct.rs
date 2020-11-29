use super::feature_geometry::FeatureGeometry;
use super::feature_property::FeatureProperty;
// Signular veriosn of the struct.
#[derive(Clone, Debug)]
pub struct FeatureStruct {
    pub properties: Vec<FeatureProperty>,
    pub geometry: FeatureGeometry,
}