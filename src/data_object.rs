use num_traits::Float;

#[derive(Debug)]
pub enum FeatureGeometry<F>
where
  F: Float,
{
  Polygon { coordinates: Vec<Vec<[F; 2]>> },
  LineString { coordinates: Vec<[F; 2]> },
}

#[derive(Debug)]
pub enum FeatureProperty<F>
where
  F: Float,
{
  Circumecenter([F; 2]),
  Length(F),
  Source(F),
  Target(F),
  Urquhart(bool),
  Site(F),
  Sitecoordinates([F;2]),
  Neighbors(Vec<usize>),
}

// Signular veriosn of the struct.
#[derive(Debug)]
pub struct FeatureStruct<F>
where
  F: Float,
{
  pub properties: Vec<FeatureProperty<F>>,
  pub geometry: FeatureGeometry<F>,
}

// Pluralization of the struct,
#[derive(Debug)]
pub struct FeaturesStruct<F>
where
  F: Float,
{
  pub properties: Vec<FeatureProperty<F>>,
  pub geometry: Vec<FeatureGeometry<F>>,
}

/// The input data type use in D3
///  Can be special object ( DataObject )
///  or a vector of stuff
///  Null - here a blank.
#[derive(Debug)]
pub enum DataObject<F>
where
  F: Float,
{
  //   * Point - a single position.
  // * MultiPoint - an array of positions.
  // * LineString - an array of positions forming a continuous line.
  LineString {
    coordinates: Vec<[F; 2]>,
  },
  /// MultiLineString - an array of arrays of positions forming several lines.
  MultiLineString {
    coordinates: Vec<Vec<[F; 2]>>,
  },
  // * Polygon - an array of arrays of positions forming a polygon (possibly with holes).
  Polygon {
    coordinates: Vec<Vec<[F; 2]>>,
  },
  // * MultiPolygon - a multidimensional array of positions forming multiple polygons.
  // * GeometryCollection - an array of geometry objects.
  /// Feature - a feature containing one of the above geometry objects.
  Feature {
    feature: FeatureStruct<F>,
  },
  /// FeatruesCollection - An array of feature objects.
  FeatureCollection {
    features: Vec<FeaturesStruct<F>>,
  },
  // A feature containing one of the above geometry objects.
  // Polygon{coordinates: Vec<usize>},
}

#[derive(Debug)]
pub enum DataType<F>
where
  F: Float,
{
  Object(DataObject<F>),
  Vec(Vec<F>),
  // Float(F),
  Blank,
}
