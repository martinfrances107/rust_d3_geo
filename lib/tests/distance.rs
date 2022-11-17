#[cfg(not(tarpaulin_include))]
mod distance {
    extern crate pretty_assertions;

    use geo_types::Coord;
    use pretty_assertions::assert_eq;
    use rust_d3_geo::distance::distance;
    use rust_d3_geo::in_delta::in_delta;

    #[test]
    fn great_arc() {
        println!(
      "geoDistance(a, b) computes the great-arc distance in radians between the two points a and b."
    );
        assert_eq!(
            distance(&Coord { x: 0f64, y: 0f64 }, &Coord { x: 0f64, y: 0f64 }),
            0f64
        );
        assert!(in_delta(
            distance(
                &Coord {
                    x: 118f64 + 24f64 / 60f64,
                    y: 33f64 + 57f64 / 60f64
                },
                &Coord {
                    x: 73f64 + 47f64 / 60f64,
                    y: 40f64 + 38f64 / 60f64
                }
            ),
            3974f64 / 6371f64,
            0.5f64
        ));
    }

    #[test]
    fn small_distances() {
        println!("geoDistance(a, b) correctly computes small distances.");
        assert!(distance(&Coord { x: 0f64, y: 0f64 }, &Coord { x: 0f64, y: 1e-12 }) > 0f64);
    }
}
