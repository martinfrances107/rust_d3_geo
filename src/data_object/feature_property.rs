use delaunator::Point;

#[derive(Clone, Debug)]
pub enum FeatureProperty {
    Circumecenter(Point),
    Length(f64),
    Source(Point),
    Target(Point),
    Urquhart(bool),
    Site(f64),
    Sitecoordinates(Point),
    Neighbors(Vec<usize>),
}
