#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod stereographic_tests {
    use geo::Coordinate;
    use rust_d3_geo::projection::projection::Projection;
    use rust_d3_geo::projection::projection_equal::projection_equal;
    use rust_d3_geo::projection::stereographic::StereographicRaw;

    #[test]
    fn test_stereographic() {
        let mut stereo = StereographicRaw::gen_projection_mutator();
        stereo.translate(Some(&Coordinate { x: 0f64, y: 0f64 }));
        stereo.scale(1f64);

        assert!(projection_equal(
            &stereo,
            &Coordinate { x: 0f64, y: 0f64 },
            &Coordinate { x: 0f64, y: 0f64 },
            None
        ));
        assert!(projection_equal(
            &stereo,
            &Coordinate { x: -90f64, y: 0f64 },
            &Coordinate { x: -1f64, y: 0f64 },
            None
        ));
        assert!(projection_equal(
            &stereo,
            &Coordinate { x: 90f64, y: 0f64 },
            &Coordinate { x: 1f64, y: 0f64 },
            None
        ));
        assert!(projection_equal(
            &stereo,
            &Coordinate { x: 0f64, y: -90f64 },
            &Coordinate { x: 0f64, y: 1f64 },
            None
        ));
        assert!(projection_equal(
            &stereo,
            &Coordinate { x: 0f64, y: 90f64 },
            &Coordinate { x: 0f64, y: -1f64 },
            None
        ));
    }
}
