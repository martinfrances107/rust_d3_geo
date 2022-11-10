#[cfg(not(tarpaulin_include))]
mod reflect {

    extern crate pretty_assertions;

    use geo::Coordinate;
    use pretty_assertions::assert_eq;

    use rust_d3_geo::clip::circle::ClipCircleC;
    use rust_d3_geo::clip::circle::ClipCircleU;
    use rust_d3_geo::identity::Identity;
    use rust_d3_geo::in_delta::in_delta;
    use rust_d3_geo::projection::builder::template::ResampleNoPCNC;
    use rust_d3_geo::projection::builder::template::ResampleNoPCNU;
    use rust_d3_geo::projection::builder::Builder;
    use rust_d3_geo::projection::builder_mercator::Builder as MercatorBuilder;
    use rust_d3_geo::projection::gnomic::Gnomic;
    use rust_d3_geo::projection::mercator::Mercator;
    use rust_d3_geo::projection::projection_equal::projection_equal;
    use rust_d3_geo::projection::AngleGet;
    use rust_d3_geo::projection::AngleSet;
    use rust_d3_geo::projection::Build;
    use rust_d3_geo::projection::ProjectionRawBase;
    use rust_d3_geo::projection::ReflectGet;
    use rust_d3_geo::projection::ReflectSet;
    use rust_d3_geo::projection::ScaleSet;
    use rust_d3_geo::projection::TranslateSet;
    use rust_d3_geo::projection::REFLECT;
    use rust_d3_geo::stream::DrainStub;
    use rust_d3_geo::stream::Unconnected;
    use rust_d3_geo::Transform;

    type GB = Builder<
        ClipCircleC<ResampleNoPCNC<DrainStub<f64>, Gnomic<DrainStub<f64>, f64>, f64>, f64>,
        ClipCircleU<ResampleNoPCNC<DrainStub<f64>, Gnomic<DrainStub<f64>, f64>, f64>, f64>,
        DrainStub<f64>,
        Identity<Unconnected>,
        Gnomic<DrainStub<f64>, f64>,
        ResampleNoPCNC<DrainStub<f64>, Gnomic<DrainStub<f64>, f64>, f64>,
        ResampleNoPCNU<Gnomic<DrainStub<f64>, f64>, f64>,
        f64,
    >;

    #[test]
    fn x_defaults_to_false() {
        println!("projection.reflectX(…) defaults to false");

        let mut builder: GB = Gnomic::builder();
        builder.scale_set(1f64);
        builder.translate_set(&Coordinate { x: 0_f64, y: 0_f64 });

        assert_eq!(builder.is_x_reflected(), false);
        assert_eq!(builder.is_y_reflected(), false);

        let projection = builder.build();
        assert!(projection_equal(
            &projection,
            &Coordinate { x: 0_f64, y: 0_f64 },
            &Coordinate { x: 0_f64, y: 0_f64 },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coordinate {
                x: 10_f64,
                y: 0_f64
            },
            &Coordinate {
                x: 0.17632698070846498_f64,
                y: 0_f64
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coordinate {
                x: 0_f64,
                y: 10_f64
            },
            &Coordinate {
                x: 0_f64,
                y: -0.17632698070846498_f64
            },
            None
        ));
    }

    #[test]
    fn mirrors_x_after_processing() {
        println!("projection.reflectX(…) mirrors x after projecting");
        let mut builder: GB = Gnomic::builder();
        builder.scale_set(1_f64);
        builder.translate_set(&Coordinate { x: 0_f64, y: 0_f64 });

        builder.reflect_x_set(REFLECT::Flipped);

        assert_eq!(builder.is_x_reflected(), true);

        let projection = builder.build();

        assert!(projection_equal(
            &projection,
            &Coordinate { x: 0_f64, y: 0_f64 },
            &Coordinate { x: 0_f64, y: 0_f64 },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coordinate {
                x: 10_f64,
                y: 0_f64
            },
            &Coordinate {
                x: -0.17632698070846498_f64,
                y: 0_f64
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coordinate {
                x: 0_f64,
                y: 10_f64
            },
            &Coordinate {
                x: 0_f64,
                y: -0.17632698070846498_f64
            },
            None
        ));

        builder
            .reflect_x_set(REFLECT::Unflipped)
            .reflect_y_set(REFLECT::Flipped);
        let projection = builder.build();
        assert_eq!(builder.is_x_reflected(), false);
        assert_eq!(builder.is_y_reflected(), true);

        assert!(projection_equal(
            &projection,
            &Coordinate { x: 0_f64, y: 0_f64 },
            &Coordinate { x: 0_f64, y: 0_f64 },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coordinate {
                x: 10_f64,
                y: 0_f64
            },
            &Coordinate {
                x: 0.17632698070846498_f64,
                y: 0_f64
            },
            None
        ));

        assert!(projection_equal(
            &projection,
            &Coordinate {
                x: 0_f64,
                y: 10_f64
            },
            &Coordinate {
                x: 0_f64,
                y: 0.17632698070846498_f64
            },
            None
        ));
    }

    #[test]
    fn x_works_with_projection_angle() {
        println!("projection.reflectX(…) works with projection.angle()");
        let mut builder: MercatorBuilder<_, _, DrainStub<f64>, _, _, _, _, f64> =
            Mercator::builder();
        builder.scale_set(1_f64).translate_set(&Coordinate {
            x: 10_f64,
            y: 20_f64,
        });

        builder.reflect_x_set(REFLECT::Flipped).angle_set(45_f64);

        assert_eq!(builder.is_x_reflected(), true);
        assert!(in_delta(45_f64, builder.angle(), 1e-6));
        let p = builder.build();
        assert_eq!(
            p.transform(&Coordinate { x: 0_f64, y: 0_f64 }),
            Coordinate {
                x: 10_f64,
                y: 20_f64
            }
        );
        assert_eq!(
            p.transform(&Coordinate {
                x: 10_f64,
                y: 0_f64
            }),
            Coordinate {
                x: 9.876586585051157_f64,
                y: 20.123413414948843_f64
            }
        );
        assert_eq!(
            p.transform(&Coordinate {
                x: 0_f64,
                y: 10_f64
            }),
            Coordinate {
                x: 9.875955206257924_f64,
                y: 19.875955206257924_f64
            }
        );
    }
}
