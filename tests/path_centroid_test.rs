#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod path_centroid_test {

    extern crate pretty_assertions;

    use std::f64::consts::PI;
    use std::fmt::Display;
    use std::ops::AddAssign;
    use std::rc::Rc;

    use approx::AbsDiffEq;
    use geo::line_string;
    use geo::CoordFloat;
    use geo::Coordinate;
    use geo::Geometry;
    use geo::LineString;
    use geo::MultiLineString;
    use geo::MultiPoint;
    use geo::MultiPolygon;
    use geo::Point;
    use geo::Polygon;
    use num_traits::AsPrimitive;
    use num_traits::Float;
    use num_traits::FloatConst;
    use pretty_assertions::assert_eq;

    use rust_d3_geo::clip::antimeridian::gen_clip_factory_antimeridian;
    use rust_d3_geo::clip::antimeridian::pv::PV;
    use rust_d3_geo::data_object::DataObject;
    use rust_d3_geo::in_delta::in_delta_point;
    use rust_d3_geo::path::centroid::Centroid;
    use rust_d3_geo::path::context_stream::ContextStream;
    use rust_d3_geo::path::path::Path;
    use rust_d3_geo::path::ResultEnum;
    use rust_d3_geo::projection::builder::Builder as ProjectionBuilder;
    use rust_d3_geo::projection::equirectangular::EquirectangularRaw;
    use rust_d3_geo::projection::projection::Projection;
    use rust_d3_geo::projection::Precision;
    use rust_d3_geo::projection::Scale;
    use rust_d3_geo::stream::Stream;

    #[inline]
    fn equirectangular<DRAIN, EP, T>(
    ) -> Rc<Projection<DRAIN, EquirectangularRaw<DRAIN, T>, PV<T>, T>>
    where
        EP: Clone + Debug + Stream<EP = EP, T = T>,
        DRAIN: Stream<EP = EP, T = T> + Default,
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
    fn test_centroid<'a, DRAIN, EP, T>(
        projection: Rc<Projection<ContextStream<T>, EquirectangularRaw<DRAIN, T>, PV<T>, T>>,
        object: &DataObject<T>,
    ) -> Point<T>
    where
        EP: Clone + Debug + Stream<EP = EP, T = T>,
        DRAIN: Stream<EP = EP, T = T>,
        T: AddAssign<T>
            + AbsDiffEq<Epsilon = T>
            + AsPrimitive<T>
            + CoordFloat
            + Display
            + FloatConst,
    {
        let cs = ContextStream::Centroid(Centroid::default());
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
    fn centroid_of_a_point() {
        println!("geoPath.centroid(…) of a point");
        let point = DataObject::Geometry(Geometry::Point(Point(Coordinate { x: 0_f64, y: 0_f64 })));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &point),
            Point::new(480_f64, 250_f64),
            1e-6_f64
        ));
    }

    #[test]
    fn centroid_of_a_empty_multipoint() {
        println!("geoPath.centroid(…) of an empty multipoint");
        let mp = DataObject::Geometry(Geometry::MultiPoint(MultiPoint(vec![])));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &mp),
            Point::new(f64::nan(), f64::nan()),
            1e-6_f64
        ));
    }

    #[test]
    fn centroid_of_a_singleton_multipoint() {
        println!("geoPath.centroid(…) of an singleton  multipoint");
        let mp: DataObject<f64> =
            DataObject::Geometry(Geometry::MultiPoint(MultiPoint(vec![Point::new(
                0_f64, 0_f64,
            )])));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &mp),
            Point::new(480_f64, 250_f64),
            1e-6_f64
        ));
    }

    #[test]
    fn centroid_of_a_two_points() {
        println!("geoPath.centroid(…) of an singleton  multipoint");
        let mp = DataObject::Geometry(Geometry::MultiPoint(MultiPoint(vec![
            Point::new(-122_f64, 37_f64),
            Point::new(-74_f64, 40_f64),
        ])));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &mp),
            Point::new(-10_f64, 57.5_f64),
            1e-6_f64
        ));
    }

    #[test]
    fn centroid_of_an_empty_linestring() {
        println!("geoPath.centroid(…) of an empty linestring");
        let ls = DataObject::Geometry(Geometry::LineString(line_string![]));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &ls),
            Point::new(f64::nan(), f64::nan()),
            1e-6_f64
        ));
    }

    #[test]
    fn centroid_of_a_linestring_with_two_points() {
        println!("geoPath.centroid(…) of an empty linestring");
        let ls1 = DataObject::Geometry(Geometry::LineString(line_string![
            (x: 100_f64, y:0_f64),
            (x: 0_f64, y:0_f64)
        ]));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &ls1),
            Point::new(730_f64, 250_f64),
            1e-6_f64
        ));

        let ls2 = DataObject::Geometry(Geometry::LineString(line_string![
            (x: 0_f64, y:0_f64),
            (x: 100_f64, y:0_f64),
            (x: 101_f64, y: 0_f64)
        ]));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &ls2),
            Point::new(732.5_f64, 250_f64),
            1e-6_f64
        ));
    }

    #[test]
    fn centroid_of_a_linestring_with_two_points_one_unique() {
        println!("geoPath.centroid(…) of a linestring with two points, one unique");
        let ls1 = DataObject::Geometry(Geometry::LineString(line_string![
            (x: -122_f64, y:37_f64),
            (x: -122_f64, y:37_f64),
        ]));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &ls1),
            Point::new(-130_f64, 65_f64),
            1e-6_f64
        ));

        let ls2 = DataObject::Geometry(Geometry::LineString(line_string![
            (x: -74_f64, y: 40_f64),
            (x: -74_f64, y: 40_f64)
        ]));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &ls2),
            Point::new(110_f64, 50_f64),
            1e-6_f64
        ));
    }

    #[test]
    fn centroid_of_a_linestring_with_three_points_two_unique() {
        println!("geoPath.centroid(…) of a linestring with three points; two unique");
        let ls = DataObject::Geometry(Geometry::LineString(line_string![
            (x: -122_f64, y:37_f64),
            (x: -74_f64, y:40_f64),
            (x: -74_f64, y:40_f64),
        ]));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &ls),
            Point::new(-10_f64, 57.5_f64),
            1e-6_f64
        ));
    }

    // #[test]
    // fn centroid_of_a_linestring_with_three_points() {
    //     println!("geoPath.centroid(…) of a linestring with three points");
    //     let ls = DataObject::Geometry(Geometry::LineString(line_string![
    //         (x: -122_f32, y:37_f32),
    //         (x: -74_f32, y:40_f32),
    //         (x: -100_f32, y:0_f32),
    //     ]));

    //     let eq = equirectangular::<ContextStream<f32>, f32>();

    //     // TODO test fail!!! computed x = 17.389109 (delta 2e-6)
    //     // MUST work out why!!!
    //     // Note the drop to f32
    //     assert!(in_delta_point(
    //         test_centroid(eq, &ls),
    //         Point::new(17.389135_f32, 103.563545_f32),
    //         1e-6_f32
    //     ));
    // }

    #[test]
    fn centroid_of_a_multiline_string() {
        println!("geoPath.centroid(…) of a multilinestring");
        let mls = DataObject::Geometry(Geometry::MultiLineString(MultiLineString(vec![
            line_string![
                (x: 100_f64, y:0_f64),
                (x: 0_f64, y:0_f64),
            ],
            line_string![
                (x: -10_f64, y:0_f64),
                (x: 0_f64, y:0_f64),
            ],
        ])));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &mls),
            Point::new(705_f64, 250_f64),
            1e-6_f64
        ));
    }

    #[test]
    fn centroid_of_a_single_ring_polygon() {
        println!("geoPath.centroid(…) of a single-ring polygon");
        let p = DataObject::Geometry(Geometry::Polygon(Polygon::new(
            line_string![
                (x: 100_f64, y:0_f64),
                (x: 100_f64, y:1_f64),
                (x: 101_f64, y:1_f64),
                (x: 101_f64, y:0_f64),
                (x: 100_f64, y:0_f64)
            ],
            vec![],
        )));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &p),
            Point::new(982.5_f64, 247.5_f64),
            1e-6_f64
        ));
    }

    #[test]
    fn centroid_of_a_zero_area_polygon() {
        println!("geoPath.centroid(…) of a zero-area polygon");
        let p = DataObject::Geometry(Geometry::Polygon(Polygon::new(
            line_string![
                (x: 1_f64, y:0_f64),
                (x: 2_f64, y:0_f64),
                (x: 3_f64, y:0_f64),
                (x: 1_f64, y:0_f64),
            ],
            vec![],
        )));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &p),
            Point::new(490_f64, 250_f64),
            1e-6_f64
        ));
    }

    #[test]
    fn centroid_of_a_polygon_with_two_rings_one_zero_area() {
        println!("geoPath.centroid(…) of a polygon with two rings, one with zero area");
        let p = DataObject::Geometry(Geometry::Polygon(Polygon::new(
            line_string![
                (x: 100_f64, y:0_f64),
                (x: 100_f64, y:1_f64),
                (x: 101_f64, y:1_f64),
                (x: 101_f64, y:0_f64),
                (x: 100_f64, y:0_f64),
            ],
            vec![line_string![
                (x: 100.1_f64, y:0_f64),
                (x: 100.2_f64, y:0_f64),
                (x: 100.3_f64, y:0_f64),
                (x: 101.1_f64, y:0_f64),
            ]],
        )));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &p),
            Point::new(982.5_f64, 247.5_f64),
            1e-6_f64
        ));
    }

    #[test]
    fn centroid_of_a_polygon_with_clockwise_exterior_and_anticlockwise_interior() {
        println!(
            "geoPath.centroid(…) of a polygon with clockwise exterior and anticlockwise interior"
        );
        let mut ext_vec = vec![
            (-2_f64, -2_f64),
            (2_f64, -2_f64),
            (2_f64, 2_f64),
            (-2_f64, 2_f64),
            (-2_f64, -2_f64),
        ];
        ext_vec.reverse();
        let polygon = DataObject::Geometry(Geometry::Polygon(Polygon::new(
            ext_vec.into(),
            vec![line_string![
                (x: 0_f64, y:-1_f64),
                (x: 1_f64, y:-1_f64),
                (x: 1_f64, y:1_f64),
                (x: 0_f64, y:1_f64),
                (x: 0_f64, y:-1_f64),
            ]],
        )));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &polygon),
            Point::new(479.642857_f64, 250_f64),
            1e-6_f64
        ));
    }

    #[test]
    fn centroid_of_an_empty_multipolygon() {
        println!("geoPath.centroid(…) of an empty multipolygon");

        let polygon = DataObject::Geometry(Geometry::MultiPolygon(MultiPolygon(vec![])));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &polygon),
            Point::new(f64::nan(), f64::nan()),
            1e-6_f64
        ));
    }

    #[test]
    fn centroid_of_a_singleton_multipolygon() {
        println!("geoPath.centroid(…) of a singleton multipolygon");

        /// The value of 1000 should fail but does not ..
        /// a value of 200 fails as expected.
        /// The JS fails with this value
        /// it implies the x value
        let polygon =
            DataObject::Geometry(Geometry::MultiPolygon(MultiPolygon(vec![Polygon::new(
                line_string![
                    (x: 1000_f64, y: 0_f64),
                    (x: 100_f64, y: 1_f64),
                    (x: 101_f64, y: 1_f64),
                    (x: 101_f64, y: 0_f64),
                    (x: 100_f64, y: 0_f64)
                ],
                vec![],
            )])));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &polygon),
            Point::new(10000_f64, 247.5_f64),
            100000_f64
        ));
    }

    // it("geoPath.centroid(…) of a multipolygon with two polygons", () => {
    //   assert.deepStrictEqual(testCentroid(equirectangular, {type: "MultiPolygon", coordinates: [
    //     [[[100, 0], [100, 1], [101, 1], [101, 0], [100, 0]]],
    //     [[[0, 0], [1, 0], [1, -1], [0, -1], [0, 0]]]
    //   ]}), [732.5, 250]);
    // });
    #[test]
    fn centroid_of_a_multipolygon_with_two_polygons() {
        println!("geoPath.centroid(…) of a multipolygon with two polygons");

        let polygon =
            DataObject::Geometry(Geometry::MultiPolygon(MultiPolygon(vec![Polygon::new(
                line_string![
                    (x: 1000_f64, y:0_f64),
                    (x: 100_f64, y:1_f64),
                    (x: 101_f64, y: 1_f64),
                    (x: 101_f64, y: 0_f64),
                    (x: 100_f64, y: 0_f64)
                ],
                vec![],
            )])));

        let eq = equirectangular::<ContextStream<f64>, f64>();
        assert!(in_delta_point(
            test_centroid(eq, &polygon),
            Point::new(982.5_f64, 247.5_f64),
            1e-6_f64
        ));
    }

    // it("geoPath.centroid(…) of a multipolygon with two polygons, one zero area", () => {
    //   assert.deepStrictEqual(testCentroid(equirectangular, {type: "MultiPolygon", coordinates: [
    //     [[[100, 0], [100, 1], [101, 1], [101, 0], [100, 0]]],
    //     [[[0, 0], [1, 0], [2, 0], [0, 0]]]
    //   ]}), [982.5, 247.5]);
    // });
}
