#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod path_centroid_test {

    extern crate pretty_assertions;

    use std::cell::RefCell;
    use std::f64::consts::PI;
    use std::fmt::Display;
    use std::ops::AddAssign;
    use std::rc::Rc;

    use approx::AbsDiff;
    use approx::AbsDiffEq;
    use geo::CoordFloat;
    use geo::Coordinate;
    use geo::LineString;
    use geo::Point;
    use geo::Polygon;
    use num_traits::AsPrimitive;
    use num_traits::FloatConst;
    use pretty_assertions::assert_eq;
    use rust_d3_geo::path::path::Path;

    use geo::line_string;
    use geo::Geometry;

    use rust_d3_geo::clip::antimeridian::gen_clip_factory_antimeridian;
    use rust_d3_geo::clip::antimeridian::pv::PV;
    use rust_d3_geo::data_object::DataObject;
    use rust_d3_geo::in_delta::in_delta_point;
    use rust_d3_geo::path::builder::Builder as PathBuilder;
    use rust_d3_geo::path::centroid::Centroid;
    use rust_d3_geo::path::context_stream::ContextStream;
    use rust_d3_geo::path::ResultEnum;
    use rust_d3_geo::projection::builder::Builder as ProjectionBuilder;
    use rust_d3_geo::projection::builder::Builder;
    use rust_d3_geo::projection::equirectangular::EquirectangularRaw;
    use rust_d3_geo::projection::projection::Projection;
    use rust_d3_geo::projection::Precision;
    use rust_d3_geo::projection::Scale;
    use rust_d3_geo::stream::Stream;

    #[inline]
    fn equirectangular<DRAIN, T>() -> Rc<Projection<DRAIN, EquirectangularRaw<DRAIN, T>, PV<T>, T>>
    where
        DRAIN: Stream<T = T> + Default,
        T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        Rc::new(
            ProjectionBuilder::new(
                gen_clip_factory_antimeridian(),
                EquirectangularRaw::default(),
            )
            .scale(T::from(900f64 / PI).unwrap())
            .precision(&T::zero())
            .build(),
        )
    }

    #[inline]
    fn test_centroid<'a, DRAIN, T>(
        projection: Rc<Projection<ContextStream<T>, EquirectangularRaw<DRAIN, T>, PV<T>, T>>,
        object: &DataObject<T>,
    ) -> Point<T>
    where
        DRAIN: Stream<T = T>,
        T: AddAssign<T>
            + AbsDiffEq<Epsilon = T>
            + AsPrimitive<T>
            + CoordFloat
            + Display
            + FloatConst,
    {
        let cs = Rc::new(RefCell::new(ContextStream::Centroid(Centroid::default())));
        let result = Path::new(cs, projection).centroid(object);
        match result {
            Some(r) => match r {
                ResultEnum::Centroid(c) => Point(c),
                _ => {
                    panic!("Failed to return a centroid");
                }
            },
            None => {
                panic!("Failed to return a result.");
            }
        }
    }

    #[test]
    fn a_set_of_line_strings_is_the_spherical_average_of_its_great_arc_segments() {
        println!("geoPath.centroid(…) of a point");
        let point = DataObject::Geometry(Geometry::Point(Point(Coordinate { x: 0_f64, y: 0_f64 })));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &point),
            Point::new(480_f64, 250_f64),
            1e-6_f64
        ));
    }
}
