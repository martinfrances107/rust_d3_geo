#[cfg(not(tarpaulin_include))]
mod mercator {

    extern crate pretty_assertions;

    use geo::Geometry;
    use geo::LineString;
    use geo::Polygon;
    use geo_types::Coord;
    use pretty_assertions::assert_eq;

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
        println!("mercator.clipExtent(null) sets the default automatic clip extent");
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
        println!("mercator.center(center) sets the correct automatic clip extent");
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
        println!("mercator.rotate(…) does not affect the automatic clip extent");

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

    // Part of the resolution of an issue
    // reported by Robert Beekman.
    //
    // https://github.com/martinfrances107/rust_d3_geo/issues/31
    //
    // The "object" is a box roughly the size of Russia but extended
    // over the 180th meridian. ( as if to cover Alaska )
    // The antemeridian clipping strategy should split the box
    // into two. ( Under this projection Alaska would be on the left,
    // with Russia on the right.
    //
    #[ignore]
    #[test]
    fn russia_issue() {
        println!("russia issue");
        let object = Geometry::Polygon(Polygon::new(
            LineString(vec![
                Coord {
                    x: 30_f64,
                    y: 70_f64,
                },
                Coord {
                    x: -170_f64,
                    y: 70_f64,
                },
                Coord {
                    x: -170_f64,
                    y: 50_f64,
                },
                Coord {
                    x: 30_f64,
                    y: 50_f64,
                },
                Coord {
                    x: 30_f64,
                    y: 70_f64,
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

        let object = Sphere::default();

        let s = path_builder.build(projection).object(&object);
        assert_eq!(s, "M115.226,71.009L127.770,112.248L134.425,143.024L134.425,222.745L127.158,194.489L115.226,159.133L115.226,71.009ZM754.763,-11.655L767.964,2.249L786.457,28.499L798.301,51.341L806.377,71.009L806.377,159.133L806.377,159.133L806.377,159.133L798.357,140.666L787.101,119.651L770.435,95.942L758.973,83.355L744.626,70.835L726.814,59.318L705.359,50.304L681.045,45.588L668.400,45.276L655.846,46.394L632.189,52.498L611.738,62.402L594.953,74.330L581.488,86.941L562.066,111.449L549.191,133.447L533.570,169.837L524.462,198.742L518.397,222.745L518.397,143.024L518.397,143.024L530.164,93.416L542.594,58.366L552.920,36.665L568.697,11.533L579.885,-2.187L589.623,-11.655L754.763,-11.655Z");
    }

    // The test "russia_issue" does not have an equivalent in the javascript
    // library. d3-geo: Is a relatively stable library and has no interest
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
    // it("mercator.russia", () => {
    //   const projection = geoMercator();
    //
    //   projection.scale(110);
    //   projection.center([10, 40]);
    //
    //
    //   const object = {
    //     type: "MultiPolygon",
    //     coordinates: [[[
    //       [30, 70],
    //       [-170, 70],
    //       [-170, 50],
    //       [30, 50],
    //       [30, 70]
    //     ]]]
    //   };
    //
    //
    //   assertPathEqual(geoPath(projection)(object), "M115.226,71.009L127.770,112.248L134.425,143.024L134.425,222.745L127.158,194.489L115.226,159.133L115.226,71.009ZM754.763,-11.655L767.964,2.249L786.457,28.499L798.301,51.341L806.377,71.009L806.377,159.133L806.377,159.133L806.377,159.133L798.357,140.666L787.101,119.651L770.435,95.942L758.973,83.355L744.626,70.835L726.814,59.318L705.359,50.304L681.045,45.588L668.400,45.276L655.846,46.394L632.189,52.498L611.738,62.402L594.953,74.330L581.488,86.941L562.066,111.449L549.191,133.447L533.570,169.837L524.462,198.742L518.397,222.745L518.397,143.024L518.397,143.024L530.164,93.416L542.594,58.366L552.920,36.665L568.697,11.533L579.885,-2.187L589.623,-11.655L754.763,-11.655Z");
    //
    // });
}
