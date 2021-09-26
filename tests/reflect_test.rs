#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod reflect_tests {
    use geo::Coordinate;

    use rust_d3_geo::clip::circle::line::Line;
    use rust_d3_geo::in_delta::in_delta;
    use rust_d3_geo::clip::circle::pv::PV;
    use rust_d3_geo::projection::builder::Builder;
    use rust_d3_geo::projection::gnomic::Gnomic;
    use rust_d3_geo::projection::mercator::Mercator;
    use rust_d3_geo::projection::mercator_builder::MercatorBuilder;
    use rust_d3_geo::projection::projection_equal::projection_equal;
    use rust_d3_geo::projection::Raw;
    use rust_d3_geo::projection::Scale;
    use rust_d3_geo::projection::Angle;
    use rust_d3_geo::projection::Reflect;
    use rust_d3_geo::projection::Translate;
    use rust_d3_geo::Transform;
    use rust_d3_geo::stream::StreamDrainStub;

    #[test]
    fn test_reflect_x_defaults_to_false() {
        println!("projection.reflectX(…) defaults to false");

        let builder: Builder<
            StreamDrainStub<f64>,
            Line<f64>,
            Gnomic<StreamDrainStub<f64>, f64>,
            PV<f64>,
            f64,
        > = Gnomic::builder()
            .scale(1f64)
            .translate(&Coordinate { x: 0_f64, y: 0_f64 });

        assert_eq!(builder.get_reflect_x(), false);
        assert_eq!(builder.get_reflect_y(), false);

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
    fn test_reflect_mirrors_x_after_processing() {
        println!("projection.reflectX(…) mirrors x after projecting");
        let mut builder: Builder<
            StreamDrainStub<f64>,
            Line<f64>,
            Gnomic<StreamDrainStub<f64>, f64>,
            PV<f64>,
            f64,
        > = Gnomic::builder()
            .scale(1_f64)
            .translate(&Coordinate { x: 0_f64, y: 0_f64 })
            .reflect_x(true);

        assert_eq!(builder.get_reflect_x(), true);

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

        builder = builder.reflect_x(false).reflect_y(true);
        let projection = builder.build();
        assert_eq!(builder.get_reflect_x(), false);
        assert_eq!(builder.get_reflect_y(), true);

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
    fn reflect_x_works_with_projection_angle(){
        println!("projection.reflectX(…) works with projection.angle()");
        let builder = Mercator::builder()
            .scale(1_f32)
            .translate(&Coordinate { x: 10_f32, y: 20_f32 })
            .reflect_x(true).angle(45_f32);

            assert_eq!( builder.get_reflect_x(), true);
            assert!(in_delta(45_f32, builder.get_angle(), 1e-6));
            let p = builder.build();
            assert_eq!(p.transform(&Coordinate{x: 0_f32, y: 0_f32}), Coordinate{x: 10_f32, y:20_f32});
            assert_eq!(p.transform(&Coordinate{x: 10_f32, y: 0_f32}), Coordinate{x: 9.87658658_f32, y:20.12341341_f32});
            assert_eq!(p.transform(&Coordinate{x: 0_f32, y: 10_f32}), Coordinate{x: 9.87595521_f32, y:19.87595521_f32});

    }
}
