#[cfg(not(tarpaulin_include))]
mod stereographic_tests {
    use geo::Coordinate;
    use rust_d3_geo::projection::ScaleSet;

    use rust_d3_geo::projection::projection_equal::projection_equal;
    use rust_d3_geo::projection::projector::types::ProjectorCircleResampleNoClip;
    use rust_d3_geo::projection::stereographic::Stereographic;
    use rust_d3_geo::projection::Build;
    use rust_d3_geo::projection::ProjectionRawBase;
    use rust_d3_geo::projection::TranslateSet;
    use rust_d3_geo::stream::DrainStub;

    #[test]
    fn stereographic() {
        let stereo: ProjectorCircleResampleNoClip<
            DrainStub<f64>,
            Stereographic<DrainStub<f64>, f64>,
            f64,
        > = Stereographic::builder()
            .translate_set(&Coordinate { x: 0f64, y: 0f64 })
            .scale_set(1f64)
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
