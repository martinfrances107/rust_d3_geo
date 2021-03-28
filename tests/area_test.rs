// #[cfg(not(tarpaulin_include))]
// #[cfg(test)]

mod area_test {

    use geo::CoordFloat;
    use geo::Coordinate;
    use geo::Geometry;
    use geo::LineString;
    use geo::Polygon;
    use num_traits::FloatConst;
    use rust_d3_geo::data_object::DataObject;
    use rust_d3_geo::projection::projection::Projection;
    // use rust_d3_geo::projection::projection_equal::projection_equal;
    use rust_d3_geo::path::Path;
    use rust_d3_geo::path::PathResultEnum;
    use rust_d3_geo::projection::{
        equirectangular::EquirectangularRaw, projection_mutator::ProjectionMutator,
    };
    // use  projection::equirectangular;
    use std::f64::consts::PI;
    use std::fmt::Display;
    use std::ops::AddAssign;

    #[inline]
    fn equirectangular<'a, T: AddAssign + CoordFloat + FloatConst + Default>(
    ) -> ProjectionMutator<T> {
        EquirectangularRaw::gen_projection_mutator()
            .scale(T::from(900f64 / PI).unwrap())
            .precision(T::zero())
    }

    #[inline]
    fn test_area<'a, T>(projection: ProjectionMutator<T>, object: &DataObject<T>) -> T
    where
        T: CoordFloat + FloatConst + Display + AddAssign + Default,
    {
        match Path::generate(projection, None).area(object) {
            Some(p) => match p {
                PathResultEnum::Area(a) => return a,
                _ => panic!("expecting an area"),
            },
            None => {
                panic!("expecting a result");
            }
        }
    }

    #[test]
    fn test_polygon_with_no_holes() {
        println!("geoPath.area(…) of a polygon with no holes");
        let object = DataObject::Geometry(Geometry::Polygon(Polygon::new(
            LineString::from(vec![
                Coordinate { x: 100., y: 0. },
                Coordinate { x: 100., y: 1. }, //  [101, 1], [101, 0], [100, 0]
                Coordinate { x: 101., y: 1. },
                Coordinate { x: 101., y: 0. },
                Coordinate { x: 100., y: 0. },
            ]),
            vec![],
        )));
        let ep = equirectangular::<f64>();
        // assert_eq!(test_area(ep, &object), 4.0);
    }
}

// // tape("geoPath.area(…) of a polygon with no holes", function(test) {
// //   test.equal(testArea(equirectangular, {
// //     type: "Polygon",
// //     coordinates: [[[100, 0], [100, 1], [101, 1], [101, 0], [100, 0]]]
// //   }), 25);
// //   test.end();
// // });

// // tape("geoPath.area(…) of a polygon with holes", function(test) {
// //   test.equal(testArea(equirectangular, {
// //     type: "Polygon",
// //     coordinates: [[[100, 0], [100, 1], [101, 1], [101, 0], [100, 0]], [[100.2, 0.2], [100.8, 0.2], [100.8, 0.8], [100.2, 0.8], [100.2, 0.2]]]
// //   }), 16);
// //   test.end();
// // });

// // tape("geoPath.area(…) of a sphere", function(test) {
// //   test.equal(testArea(equirectangular, {
// //     type: "Sphere",
// //   }), 1620000);
// //   test.end();
// // });
