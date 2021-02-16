#[cfg(not(tarpaulin_include))]
#[cfg(test)]

mod equirectangular_test {

    use geo::Geometry;
    use geo::{CoordFloat, Coordinate, LineString, Polygon};
    use num_traits::FloatConst;
    use rust_d3_geo::data_object::DataObject;
    use rust_d3_geo::projection::projection::Projection;
    use rust_d3_geo::projection::projection_equal::projection_equal;
    use rust_d3_geo::projection::{
        equirectangular::EquirectangularRaw, projection_mutator::ProjectionMutator,
    };
    use rust_d3_geo::{path::Path, projection::equirectangular};
    use std::f64::consts::PI;
    use std::fmt::Display;
    use std::ops::AddAssign;

    fn equirectangular() -> ProjectionMutator<'static, f64> {
        let mut pm = EquirectangularRaw::<f64>::gen_projection_mutator();
        pm.scale(900f64 / PI);
        pm.precision(0f64);
        pm
    }

    #[inline]
    fn test_area<'a, T: 'static>(
        projection: Option<ProjectionMutator<'a, T>>,
        object: &DataObject<T>,
    ) -> T
    where
        T: CoordFloat + FloatConst + Display + AddAssign,
    {
        Path::projection(projection).area(object)
    }

    // #[test]
    // fn test_polygon_with_no_holes() {
    //     println!("geoPath.area(…) of a polygon with no holes");
    //     let object = DataObject::Geometry(Geometry::Polygon(Polygon::new(
    //         LineString::from(vec![
    //             Coordinate { x: 100., y: 0. },
    //             Coordinate { x: 100., y: 1. }, //  [101, 1], [101, 0], [100, 0]
    //         ]),
    //         vec![],
    //     )));
    //     let ep = equirectangular();
    //     assert_eq!(test_area(Some(ep), &object), 4.0);
    // }
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
