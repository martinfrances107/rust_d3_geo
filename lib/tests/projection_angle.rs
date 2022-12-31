#[cfg(not(tarpaulin_include))]
mod projection_angle {

    use geo_types::Coord;
    use pretty_assertions::assert_eq;

    use d3_geo_rs::in_delta::in_delta;
    use d3_geo_rs::projection::builder::template::NoPCNC;
    use d3_geo_rs::projection::builder::template::NoPCNU;
    use d3_geo_rs::projection::builder_identity::Builder as BuilderIdentity;
    use d3_geo_rs::projection::equality::projection_equal;
    use d3_geo_rs::projection::gnomic::Gnomic;
    use d3_geo_rs::projection::AngleGet;
    use d3_geo_rs::projection::AngleSet;
    use d3_geo_rs::projection::Build;
    use d3_geo_rs::projection::RawBase;
    use d3_geo_rs::projection::ScaleSet;
    use d3_geo_rs::projection::TranslateSet;
    use d3_geo_rs::stream::DrainStub;
    use d3_geo_rs::Transform;

    #[test]
    fn angle_defaults_to_zero() {
        println!("projection.angle(…) defaults to zero");
        let mut pb = Gnomic::<DrainStub<f64>, f64>::builder();
        pb.scale_set(1_f64);
        pb.translate_set(&Coord { x: 0_f64, y: 0_f64 });
        assert_eq!(pb.angle(), 0_f64);
        let projection = pb.build();

        assert!(projection_equal(
            &projection,
            &Coord { x: 0f64, y: 0f64 },
            &Coord { x: 0f64, y: 0f64 },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: 10_f64,
                y: 0_f64
            },
            &Coord {
                x: 0.17632698070846498_f64,
                y: 0_f64
            },
            None
        ));
        assert!(projection_equal(
            &projection,
            &Coord {
                x: -10_f64,
                y: 0_f64
            },
            &Coord {
                x: -0.17632698070846498_f64,
                y: 0_f64
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: 0_f64,
                y: 10_f64
            },
            &Coord {
                x: 0_f64,
                y: -0.17632698070846498_f64
            },
            None
        ));
        assert!(projection_equal(
            &projection,
            &Coord {
                x: 0_f64,
                y: -10_f64
            },
            &Coord {
                x: 0_f64,
                y: 0.17632698070846498_f64
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: 10_f64,
                y: 10_f64
            },
            &Coord {
                x: 0.17632698070846495_f64,
                y: -0.17904710860483972_f64
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: 10_f64,
                y: -10_f64
            },
            &Coord {
                x: 0.17632698070846495_f64,
                y: 0.17904710860483972_f64
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: -10_f64,
                y: 10_f64
            },
            &Coord {
                x: -0.17632698070846495_f64,
                y: -0.17904710860483972_f64
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: -10_f64,
                y: -10_f64
            },
            &Coord {
                x: -0.17632698070846495_f64,
                y: 0.17904710860483972_f64
            },
            None
        ));
    }

    #[test]
    fn angle_rotates_by_plus_30() {
        println!("projection.angle(…) defaults to zero");
        let mut pb = Gnomic::<DrainStub<f64>, f64>::builder();
        pb.scale_set(1_f64)
            .translate_set(&Coord { x: 0_f64, y: 0_f64 });
        let pb = pb.angle_set(30_f64);

        // this rounds to 29.9999999 not 30!!
        // assert_eq!(pb.get_angle(), 30_f64);
        assert!(in_delta(pb.angle(), 30_f64, 1e-6));
        let projection = pb.build();

        assert!(projection_equal(
            &projection,
            &Coord { x: 0f64, y: 0f64 },
            &Coord { x: 0f64, y: 0f64 },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: 10_f64,
                y: 0_f64
            },
            &Coord {
                x: 0.1527036446661393_f64,
                y: -0.08816349035423247_f64
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: -10_f64,
                y: 0_f64
            },
            &Coord {
                x: -0.1527036446661393_f64,
                y: 0.08816349035423247_f64
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: 0_f64,
                y: 10_f64
            },
            &Coord {
                x: -0.08816349035423247_f64,
                y: -0.1527036446661393_f64
            },
            None
        ));
        assert!(projection_equal(
            &projection,
            &Coord {
                x: 0_f64,
                y: -10_f64
            },
            &Coord {
                x: 0.08816349035423247_f64,
                y: 0.1527036446661393_f64
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: 10_f64,
                y: 10_f64
            },
            &Coord {
                x: 0.06318009036371944_f64,
                y: -0.24322283488017502_f64
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: 10_f64,
                y: -10_f64
            },
            &Coord {
                x: 0.24222719896855913_f64,
                y: 0.0668958541717101
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: -10_f64,
                y: 10_f64
            },
            &Coord {
                x: -0.24222719896855913_f64,
                y: -0.0668958541717101
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: -10_f64,
                y: -10_f64
            },
            &Coord {
                x: -0.06318009036371944_f64,
                y: 0.24322283488017502
            },
            None
        ));
    }

    #[test]
    fn angle_rotates_by_minus_30() {
        println!("projection.angle(…) defaults to zero");
        let mut pb = Gnomic::<DrainStub<f64>, f64>::builder();
        pb.scale_set(1_f64)
            .translate_set(&Coord { x: 0_f64, y: 0_f64 });

        pb.angle_set(-30_f64);

        // this rounds to 29.9999999 not 30!!
        assert!(in_delta(pb.angle(), -30_f64, 1e-6));
        let projection = pb.build();

        assert!(projection_equal(
            &projection,
            &Coord { x: 0f64, y: 0f64 },
            &Coord { x: 0f64, y: 0f64 },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: 10_f64,
                y: 0_f64
            },
            &Coord {
                x: 0.1527036446661393_f64,
                y: 0.08816349035423247_f64
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: -10_f64,
                y: 0_f64
            },
            &Coord {
                x: -0.1527036446661393_f64,
                y: -0.08816349035423247_f64
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: 0_f64,
                y: 10_f64
            },
            &Coord {
                x: 0.08816349035423247_f64,
                y: -0.1527036446661393_f64
            },
            None
        ));
        assert!(projection_equal(
            &projection,
            &Coord {
                x: 0_f64,
                y: -10_f64
            },
            &Coord {
                x: -0.08816349035423247_f64,
                y: 0.1527036446661393_f64
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: 10_f64,
                y: 10_f64
            },
            &Coord {
                x: 0.24222719896855913_f64,
                y: -0.0668958541717101_f64
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: 10_f64,
                y: -10_f64
            },
            &Coord {
                x: 0.06318009036371944_f64,
                y: 0.24322283488017502_f64,
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: -10_f64,
                y: 10_f64
            },
            &Coord {
                x: -0.06318009036371944_f64,
                y: -0.24322283488017502_f64
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: -10_f64,
                y: -10_f64
            },
            &Coord {
                x: -0.24222719896855913_f64,
                y: 0.0668958541717101_f64
            },
            None
        ));
    }

    #[test]
    fn wraps_360() {
        println!("projection.angle(…) wraps around 360°");
        let mut pb = Gnomic::<DrainStub<f64>, f64>::builder();
        pb.scale_set(1_f64);
        pb.translate_set(&Coord { x: 0_f64, y: 0_f64 });
        pb.angle_set(360_f64);

        assert!(in_delta(pb.angle(), 0_f64, 1e-6));
    }

    #[test]
    // Using f32 as f64 has rounding errors in the last digit.
    fn rotates_geo_identity() {
        println!("identity.angle(…) rotates geoIdentity");

        let mut pb: d3_geo_rs::projection::builder_identity::Builder<DrainStub<f32>, NoPCNU, f32> =
            BuilderIdentity::default();
        pb.angle_set(-45_f32);

        let sqrt2_2 = 2f32.sqrt() / 2f32;

        let projector = pb.build::<NoPCNC<DrainStub<f32>>>();

        assert_eq!(
            projector.transform(&Coord { x: 0f32, y: 0f32 }),
            Coord { x: 0f32, y: 0f32 }
        );

        assert_eq!(
            projector.transform(&Coord { x: 1f32, y: 0f32 }),
            Coord {
                x: sqrt2_2,
                y: sqrt2_2
            }
        );
        assert_eq!(
            projector.transform(&Coord { x: -1f32, y: 0f32 }),
            Coord {
                x: -sqrt2_2,
                y: -sqrt2_2
            }
        );
        assert_eq!(
            projector.transform(&Coord { x: 0f32, y: 1f32 }),
            Coord {
                x: -sqrt2_2,
                y: sqrt2_2
            }
        );
    }
}
