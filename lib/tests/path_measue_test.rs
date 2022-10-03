#[cfg(not(tarpaulin_include))]
mod path_measure_test {

    use std::f64::consts::PI;
    use std::fmt::Display;

    use geo::CoordFloat;
    use geo::Geometry;
    use geo::LineString;
    use geo::MultiLineString;
    use geo::MultiPoint;
    use geo::MultiPolygon;
    use geo::Polygon;
    use num_traits::AsPrimitive;
    use num_traits::FloatConst;

    use rust_d3_geo::path::builder::Builder as PathBuilder;
    use rust_d3_geo::path::measure::Measure;
    use rust_d3_geo::projection::builder::types::BuilderAntimeridianResampleNoClip;
    use rust_d3_geo::projection::equirectangular::Equirectangular;
    use rust_d3_geo::projection::projector::types::ProjectorAntimeridianResampleNoneNoClip;
    use rust_d3_geo::projection::Build;
    use rust_d3_geo::projection::PrecisionBypass;
    use rust_d3_geo::projection::ProjectionRawBase;
    use rust_d3_geo::projection::ScaleSet;
    use rust_d3_geo::stream::Streamable;

    #[inline]
    fn equirectangular<T>(
    ) -> ProjectorAntimeridianResampleNoneNoClip<Measure<T>, Equirectangular<Measure<T>, T>, T>
    where
        T: CoordFloat + Default + Display + FloatConst,
    {
        let mut ba: BuilderAntimeridianResampleNoClip<
            Measure<T>,
            Equirectangular<Measure<T>, T>,
            T,
        > = Equirectangular::<Measure<T>, T>::builder();
        ba.scale_set(T::from(900f64 / PI).unwrap());

        let builder = ba.precision_bypass();
        let out = builder.build();

        out
    }

    #[inline]
    fn test_measure<'a, T>(
        projection: ProjectorAntimeridianResampleNoneNoClip<
            Measure<T>,
            Equirectangular<Measure<T>, T>,
            T,
        >,
        object: impl Streamable<T = T>,
    ) -> T
    where
        T: AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        let builder = PathBuilder::new(Measure::default());
        let b = builder.build(projection);
        b.measure(&object)
    }

    // it("geoPath.measure(…) of a Point", () => {
    //   assert.strictEqual(geoPath().measure({
    //     type: "Point",
    //     coordinates: [0, 0]
    //   }), 0);
    // });
    #[test]
    fn measure_of_a_point() {
        println!("geoPath.measure(…) of a Point");
        assert_eq!(
            0_f64,
            test_measure(
                equirectangular::<f64>(),
                Geometry::Point((0_f64, 0_f64).into())
            )
        );
    }

    // it("geoPath.measure(…) of a MultiPoint", () => {
    //   assert.strictEqual(geoPath().measure({
    //     type: "Point",
    //     coordinates: [[0, 0], [0, 1], [1, 1], [1, 0]]
    //   }), 0);
    // });
    #[test]
    fn measure_of_a_mulitpoint() {
        println!("geoPath.measure(…) of a MultiPoint");
        assert_eq!(
            0_f64,
            test_measure(
                equirectangular::<f64>(),
                Geometry::MultiPoint(MultiPoint(vec![(0_f64, 0_f64).into()]))
            )
        );
    }

    // it("geoPath.measure(…) of a LineString", () => {
    //   assert.strictEqual(geoPath().measure({
    //     type: "LineString",
    //     coordinates: [[0, 0], [0, 1], [1, 1], [1, 0]]
    //   }), 3);
    // });

    #[ignore]
    #[test]
    fn measure_of_a_linestring() {
        println!("geoPath.measure(…) of a LineString");
        assert_eq!(
            4_f64,
            test_measure(
                equirectangular::<f64>(),
                Geometry::LineString(LineString(vec![
                    (0_f64, 0_f64).into(),
                    (0_f64, 1_f64).into(),
                    (1_f64, 1_f64).into(),
                    (1_f64, 0_f64).into()
                ]))
            )
        );
    }
    // it("geoPath.measure(…) of a MultiLineString", () => {
    //   assert.strictEqual(geoPath().measure({
    //     type: "MultiLineString",
    //     coordinates: [[[0, 0], [0, 1], [1, 1], [1, 0]]]
    //   }), 3);
    // });

    #[ignore]
    #[test]
    fn measure_of_a_multilinestring() {
        println!("geoPath.measure(…) of a MultiLineString");
        assert_eq!(
            4_f64,
            test_measure(
                equirectangular::<f64>(),
                Geometry::MultiLineString(MultiLineString(vec![LineString(vec![
                    (0_f64, 0_f64).into(),
                    (0_f64, 1_f64).into(),
                    (1_f64, 1_f64).into(),
                    (1_f64, 0_f64).into()
                ])]))
            )
        );
    }

    // it("geoPath.measure(…) of a Polygon", () => {
    //   assert.strictEqual(geoPath().measure({
    //     type: "Polygon",
    //     coordinates: [[[0, 0], [0, 1], [1, 1], [1, 0], [0, 0]]]
    //   }), 4);
    // });

    #[ignore]
    #[test]
    fn measure_of_a_polygon() {
        println!("geoPath.measure(…) of a Polygon");
        assert_eq!(
            4_f64,
            test_measure(
                equirectangular::<f64>(),
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

    // it("geoPath.measure(…) of a Polygon with a hole", () => {
    //   assert.strictEqual(geoPath().measure({
    //     type: "Polygon",
    //     coordinates: [[[-1, -1], [-1, 2], [2, 2], [2, -1], [-1, -1]], [[0, 0], [1, 0], [1, 1], [0, 1], [0, 0]]]
    //   }), 16);
    // });

    #[ignore]
    #[test]
    fn measure_of_a_polygon_with_a_hole() {
        println!("geoPath.measure(…) of a Polygon with a hole");
        assert_eq!(
            4_f64,
            test_measure(
                equirectangular::<f64>(),
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

    // it("geoPath.measure(…) of a MultiPolygon", () => {
    //   assert.strictEqual(geoPath().measure({
    //     type: "MultiPolygon",
    //     coordinates: [[[[-1, -1], [-1, 2], [2, 2], [2, -1], [-1, -1]]], [[[0, 0], [0, 1], [1, 1], [1, 0], [0, 0]]]]
    //   }), 16);
    // });

    #[ignore]
    #[test]
    fn measure_of_a_multipolygon() {
        println!("geoPath.measure(…) of a MultiPolygon");
        assert_eq!(
            4_f64,
            test_measure(
                equirectangular::<f64>(),
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
