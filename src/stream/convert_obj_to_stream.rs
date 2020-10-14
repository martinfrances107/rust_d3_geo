use delaunator::Point;

use super::geometry_processor::processor;
use super::Stream;
use crate::data_object::DataObject;
use crate::data_object::FeatureGeometry;
use crate::data_object::FeatureStruct;

pub fn convert_obj_to_stream(object: &DataObject, stream: &mut impl Stream)
{
  match object {
    DataObject::Feature {
      feature: FeatureStruct { geometry, .. },
    } => {
      return processor(&geometry, stream);
    }

    DataObject::FeatureCollection { features } => {
      for f in features {
        for geometry in &f.geometry {
          processor(&geometry, stream);
        }
      }
    }

    DataObject::Polygon { coordinates, .. } => {
      let g = FeatureGeometry::Polygon {
        coordinates: coordinates.to_vec(),
      };
      processor(&g, stream);
    }

    // What remains is a Geometry object.
    DataObject::LineString { coordinates, .. } => {
      let g = FeatureGeometry::LineString {
        coordinates: coordinates.to_vec(),
      };
      processor(&g, stream);
    }

    DataObject::MultiLineString { coordinates: _, .. } => {}

    DataObject::Vec(_) => {
      panic!("Must implemet a method for converting a vec to a stream!");
    }

    DataObject::Blank => {
      panic!("No method of converting blank to stream.");
    }
  }
}
