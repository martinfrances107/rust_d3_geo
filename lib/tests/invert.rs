#[cfg(not(tarpaulin_include))]
mod invert {

    use geo::Point;
    use geo_types::Coord;

    use d3_geo_rs::last_point::LastPoint;
    use d3_geo_rs::path::Result;
    use d3_geo_rs::projection::albers::albers as albers_builder;
    use d3_geo_rs::projection::albers_usa::AlbersUsa;
    use d3_geo_rs::projection::azimuthal_equal_area::AzimuthalEqualArea;
    use d3_geo_rs::projection::azimuthal_equidistant::AzimuthalEquiDistant;
    use d3_geo_rs::projection::builder_conic::ParallelsSet;
    use d3_geo_rs::projection::conformal::Conformal;
    use d3_geo_rs::projection::equal_area::EqualArea;
    use d3_geo_rs::projection::equal_earth::EqualEarth;
    use d3_geo_rs::projection::equality::projection_equal;
    use d3_geo_rs::projection::equidistant::Equidistant;
    use d3_geo_rs::projection::equirectangular::Equirectangular;
    use d3_geo_rs::projection::gnomic::Gnomic;
    use d3_geo_rs::projection::mercator::Mercator;
    use d3_geo_rs::projection::mercator_transverse::MercatorTransverse;
    use d3_geo_rs::projection::orthographic::Orthographic;
    use d3_geo_rs::projection::projector_albers_usa::multidrain::Multidrain;
    use d3_geo_rs::projection::stereographic::Stereographic;
    use d3_geo_rs::projection::Build;
    use d3_geo_rs::projection::Projector;
    use d3_geo_rs::projection::RawBase;
    use d3_geo_rs::projection::RotateSet;
    use d3_geo_rs::stream::DrainStub;
    use d3_geo_rs::stream::Stream;
    use d3_geo_rs::stream::Streamable;
    use d3_geo_rs::Transform;
    use geo_types::Geometry;

    fn symmetric_invert<PM>(pm: PM)
    where
        PM: Transform<T = f64>,
    {
        for p in [
            &Coord {
                x: 0.0f64,
                y: 0.0f64,
            },
            &Coord {
                x: 30.3f64,
                y: 24.1f64,
            },
            &Coord {
                x: -10f64,
                y: 42f64,
            },
            &Coord {
                x: -2.0f64,
                y: -5.0f64,
            },
        ] {
            assert!(projection_equal(&pm, p, &pm.transform(p), None));
        }
    }

    #[test]
    fn albers() {
        let a = albers_builder::<DrainStub<f64>, f64>().build();
        symmetric_invert(a);
    }

    #[test]
    fn azimuthal_equal_area() {
        let a = AzimuthalEqualArea::<f64>::builder::<DrainStub<f64>>().build();
        symmetric_invert(a);
    }

    #[test]
    fn azimuthal_equidistant() {
        let a =
            AzimuthalEquiDistant::<f64>::builder::<DrainStub<f64>>().build();
        symmetric_invert(a);
    }

    #[test]
    fn conformal() {
        let mut builder = Conformal::builder::<DrainStub<f64>>();
        symmetric_invert(builder.build());
        symmetric_invert(builder.parallels_set(20_f64, 30_f64).build());
        symmetric_invert(builder.parallels_set(30_f64, 30_f64).build());
        symmetric_invert(builder.parallels_set(-35_f64, -50_f64).build());
        symmetric_invert(
            builder
                .parallels_set(40_f64, 60_f64)
                .rotate2_set(&[-120_f64, 0_f64])
                .build(),
        );
    }

    #[test]
    fn conic_equal_area() {
        let mut builder = EqualArea::<f64>::builder::<DrainStub<f64>>();
        symmetric_invert(builder.build());
        symmetric_invert(builder.parallels_set(20_f64, 30_f64).build());
        symmetric_invert(builder.parallels_set(-30_f64, 30_f64).build());
        symmetric_invert(builder.parallels_set(-35_f64, -50_f64).build());
        symmetric_invert(
            builder
                .parallels_set(40_f64, 60_f64)
                .rotate2_set(&[-120_f64, 0_f64])
                .build(),
        );
    }

