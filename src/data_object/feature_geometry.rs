use delaunator::Point;

#[derive(Clone, Debug)]
pub enum FeatureGeometry {
    Point { coordinate: Point },
    MultiPoint { coordinates: Vec<Point> },
    Polygon { coordinates: Vec<Vec<Point>> },
    LineString { coordinates: Vec<Point> },
}
