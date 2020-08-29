use num_traits::Float;

pub enum FeatureGeometry<'a, F>
where
  F: Float,
{
  Polygon { coordinates: &'a Vec<Vec<[F; 2]>> },
  LineString { coordinates: &'a Vec<[F; 2]> },
}

pub enum FeatureProperty<F>
where
  F: Float,
{
  Circumecenter(Vec<[F; 2]>),
  Length(F),
  Source(F),
  Target(F),
  Urquhart(bool),
}

// Signular veriosn of the struct.
pub struct FeatureStruct<'a, F>
where
  F: Float,
{
  pub properties: Vec<FeatureProperty<F>>,
  pub geometry: FeatureGeometry<'a, F>,
}

// Pluralization of the struct,
pub struct FeaturesStruct<'a, F>
where
  F: Float,
{
  pub properties: Vec<FeatureProperty<F>>,
  pub geometry: Vec<FeatureGeometry<'a, F>>,
}

/// The input data type use in D3
///  Can be special object ( DataObject )
///  or a vector of stuff
///  Null - here a blank.

pub enum DataObject<'a, F>
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
    feature: FeatureStruct<'a, F>,
  },
  /// FeatruesCollection - An array of feature objects.
  FeaturesCollection {
    features: Vec<FeaturesStruct<'a, F>>,
  },
  // A feature containing one of the above geometry objects.
  // Polygon{coordinates: Vec<usize>},
}

pub enum DataType<'a, F>
where
  F: Float,
{
  Object(DataObject<'a, F>),
  Vec(Vec<F>),
  // Float(F),
  Blank,
}
