use geo::CoordFloat;
use geo::Coordinate;
use geo::Geometry;
use num_traits::FloatConst;

pub mod sphere;

use crate::stream::{Stream, Streamable};

use sphere::Sphere;

#[derive(Clone, Debug)]
pub enum FeatureProperty<T>
where
    T: CoordFloat,
{
    Circumecenter(Coordinate<T>),
    Length(T),
    Source(Coordinate<T>),
    Target(Coordinate<T>),
    Urquhart(bool),
    Site(Coordinate<T>),
    Sitecoordinates(Coordinate<T>),
    Neighbors(Vec<usize>),
}

/// FeatruesCollection - An array of feature objects.
#[derive(Clone, Debug)]
pub struct FeatureCollection<T: CoordFloat>(pub Vec<Features<T>>);

// Signular version of the struct.
#[derive(Clone, Debug)]
pub struct Feature<T: CoordFloat> {
    pub properties: Vec<FeatureProperty<T>>,
    pub geometry: Geometry<T>,
}

// Pluralization of the struct,
#[derive(Clone, Debug)]
pub struct Features<T: CoordFloat> {
    pub properties: Vec<FeatureProperty<T>>,
    pub geometry: Vec<Geometry<T>>,
}

#[derive(Clone, Debug)]
pub enum Collection<T>
where
    T: CoordFloat,
{
    /// Feature - a feature containing one of the above geometry objects.
    Feature { feature: Feature<T> },
}
#[derive(Clone, Debug)]
pub enum DataObject<T>
where
    T: CoordFloat,
{
    Sphere(Sphere<T>),
    Geometry(Geometry<T>),
    Collection(Collection<T>),
}

impl<T> Streamable for DataObject<T>
where
    T: CoordFloat + FloatConst,
{
    type T = T;
    // type SD = DataObject<T>;
    // type SD = Self::SD;
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
