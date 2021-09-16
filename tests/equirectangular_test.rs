#[cfg(not(tarpaulin_include))]
#[cfg(test)]

mod equirectangular_test {

    use geo::Coordinate;

    use rust_d3_geo::clip::antimeridian::gen_clip_factory_antimeridian;
    use rust_d3_geo::clip::antimeridian::line::Line;
    use rust_d3_geo::clip::antimeridian::pv::PV;
    use rust_d3_geo::projection::builder::Builder;
    use rust_d3_geo::projection::equirectangular::EquirectangularRaw;
    use rust_d3_geo::projection::projection::Projection;
    use rust_d3_geo::projection::projection_equal::projection_equal;
    use rust_d3_geo::projection::Raw;
    use rust_d3_geo::projection::Scale;
    use rust_d3_geo::projection::Translate;
    use rust_d3_geo::stream::StreamDrainStub;

    #[test]
    fn test_return_expected_result() {
        println!("equirectangular(point) returns the expected result");
        let equirectangular: Projection<
            StreamDrainStub<f64>,
            Line<f64>,
            EquirectangularRaw<StreamDrainStub<f64>, f64>,
            PV<f64>,
            f64,
        > = Builder::new(
            gen_clip_factory_antimeridian(),
            EquirectangularRaw::default(),
        )
        .translate(&Coordinate { x: 0f64, y: 0f64 })
        .scale(1_f64)
        .build();

        let pi = std::f64::consts::PI;

        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 0f64, y: 0f64 },
            &Coordinate { x: 0f64, y: 0f64 },
            None
        ));

        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: -180f64,
                y: 0f64
            },
            &Coordinate { x: -pi, y: 0f64 },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 180f64, y: 0f64 },
            &Coordinate { x: pi, y: 0f64 },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 0f64, y: 30f64 },
            &Coordinate {
                x: 0f64,
                y: -pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 0f64, y: -30f64 },
            &Coordinate {
                x: 0f64,
                y: pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 30f64, y: 30f64 },
            &Coordinate {
                x: pi / 6f64,
                y: -pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: 30f64,
                y: -30f64
            },
            &Coordinate {
                x: pi / 6f64,
                y: pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: -30f64,
                y: 30f64
            },
            &Coordinate {
                x: -pi / 6f64,
                y: -pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: -30f64,
                y: -30f64
            },
            &Coordinate {
                x: -pi / 6f64,
                y: pi / 6f64
            },
            None
        ));
    }

    #[test]
    fn test_rotate_30_0() {
        println!("equirectangular(point) returns the expected result");
        let equirectangular: Projection<
            StreamDrainStub<f64>,
            Line<f64>,
            EquirectangularRaw<StreamDrainStub<f64>, f64>,
            PV<f64>,
            f64,
        > = Builder::new(
            gen_clip_factory_antimeridian(),
            EquirectangularRaw::default(),
        )
        .rotate([30f64, 0f64, 0f64])
        .translate(&Coordinate { x: 0f64, y: 0f64 })
        .scale(1_f64)
        .build();

        let pi = std::f64::consts::PI;

        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 0f64, y: 0f64 },
            &Coordinate {
                x: pi / 6f64,
                y: 0f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: -180f64,
                y: 0f64
            },
            &Coordinate {
                x: -5f64 / 6f64 * pi,
                y: 0f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 180f64, y: 0f64 },
            &Coordinate {
                x: -5f64 / 6f64 * pi,
                y: 0f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 0f64, y: 30f64 },
            &Coordinate {
                x: pi / 6f64,
                y: -pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 0f64, y: -30f64 },
            &Coordinate {
                x: pi / 6f64,
                y: pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 30f64, y: 30f64 },
            &Coordinate {
                x: pi / 3f64,
                y: -pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: 30f64,
                y: -30f64
            },
            &Coordinate {
                x: pi / 3f64,
                y: pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: -30f64,
                y: 30f64
            },
            &Coordinate {
                x: 0f64,
                y: -pi / 6f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: -30f64,
                y: -30f64
            },
            &Coordinate {
                x: 0f64,
                y: pi / 6f64
            },
            None
        ));
    }
    #[test]
    fn test_rotate_30_30() {
        println!("equirectangular.rotate([30, 30])(point) returns the expected result");
        let equirectangular: Projection<
            StreamDrainStub<f64>,
            Line<f64>,
            EquirectangularRaw<StreamDrainStub<f64>, f64>,
            PV<f64>,
            f64,
        > = EquirectangularRaw::builder()
            .rotate([30f64, 30f64, 0f64])
            .translate(&Coordinate { x: 0f64, y: 0f64 })
            .scale(1_f64)
            .build();

        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 0f64, y: 0f64 },
            &Coordinate {
                x: 0.5880026035475674,
                y: -0.44783239692893245
            },
            None
        ));

        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: -180f64,
                y: 0f64
            },
            &Coordinate {
                x: -2.5535900500422257f64,
                y: 0.44783239692893245
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 180f64, y: 0f64 },
            &Coordinate {
                x: -2.5535900500422257f64,
                y: 0.44783239692893245
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 0f64, y: 30f64 },
            &Coordinate {
                x: 0.8256075561643480f64,
                y: -0.94077119517052080
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 0f64, y: -30f64 },
            &Coordinate {
                x: 0.4486429615608479f64,
                y: 0.05804529130778048
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 30f64, y: 30f64 },
            &Coordinate {
                x: 1.4056476493802694f64,
                y: -0.70695172788721770
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: 30f64,
                y: -30f64
            },
            &Coordinate {
                x: 0.8760580505981933f64,
                y: 0.21823451436745955
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: -30f64,
                y: 30f64
            },
            &Coordinate {
                x: 0.0000000000000000f64,
                y: -1.04719755119659760
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: -30f64,
                y: -30f64
            },
            &Coordinate {
                x: 0.0000000000000000f64,
                y: 0.00000000000000000
            },
            None
        ));
    }

    #[test]
    fn test_rotate_0_0_30() {
        println!("equirectangular.rotate([0, 0, 30])(point) returns the expected result");
        let equirectangular: Projection<
            StreamDrainStub<f64>,
            Line<f64>,
            EquirectangularRaw<StreamDrainStub<f64>, f64>,
            PV<f64>,
            f64,
        > = EquirectangularRaw::builder()
            .rotate([0f64, 0f64, 30f64])
            .translate(&Coordinate { x: 0f64, y: 0f64 })
            .scale(1f64)
            .build();

        let pi = std::f64::consts::PI;

        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 0f64, y: 0f64 },
            &Coordinate { x: 0f64, y: 0f64 },
            None
        ));

        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: -180f64,
                y: 0f64
            },
            &Coordinate { x: -pi, y: 0f64 },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 180f64, y: 0f64 },
            &Coordinate { x: pi, y: 0f64 },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 0f64, y: 30f64 },
            &Coordinate {
                x: -0.2810349015028135f64,
                y: -0.44783239692893245f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 0f64, y: -30f64 },
            &Coordinate {
                x: 0.2810349015028135f64,
                y: 0.44783239692893245
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 30f64, y: 30f64 },
            &Coordinate {
                x: 0.1651486774146268f64,
                y: -0.70695172788721760f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: 30_f64,
                y: -30_f64
            },
            &Coordinate {
                x: 0.6947382761967031_f64,
                y: 0.21823451436745964_f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: -30_f64,
                y: 30_f64
            },
            &Coordinate {
                x: -0.6947382761967031f64,
                y: -0.21823451436745964f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: -30f64,
                y: -30f64
            },
            &Coordinate {
                x: -0.1651486774146268f64,
                y: 0.70695172788721760f64
            },
            None
        ));
    }

    #[test]
    fn test_rotate_30_30_30() {
        println!("equirectangular.rotate([30, 30, 30])(point) returns the expected result");
        let equirectangular: Projection<
            StreamDrainStub<f64>,
            Line<f64>,
            EquirectangularRaw<StreamDrainStub<f64>, f64>,
            PV<f64>,
            f64,
        > = EquirectangularRaw::builder()
            .rotate([30f64, 30f64, 30f64])
            .translate(&Coordinate { x: 0f64, y: 0f64 })
            .scale(1f64)
            .build();

        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 0f64, y: 0f64 },
            &Coordinate {
                x: 0.2810349015028135f64,
                y: -0.67513153293703170f64
            },
            None
        ));

        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: -180f64,
                y: 0f64
            },
            &Coordinate {
                x: -2.8605577520869800f64,
                y: 0.67513153293703170f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 180f64, y: 0f64 },
            &Coordinate {
                x: -2.8605577520869800f64,
                y: 0.67513153293703170f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 0f64, y: 30f64 },
            &Coordinate {
                x: -0.0724760059270816f64,
                y: -1.15865677086597720f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 0f64, y: -30f64 },
            &Coordinate {
                x: 0.4221351552567053f64,
                y: -0.16704161863132252f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate { x: 30f64, y: 30f64 },
            &Coordinate {
                x: 1.2033744221750944f64,
                y: -1.21537512510467320f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: 30f64,
                y: -30f64
            },
            &Coordinate {
                x: 0.8811235701944905f64,
                y: -0.18861638617540410f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: -30f64,
                y: 30f64
            },
            &Coordinate {
                x: -0.7137243789447654f64,
                y: -0.84806207898148100f64
            },
            None
        ));
        assert!(projection_equal(
            &equirectangular,
            &Coordinate {
                x: -30f64,
                y: -30f64
            },
            &Coordinate { x: 0f64, y: 0f64 },
            None
        ));
    }
}
