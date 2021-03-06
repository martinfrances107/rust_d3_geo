#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod area_test {

    use std::f64::consts::PI;
    use std::fmt::Display;
    use std::ops::AddAssign;

    use geo::CoordFloat;
    use geo::Coordinate;
    use geo::Geometry;
    use geo::LineString;
    use geo::Polygon;
    use num_traits::AsPrimitive;
    use num_traits::FloatConst;

    use rust_d3_geo::data_object::sphere::Sphere;
    use rust_d3_geo::data_object::DataObject;
    use rust_d3_geo::path::path::Path;
    use rust_d3_geo::path::PathResultEnum;
    use rust_d3_geo::projection::projection::Projection;
    use rust_d3_geo::projection::{
        equirectangular::EquirectangularRaw, projection_mutator::ProjectionMutator,
    };

    #[inline]
    fn equirectangular<
        'a,
        T: AsPrimitive<T> + AddAssign + CoordFloat + Default + Display + FloatConst,
    >() -> ProjectionMutator<EquirectangularRaw<T>, T> {
        EquirectangularRaw::gen_projection_mutator()
            .scale(T::from(900f64 / PI).unwrap())
            .precision(T::zero())
    }

    #[inline]
    fn test_area<'a, T>(
        projection: ProjectionMutator<EquirectangularRaw<T>, T>,
        object: &DataObject<T>,
    ) -> T
    where
        T: AsPrimitive<T> + CoordFloat + FloatConst + Display + AddAssign + Default,
    {
        match Path::generate(Some(projection), None).area(object) {
            Some(p) => match p {
                PathResultEnum::Area(a) => return a,
                _ => panic!("Expecting an area."),
            },
            None => {
                panic!("Expecting an area result.");
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
        let eq = equirectangular::<f64>();
        assert_eq!(test_area(eq, &object), 25.0);
    }

    #[test]
    fn test_polygon_with_holes() {
        println!("geoPath.area(…) of a polygon with holes");
        let object = DataObject::Geometry(Geometry::Polygon(Polygon::new(
            LineString::from(vec![
                Coordinate { x: 100., y: 0. },
                Coordinate { x: 100., y: 1. }, //  [101, 1], [101, 0], [100, 0]
                Coordinate { x: 101., y: 1. },
                Coordinate { x: 101., y: 0. },
                Coordinate { x: 100., y: 0. },
            ]),
            vec![
                // [100.2, 0.2], [100.8, 0.2], [100.8, 0.8], [100.2, 0.8], [100.2, 0.2]
                LineString::from(vec![
                    Coordinate { x: 100.2, y: 0.2 },
                    Coordinate { x: 100.8, y: 0.2 },
                    Coordinate { x: 100.8, y: 0.8 },
                    Coordinate { x: 100.2, y: 0.8 },
                    Coordinate { x: 100.2, y: 0.2 },
                ]),
            ],
        )));
        let eq = equirectangular::<f64>();
        assert_eq!(test_area(eq, &object), 16.0);
    }

    #[test]
    fn test_area_of_a_sphere() {
        println!("geoPath.area(…) of a sphere");
        let eq = equirectangular::<f64>();
        let object = DataObject::Sphere(Sphere {});
        assert_eq!(test_area(eq, &object), 1620000.0);
    }
}
