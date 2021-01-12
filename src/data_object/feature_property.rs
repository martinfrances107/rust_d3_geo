use geo::{Coordinate, CoordinateType};
#[derive(Clone, Debug)]
pub enum FeatureProperty<T>
where
    T: CoordinateType,
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
