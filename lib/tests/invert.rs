#[cfg(not(tarpaulin_include))]
mod invert {

    use d3_geo_rs::path::Result;
    use d3_geo_rs::stream::Streamable;
    use geo::Point;
    use geo_types::Coord;

    use d3_geo_rs::last_point::LastPoint;
    use d3_geo_rs::multidrain::Multidrain;
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
    use d3_geo_rs::projection::projector_identity::transformer;
    use d3_geo_rs::projection::stereographic::Stereographic;
    use d3_geo_rs::projection::Build;
    use d3_geo_rs::projection::Projector;
    use d3_geo_rs::projection::RawBase;
    use d3_geo_rs::projection::RotateSet;
    use d3_geo_rs::stream::DrainStub;
    use d3_geo_rs::stream::Stream;
    use d3_geo_rs::Transform;

    fn symetric_invert<PM>(pm: PM)
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
        symetric_invert(a);
    }

    #[test]
    fn azimuthal_equal_area() {
        let a = AzimuthalEqualArea::<DrainStub<f64>, f64>::builder().build();
        symetric_invert(a);
    }

    #[test]
    fn azimuthal_equidistant() {
        let a = AzimuthalEquiDistant::<DrainStub<f64>, f64>::builder().build();
        symetric_invert(a);
    }

    #[test]
    fn conformal() {
        let mut builder = Conformal::<DrainStub<f64>>::builder();
        symetric_invert(builder.build());
        symetric_invert(builder.parallels_set(20_f64, 30_f64).build());
        symetric_invert(builder.parallels_set(30_f64, 30_f64).build());
        symetric_invert(builder.parallels_set(-35_f64, -50_f64).build());
        symetric_invert(
            builder
                .parallels_set(40_f64, 60_f64)
                .rotate2_set(&[-120_f64, 0_f64])
                .build(),
        );
    }

    #[test]
    fn conic_equal_area() {
        let mut builder = EqualArea::<DrainStub<f64>, f64>::builder();
        symetric_invert(builder.build());
        symetric_invert(builder.parallels_set(20_f64, 30_f64).build());
        symetric_invert(builder.parallels_set(-30_f64, 30_f64).build());
        symetric_invert(builder.parallels_set(-35_f64, -50_f64).build());
        symetric_invert(
            builder
                .parallels_set(40_f64, 60_f64)
                .rotate2_set(&[-120_f64, 0_f64])
                .build(),
        );
    }

    #[test]
    fn conic_equidistant() {
        let mut builder = Equidistant::<DrainStub<f64>>::builder();
        symetric_invert(builder.build());
        symetric_invert(builder.parallels_set(20_f64, 30_f64).build());
        symetric_invert(builder.parallels_set(30_f64, 30_f64).build());
        symetric_invert(builder.parallels_set(-35_f64, -50_f64).build());
        symetric_invert(
            builder
                .parallels_set(40_f64, 60_f64)
                .rotate2_set(&[-120_f64, 0_f64])
                .build(),
        );
    }

    #[test]
    fn equirectangular() {
        let e = Equirectangular::<DrainStub<f64>, f64>::builder().build();
        symetric_invert(e);
    }

    #[test]
    fn equal_earth() {
        let e = EqualEarth::<DrainStub<f64>, f64>::builder().build();
        symetric_invert(e);
    }

    #[test]
    fn gnomic() {
        let g = Gnomic::<DrainStub<f64>, f64>::builder().build();
        symetric_invert(g);
    }

    #[test]
    fn mercator() {
        let m = Mercator::<DrainStub<f64>>::builder().build();
        symetric_invert(m);
    }

    #[test]
    fn mercator_traverse() {
        let m = MercatorTransverse::<DrainStub<f64>>::builder().build();
        symetric_invert(m);
    }

    #[test]
    fn orthographic() {
        let o = Orthographic::<DrainStub<f64>, f64>::builder().build();
        symetric_invert(o);
    }

    #[test]
    fn stereographic() {
        let s = Stereographic::<DrainStub<f64>, f64>::builder().build();
        symetric_invert(s);
    }

    #[ignore]
    #[test]
    fn albers_usa() {
        println!("albersUsa(point) and albersUsa.invert(point) are symmetric");

        let builder = AlbersUsa::<Multidrain<3, LastPoint<f64>, f64>>::builder();
        let mut projection = builder.build();

        let mut lp = Multidrain::<3, LastPoint<f64>, f64>::default();

        let mut transformer = projection.stream(&lp);

        for p in [
            Point(Coord {
                x: -122.4194_f64,
                y: 37.7749_f64,
            }),
            Point(Coord {
                x: -74.0059_f64,
                y: 40.7128_f64,
            }),
            Point(Coord {
                x: -149.9003_f64,
                y: 61.2181_f64,
            }),
            Point(Coord {
                x: -157.8583_f64,
                y: 21.3069_f64,
            }),
        ] {
            println!("built");
            // assert!(projection_equal(&ep, p, &ep.transform(p), None));
            p.to_stream(&mut transformer);

            let projected = lp.result().unwrap();

            dbg!(projected);
            assert!(false);
        }
    }
}
