mod distance_test {
  #[cfg(test)]
  extern crate pretty_assertions;

  use rust_d3_geo::distance::distance;
  use rust_d3_geo::in_delta::in_delta;

  // use rust_d3_geo::distance::distance;

  #[test]
  fn great_arc() {
    println!(
      "geoDistance(a, b) computes the great-arc distance in radians between the two points a and b"
    );
    assert_eq!(distance(&[0f64, 0f64], &[0f64, 0f64]), 0f64);
    assert!(in_delta(
      distance(
        &[118f64 + 24f64 / 60f64, 33f64 + 57f64 / 60f64],
        &[73f64 + 47f64 / 60f64, 40f64 + 38f64 / 60f64]
      ),
      3974f64 / 6371f64,
      0.5f64
    ));
  }

  // tape("geoDistance(a, b) computes the great-arc distance in radians between the two points a and b", function(test) {
  //   test.equal(d3.geoDistance([0, 0], [0, 0]), 0);
  //   test.inDelta(d3.geoDistance([118 + 24 / 60, 33 + 57 / 60], [73 + 47 / 60, 40 + 38 / 60]), 3973 / 6371, 0.5);
  //   test.end();
  // });

  // tape("geoDistance(a, b) correctly computes small distances", function(test) {
  //   test.assert(d3.geoDistance([0, 0], [0, 1e-12]) > 0);
  //   test.end();
  // });
}
