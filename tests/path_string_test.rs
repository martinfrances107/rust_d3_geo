#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod path_string_test {

    use std::f64::consts::PI;
    use std::fmt::Display;
    use std::ops::AddAssign;
    use std::rc::Rc;

    use approx::AbsDiffEq;
    use geo::point;
    use geo::CoordFloat;
    use geo::Geometry;
    use num_traits::AsPrimitive;
    use num_traits::FloatConst;
    use pretty_assertions::assert_eq;

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
    fn equirectangular<DRAIN, T>() -> Rc<Projection<DRAIN, EquirectangularRaw<DRAIN, T>, PV<T>, T>>
    where
        DRAIN: Stream<T = T> + Default,
        T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        Rc::new(
            EquirectangularRaw::builder()
                .scale(T::from(900f64 / PI).unwrap())
                .precision(&T::zero())
                .build(),
        )
    }

    #[inline]
    fn test_path<'a, DRAIN, T>(
        projection: Rc<Projection<ContextStream<T>, EquirectangularRaw<DRAIN, T>, PV<T>, T>>,
        object: DataObject<T>,
    ) -> String
    where
        DRAIN: Stream<T = T>,
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
    fn test_point_renders_a_point() {
        println!("geoPath.point(…) renders a point");
        let object = DataObject::Geometry(Geometry::Point(point!(x: -63_f64, y:18_f64)));
        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert_eq!(
            test_path(eq, object),
            "M165,160m0,4.5a4.5,4.5 0 1,1 0,-9a4.5,4.5 0 1,1 0,9z"
        );
    }

    #[test]
    fn test_point_renders_a_point_of_given_radius() {
        println!("geoPath.point(…) renders a point of a given radius");

        let builder: PathBuilder<EquirectangularRaw<ContextStream<f64>, f64>, PV<f64>, f64> =
            PathBuilder::context_pathstring().point_radius(Some(10_f64));

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
    fn test_renders_multipoint() {
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
}
