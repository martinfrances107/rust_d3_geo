#[cfg(not(tarpaulin_include))]
#[cfg(test)]

mod equirectangular_test {

    use geo::Coordinate;
    // use rust_d3_geo::path::Path;
    use rust_d3_geo::projection::projection::Projection;
    use rust_d3_geo::projection::projection_equal::projection_equal;
    use rust_d3_geo::projection::{
        equirectangular::EquirectangularRaw, projection_mutator::ProjectionMutator,
    };

    // function testArea(projection, object) {
    //   return d3_geo.geoPath()
    //        .projection(projection)
    //       .area(object);
    // }

    fn test_area(projection: ProjectionMutator<f64>) {
        // return Path()
    }
    #[test]
    fn test_polygon_with_no_holes() {
        // let mut equirectangular = EquirectangularRaw::gen_projection_mutator();
        // equirectangular.scale(Some(&(900f64 / 3.14)));
        // equirectangular.precision(&0);
    }
}

// tape("geoPath.area(…) of a polygon with no holes", function(test) {
//   test.equal(testArea(equirectangular, {
//     type: "Polygon",
//     coordinates: [[[100, 0], [100, 1], [101, 1], [101, 0], [100, 0]]]
//   }), 25);
//   test.end();
// });

// tape("geoPath.area(…) of a polygon with holes", function(test) {
//   test.equal(testArea(equirectangular, {
//     type: "Polygon",
//     coordinates: [[[100, 0], [100, 1], [101, 1], [101, 0], [100, 0]], [[100.2, 0.2], [100.8, 0.2], [100.8, 0.8], [100.2, 0.8], [100.2, 0.2]]]
//   }), 16);
//   test.end();
// });

// tape("geoPath.area(…) of a sphere", function(test) {
//   test.equal(testArea(equirectangular, {
//     type: "Sphere",
//   }), 1620000);
//   test.end();
// });
