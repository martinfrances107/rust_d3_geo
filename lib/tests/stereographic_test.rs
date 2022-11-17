#[cfg(not(tarpaulin_include))]
mod stereographic_tests {
    use geo_types::Coord;
    use rust_d3_geo::projection::ScaleSet;

    use rust_d3_geo::projection::equality::projection_equal;
    use rust_d3_geo::projection::projector::types::ProjectorCircleResampleNoClip;
    use rust_d3_geo::projection::stereographic::Stereographic;
    use rust_d3_geo::projection::Build;
    use rust_d3_geo::projection::RawBase;
    use rust_d3_geo::projection::TranslateSet;
    use rust_d3_geo::stream::DrainStub;

    #[test]
    fn stereographic() {
        let stereo: ProjectorCircleResampleNoClip<
            DrainStub<f64>,
            Stereographic<DrainStub<f64>, f64>,
            f64,
        > = Stereographic::builder()
            .translate_set(&Coord { x: 0f64, y: 0f64 })
            .scale_set(1f64)
            .build();

        assert!(projection_equal(
            &stereo,
            &Coord { x: 0_f64, y: 0_f64 },
            &Coord { x: 0_f64, y: 0_f64 },
            None
        ));
        assert!(projection_equal(
            &stereo,
            &Coord {
                x: -90_f64,
                y: 0_f64
            },
            &Coord {
                x: -1_f64,
                y: 0_f64
            },
            None
        ));
        assert!(projection_equal(
            &stereo,
            &Coord {
                x: 90_f64,
                y: 0_f64
            },
            &Coord { x: 1_f64, y: 0_f64 },
            None
        ));
        assert!(projection_equal(
            &stereo,
            &Coord {
                x: 0_f64,
                y: -90_f64
            },
            &Coord { x: 0_f64, y: 1_f64 },
            None
        ));
        assert!(projection_equal(
            &stereo,
            &Coord {
                x: 0_f64,
                y: 90_f64
            },
            &Coord {
                x: 0_f64,
                y: -1_f64
            },
            None
        ));
    }
}
