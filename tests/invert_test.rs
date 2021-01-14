#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod innvert_test {
    extern crate pretty_assertions;

    use geo::Coordinate;
    use rust_d3_geo::projection::orthographic::OrthographicRaw;
    use rust_d3_geo::projection::projection_equal::projection_equal;
    use rust_d3_geo::projection::stereographic::StereographicRaw;
    use rust_d3_geo::Transform;

    #[test]
    fn symetric_invert() {
        let projectors = vec![
            StereographicRaw::gen_projection_mutator(),
            OrthographicRaw::gen_projection_mutator(),
        ];

        for pm in projectors {
            for p in vec![
                Coordinate {
                    x: 0.0f64,
                    y: 0.0f64,
                },
                Coordinate {
                    x: 30.3f64,
                    y: 24.1f64,
                },
                Coordinate {
                    x: -10f64,
                    y: 42f64,
                },
                Coordinate {
                    x: -2.0f64,
                    y: -5.0f64,
                },
            ] {
                assert!(projection_equal(&pm, &p, &pm.transform(&p), None));
            }
        }
    }
}