    #[test]
    fn conic_equidistant() {
        let mut builder = Equidistant::builder::<DrainStub<f64>>();
        symmetric_invert(builder.build());
        symmetric_invert(builder.parallels_set(20_f64, 30_f64).build());
        symmetric_invert(builder.parallels_set(30_f64, 30_f64).build());

        symmetric_invert(builder.parallels_set(-35_f64, -50_f64).build());
        symmetric_invert(
            builder
                .parallels_set(40_f64, 60_f64)
                .rotate2_set(&[-120_f64, 0_f64])
                .build(),
        );
    }

    #[test]
    fn equirectangular() {
        let e = Equirectangular::<f64>::builder::<DrainStub<f64>>().build();
        symmetric_invert(e);
    }

    #[test]
    fn equal_earth() {
        let e = EqualEarth::<f64>::builder::<DrainStub<f64>>().build();
        symmetric_invert(e);
    }

    #[test]
    fn gnomic() {
        let g = Gnomic::<f64>::builder::<DrainStub<f64>>().build();
        symmetric_invert(g);
    }

    #[test]
    fn mercator() {
        let m = Mercator::builder::<DrainStub<f64>>().build();
        symmetric_invert(m);
    }

    #[test]
    fn mercator_traverse() {
        let m = MercatorTransverse::builder::<DrainStub<f64>>().build();
        symmetric_invert(m);
    }

    #[test]
    fn orthographic() {
        let o = Orthographic::<f64>::builder::<DrainStub<f64>>().build();
        symmetric_invert(o);
    }

    #[test]
    fn stereographic() {
        let s = Stereographic::<f64>::builder::<DrainStub<f64>>().build();
        symmetric_invert(s);
    }

    #[test]
    fn albers_usa() {
        println!("albersUsa(point) and albersUsa.invert(point) are symmetric");

        let builder = AlbersUsa::<DrainStub<f64>, f64>::builder();
        let projector = builder.build();

        let builder_s = AlbersUsa::<LastPoint<f64>, f64>::builder();
        let mut projector_s = builder_s.build();

        let md = Multidrain::new(LastPoint::<f64>::default());
        let mut stream = projector_s.stream(&md);

        // Test points in the lower_48, and the two insets (Alaska and Hawaii).
        // p -projected, e - expected
        for (p, e) in [
            (
                Coord {
                    // San Francisco
                    x: -122.4194_f64,
                    y: 37.7749_f64,
                },
                Coord {
                    x: 107.42689983179525,
                    y: 214.14309852394865,
                },
            ),
            (
                Coord {
                    // NY, NY
                    x: -74.0059_f64,
                    y: 40.7128_f64,
                },
                Coord {
                    x: 794.5968111295658,
                    y: 176.53226149775173,
                },
            ),
            (
                Coord {
                    // Anchorage
                    x: -149.9003_f64,
                    y: 61.2181_f64,
                },
                Coord {
                    x: 171.16295961507146,
                    y: 446.9441310429266,
                },
            ),
            (
                Coord {
                    // Honolulu
                    x: -157.8583_f64,
                    y: 21.3069_f64,
                },
                Coord {
                    x: 298.47857157110445,
                    y: 450.98746080412093,
                },
            ),
        ] {
            assert!(projection_equal(
                &projector,
                &p,
                &projector.transform(&p),
                None
            ));

            // This test does not exist in the javascript original.
            // I created it because the code in this area is rust specific.
            // This tests the multidrain, multiplex code.
            let object = Geometry::Point(Point(p));
            object.to_stream(&mut stream);

            let point_stream = stream.endpoint().result().unwrap();
            assert_eq!(point_stream, e);
        }
    }
}
