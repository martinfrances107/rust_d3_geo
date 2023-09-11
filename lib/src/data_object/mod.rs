/// Related to the unit sphere.
pub mod sphere;

use core::fmt::Debug;

use geo::CoordFloat;
use geo::Geometry;
use geo_types::Coord;

/// Part of a Feature Object.
#[derive(Clone, Debug)]
pub enum FeatureProperty<T>
where
    T: CoordFloat,
{
    /// Circumcenter.
    Circumecenter(Coord<T>),
    /// Length.
    Length(T),
    /// Source.
    Source(Coord<T>),
    /// Target.
    Target(Coord<T>),
    /// Urquhart distances.
    Urquhart(bool),
    /// Site Coordinate.
    Site(Coord<T>),
    /// Sites Coordinates.
    Sitecoordinates(Coord<T>),
    ///  A collection of indexes.
    Neighbors(Vec<usize>),
}

/// An grouping  of [Features].
#[derive(Clone, Debug)]
pub struct FeatureCollection<T: CoordFloat>(pub Vec<Features<T>>);

/// Binds groups of ['FeatureProperty'] to a ['Geomemtry'] object.
#[derive(Clone, Debug)]
pub struct Feature<T>
where
    T: CoordFloat,
{
    /// A collection of feature properties.
    pub properties: Vec<FeatureProperty<T>>,
    /// The associated geometry function.
    pub geometry: Geometry<T>,
}

/// Binds groups of [FeatureProperty] to a grouping of [Geometry] object.
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
/// A collection of [Features].
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
