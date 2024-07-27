#[cfg(not(tarpaulin_include))]
mod mercator {

    extern crate pretty_assertions;

    use std::sync::LazyLock;

    use geo::Geometry;
    use geo::LineString;
    use geo::Polygon;
    use geo_types::Coord;
    use pretty_assertions::assert_eq;
    use regex::Regex;

    use d3_geo_rs::data_object::sphere::Sphere;
    use d3_geo_rs::in_delta::coordinate as in_delta_coordinate;
    use d3_geo_rs::path::builder::Builder as PathBuilder;
    use d3_geo_rs::projection::mercator::Mercator;
    use d3_geo_rs::projection::Build;
    use d3_geo_rs::projection::CenterSet;
    use d3_geo_rs::projection::ClipExtentAdjust;
    use d3_geo_rs::projection::ClipExtentGet;
    use d3_geo_rs::projection::Fit;
    use d3_geo_rs::projection::PrecisionBypass;
    use d3_geo_rs::projection::RawBase;
    use d3_geo_rs::projection::RotateGet;
    use d3_geo_rs::projection::RotateSet;
    use d3_geo_rs::projection::ScaleGet;
    use d3_geo_rs::projection::ScaleSet;
    use d3_geo_rs::projection::TranslateGet;
    use d3_geo_rs::projection::TranslateSet;

    #[test]
    fn clip_extent_defaults_to_automatic() {
        println!(
            "mercator.clipExtent(null) sets the default automatic clip extent"
        );
        let pb = Mercator::builder()
            .translate_set(&Coord { x: 0_f64, y: 0_f64 })
            .scale_set(1_f64)
            .precision_bypass();

        let projection = pb.build();
        let path_builder = PathBuilder::pathstring();

        let object = Sphere::<f64>::default();

        let s = path_builder.build(projection).object(&object);
        assert_eq!(s, "M3.141593,-3.141593L3.141593,0L3.141593,3.141593L3.141593,3.141593L-3.141593,3.141593L-3.141593,3.141593L-3.141593,0L-3.141593,-3.141593L-3.141593,-3.141593L3.141593,-3.141593Z");
    }

    #[test]
    fn center_set_correct_automatic() {
        println!(
            "mercator.center(center) sets the correct automatic clip extent"
        );
        let pb = Mercator::builder()
            .translate_set(&Coord { x: 0_f64, y: 0_f64 })
            .center_set(&Coord {
                x: 10_f64,
                y: 10_f64,
            })
            .scale_set(1_f64)
            .precision_bypass();

        let projection = pb.build();
        let path_builder = PathBuilder::pathstring();

        let object = Sphere::default();

        //Bodge: I have had to alter the expected string ... dropping trailing 0 from some floats.
        // This is a trivial difference to between rust and Javascript.
        let s = path_builder.build(projection).object(&object);
        assert_eq!(s, "M2.96706,-2.966167L2.96706,0.175426L2.96706,3.317018L2.96706,3.317018L-3.316126,3.317018L-3.316126,3.317019L-3.316126,0.175426L-3.316126,-2.966167L-3.316126,-2.966167L2.96706,-2.966167Z");

        // assert_eq!(pb.get_clip_extent(), None);
    }

    #[test]
    fn intersected_clip_extent() {
        println!(
	            "mercator.clipExtent(extent) intersects the specified clip extent with the automatic clip extent"
	        );
        let pb = Mercator::builder()
            .translate_set(&Coord { x: 0_f64, y: 0_f64 })
            .clip_extent_adjust(&[
                Coord {
                    x: -10_f64,
                    y: -10_f64,
                },
                Coord {
                    x: 10_f64,
                    y: 10_f64,
                },
            ])
            .scale_set(1_f64)
            .precision_bypass();

        let projection = pb.build();

        let path_builder = PathBuilder::pathstring();

        let object = Sphere::default();

        let s = path_builder.build(projection).object(&object);
        assert_eq!(s, "M3.141593,-10L3.141593,0L3.141593,10L3.141593,10L-3.141593,10L-3.141593,10L-3.141593,0L-3.141593,-10L-3.141593,-10L3.141593,-10Z");
        assert_eq!(
            pb.clip_extent(),
            [
                Coord {
                    x: -10_f64,
                    y: -10_f64,
                },
                Coord {
                    x: 10_f64,
                    y: 10_f64,
                },
            ]
        );
    }

