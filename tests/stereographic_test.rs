#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod stereographic_tests {
    use geo::Coordinate;

    use rust_d3_geo::clip::circle::line::Line;
    use rust_d3_geo::clip::circle::pv::PV;
    use rust_d3_geo::projection::projection::Projection;
    use rust_d3_geo::projection::projection_equal::projection_equal;
    use rust_d3_geo::projection::stereographic::Stereographic;
    use rust_d3_geo::projection::Raw;
    use rust_d3_geo::projection::Scale;
    use rust_d3_geo::projection::Translate;
    use rust_d3_geo::stream::StreamDrainStub;

    #[test]
    fn test_stereographic() {
        let stereo: Projection<
            StreamDrainStub<f64>,
            Line<f64>,
            Stereographic<StreamDrainStub<f64>, f64>,
            PV<f64>,
            f64,
        > = Stereographic::builder()
            .translate(&Coordinate { x: 0f64, y: 0f64 })
            .scale(1f64)
            .build();

        assert!(projection_equal(
            &stereo,
            &Coordinate { x: 0_f64, y: 0_f64 },
            &Coordinate { x: 0_f64, y: 0_f64 },
            None
        ));
        assert!(projection_equal(
            &stereo,
            &Coordinate {
                x: -90_f64,
                y: 0_f64
            },
            &Coordinate {
                x: -1_f64,
                y: 0_f64
            },
            None
        ));
        assert!(projection_equal(
            &stereo,
            &Coordinate {
                x: 90_f64,
                y: 0_f64
            },
            &Coordinate { x: 1_f64, y: 0_f64 },
            None
        ));
        assert!(projection_equal(
            &stereo,
            &Coordinate {
                x: 0_f64,
                y: -90_f64
            },
            &Coordinate { x: 0_f64, y: 1_f64 },
            None
        ));
        assert!(projection_equal(
            &stereo,
            &Coordinate {
                x: 0_f64,
                y: 90_f64
            },
            &Coordinate {
                x: 0_f64,
                y: -1_f64
            },
            None
        ));
    }
}
