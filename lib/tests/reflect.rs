#[cfg(not(tarpaulin_include))]
mod reflect {

    extern crate pretty_assertions;

    use geo_types::Coord;
    use pretty_assertions::assert_eq;

    use d3_geo_rs::in_delta::in_delta;
    use d3_geo_rs::projection::builder::Builder;
    use d3_geo_rs::projection::equality::projection_equal;
    use d3_geo_rs::projection::gnomic::Gnomic;
    use d3_geo_rs::projection::mercator::Mercator;
    use d3_geo_rs::projection::AngleGet;
    use d3_geo_rs::projection::AngleSet;
    use d3_geo_rs::projection::Build;
    use d3_geo_rs::projection::RawBase;
    use d3_geo_rs::projection::Reflect;
    use d3_geo_rs::projection::ReflectGet;
    use d3_geo_rs::projection::ReflectSet;
    use d3_geo_rs::projection::ScaleSet;
    use d3_geo_rs::projection::TranslateSet;
    use d3_geo_rs::stream::DrainStub;
    use d3_geo_rs::Transform;

    #[test]
    fn x_defaults_to_false() {
        println!("projection.reflectX(…) defaults to false");

        let mut builder: Builder<_, DrainStub<f64>, _, _, _, f64> =
            Gnomic::builder();
        builder.scale_set(1f64);
        builder.translate_set(&Coord { x: 0_f64, y: 0_f64 });

        assert_eq!(builder.is_x_reflected(), false);
        assert_eq!(builder.is_y_reflected(), false);

        let projection = builder.build();
        assert!(projection_equal(
            &projection,
            &Coord { x: 0_f64, y: 0_f64 },
            &Coord { x: 0_f64, y: 0_f64 },
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
                x: 0_f64,
                y: 10_f64
            },
            &Coord {
                x: 0_f64,
                y: -0.17632698070846498_f64
            },
            None
        ));
    }

    #[test]
    fn mirrors_x_after_processing() {
        println!("projection.reflectX(…) mirrors x after projecting");
        let mut builder: Builder<_, DrainStub<f64>, _, _, _, f64> =
            Gnomic::builder();
        builder.scale_set(1_f64);
        builder.translate_set(&Coord { x: 0_f64, y: 0_f64 });

        builder.reflect_x_set(Reflect::Flipped);

        assert_eq!(builder.is_x_reflected(), true);

        let projection = builder.build();

        assert!(projection_equal(
            &projection,
            &Coord { x: 0_f64, y: 0_f64 },
            &Coord { x: 0_f64, y: 0_f64 },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coord {
                x: 10_f64,
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

        builder
            .reflect_x_set(Reflect::Unflipped)
            .reflect_y_set(Reflect::Flipped);
        let projection = builder.build();
        assert_eq!(builder.is_x_reflected(), false);
        assert_eq!(builder.is_y_reflected(), true);

        assert!(projection_equal(
            &projection,
            &Coord { x: 0_f64, y: 0_f64 },
            &Coord { x: 0_f64, y: 0_f64 },
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
                x: 0_f64,
                y: 10_f64
            },
            &Coord {
                x: 0_f64,
                y: 0.17632698070846498_f64
            },
            None
        ));
    }

    #[test]
    fn x_works_with_projection_angle() {
        println!("projection.reflectX(…) works with projection.angle()");
        let mut builder = Mercator::builder::<DrainStub<f64>>();
        builder.scale_set(1_f64).translate_set(&Coord {
            x: 10_f64,
            y: 20_f64,
        });

        builder.reflect_x_set(Reflect::Flipped).angle_set(45_f64);

        assert_eq!(builder.is_x_reflected(), true);
        assert!(in_delta(45_f64, builder.angle(), 1e-6));
        let p = builder.build();
        assert_eq!(
            p.transform(&Coord { x: 0_f64, y: 0_f64 }),
            Coord {
                x: 10_f64,
                y: 20_f64
            }
        );
        assert_eq!(
            p.transform(&Coord {
                x: 10_f64,
                y: 0_f64
            }),
            Coord {
                x: 9.876586585051157_f64,
                y: 20.123413414948843_f64
            }
        );
        assert_eq!(
            p.transform(&Coord {
                x: 0_f64,
                y: 10_f64
            }),
            Coord {
                x: 9.875955206257924_f64,
                y: 19.875955206257924_f64
            }
        );
    }
}
