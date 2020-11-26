// use delaunator::Point;

use super::geometry_processor::processor;
use super::Stream;
use crate::data_object::DataObject;
use crate::data_object::FeatureGeometry;
use crate::data_object::FeatureStruct;

pub fn convert_obj_to_stream(object: &DataObject, stream: &mut impl Stream) {
    match object {
        DataObject::Point { coordinate } => {
            let g = FeatureGeometry::Point {
                coordinate: coordinate.clone(),
            };
            processor(&g, stream);
        }

        DataObject::MultiPoint { coordinates } => {
            for coordinate in coordinates {
                let g = FeatureGeometry::Point {
                    coordinate: coordinate.clone(),
                };
                processor(&g, stream);
            }
        }

        DataObject::Feature {
            feature: FeatureStruct { geometry, .. },
        } => {
            processor(&geometry, stream);
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

        DataObject::MultiPolygon { coordinates, .. } => {
            for c in coordinates {
                let g = FeatureGeometry::Polygon {
                    coordinates: c.to_vec(),
                };
                processor(&g, stream);
            }
        }

        DataObject::GeometryCollection { geometries } => {
            for g in geometries {
                processor(&g, stream);
            }
        }

        DataObject::LineString { coordinates, .. } => {
            let g = FeatureGeometry::LineString {
                coordinates: coordinates.to_vec(),
            };
            processor(&g, stream);
        }

        DataObject::MultiLineString { coordinates, .. } => {
            for coordinate in coordinates {
                let g = FeatureGeometry::LineString {
                    coordinates: coordinate.to_vec(),
                };
                processor(&g, stream);
            }
        }

        DataObject::Vec(_) => {
            unimplemented!("Must implement a method for converting a vec to a stream!");
        }

        DataObject::Blank => {
            unimplemented!("No method of converting blank to stream.");
        }
    }
}
