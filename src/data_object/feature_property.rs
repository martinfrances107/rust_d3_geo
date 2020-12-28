use geo::{Coordinate, CoordinateType, Point};

#[derive(Clone, Debug)]
pub enum FeatureProperty<T>
where
    T: CoordinateType,
{
    Circumecenter(Coordinate<T>),
    Length(T),
    Source(Point<T>),
    Target(Point<T>),
    Urquhart(bool),
    Site(T),
    Sitecoordinates(Point<T>),
    Neighbors(Vec<usize>),
}
