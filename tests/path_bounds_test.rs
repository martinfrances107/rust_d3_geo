#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod bounds_test {

    use std::cell::RefCell;
    use std::f64::consts::PI;
    use std::fmt::Display;
    use std::ops::AddAssign;
    use std::rc::Rc;

    use approx::AbsDiffEq;
    use geo::CoordFloat;
    use geo::Coordinate;
    use geo::Geometry;
    use geo::LineString;
    use geo::Polygon;
    use num_traits::AsPrimitive;
    use num_traits::FloatConst;
    use pretty_assertions::assert_eq;

    use rust_d3_geo::clip::antimeridian::pv::PV;
    use rust_d3_geo::data_object::sphere::Sphere;
    use rust_d3_geo::data_object::DataObject;
    use rust_d3_geo::path::builder::Builder;
    use rust_d3_geo::path::context_stream::ContextStream;
    use rust_d3_geo::path::ResultEnum;
    use rust_d3_geo::projection::equirectangular::EquirectangularRaw;
    use rust_d3_geo::projection::projection::Projection;
    use rust_d3_geo::projection::Precision;
    use rust_d3_geo::projection::Raw as ProjectionRaw;
    use rust_d3_geo::projection::Scale;

    #[inline]
    fn equirectangular<
        T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + AddAssign + CoordFloat + Display + FloatConst,
    >() -> Rc<Projection<ContextStream<T>, EquirectangularRaw<ContextStream<T>, T>, PV<T>, T>> {
        Rc::new(
            EquirectangularRaw::builder()
                .scale(T::from(900f64 / PI).unwrap())
                .precision(&T::zero())
                .build(),
        )
    }

    #[inline]
    fn test_bounds<'a, T>(
        projection: Rc<
            Projection<ContextStream<T>, EquirectangularRaw<ContextStream<T>, T>, PV<T>, T>,
        >,
        object: &DataObject<T>,
    ) -> [Coordinate<T>; 2]
    where
        T: AbsDiffEq<Epsilon = T>
            + AsPrimitive<T>
            + CoordFloat
            + FloatConst
            + Display
            + AddAssign
            + Default,
    {
        let cs = Rc::new(RefCell::new(ContextStream::default()));
        match Builder::new(cs).build(projection).bounds(object) {
            Some(p) => match p {
                ResultEnum::Bounds(b) => return b,
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
        assert_eq!(
            test_bounds(eq, &object),
            [
                Coordinate {
                    x: 980_f64,
                    y: 245_f64
                },
                Coordinate {
                    x: 985_f64,
                    y: 250_f64
                }
            ]
        );
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
        assert_eq!(
            test_bounds(eq, &object),
            [
                Coordinate {
                    x: 980_f64,
                    y: 245_f64
                },
                Coordinate {
                    x: 985_f64,
                    y: 250_f64
                }
            ]
        );
    }

    #[test]
    fn test_area_of_a_sphere() {
        println!("geoPath.area(…) of a sphere");
        let eq = equirectangular::<f64>();
        let object = DataObject::Sphere(Sphere::default());
        assert_eq!(
            test_bounds(eq, &object),
            [
                Coordinate {
                    x: -420_f64,
                    y: -200_f64
                },
                Coordinate {
                    x: 1380_f64,
                    y: 700_f64
                }
            ]
        );
    }
}
