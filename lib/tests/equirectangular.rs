#[cfg(not(tarpaulin_include))]
#[allow(clippy::excessive_precision)]
mod equirectangular {

    use geo_types::Coord;

    use d3_geo_rs::projection::builder::Builder;
    use d3_geo_rs::projection::equality::projection_equal;
    use d3_geo_rs::projection::equirectangular::Equirectangular;
    use d3_geo_rs::projection::projector_common::types::ProjectorAntimeridianResampleNoClip;
    use d3_geo_rs::projection::Build;
    use d3_geo_rs::projection::BuilderTrait;
    use d3_geo_rs::projection::RawBase;
    use d3_geo_rs::projection::RotateSet;
    use d3_geo_rs::projection::ScaleSet;
    use d3_geo_rs::projection::TranslateSet;
    use d3_geo_rs::stream::DrainStub;

    #[test]
    fn return_expected_result() {
        println!("equirectangular(point) returns the expected result");
        let mut b: Builder<_, DrainStub<f64>, _, _, _, _> = Equirectangular::builder();
        b.translate_set(&Coord { x: 0f64, y: 0f64 });
        b.scale_set(1_f64);

        let equirectangular = b.build();

        let pi = std::f64::consts::PI;

        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 0f64, y: 0f64 },
            &Coord { x: 0f64, y: 0f64 },
            None
        ));

        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: -180f64,
                y: 0f64
            },
            &Coord { x: -pi, y: 0f64 },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 180f64, y: 0f64 },
            &Coord { x: pi, y: 0f64 },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 0f64, y: 30f64 },
            &Coord {
                x: 0f64,
                y: -pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 0f64, y: -30f64 },
            &Coord {
                x: 0f64,
                y: pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 30f64, y: 30f64 },
            &Coord {
                x: pi / 6f64,
                y: -pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: 30f64,
                y: -30f64
            },
            &Coord {
                x: pi / 6f64,
                y: pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: -30f64,
                y: 30f64
            },
            &Coord {
                x: -pi / 6f64,
                y: -pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: -30f64,
                y: -30f64
            },
            &Coord {
                x: -pi / 6f64,
                y: pi / 6f64
            },
            None
        ));
    }

    #[test]
    fn rotate_30_0() {
        println!("equirectangular(point) returns the expected result");
        let equirectangular: ProjectorAntimeridianResampleNoClip<
            DrainStub<f64>,
            Equirectangular<f64>,
            f64,
        > = Builder::new(Equirectangular::<f64>::default())
            .rotate2_set(&[30f64, 0f64])
            .translate_set(&Coord { x: 0f64, y: 0f64 })
            .scale_set(1_f64)
            .build();

        let pi = std::f64::consts::PI;

        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 0f64, y: 0f64 },
            &Coord {
                x: pi / 6f64,
                y: 0f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: -180f64,
                y: 0f64
            },
            &Coord {
                x: -5f64 / 6f64 * pi,
                y: 0f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 180f64, y: 0f64 },
            &Coord {
                x: -5f64 / 6f64 * pi,
                y: 0f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 0f64, y: 30f64 },
            &Coord {
                x: pi / 6f64,
                y: -pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 0f64, y: -30f64 },
            &Coord {
                x: pi / 6f64,
                y: pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 30f64, y: 30f64 },
            &Coord {
                x: pi / 3f64,
                y: -pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: 30f64,
                y: -30f64
            },
            &Coord {
                x: pi / 3f64,
                y: pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: -30f64,
                y: 30f64
            },
            &Coord {
                x: 0f64,
                y: -pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: -30f64,
                y: -30f64
            },
            &Coord {
                x: 0f64,
                y: pi / 6f64
            },
            None
        ));
    }

    #[test]
    #[allow(clippy::excessive_precision)]
    fn rotate_30_30() {
        println!("equirectangular.rotate([30, 30])(point) returns the expected result");
        let mut b: Builder<_, DrainStub<f64>, _, _, _, _> = Equirectangular::builder();

        b.rotate2_set(&[30f64, 30f64]);
        b.translate_set(&Coord { x: 0f64, y: 0f64 });
        b.scale_set(1_f64);

        let equirectangular = b.build();

        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 0f64, y: 0f64 },
            &Coord {
                x: 0.5880026035475674,
                y: -0.44783239692893245
            },
            None
        ));

        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: -180f64,
                y: 0f64
            },
            &Coord {
                x: -2.5535900500422257f64,
                y: 0.44783239692893245
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 180f64, y: 0f64 },
            &Coord {
                x: -2.5535900500422257f64,
                y: 0.44783239692893245
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 0f64, y: 30f64 },
            &Coord {
                x: 0.8256075561643480f64,
                y: -0.94077119517052080
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 0f64, y: -30f64 },
            &Coord {
                x: 0.4486429615608479f64,
                y: 0.05804529130778048
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 30f64, y: 30f64 },
            &Coord {
                x: 1.4056476493802694f64,
                y: -0.70695172788721770
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: 30f64,
                y: -30f64
            },
            &Coord {
                x: 0.8760580505981933f64,
                y: 0.21823451436745955
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: -30f64,
                y: 30f64
            },
            &Coord {
                x: 0.0000000000000000f64,
                y: -1.04719755119659760
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: -30f64,
                y: -30f64
            },
            &Coord {
                x: 0.0000000000000000f64,
                y: 0.00000000000000000
            },
            None
        ));
    }

    #[test]
    fn rotate_0_0_30() {
        println!("equirectangular.rotate([0, 0, 30])(point) returns the expected result");

        let mut b: Builder<_, DrainStub<f64>, _, _, _, _> = Equirectangular::builder();
        b.rotate3_set(&[0f64, 0f64, 30f64]);
        b.translate_set(&Coord { x: 0f64, y: 0f64 });
        b.scale_set(1f64);

        let equirectangular = b.build();

        let pi = std::f64::consts::PI;

        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 0f64, y: 0f64 },
            &Coord { x: 0f64, y: 0f64 },
            None
        ));

        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: -180f64,
                y: 0f64
            },
            &Coord { x: -pi, y: 0f64 },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 180f64, y: 0f64 },
            &Coord { x: pi, y: 0f64 },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 0f64, y: 30f64 },
            &Coord {
                x: -0.2810349015028135f64,
                y: -0.44783239692893245f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 0f64, y: -30f64 },
            &Coord {
                x: 0.2810349015028135f64,
                y: 0.44783239692893245
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 30f64, y: 30f64 },
            &Coord {
                x: 0.1651486774146268f64,
                y: -0.70695172788721760f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: 30_f64,
                y: -30_f64
            },
            &Coord {
                x: 0.6947382761967031_f64,
                y: 0.21823451436745964_f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: -30_f64,
                y: 30_f64
            },
            &Coord {
                x: -0.6947382761967031f64,
                y: -0.21823451436745964f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: -30f64,
                y: -30f64
            },
            &Coord {
                x: -0.1651486774146268f64,
                y: 0.70695172788721760f64
            },
            None
        ));
    }

    #[test]
    fn rotate_30_30_30() {
        println!("equirectangular.rotate([30, 30, 30])(point) returns the expected result");
        let mut b: Builder<_, DrainStub<f64>, _, _, _, _> = Equirectangular::builder();
        b.rotate3_set(&[30f64, 30f64, 30f64]);
        b.translate_set(&Coord { x: 0f64, y: 0f64 });
        b.scale_set(1f64);

        let equirectangular = b.build();

        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 0f64, y: 0f64 },
            &Coord {
                x: 0.2810349015028135f64,
                y: -0.67513153293703170f64
            },
            None
        ));

        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: -180f64,
                y: 0f64
            },
            &Coord {
                x: -2.8605577520869800f64,
                y: 0.67513153293703170f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 180f64, y: 0f64 },
            &Coord {
                x: -2.8605577520869800f64,
                y: 0.67513153293703170f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 0f64, y: 30f64 },
            &Coord {
                x: -0.0724760059270816f64,
                y: -1.15865677086597720f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 0f64, y: -30f64 },
            &Coord {
                x: 0.4221351552567053f64,
                y: -0.16704161863132252f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord { x: 30f64, y: 30f64 },
            &Coord {
                x: 1.2033744221750944f64,
                y: -1.21537512510467320f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: 30f64,
                y: -30f64
            },
            &Coord {
                x: 0.8811235701944905f64,
                y: -0.18861638617540410f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: -30f64,
                y: 30f64
            },
            &Coord {
                x: -0.7137243789447654f64,
                y: -0.84806207898148100f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coord {
                x: -30f64,
                y: -30f64
            },
            &Coord { x: 0f64, y: 0f64 },
            None
        ));
    }
}
