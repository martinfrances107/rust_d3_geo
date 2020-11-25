#[cfg(test)]
mod innvert_test {
    extern crate pretty_assertions;

    use delaunator::Point;

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
                Point {
                    x: 0.0f64,
                    y: 0.0f64,
                },
                Point {
                    x: 30.3f64,
                    y: 24.1f64,
                },
                Point {
                    x: -10f64,
                    y: 42f64,
                },
                Point {
                    x: -2.0f64,
                    y: -5.0f64,
                },
            ] {
                assert!(projection_equal(&pm, &p, &pm.transform(&p), None));
            }
        }
    }
}
