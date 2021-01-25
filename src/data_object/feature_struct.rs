use super::feature_geometry::FeatureGeometry;
use super::feature_property::FeatureProperty;
use geo::CoordFloat;

// Signular veriosn of the struct.
#[derive(Clone, Debug)]
pub struct FeatureStruct<T: CoordFloat> {
    pub properties: Vec<FeatureProperty<T>>,
    pub geometry: FeatureGeometry<T>,
}
