use super::feature_geometry::FeatureGeometry;
use super::feature_property::FeatureProperty;
use geo::Geometry;

use num_traits::Float;
// Pluralization of the struct,
#[derive(Clone, Debug)]
pub struct FeaturesStruct<T: Float> {
    pub properties: Vec<FeatureProperty>,
    pub geometry: Vec<Geometry<T>>,
}
