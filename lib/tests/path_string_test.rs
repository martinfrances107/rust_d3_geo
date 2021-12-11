#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod path_string_test {

    use std::f64::consts::PI;
    use std::fmt::Display;
    use std::ops::AddAssign;
    use std::rc::Rc;

    use approx::AbsDiffEq;
    use geo::line_string;
    use geo::point;
    use geo::CoordFloat;
    use geo::Geometry;
    use geo::GeometryCollection;
    use geo::Polygon;
    use num_traits::AsPrimitive;
    use num_traits::FloatConst;
    use pretty_assertions::assert_eq;

    use rust_d3_geo::clip::antimeridian::line::Line;
    use rust_d3_geo::clip::antimeridian::pv::PV;
    use rust_d3_geo::data_object::DataObject;
    use rust_d3_geo::path::builder::Builder as PathBuilder;
    use rust_d3_geo::path::context_stream::ContextStream;
    use rust_d3_geo::path::ResultEnum;
    use rust_d3_geo::projection::equirectangular::EquirectangularRaw;
    use rust_d3_geo::projection::projection::Projection;
    use rust_d3_geo::projection::Precision;
    use rust_d3_geo::projection::Raw;
    use rust_d3_geo::projection::Scale;
    use rust_d3_geo::stream::Stream;

    #[inline]
    fn equirectangular<DRAIN, T>(
    ) -> Projection<DRAIN, Line<T>, EquirectangularRaw<DRAIN, T>, PV<T>, T>
    where
        DRAIN: Stream<EP = DRAIN, T = T> + Default,
        T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        EquirectangularRaw::builder()
            .scale(T::from(900f64 / PI).unwrap())
            .precision(&T::zero())
            .build()
    }

    #[inline]
    fn test_path<'a, DRAIN, T>(
        projection: Projection<ContextStream<T>, Line<T>, EquirectangularRaw<DRAIN, T>, PV<T>, T>,
        object: DataObject<T>,
    ) -> String
    where
        DRAIN: Stream<EP = DRAIN, T = T>,
        T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        // let stream_dst = Rc::new(RefCell::new(ContextStream::S(PathString::default())));
        let builder = PathBuilder::context_pathstring();
        let string = builder.build(projection).object(&object);
        match string {
            Some(p) => match p {
                ResultEnum::String(s) => return s,
                _ => panic!("Expecting a string."),
            },
            None => {
                panic!("Expecting an string result.");
            }
        }
    }

    #[test]
    fn point_renders_a_point() {
        println!("geoPath.point(…) renders a point");
        let object = DataObject::Geometry(Geometry::Point(point!(x: -63_f64, y:18_f64)));
        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert_eq!(
            test_path(eq, object),
            "M165,160m0,4.5a4.5,4.5 0 1,1 0,-9a4.5,4.5 0 1,1 0,9z"
        );
    }

    #[test]
    fn point_renders_a_point_of_given_radius() {
        println!("geoPath.point(…) renders a point of a given radius");

        let builder: PathBuilder<
            Line<f64>,
            EquirectangularRaw<ContextStream<f64>, f64>,
            PV<f64>,
            f64,
        > = PathBuilder::context_pathstring().point_radius(10_f64);

        let eq = equirectangular::<ContextStream<f64>, f64>();
        let mut path = builder.build(eq);
        let object = DataObject::Geometry(Geometry::Point(point!(x: -63_f64, y:18_f64)));

        let result = path.object(&object);
        match result {
            Some(ResultEnum::String(result)) => {
                assert_eq!(result, "M165,160m0,10a10,10 0 1,1 0,-20a10,10 0 1,1 0,20z");
            }
            Some(_) => {
                panic!("was expecting a String result")
            }
            None => {
                panic!("was expecting a result");
            }
        };
    }

    #[test]
    fn renders_multipoint() {
        println!("geoPath(MultiPoint) renders a point");
        let object = DataObject::Geometry(Geometry::MultiPoint(
            vec![
                point![x:-63_f64, y:18_f64],
                point![x:-62_f64, y:18_f64],
                point![x:-62_f64, y:17_f64],
            ]
            .into(),
        ));
        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert_eq!(test_path(eq, object),
			"M165,160m0,4.5a4.5,4.5 0 1,1 0,-9a4.5,4.5 0 1,1 0,9zM170,160m0,4.5a4.5,4.5 0 1,1 0,-9a4.5,4.5 0 1,1 0,9zM170,165m0,4.5a4.5,4.5 0 1,1 0,-9a4.5,4.5 0 1,1 0,9z");
    }

    #[test]
    fn renders_a_string() {
        let object = DataObject::Geometry(Geometry::LineString(line_string![
            (x:-63_f64, y:18_f64), (x:-62_f64, y:18_f64), (x:-62_f64, y:17_f64)
        ]));
        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert_eq!(test_path(eq, object), "M165,160L170,160L170,165");
    }

    #[test]
    fn renders_a_polygon() {
        let object = DataObject::Geometry(Geometry::Polygon(Polygon::new(
            line_string![
                (x:-63_f64, y:18_f64), (x:-62_f64, y:18_f64), (x:-62_f64, y:17_f64)
            ],
            vec![],
        )));
        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert_eq!(test_path(eq, object), "M165,160L170,160L170,165Z");
    }

    #[test]
    fn renders_a_geometry_collection() {
        let object = DataObject::Geometry(Geometry::GeometryCollection(GeometryCollection(vec![
            Geometry::Polygon(Polygon::new(
                line_string![
                    (x:-63_f64, y:18_f64), (x:-62_f64, y:18_f64), (x:-62_f64, y:17_f64)
                ],
                vec![],
            )),
        ])));
        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert_eq!(test_path(eq, object), "M165,160L170,160L170,165Z");
    }

    // Missing Feature, FeatureCollection test.

    #[test]
    fn line_string_then_point() {
        println!(
            "geoPath(LineString) then geoPath(Point) does not treat the point as part of a line"
        );
        let line_object = DataObject::Geometry(Geometry::LineString(line_string![
            (x:-63_f64, y:18_f64), (x:-62_f64, y:18_f64), (x:-62_f64, y:17_f64)
        ]));
        let point_object = DataObject::Geometry(Geometry::Point(point!(x: -63_f64, y:18_f64)));
        let eq = equirectangular::<ContextStream<f64>, f64>();

        assert_eq!(
            test_path(eq.clone(), line_object),
            "M165,160L170,160L170,165"
        );

        assert_eq!(
            test_path(eq, point_object),
            "M165,160m0,4.5a4.5,4.5 0 1,1 0,-9a4.5,4.5 0 1,1 0,9z"
        );
    }
}
