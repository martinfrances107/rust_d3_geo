use super::feature_geometry::FeatureGeometry;
use super::feature_property::FeatureProperty;

// Pluralization of the struct,
#[derive(Clone, Debug)]
pub struct FeaturesStruct {
    pub properties: Vec<FeatureProperty>,
    pub geometry: Vec<FeatureGeometry>,
}
