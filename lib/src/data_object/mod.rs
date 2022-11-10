/// Related to the unit sphere.
pub mod sphere;

use std::fmt::Debug;

use geo::CoordFloat;
use geo::Coordinate;
use geo::Geometry;

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

/// An array of feature objects.
#[derive(Clone, Debug)]
pub struct FeatureCollection<T: CoordFloat>(pub Vec<Features<T>>);

/// Signular version of the struct.
#[derive(Clone, Debug)]
pub struct Feature<T>
where
    T: CoordFloat,
{
    /// A collection of feature properties.
    pub properties: Vec<FeatureProperty<T>>,
    /// The associated gemetry function.
    pub geometry: Geometry<T>,
}

/// Pluralization of the struct,
#[derive(Clone, Debug)]
pub struct Features<T>
where
    T: CoordFloat,
{
    /// A collection of feature properties.
    pub properties: Vec<FeatureProperty<T>>,
    /// A collections of associated geometries.
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
