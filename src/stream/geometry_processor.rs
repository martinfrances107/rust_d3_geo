use num_traits::Float;

use super::line::line;
use super::polygon::polygon;
use crate::data_object::FeatureGeometry;
use crate::stream::Stream;

pub fn processor<F>(geometry: &FeatureGeometry<F>, stream: &mut impl Stream<F>)
where
  F: Float,
{
  match geometry {
    FeatureGeometry::LineString { coordinates, .. } => {
      line(coordinates, stream, 0);
    }
    FeatureGeometry::Polygon { coordinates, .. } => {
      polygon(coordinates, stream);
    }
  }
}
