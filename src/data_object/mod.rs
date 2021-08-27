/// Related to the unit sphere.
pub mod sphere;

use geo::CoordFloat;
use geo::Coordinate;
use geo::Geometry;
use num_traits::FloatConst;

use crate::stream::{Stream, Streamable};

use sphere::Sphere;

/// Specifies a valie feature property.
#[derive(Clone, Debug)]
pub enum FeatureProperty<T>
where
    T: CoordFloat,
{
    /// Circumcenter.
    Circumecenter(Coordinate<T>),
    /// Length.
    Length(T),
    /// Source.
    Source(Coordinate<T>),
    /// Target.
    Target(Coordinate<T>),
    /// Urquhart distances.
    Urquhart(bool),
    /// Site Coordinate.
    Site(Coordinate<T>),
    /// Sites Coordinates.
    Sitecoordinates(Coordinate<T>),
    ///  A collection of indexes.
    Neighbors(Vec<usize>),
}

/// FeatruesCollection - An array of feature objects.
#[derive(Clone, Debug)]
pub struct FeatureCollection<T: CoordFloat>(pub Vec<Features<T>>);

/// Signular version of the struct.
#[derive(Clone, Debug)]
pub struct Feature<T: CoordFloat> {
    /// A collection of feature properties.
    pub properties: Vec<FeatureProperty<T>>,
    /// The associated gemetry function.
    pub geometry: Geometry<T>,
}

/// Pluralization of the struct,
#[derive(Clone, Debug)]
pub struct Features<T: CoordFloat> {
    /// A collection of feature properties.
    pub properties: Vec<FeatureProperty<T>>,
    /// A collections of assocated geometries.
    pub geometry: Vec<Geometry<T>>,
}

#[derive(Clone, Debug)]
/// A collection of features.
pub enum Collection<T>
where
    T: CoordFloat,
{
    /// Feature - a feature containing one of the above geometry objects.
    Feature {
        /// The feature.
        feature: Feature<T>,
    },
}

/// Related to D3 data objects.
#[derive(Clone, Debug)]
pub enum DataObject<T>
where
    T: CoordFloat,
{
    /// D3 sphere
    Sphere(Sphere<T>),
    /// D3 geometry
    Geometry(Geometry<T>),
    /// A D3 geometry collect.
    Collection(Collection<T>),
}

impl<T> Streamable for DataObject<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;
    fn to_stream<SD: Stream<T = T>>(&self, stream: &mut SD) {
        match self {
            DataObject::Collection(Collection::Feature { feature: _ }) => {
                todo!("fixme");
            }
            DataObject::Geometry(g) => g.to_stream(stream),
            DataObject::Sphere(s) => s.to_stream(stream),
        }
    }
}