    #[test]
    fn scale_updates_the_intersected_clip_extent() {
        println!(
            "mercator.clipExtent(extent).translate(scale) updates the intersected clip extent"
        );
        let pb = Mercator::builder()
            .translate_set(&Coord { x: 0_f64, y: 0_f64 })
            .clip_extent_adjust(&[
                Coord {
                    x: -10_f64,
                    y: -10_f64,
                },
                Coord {
                    x: 10_f64,
                    y: 10_f64,
                },
            ])
            .scale_set(1_f64)
            .precision_bypass();
        let projection = pb.build();

        let path_builder = PathBuilder::pathstring();

        let object = Sphere::default();

        let s = path_builder.build(projection).object(&object);
        assert_eq!(s, "M3.141593,-10L3.141593,0L3.141593,10L3.141593,10L-3.141593,10L-3.141593,10L-3.141593,0L-3.141593,-10L-3.141593,-10L3.141593,-10Z");
        assert_eq!(
            pb.clip_extent(),
            [
                Coord {
                    x: -10_f64,
                    y: -10_f64,
                },
                Coord {
                    x: 10_f64,
                    y: 10_f64,
                },
            ]
        );
    }

    #[test]
    fn translate_updates_the_intersected_clip_extent() {
        println!(
            "mercator.clipExtent(extent).translate(translate) updates the intersected clip extent"
        );
        let mut pb = Mercator::builder();
        pb.scale_set(1_f64);
        pb.clip_extent_adjust(&[
            Coord {
                x: -10_f64,
                y: -10_f64,
            },
            Coord {
                x: 10_f64,
                y: 10_f64,
            },
        ]);
        let pb = pb.translate_set(&Coord { x: 0_f64, y: 0_f64 });
        let pb = pb.precision_bypass();

        let projection = pb.build();
        let path_builder = PathBuilder::pathstring();

        let object = Sphere::default();

        let s = path_builder.build(projection).object(&object);
        assert_eq!(s, "M3.141593,-10L3.141593,0L3.141593,10L3.141593,10L-3.141593,10L-3.141593,10L-3.141593,0L-3.141593,-10L-3.141593,-10L3.141593,-10Z");
        assert_eq!(
            pb.clip_extent(),
            [
                Coord {
                    x: -10_f64,
                    y: -10_f64,
                },
                Coord {
                    x: 10_f64,
                    y: 10_f64,
                },
            ]
        );
    }

    #[test]
    fn rotate_does_not_affect_automatic_clip_extent() {
        println!(
            "mercator.rotate(…) does not affect the automatic clip extent"
        );

        let object: Geometry<f64> = Geometry::MultiPoint(
            vec![
                (-82.35024908550241, 29.649391549778745),
                (-82.35014449996858, 29.65075946917633),
                (-82.34916073446641, 29.65070265688781),
                (-82.3492653331286, 29.64933474064504),
            ]
            .into(),
        );

        let pb = Mercator::builder();
        let mut pb = pb.fit_extent(
            [
                Coord { x: 0_f64, y: 0_f64 },
                Coord {
                    x: 960_f64,
                    y: 600_f64,
                },
            ],
            &object,
        );
        assert_eq!(pb.scale(), 20969742.365692537_f64);
        assert_eq!(
            pb.translate(),
            Coord {
                x: 30139734.76760269_f64,
                y: 11371473.949706702_f64
            }
        );

        pb.rotate2_set(&[0_f64, 95_f64]);
        let pb = pb.fit_extent(
            [
                Coord { x: 0_f64, y: 0f64 },
                Coord {
                    x: 960_f64,
                    y: 600_f64,
                },
            ],
            &object,
        );
        assert_eq!(pb.rotate(), [0_f64, 95_f64, 0_f64]);
        assert_eq!(pb.scale(), 35781690.650920525_f64);
        assert!(in_delta_coordinate(
            &pb.translate(),
            &Coord {
                x: 75115911.95344563_f64,
                y: 2586046.4116968135_f64
            },
            1e-6
        ));
    }

