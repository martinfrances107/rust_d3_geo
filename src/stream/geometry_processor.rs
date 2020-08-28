use num_traits::Float;

use super::line::line;
use crate::data_object::FeatureGeometry;
use crate::stream::Stream;

pub fn processor<F>(geometry: &FeatureGeometry<F>, stream: &mut impl Stream<F>)
where
  F: Float,
{
  match geometry {
    // DataObject::Sphere { coordinates, .. } => {
    //   line(coordinates[i], stream, false);
    //   stream.sphere();
    // }
    // DataObject::Point { coordinates } => {
    //   stream.point(coordinates[0], coordinates[1], coordinates[2]);
    // }
    // DataObject::MultiPoint { coordinates, .. } => {
    //   // var coordinates = object.coordinates, i = -1, n = coordinates.length;
    //   // while (++i < n) object = coordinates[i], stream.point(object[0], object[1], object[2]);
    //   // TODO must reverse.
    //   for c in coordinates {
    //     stream.point(c[0], c[1], Some(c[2]));
    //   }
    // }
    FeatureGeometry::LineString { coordinates, .. } => {
      line(coordinates, stream, 0);
    }
    FeatureGeometry::Polygon {
      coordinates: _c, ..
    } => {} // DataObject::MultiLineString { coordinates, .. } => {
            //   // var coordinates = object.coordinates, i = -1, n = coordinates.length;
            //   // while (++i < n) stream_line(coordinates[i], stream, 0);
            //   // TODO must reverse.
            //   for c in coordinates {
            //     line(c, stream, false);
            //   }
            // }
            // DataObject::Polygon { coordinates, .. } => {
            //   stream_polygon(coordinates, stream);
            // }
            // DataObject::MultiPolygon { coordinates, .. } => {
            //   // var coordinates = object.coordinates, i = -1, n = coordinates.length;
            //   for c in coordinates {
            //     polygon(c, stream);
            //   }
            //   // while (++i < n) stream_polygon(coordinates[i], stream);
            // }
            // GeometryCollection: function(object, stream) {
            //   var geometries = object.geometries, i = -1, n = geometries.length;
            //   while (++i < n) processor(geometries[i], stream);
            // }
            // };
  }
}
