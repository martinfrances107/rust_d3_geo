use delaunator::Point;

#[derive(Clone, Debug)]
pub enum FeatureGeometry {
    Point { coordinate: Point },
    MultiPoint { coordinates: Vec<Point> },
    Polygon { coordinates: Vec<Vec<Point>> },
    LineString { coordinates: Vec<Point> },
}

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

// Signular veriosn of the struct.
#[derive(Clone, Debug)]
pub struct FeatureStruct {
    pub properties: Vec<FeatureProperty>,
    pub geometry: FeatureGeometry,
}

// Pluralization of the struct,
#[derive(Clone, Debug)]
pub struct FeaturesStruct {
    pub properties: Vec<FeatureProperty>,
    pub geometry: Vec<FeatureGeometry>,
}

/// The input data type use in D3
///  Can be special object ( DataObject )
///  or a vector of stuff
///  Null - here a blank.
#[derive(Clone, Debug)]
pub enum DataObject {
    /// Point - a single position.
    Point {
        coordinate: Point,
    },
    ///  MultiPoint - an array of positions.
    MultiPoint {
        coordinates: Vec<Point>,
    },
    ///  LineString - an array of positions forming a continuous line.
    LineString {
        coordinates: Vec<Point>,
    },
    /// MultiLineString - an array of arrays of positions forming several lines.
    MultiLineString {
        coordinates: Vec<Vec<Point>>,
    },
    /// Polygon - an array of arrays of positions forming a polygon (possibly with holes).
    Polygon {
        coordinates: Vec<Vec<Point>>,
    },
    /// MultiPolygon - a multidimensional array of positions forming multiple polygons.
    MultiPolygon {
        coordinates: Vec<Vec<Vec<Point>>>,
    },
    /// GeometryCollection - an array of geometry objects.
    GeometryCollection {
        geometries: Vec<FeatureGeometry>,
    },
    /// Feature - a feature containing one of the above geometry objects.
    Feature {
        feature: FeatureStruct,
    },
    /// FeatruesCollection - An array of feature objects.
    FeatureCollection {
        features: Vec<FeaturesStruct>,
    },

    Vec(Vec<Point>),

    Blank,
}
