#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod reflect_tests {
    use geo::Coordinate;
    use rust_d3_geo::projection::builder_trait::BuilderTrait;
    use rust_d3_geo::projection::gnomic::Gnomic;
    use rust_d3_geo::projection::projection::Projection;
    use rust_d3_geo::projection::projection_equal::projection_equal;
    use rust_d3_geo::projection::scale::Scale;
    use rust_d3_geo::projection::translate::Translate;
    use rust_d3_geo::stream::StreamDrainStub;

    #[test]
    fn test_reflect_x_defaults_to_false() {
        println!("projection.reflectX(…) defaults to false");
        let projection = Gnomic::gen_projection_mutator()
            .scale(1f64)
            .translate(&Coordinate { x: 0_f64, y: 0_f64 });

        assert_eq!(projection.get_reflect_x(), false);
        assert_eq!(projection.get_reflect_y(), false);
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
        let mut projection = Gnomic::gen_projection_mutator()
            .scale(1_f64)
            .translate(&Coordinate { x: 0_f64, y: 0_f64 })
            .reflect_x(true);

        assert_eq!(projection.get_reflect_x(), true);

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

        projection = projection.reflect_x(false).reflect_y(true);

        assert_eq!(projection.get_reflect_x(), false);
        assert_eq!(projection.get_reflect_y(), true);

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
