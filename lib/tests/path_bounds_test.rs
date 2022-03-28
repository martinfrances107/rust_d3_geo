#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod path_bounds_test {

    use std::f64::consts::PI;
    use std::fmt::Display;
    use std::ops::AddAssign;

    use approx::AbsDiffEq;
    use geo::CoordFloat;
    use geo::Coordinate;
    use geo::Geometry;
    use geo::LineString;
    use geo::Polygon;
    use num_traits::AsPrimitive;
    use num_traits::FloatConst;
    use pretty_assertions::assert_eq;

    // use rust_d3_geo::clip::antimeridian::line::Line;
    // use rust_d3_geo::clip::antimeridian::pv::PV;
    use rust_d3_geo::clip::buffer::Buffer;
    use rust_d3_geo::data_object::sphere::Sphere;
    use rust_d3_geo::identity::Identity;
    use rust_d3_geo::path::bounds::Bounds;
    use rust_d3_geo::path::builder::Builder;
    use rust_d3_geo::projection::equirectangular::Equirectangular;
    use rust_d3_geo::projection::projector::Projector;
    use rust_d3_geo::projection::PrecisionBypass;
    use rust_d3_geo::projection::ProjectionRawBase;
    // use rust_d3_geo::projection::builder::template::ResampleNoClipC;
    // use rust_d3_geo::projection::builder::template::ResampleNoClipU;
    use rust_d3_geo::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
    use rust_d3_geo::clip::antimeridian::line::Line as LineAntimeridian;
    use rust_d3_geo::clip::antimeridian::pv::PV as PVAntimeridian;
    use rust_d3_geo::projection::builder::template::ResampleNoneNoClipC;
    use rust_d3_geo::projection::builder::template::ResampleNoneNoClipU;

    use rust_d3_geo::projection::ScaleSet;
    use rust_d3_geo::stream::Connected;
    use rust_d3_geo::stream::Streamable;
    use rust_d3_geo::stream::Unconnected;

    #[inline]
    fn equirectangular<
        T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + AddAssign + CoordFloat + Display + FloatConst,
    >() -> Projector<
        Bounds<T>,
        InterpolateAntimeridian<
            Bounds<T>,
            ResampleNoneNoClipC<Bounds<T>, Equirectangular<Bounds<T>, T>, T>,
            T,
        >,
        LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
        LineAntimeridian<
            Bounds<T>,
            ResampleNoneNoClipC<Bounds<T>, Equirectangular<Bounds<T>, T>, T>,
            Connected<ResampleNoneNoClipC<Bounds<T>, Equirectangular<Bounds<T>, T>, T>>,
            T,
        >,
        LineAntimeridian<
            Bounds<T>,
            ResampleNoneNoClipC<Bounds<T>, Equirectangular<Bounds<T>, T>, T>,
            Unconnected,
            T,
        >,
        Identity<Bounds<T>, Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
        Identity<Bounds<T>, Bounds<T>, Bounds<T>, Unconnected, T>,
        Equirectangular<Bounds<T>, T>,
        PVAntimeridian<T>,
        ResampleNoneNoClipC<Bounds<T>, Equirectangular<Bounds<T>, T>, T>,
        ResampleNoneNoClipU<Bounds<T>, Equirectangular<Bounds<T>, T>, T>,
        T,
    > {
        Equirectangular::builder()
            .scale(T::from(900f64 / PI).unwrap())
            .precision_bypass()
            .build()
    }

    #[inline]
    fn test_bounds<'a, T>(
        projection: Projector<
            Bounds<T>,
            InterpolateAntimeridian<
                Bounds<T>,
                ResampleNoneNoClipC<Bounds<T>, Equirectangular<Bounds<T>, T>, T>,
                T,
            >,
            LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
            LineAntimeridian<
                Bounds<T>,
                ResampleNoneNoClipC<Bounds<T>, Equirectangular<Bounds<T>, T>, T>,
                Connected<ResampleNoneNoClipC<Bounds<T>, Equirectangular<Bounds<T>, T>, T>>,
                T,
            >,
            LineAntimeridian<
                Bounds<T>,
                ResampleNoneNoClipC<Bounds<T>, Equirectangular<Bounds<T>, T>, T>,
                Unconnected,
                T,
            >,
            Identity<Bounds<T>, Bounds<T>, Bounds<T>, Connected<Bounds<T>>, T>,
            Identity<Bounds<T>, Bounds<T>, Bounds<T>, Unconnected, T>,
            Equirectangular<Bounds<T>, T>,
            PVAntimeridian<T>,
            ResampleNoneNoClipC<Bounds<T>, Equirectangular<Bounds<T>, T>, T>,
            ResampleNoneNoClipU<Bounds<T>, Equirectangular<Bounds<T>, T>, T>,
            T,
        >,

        object: &impl Streamable<T = T>,
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
        let cs = Bounds::default();
        Builder::new(cs).build(projection).bounds(object)
    }

    #[test]
    fn polygon_with_no_holes() {
        println!("geoPath.area(…) of a polygon with no holes");
        let object = Geometry::Polygon(Polygon::new(
            LineString::from(vec![
                Coordinate { x: 100., y: 0. },
                Coordinate { x: 100., y: 1. }, //  [101, 1], [101, 0], [100, 0]
                Coordinate { x: 101., y: 1. },
                Coordinate { x: 101., y: 0. },
                Coordinate { x: 100., y: 0. },
            ]),
            vec![],
        ));
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
    fn polygon_with_holes() {
        println!("geoPath.area(…) of a polygon with holes");
        let object = Geometry::Polygon(Polygon::new(
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
        ));
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
    fn area_of_a_sphere() {
        println!("geoPath.area(…) of a sphere");
        let eq = equirectangular::<f64>();
        let object = Sphere::default();
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
