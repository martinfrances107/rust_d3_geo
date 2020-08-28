use num_traits::Float;

use super::geometry_processor::processor;
use super::Stream;
use crate::data_object::DataObject;
use crate::data_object::FeatureGeometry;
use crate::data_object::FeatureStruct;

pub fn convert_obj_to_stream<F>(object: &DataObject<F>, stream: &mut impl Stream<F>)
where
  F: Float,
{
  match object {
    DataObject::Feature(FeatureStruct { geometry, .. }) => {
      return processor(&geometry, stream);
    }
    DataObject::FeaturesCollection { features } => {
      for geometry in &features.geometry {
        processor(&geometry, stream);
      }
    }

    // What remains is a Geometry object.
    DataObject::LineString { coordinates, .. } => {
      let g = FeatureGeometry::LineString {
        coordinates: coordinates,
      };
      processor(&g, stream);
    }
    DataObject::MultiLineString { coordinates: _, .. } => {}
  }
}
