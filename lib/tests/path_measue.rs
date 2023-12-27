#[cfg(not(tarpaulin_include))]
mod path_measure {

    use core::fmt::Display;
    use core::ops::AddAssign;

    use d3_geo_rs::stream::DrainStub;
    use geo::CoordFloat;
    use geo::Geometry;
    use geo::LineString;
    use geo::MultiLineString;
    use geo::MultiPoint;
    use geo::MultiPolygon;
    use geo::Polygon;
    use num_traits::AsPrimitive;
    use num_traits::FloatConst;

    use d3_geo_rs::path::measure::Measure;
    use d3_geo_rs::path_identity::builder::Builder as PathBuilder;
    use d3_geo_rs::projection::identity::Identity;
    use d3_geo_rs::projection::projector_identity::types::ProjectorIdentityAntimeridianResampleNoClip;
    use d3_geo_rs::projection::RawBase;
    use d3_geo_rs::stream::Streamable;

    #[inline]
    fn measure<T>(
        projection: ProjectorIdentityAntimeridianResampleNoClip<Measure<T>, T>,
        object: impl Streamable<T = T>,
    ) -> T
    where
        T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        let builder = PathBuilder::new(Measure::default());
        let b = builder.build(projection);
        b.measure(&object)
    }

    #[test]
    fn of_a_point() {
        println!("geoPath.measure(…) of a Point");
        assert_eq!(
            0_f64,
            measure(
                Identity::builder::<DrainStub<f64>>().build(),
                Geometry::Point((0_f64, 0_f64).into())
            )
        );
    }

    #[test]
    fn of_a_mulitpoint() {
        println!("geoPath.measure(…) of a MultiPoint");
        assert_eq!(
            0_f64,
            measure(
                Identity::builder::<DrainStub<f64>>().build(),
                Geometry::MultiPoint(MultiPoint(vec![(0_f64, 0_f64).into()]))
            )
        );
    }

    #[test]
    fn of_a_linestring() {
        println!("geoPath.measure(…) of a LineString");
        assert_eq!(
            3_f64,
            measure(
                Identity::builder::<DrainStub<f64>>().build(),
                Geometry::LineString(LineString(vec![
                    (0_f64, 0_f64).into(),
                    (0_f64, 1_f64).into(),
                    (1_f64, 1_f64).into(),
                    (1_f64, 0_f64).into()
                ]))
            )
        );
    }

    #[test]
    fn of_a_multilinestring() {
        println!("geoPath.measure(…) of a MultiLineString");
        assert_eq!(
            3_f64,
            measure(
                Identity::builder::<DrainStub<f64>>().build(),
                Geometry::MultiLineString(MultiLineString(vec![LineString(
                    vec![
                        (0_f64, 0_f64).into(),
                        (0_f64, 1_f64).into(),
                        (1_f64, 1_f64).into(),
                        (1_f64, 0_f64).into()
                    ]
                )]))
            )
        );
    }

    #[test]
    fn of_a_polygon() {
        println!("geoPath.measure(…) of a Polygon");
        assert_eq!(
            4_f64,
            measure(
                Identity::builder::<DrainStub<f64>>().build(),
                Geometry::Polygon(Polygon::new(
                    LineString(vec![
                        (0_f64, 0_f64).into(),
                        (0_f64, 1_f64).into(),
                        (1_f64, 1_f64).into(),
                        (1_f64, 0_f64).into(),
                        (0_f64, 0_f64).into()
                    ]),
                    vec![]
                ))
            )
        );
    }

    #[test]
    fn of_a_polygon_with_a_hole() {
        println!("geoPath.measure(…) of a Polygon with a hole");
        assert_eq!(
            16_f64,
            measure(
                Identity::builder::<DrainStub<f64>>().build(),
                Geometry::Polygon(Polygon::new(
                    LineString(vec![
                        (-1_f64, -1_f64).into(),
                        (-1_f64, 2_f64).into(),
                        (2_f64, 2_f64).into(),
                        (2_f64, -1_f64).into(),
                        (-1_f64, -1_f64).into(),
                    ]),
                    vec![LineString(vec![
                        (0_f64, 0_f64).into(),
                        (1_f64, 0_f64).into(),
                        (1_f64, 1_f64).into(),
                        (0_f64, 1_f64).into(),
                        (0_f64, 0_f64).into()
                    ])]
                ))
            )
        );
    }

    #[test]
    fn of_a_multipolygon() {
        println!("geoPath.measure(…) of a MultiPolygon");
        assert_eq!(
            16_f64,
            measure(
                Identity::builder::<DrainStub<f64>>().build(),
                Geometry::MultiPolygon(MultiPolygon(vec![Polygon::new(
                    LineString(vec![
                        (-1_f64, -1_f64).into(),
                        (-1_f64, 2_f64).into(),
                        (2_f64, 2_f64).into(),
                        (2_f64, -1_f64).into(),
                        (-1_f64, -1_f64).into(),
                    ]),
                    vec![LineString(vec![
                        (0_f64, 0_f64).into(),
                        (1_f64, 0_f64).into(),
                        (1_f64, 1_f64).into(),
                        (0_f64, 1_f64).into(),
                        (0_f64, 0_f64).into()
                    ])]
                )]))
            )
        );
    }
}
