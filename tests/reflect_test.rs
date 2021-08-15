#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod reflect_tests {
    use geo::Coordinate;
    use rust_d3_geo::clip::antimeridian::line::Line;
    use rust_d3_geo::clip::antimeridian::pv::PV;
    use rust_d3_geo::projection::builder::Builder;
    use rust_d3_geo::projection::gnomic::Gnomic;
    use rust_d3_geo::projection::projection_equal::projection_equal;
    use rust_d3_geo::projection::scale::Scale;
    use rust_d3_geo::projection::translate::Translate;
    use rust_d3_geo::stream::StreamDrainStub;

    #[test]
    fn test_reflect_x_defaults_to_false() {
        println!("projection.reflectX(…) defaults to false");

        let builder: Builder<StreamDrainStub<f64>, Line<f64>, Gnomic<f64>, PV<f64>, f64> =
            Gnomic::gen_projection_builder()
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
        println!("projection.reflectX(…) defaults to false");
        let mut builder: Builder<StreamDrainStub<f64>, Line<f64>, Gnomic<f64>, PV<f64>, f64> =
            Gnomic::gen_projection_builder()
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

        // projection = projection.reflect_x(false).reflect_y(true);
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
}
