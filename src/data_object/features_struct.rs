// use super::feature_geometry::FeatureGeometry;
use super::feature_property::FeatureProperty;
use geo::{CoordFloat, Geometry};

// Pluralization of the struct,
#[derive(Clone, Debug)]
pub struct FeaturesStruct<T: CoordFloat> {
    pub properties: Vec<FeatureProperty<T>>,
    pub geometry: Vec<Geometry<T>>,
}
