#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod index {

    extern crate pretty_assertions;

    use std::f64::consts::PI;
    use std::fmt::Debug;

    use approx::AbsDiffEq;
    use geo::line_string;
    use geo::CoordFloat;
    use geo::Coordinate;
    use geo::Geometry;
    use geo::GeometryCollection;
    use geo::LineString;
    use geo::MultiPoint;
    use geo::Point;
    use geo::Polygon;
    use num_traits::FloatConst;
    use pretty_assertions::assert_eq;

    use crate::path::builder::Builder as PathBuilder;
    use crate::path::context::Context;
    use crate::path_test_context::CanvasRenderingContext2d;
    use crate::projection::equirectangular::Equirectangular;
    use crate::projection::orthographic::Orthographic;
    use crate::projection::projector::types::ProjectorAntimeridianResampleNoneNoClip;
    use crate::projection::Build;
    use crate::projection::PrecisionBypass;
    use crate::projection::RawBase;
    use crate::projection::RotateSet;
    use crate::projection::ScaleSet;
    use crate::stream::Stream;
    use crate::stream::Streamable;

    #[inline]
    fn equirectangular<
        EP: Clone + Stream<EP = EP, T = T> + Debug + Default,
        T: AbsDiffEq<Epsilon = T> + CoordFloat + Default + FloatConst,
    >() -> ProjectorAntimeridianResampleNoneNoClip<EP, Equirectangular<EP, T>, T> {
        let mut e = Equirectangular::builder();
        e.scale_set(T::from(900f64 / PI).unwrap());
        e.precision_bypass().build()
    }

    fn path(
        projection: ProjectorAntimeridianResampleNoneNoClip<
            Context,
            Equirectangular<Context, f64>,
            f64,
        >,
        object: &impl Streamable<T = f64>,
    ) -> Vec<String> {
        let crc2d = CanvasRenderingContext2d::default();

        let context = Context::new(crc2d);
        let pb = PathBuilder::new(context);

        pb.build(projection).object(object)
    }

    // tape("geoPath.projection() defaults to null", function(test) {
    //   var path = d3_geo.geoPath();
    //   test.strictEqual(path.projection(), null);
    //   test.end();
    // });

    // tape("geoPath.context() defaults to null", function(test) {
    //   var path = d3_geo.geoPath();
    //   test.strictEqual(path.context(), null);
    //   test.end();
    // });

    // tape("d3.geoPath(projection) sets the initial projection", function(test) {
    //   var projection = d3_geo.geoAlbers(), path = d3_geo.geoPath(projection);
    //   test.strictEqual(path.projection(), projection);
    //   test.end();
    // });

    // tape("d3.geoPath(projection, context) sets the initial projection and context", function(test) {
    //   var context = testContext(), projection = d3_geo.geoAlbers(), path = d3_geo.geoPath(projection, context);
    //   test.strictEqual(path.projection(), projection);
    //   test.strictEqual(path.context(), context);
    //   test.end();
    // });

    #[test]
    fn path_point_renders_a_point() {
        println!("geoPath(Point) renders a point");
        let object = Geometry::Point(Point::new(-63.0_f64, 18.0_f64));
        assert_eq!(
            path(equirectangular(), &object),
            [
                "type: moveTo, x: 170.0, y: 160.0",
                "type: arc, x: 165.0, y: 160.0, r: 4.5"
            ]
        );
    }

    #[test]
    fn path_point_renders_a_multipoint() {
        println!("geoPath(MultiPoint) renders a point");
        let object = Geometry::MultiPoint(MultiPoint::new(vec![
            Point::new(-63.0_f64, 18.0_f64),
            Point::new(-62.0_f64, 18.0_f64),
            Point::new(-62.0_f64, 17.0_f64),
        ]));
        assert_eq!(
            path(equirectangular(), &object),
            [
                "type: moveTo, x: 170.0, y: 160.0",
                "type: arc, x: 165.0, y: 160.0, r: 4.5",
                "type: moveTo, x: 175.0, y: 160.0",
                "type: arc, x: 170.0, y: 160.0, r: 4.5",
                "type: moveTo, x: 175.0, y: 165.0",
                "type: arc, x: 170.0, y: 165.0, r: 4.5"
            ]
        );
    }

    #[test]
    fn render_line_string() {
        println!("geoPath(LineString) renders a line string");
        let object = Geometry::LineString(line_string![
			(x: -63_f64, y: 18_f64),(x: -62_f64, y: 18_f64), (x: -62_f64, y:17_f64) ]);

        assert_eq!(
            path(equirectangular(), &object),
            [
                "type: moveTo, x: 165.0, y: 160.0",
                "type: lineTo, x: 170.0, y: 160.0",
                "type: lineTo, x: 170.0, y: 165.0"
            ]
        );
    }

    #[test]
    fn render_a_polygon() {
        println!("geoPath(Polygon) renders a polygon");
        let object = Geometry::Polygon(Polygon::new(
            LineString(vec![
                Coordinate {
                    x: -63_f64,
                    y: 18_f64,
                },
                Coordinate {
                    x: -62_f64,
                    y: 18_f64,
                },
                Coordinate {
                    x: -62_f64,
                    y: 17_f64,
                },
            ]),
            vec![],
        ));
        assert_eq!(
            path(equirectangular(), &object),
            [
                "type: moveTo, x: 165.0, y: 160.0",
                "type: lineTo, x: 170.0, y: 160.0",
                "type: lineTo, x: 170.0, y: 165.0",
                "closePath"
            ]
        );
    }

    #[test]
    fn render_a_gc() {
        println!("geoPath(GeometryCollection) renders a geometry collection");
        let object = Geometry::GeometryCollection(GeometryCollection(vec![Geometry::Polygon(
            Polygon::new(
                LineString(vec![
                    Coordinate {
                        x: -63_f64,
                        y: 18_f64,
                    },
                    Coordinate {
                        x: -62_f64,
                        y: 18_f64,
                    },
                    Coordinate {
                        x: -62_f64,
                        y: 17_f64,
                    },
                ]),
                vec![],
            ),
        )]));
        assert_eq!(
            path(equirectangular(), &object),
            [
                "type: moveTo, x: 165.0, y: 160.0",
                "type: lineTo, x: 170.0, y: 160.0",
                "type: lineTo, x: 170.0, y: 165.0",
                "closePath"
            ]
        );
    }

    //     // tape("geoPath(Feature) renders a feature", function(test) {
    //     //   test.deepEqual(testPath(equirectangular, {
    //     //     type: "Feature",
    //     //     geometry: {
    //     //       type: "Polygon",
    //     //       coordinates: [[[-63, 18], [-62, 18], [-62, 17], [-63, 18]]]
    //     //     }
    //     //   }), [
    //     //     {type: "moveTo", x: 165, y: 160},
    //     //     {type: "lineTo", x: 170, y: 160},
    //     //     {type: "lineTo", x: 170, y: 165},
    //     //     {type: "closePath"}
    //     //   ]);
    //     //   test.end();
    //     // });

    //     // tape("geoPath(FeatureCollection) renders a feature collection", function(test) {
    //     //   test.deepEqual(testPath(equirectangular, {
    //     //     type: "FeatureCollection",
    //     //     features: [{
    //     //       type: "Feature",
    //     //       geometry: {
    //     //         type: "Polygon",
    //     //         coordinates: [[[-63, 18], [-62, 18], [-62, 17], [-63, 18]]]
    //     //       }
    //     //     }]
    //     //   }), [
    //     //     {type: "moveTo", x: 165, y: 160},
    //     //     {type: "lineTo", x: 170, y: 160},
    //     //     {type: "lineTo", x: 170, y: 165},
    //     //     {type: "closePath"}
    //     //   ]);
    //     //   test.end();
    //     // });

    #[test]
    fn wrap_longitude_outside_180() {
        println!("geoPath(…) wraps longitudes outside of ±180°");
        let object = Geometry::Point(Point::new(180_f64 + 1e-6_f64, 0_f64));
        assert_eq!(
            path(equirectangular(), &object),
            [
                "type: moveTo, x: -415.0, y: 250.0",
                "type: arc, x: -420.0, y: 250.0, r: 4.5"
            ]
        );
    }

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn observes_the_correct_winding_order_of_a_tiny_polygon() {
        println!("geoPath(…) observes the correct winding order of a tiny polygon");
        let object = Geometry::Polygon(Polygon::new(
            LineString(vec![
                Coordinate {
                    x: -0.06904102953339501,
                    y: 0.346043661846373,
                },
                Coordinate {
                    x: -6.725674252975136e-15,
                    y: 0.3981303360336475,
                },
                Coordinate {
                    x: -6.742247658534323e-15,
                    y: -0.08812465346531581,
                },
                Coordinate {
                    x: -0.17301258217724075,
                    y: -0.12278150669440671,
                },
                Coordinate {
                    x: -0.06904102953339501,
                    y: 0.346043661846373,
                },
            ]),
            vec![],
        ));
        assert_eq!(
            path(equirectangular(), &object),
            [
                "type: moveTo, x: 480.0, y: 248.0",
                "type: lineTo, x: 480.0, y: 248.0",
                "type: lineTo, x: 480.0, y: 250.0",
                "type: lineTo, x: 479.0, y: 251.0",
                "closePath"
            ]
        );
    }

    //     // tape("geoPath.projection(null)(…) does not transform coordinates", function(test) {
    //     //   test.deepEqual(testPath(null, {
    //     //     type: "Polygon",
    //     //     coordinates: [[[-63, 18], [-62, 18], [-62, 17], [-63, 18]]]
    //     //   }), [
    //     //     {type: "moveTo", x: -63, y: 18},
    //     //     {type: "lineTo", x: -62, y: 18},
    //     //     {type: "lineTo", x: -62, y: 17},
    //     //     {type: "closePath"}
    //     //   ]);
    //     //   test.end();
    //     // });

    //     // tape("geoPath.context(null)(null) returns null", function(test) {
    //     //   var path = d3_geo.geoPath();
    //     //   test.strictEqual(path(), null);
    //     //   test.strictEqual(path(null), null);
    //     //   test.strictEqual(path(undefined), null);
    //     //   test.end();
    //     // });

    //     // tape("geoPath.context(null)(Unknown) returns null", function(test) {
    //     //   var path = d3_geo.geoPath();
    //     //   test.strictEqual(path({type: "Unknown"}), null);
    //     //   test.strictEqual(path({type: "__proto__"}), null);
    //     //   test.end();
    //     // });

    #[test]
    fn does_not_treat_the_point_as_part_of_a_line() {
        println!(
            "geoPath(LineString) then geoPath(Point) does not treat the point as part of a line"
        );

        let crc2d = CanvasRenderingContext2d::default();

        let context = Context::new(crc2d);
        let pb = PathBuilder::new(context);

        let mut path = pb.build(equirectangular());

        let object = LineString(vec![
            Coordinate {
                x: -63_f64,
                y: 18_f64,
            },
            Coordinate {
                x: -62_f64,
                y: 18_f64,
            },
            Coordinate {
                x: -62_f64,
                y: 17_f64,
            },
        ]);

        assert_eq!(
            path.object(&object),
            [
                "type: moveTo, x: 165.0, y: 160.0",
                "type: lineTo, x: 170.0, y: 160.0",
                "type: lineTo, x: 170.0, y: 165.0",
            ]
        );
        let object = Geometry::Point(Point::new(-63_f64, 18_f64));
        assert_eq!(
            path.object(&object),
            [
                "type: moveTo, x: 170.0, y: 160.0",
                "type: arc, x: 165.0, y: 160.0, r: 4.5"
            ]
        );
    }

    #[test]
    fn does_not_treat_the_point_as_part_of_a_polygon() {
        println!(
            "geoPath(LineString) then geoPath(Point) does not treat the point as part of a line"
        );

        let crc2d = CanvasRenderingContext2d::default();

        let context = Context::new(crc2d);
        let pb = PathBuilder::new(context);

        let mut path = pb.build(equirectangular());

        let object = Geometry::Polygon(Polygon::new(
            LineString(vec![
                Coordinate {
                    x: -63_f64,
                    y: 18_f64,
                },
                Coordinate {
                    x: -62_f64,
                    y: 18_f64,
                },
                Coordinate {
                    x: -62_f64,
                    y: 17_f64,
                },
            ]),
            vec![],
        ));

        assert_eq!(
            path.object(&object),
            [
                "type: moveTo, x: 165.0, y: 160.0",
                "type: lineTo, x: 170.0, y: 160.0",
                "type: lineTo, x: 170.0, y: 165.0",
                "closePath",
            ]
        );
        let object = Geometry::Point(Point::new(-63_f64, 18_f64));
        assert_eq!(
            path.object(&object),
            [
                "type: moveTo, x: 170.0, y: 160.0",
                "type: arc, x: 165.0, y: 160.0, r: 4.5"
            ]
        );
    }

    /// This test is not in the javascript original.
    /// it was used to diagnose a problem in the `d3_geo_voronoi/benchmark`.
    /// After rendering polygons additional point were not drawn.
    #[test]
    fn emulate_benchmark() {
        println!("emulate benchmark");

        let crc2d = CanvasRenderingContext2d::default();

        let context = Context::new(crc2d);
        let pb = PathBuilder::new(context);

        let mut ob = Orthographic::builder();
        ob.rotate_set(&[95_f64, 0_f64, 0_f64]);
        let ortho = ob.build();
        let mut path = pb.build(ortho);

        let object = Geometry::Polygon(Polygon::new(
            LineString(vec![
                Coordinate {
                    x: -63_f64,
                    y: 18_f64,
                },
                Coordinate {
                    x: -62_f64,
                    y: 18_f64,
                },
                Coordinate {
                    x: -62_f64,
                    y: 17_f64,
                },
            ]),
            vec![],
        ));

        assert_eq!(
            path.object(&object),
            [
                "type: moveTo, x: 606.0, y: 173.0",
                "type: lineTo, x: 609.0, y: 173.0",
                "type: lineTo, x: 610.0, y: 177.0",
                "closePath",
            ]
        );
        let object = Geometry::Point(Point::new(-63_f64, 18_f64));
        assert_eq!(
            path.object(&object),
            [
                "type: moveTo, x: 610.0, y: 173.0",
                "type: arc, x: 606.0, y: 173.0, r: 4.5",
            ]
        );
    }
}