    // Part of the resolution of a Russia issue
    // reported by Robert Beekman.
    //
    // https://github.com/martinfrances107/rust_d3_geo/issues/31
    //
    // The stress test is a square polygon which crosses BOTH the 180th meridian
    // and the equator.
    //
    // Using the antemeridian clipping strategy the 180th meridian
    // is the "cutting seam" used to unwrap the surface.
    // So under projection the box is cut into two, one on the
    // left hand edge of the map and the other on the right.
    #[test]
    fn meridian_180() {
        println!("russia issue");
        let object = Geometry::Polygon(Polygon::new(
            LineString(vec![
                Coord {
                    x: 170_f64,
                    y: 10_f64,
                },
                Coord {
                    x: -170_f64,
                    y: 10_f64,
                },
                Coord {
                    x: -170_f64,
                    y: -10_f64,
                },
                Coord {
                    x: 170_f64,
                    y: -10_f64,
                },
                Coord {
                    x: 170_f64,
                    y: 10_f64,
                },
            ]),
            vec![],
        ));
        let mut projection = Mercator::builder();
        let projection = projection
            .scale_set(110f64)
            .center_set(&Coord {
                x: 10_f64,
                y: 40_f64,
            })
            .build();

        let path_builder = PathBuilder::pathstring();

        // Rounding issues:
        // Javascript mis-reports the value of log(tan(theta)) for angles close to PI/2.
        //
        // ( see mercator::transform() for a more detailed explanation ).
        //
        // So when comparing javascript and rust test -  it is impossible
        // to have IDENTICAL SVG path strings in both tests.
        //
        // That complication forces that we round down here before comparison.
        let round_down= Regex::new(r"\.\d+").unwrap();

        let s = path_builder.build(projection).object(&object);
        let rounded = round_down.replace_all(&s, "");
        assert_eq!(rounded, "M115,314L134,314L134,353L115,353L115,314ZM806,353L787,353L787,314L787,314L806,314L806,353Z");
    }

    // The test "meridian_180" does not have an equivalent in the javascript
    // library d3-geo: Is a relatively stable library and has no interest
    // in back-ports of "unit" code-coverage which does not expose
    // a underlying issue in that code base.
    // I am just including the javascript equivalent of this test so I cam confirm
    // that the two libraries handle this case identically.
    //
    // To run the test in javascript :-
    //
    // append the code below to d3-geo/test/projection/mercator-test.js
    //
    // cd d3-geo
    // npx mocha test/projection/mercator-test.js
    //
    //
    // it("meridian_180", () => {
    //   const projection = geoMercator();
    //
    //   projection.scale(110);
    //   projection.center([10, 40]);
    //
    //
    //   const object = {
    //     type: "MultiPolygon",
    //     coordinates: [[[
    //       [170, 10],
    //       [-170, 10],
    //       [-170, -10],
    //       [170, -10],
    //       [170, 10]
    //     ]]]
    //   };
    //
    //
    //   assertPathEqual(geoPath(projection)(object), "M115.226,314.329L134.425,314.623L134.425,353.217L115.226,353.512L115.226,314.329ZM806.377,353.512L787.178,353.217L787.178,314.623L787.178,314.623L806.377,314.329L806.377,353.512Z");
    //
    // });
}
