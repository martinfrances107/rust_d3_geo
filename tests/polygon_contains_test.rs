mod polygon_contains_test {
  extern crate pretty_assertions;

  #[cfg(test)]
  use pretty_assertions::assert_eq;

  use num_traits::cast::FromPrimitive;
  use num_traits::Float;
  use num_traits::FloatConst;
  use std::fmt::Debug;

  use rust_d3_geo::circle::Circle;
  use rust_d3_geo::polygon_contains::contains;

  fn polygon_contains<F>(polygon_p: &Vec<Vec<[F; 2]>>, point: &[F; 2]) -> bool
  where
    F: Float + FloatConst + FromPrimitive + Debug,
  {
    let polygon = polygon_p.clone();
    let point_radians = |p: [F; 2]| [p[0].to_radians(), p[1].to_radians()];
    let ring_radians = |ring: Vec<[F; 2]>| {
      let mut rr = ring.into_iter().map(point_radians).collect::<Vec<[F; 2]>>();
      rr.pop();
      return rr;
    };

    let polygon_radians: Vec<Vec<[F; 2]>> = polygon.into_iter().map(ring_radians).collect();
    return contains(polygon_radians, &point_radians(*point));
  }

  // tape("geoPolygonContains(empty, point) returns false", function(test) {
  //   test.equal(polygonContains([], [0, 0]), 0);
  //   test.end();
  // });

  #[test]
  fn empty_return_false() {
    println!("geoPolygonContains(empty, point) returns false");
    let polygon: Vec<Vec<[f64; 2]>> = Vec::new();
    let contained = polygon_contains(&polygon, &[0f64, 0f64]);
    assert_eq!(contained, false);
  }

  // tape("geoPolygonContains(simple, point) returns the expected value", function(test) {
  //   var polygon = [[[0, 0], [0, 1], [1, 1], [1, 0], [0, 0]]];
  //   test.equal(polygonContains(polygon, [0.1, 2]), 0);
  //   test.equal(polygonContains(polygon, [0.1, 0.1]), 1);
  //   test.end();
  // });

  #[test]
  fn simple() {
    println!("geoPolygonContains(empty, point) returns the expecpted value");
    let ring: Vec<[f64; 2]> = vec![
      [0f64, 0f64],
      [0f64, 1f64],
      [1f64, 1f64],
      [1f64, 0f64],
      [0f64, 0f64],
    ];
    let mut polygon: Vec<Vec<[f64; 2]>> = Vec::new();
    polygon.push(ring);
    assert_eq!(polygon_contains(&polygon, &[0.1f64, 2f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0.1f64, 0.1f64]), true);
  }

  // tape("geoPolygonContains(smallCircle, point) returns the expected value", function(test) {
  //   var polygon = geoCircle().radius(60)().coordinates;
  //   test.equal(polygonContains(polygon, [-180, 0]), 0);
  //   test.equal(polygonContains(polygon, [1, 1]), 1);
  //   test.end();
  // });

  #[test]
  fn small_circle() {
    println!("geoPolygonContains(smallCircle, point) returns the expected value");

    let circle1 = Circle::new(Some([0f64, 0f64]), Some(60f64), None);
    let polygon = circle1.coordinates.clone();
    assert_eq!(polygon_contains(&polygon, &[-180f64, 0f64]), false);
    assert_eq!(polygon_contains(&polygon, &[1f64, 1f64]), true);
  }

  // tape("geoPolygonContains wraps longitudes", function(test) {
  //   var polygon = geoCircle().center([300, 0])().coordinates;
  //   test.equal(polygonContains(polygon, [300, 0]), 1);
  //   test.equal(polygonContains(polygon, [-60, 0]), 1);
  //   test.equal(polygonContains(polygon, [-420, 0]), 1);
  //   test.end();
  // });

  #[test]
  fn wraps_longitudes() {
    println!("geoPolygonContains wraps longitudes");

    let circle1 = Circle::new(Some([300f64, 0f64]), None, None);
    let polygon = circle1.coordinates.clone();
    assert_eq!(polygon_contains(&polygon, &[300f64, 0f64]), true);
    assert_eq!(polygon_contains(&polygon, &[-60f64, 0f64]), true);
    assert_eq!(polygon_contains(&polygon, &[-420f64, 0f64]), true);
  }

  // tape("geoPolygonContains(southPole, point) returns the expected value", function(test) {
  //   var polygon = [[[-60, -80], [60, -80], [180, -80], [-60, -80]]];
  //   test.equal(polygonContains(polygon, [0, 0]), 0);
  //   test.equal(polygonContains(polygon, [0, -85]), 1);
  //   test.equal(polygonContains(polygon, [0, -90]), 1);
  //   test.end();
  // });

  #[test]
  fn south_pole() {
    println!("geoPolygonContains(southPole, point) returns the expected value");
    let polygon = vec![vec![
      [-60f64, -80f64],
      [60f64, -80f64],
      [180f64, -80f64],
      [-60f64, -80f64],
    ]];
    assert_eq!(polygon_contains(&polygon, &[0f64, 0f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, -85f64]), true);
    assert_eq!(polygon_contains(&polygon, &[0f64, -90f64]), true);
  }

  // tape("geoPolygonContains(northPole, point) returns the expected value", function(test) {
  //   var polygon = [[[60, 80], [-60, 80], [-180, 80], [60, 80]]];
  //   test.equal(polygonContains(polygon, [0, 0]), 0);
  //   test.equal(polygonContains(polygon, [0, 85]), 1);
  //   test.equal(polygonContains(polygon, [0, 90]), 1);
  //   test.equal(polygonContains(polygon, [-100, 90]), 1);
  //   test.equal(polygonContains(polygon, [0, -90]), 0);
  //   test.end();
  // });

  #[test]
  fn north_pole() {
    println!("geoPolygonContains(southPole, point) returns the expected value");
    let polygon = vec![vec![
      [60f64, 80f64],
      [-60f64, 80f64],
      [-180f64, 80f64],
      [60f64, 80f64],
    ]];
    assert_eq!(polygon_contains(&polygon, &[0f64, 0f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, 85f64]), true);
    assert_eq!(polygon_contains(&polygon, &[0f64, 90f64]), true);
    assert_eq!(polygon_contains(&polygon, &[-100f64, 90f64]), true);
    assert_eq!(polygon_contains(&polygon, &[0f64, -90f64]), false);
  }

  // tape("geoPolygonContains(touchingPole, Pole) returns true (issue #105)", function(test) {
  //   var polygon = [[[0, -30], [120, -30], [0, -90], [0, -30]]];
  //   test.equal(polygonContains(polygon, [0, -90]), 0);
  //   test.equal(polygonContains(polygon, [-60, -90]), 0);
  //   test.equal(polygonContains(polygon, [60, -90]), 0);
  //   polygon = [[[0, 30], [-120, 30], [0, 90], [0, 30]]];
  //   test.equal(polygonContains(polygon, [0, 90]), 0);
  //   test.equal(polygonContains(polygon, [-60, 90]), 0);
  //   test.equal(polygonContains(polygon, [60, 90]), 0);
  //   test.end();
  // });

  #[test]
  fn touching_pole() {
    println!("geoPolygonContains(touchingPole, Pole) returns true (issue #105)");
    let polygon = vec![vec![
      [0f64, -30f64],
      [120f64, -30f64],
      [0f64, -90f64],
      [0f64, -30f64],
    ]];
    assert_eq!(polygon_contains(&polygon, &[0f64, -90f64]), false);
    assert_eq!(polygon_contains(&polygon, &[-60f64, -90f64]), false);
    assert_eq!(polygon_contains(&polygon, &[60f64, -90f64]), false);
    let polygon = vec![vec![
      [0f64, 30f64],
      [-120f64, 30f64],
      [0f64, 90f64],
      [0f64, 30f64],
    ]];
    assert_eq!(polygon_contains(&polygon, &[0f64, 90f64]), false);
    assert_eq!(polygon_contains(&polygon, &[-60f64, 90f64]), false);
    assert_eq!(polygon_contains(&polygon, &[60f64, 90f64]), false);
  }

  // tape("geoPolygonContains(southHemispherePoly) returns the expected value", function(test) {
  //   var polygon = [[[0, 0], [10, -40], [-10, -40], [0, 0]]];
  //   test.equal(polygonContains(polygon, [0,-40.2]), 1);
  //   test.equal(polygonContains(polygon, [0,-40.5]), 0);
  //   test.end();
  // });

  #[test]
  fn south_hemisphere_poly() {
    println!("geoPolygonContains(southHemispherePoly) returns the expected value");
    let polygon = vec![vec![
      [0f64, 0f64],
      [10f64, -40f64],
      [-10f64, -40f64],
      [0f64, 0f64],
    ]];
    assert_eq!(polygon_contains(&polygon, &[0f64, -40.2f64]), true);
    assert_eq!(polygon_contains(&polygon, &[-60f64, -40.5f64]), false);
  }

  // tape("geoPolygonContains(largeNearOrigin, point) returns the expected value", function(test) {
  //   var polygon = [[[0, 0], [1, 0], [1, 1], [0, 1], [0, 0]]];
  //   test.equal(polygonContains(polygon, [0.1, 0.1]), 0);
  //   test.equal(polygonContains(polygon, [2, 0.1]), 1);
  //   test.end();
  // });

  #[test]
  fn large_near_origin() {
    println!("geoPolygonContains(largeNearOrigin, point) returns the expected value");
    let polygon = vec![vec![
      [0f64, 0f64],
      [1f64, 0f64],
      [1f64, 1f64],
      [0f64, 1f64],
      [0f64, 0f64],
    ]];
    assert_eq!(polygon_contains(&polygon, &[0.1f64, 0.1f64]), false);
    assert_eq!(polygon_contains(&polygon, &[2.0f64, 0.1f64]), true);
  }

  // tape("geoPolygonContains(largeNearSouthPole, point) returns the expected value", function(test) {
  //   var polygon = [[[-60, 80], [60, 80], [180, 80], [-60, 80]]];
  //   test.equal(polygonContains(polygon, [0, 85]), 0);
  //   test.equal(polygonContains(polygon, [0, 0]), 1);
  //   test.end();
  // });

  #[test]
  fn large_near_south_pole() {
    println!("geoPolygonContains(largeNearSouthPole, point) returns the expected value");
    let ring: Vec<[f64; 2]> = vec![
      [-60f64, 80f64],
      [60f64, 80f64],
      [180f64, 80f64],
      [-60f64, 80f64],
    ];
    let mut polygon: Vec<Vec<[f64; 2]>> = Vec::new();
    polygon.push(ring);
    assert_eq!(polygon_contains(&polygon, &[0.0, 85.0]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, 0f64]), true);
  }

  // tape("geoPolygonContains(largeNearNorthPole, point) returns the expected value", function(test) {
  //   var polygon = [[[60, -80], [-60, -80], [-180, -80], [60, -80]]];
  //   test.equal(polygonContains(polygon, [0, -85]), 0);
  //   test.equal(polygonContains(polygon, [0, 0]), 1);
  //   test.end();
  // });

  #[test]
  fn large_near_north_pole() {
    println!("geoPolygonContains(largeNearSouthPole, point) returns the expected value");
    let ring: Vec<[f64; 2]> = vec![
      [60f64, -80f64],
      [-60f64, -80f64],
      [-180f64, -80f64],
      [60f64, -80f64],
    ];
    let mut polygon: Vec<Vec<[f64; 2]>> = Vec::new();
    polygon.push(ring);
    assert_eq!(polygon_contains(&polygon, &[0.0, -85.0]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, 0f64]), true);
  }

  // tape("geoPolygonContains(largeCircle, point) returns the expected value", function(test) {
  //   var polygon = geoCircle().radius(120)().coordinates;
  //   test.equal(polygonContains(polygon, [-180, 0]), 0);
  //   test.equal(polygonContains(polygon, [-90, 0]), 1);
  //   test.end();
  // });

  #[test]
  fn large_circle() {
    println!("geoPolygonContains(largeCircle, point) returns the expected value");
    let circle1 = Circle::new(None, Some(120f64), None);
    let polygon = circle1.coordinates.clone();
    assert_eq!(polygon_contains(&polygon, &[-180f64, 0f64]), false);
    assert_eq!(polygon_contains(&polygon, &[-90f64, 0f64]), true);
  }

  // tape("geoPolygonContains(largeNarrowStripHole, point) returns the expected value", function(test) {
  //   var polygon = [[[-170, -1], [0, -1], [170, -1], [170, 1], [0, 1], [-170, 1], [-170, -1]]];
  //   test.equal(polygonContains(polygon, [0, 0]), 0);
  //   test.equal(polygonContains(polygon, [0, 20]), 1);
  //   test.end();
  // });

  #[test]
  fn large_narrow_strip_hole() {
    println!("geoPolygonContains(largeNearSouthPole, point) returns the expected value");
    let ring: Vec<[f64; 2]> = vec![
      [-170f64, -1f64],
      [0f64, -1f64],
      [170f64, -1f64],
      [170f64, 1f64],
      [0f64, 1f64],
      [-170f64, 1f64],
      [-170f64, -1f64],
    ];
    let mut polygon: Vec<Vec<[f64; 2]>> = Vec::new();
    polygon.push(ring);
    assert_eq!(polygon_contains(&polygon, &[0.0, 0.0]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, 20f64]), true);
  }

  // tape("geoPolygonContains(largeNarrowEquatorialHole, point) returns the expected value", function(test) {
  //   var circle = geoCircle().center([0, -90]),
  //       ring0 = circle.radius(90 - 0.01)().coordinates[0],
  //       ring1 = circle.radius(90 + 0.01)().coordinates[0].reverse(),
  //       polygon = [ring0, ring1];
  //   test.equal(polygonContains(polygon, [0, 0]), 0);
  //   test.equal(polygonContains(polygon, [0, -90]), 1);
  //   test.end();
  // });

  #[test]
  fn large_narrow_equatorial_hole() {
    println!("geoPolygonContains(empty, point) returns false");
    let circle1 = Circle::new(Some([0f64, -90f64]), Some(90f64 - 0.1f64), None);
    let ring0 = circle1.coordinates[0].clone();

    let circle2 = Circle::new(Some([0f64, -90f64]), Some(90f64 + 0.1f64), None);
    let mut ring1 = circle2.coordinates[0].clone();
    ring1.reverse();
    let mut polygon: Vec<Vec<[f64; 2]>> = Vec::new();
    polygon.push(ring0);
    polygon.push(ring1);

    assert_eq!(polygon_contains(&polygon, &[0f64, 0f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, -90f64]), true);
  }

  // tape("geoPolygonContains(largeNarrowEquatorialStrip, point) returns the expected value", function(test) {
  //   var circle = geoCircle().center([0, -90]),
  //       ring0 = circle.radius(90 + 0.01)().coordinates[0],
  //       ring1 = circle.radius(90 - 0.01)().coordinates[0].reverse(),
  //       polygon = [ring0, ring1];
  //   test.equal(polygonContains(polygon, [0, -90]), 0);
  //   test.equal(polygonContains(polygon, [0, 0]), 1);
  //   test.end();
  // });

  #[test]
  fn large_narrow_equatorial_strip() {
    println!("geoPolygonContains(empty, point) returns false");
    let circle1 = Circle::new(Some([0f64, -90f64]), Some(90f64 + 0.1f64), None);
    let ring0 = circle1.coordinates[0].clone();

    let circle2 = Circle::new(Some([0f64, -90f64]), Some(90f64 - 0.1f64), None);
    let mut ring1 = circle2.coordinates[0].clone();
    ring1.reverse();
    let mut polygon: Vec<Vec<[f64; 2]>> = Vec::new();
    polygon.push(ring0);
    polygon.push(ring1);

    assert_eq!(polygon_contains(&polygon, &[0f64, -90f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, 0f64]), true);
  }

  // tape("geoPolygonContains(ringNearOrigin, point) returns the expected value", function(test) {
  //   var ring0 = [[0, 0], [0, 1], [1, 1], [1, 0], [0, 0]],
  //       ring1 = [[0.4, 0.4], [0.6, 0.4], [0.6, 0.6], [0.4, 0.6], [0.4, 0.4]],
  //       polygon = [ring0, ring1];
  //   test.equal(polygonContains(polygon, [0.5, 0.5]), 0);
  //   test.equal(polygonContains(polygon, [0.1, 0.5]), 1);
  //   test.end();
  // });

  #[test]
  fn ring_near_origin() {
    println!("geoPolygonContains(ringNearOrigin, point) returns the expected value");
    let ring0 = vec![
      [0f64, 0f64],
      [0f64, 1f64],
      [1f64, 1f64],
      [1f64, 0f64],
      [0f64, 0f64],
    ];
    let ring1 = vec![
      [0.4f64, 0.4f64],
      [0.6f64, 0.4f64],
      [0.6f64, 0.6f64],
      [0.4f64, 0.6f64],
      [0.4f64, 0.4f64],
    ];
    let polygon = vec![ring0, ring1];

    assert_eq!(polygon_contains(&polygon, &[0.5f64, 0.5f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0.1f64, 0.5f64]), true);
  }

  // tape("geoPolygonContains(ringEquatorial, point) returns the expected value", function(test) {
  //   var ring0 = [[0, -10], [-120, -10], [120, -10], [0, -10]],
  //       ring1 = [[0, 10], [120, 10], [-120, 10], [0, 10]],
  //       polygon = [ring0, ring1];
  //   test.equal(polygonContains(polygon, [0, 20]), 0);
  //   test.equal(polygonContains(polygon, [0, 0]), 1);
  //   test.end();
  // });

  #[test]
  fn ring_equatorial() {
    println!("geoPolygonContains(ringEquatorial, point) returns the expected value");
    let ring0 = vec![
      [0f64, -10f64],
      [-120f64, -10f64],
      [120f64, -10f64],
      [0f64, -10f64],
    ];
    let ring1 = vec![
      [0f64, 10f64],
      [120f64, 10f64],
      [-120f64, 10f64],
      [0f64, 10f64],
    ];
    let polygon = vec![ring0, ring1];

    assert_eq!(polygon_contains(&polygon, &[0f64, 20f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, 0f64]), true);
  }

  // tape("geoPolygonContains(ringExcludingBothPoles, point) returns the expected value", function(test) {
  //   var ring0 = [[10, 10], [-10, 10], [-10, -10], [10, -10], [10, 10]].reverse(),
  //       ring1 = [[170, 10], [170, -10], [-170, -10], [-170, 10], [170, 10]].reverse(),
  //       polygon = [ring0, ring1];
  //   test.equal(polygonContains(polygon, [0, 90]), 0);
  //   test.equal(polygonContains(polygon, [0, 0]), 1);
  //   test.end();
  // });

  #[test]
  fn ring_excluding_both_poles() {
    println!("geoPolygonContains(ringExcludingBothPoles, point) returns the expected value");
    let mut ring0 = vec![
      [10f64, 10f64],
      [-10f64, 10f64],
      [-10f64, -10f64],
      [10f64, -10f64],
      [10f64, 10f64],
    ];
    ring0.reverse();
    let mut ring1 = vec![
      [170f64, 10f64],
      [170f64, -10f64],
      [-170f64, -10f64],
      [-170f64, 10f64],
      [170f64, 10f64],
    ];
    ring1.reverse();
    let polygon = vec![ring0, ring1];
    assert_eq!(polygon_contains(&polygon, &[0f64, 90f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, 0f64]), true);
  }

  // tape("geoPolygonContains(ringContainingBothPoles, point) returns the expected value", function(test) {
  //   var ring0 = [[10, 10], [-10, 10], [-10, -10], [10, -10], [10, 10]],
  //       ring1 = [[170, 10], [170, -10], [-170, -10], [-170, 10], [170, 10]],
  //       polygon = [ring0, ring1];
  //   test.equal(polygonContains(polygon, [0, 0]), 0);
  //   test.equal(polygonContains(polygon, [0, 20]), 1);
  //   test.end();
  // });

  #[test]
  fn ring_containing_both_poles() {
    println!("geoPolygonContains(ringExcludingBothPoles, point) returns the expected value");
    let ring0 = vec![
      [10f64, 10f64],
      [-10f64, 10f64],
      [-10f64, -10f64],
      [10f64, -10f64],
      [10f64, 10f64],
    ];
    let ring1 = vec![
      [170f64, 10f64],
      [170f64, -10f64],
      [-170f64, -10f64],
      [-170f64, 10f64],
      [170f64, 10f64],
    ];
    let polygon = vec![ring0, ring1];

    assert_eq!(polygon_contains(&polygon, &[0f64, 0f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, 20f64]), true);
  }

  // tape("geoPolygonContains(ringContainingSouthPole, point) returns the expected value", function(test) {
  //   var ring0 = [[10, 10], [-10, 10], [-10, -10], [10, -10], [10, 10]],
  //       ring1 = [[0, 80], [120, 80], [-120, 80], [0, 80]],
  //       polygon = [ring0, ring1];
  //   test.equal(polygonContains(polygon, [0, 90]), 0);
  //   test.equal(polygonContains(polygon, [0, -90]), 1);
  //   test.end();
  // });

  #[test]
  fn ring_containing_south_pole() {
    println!("geoPolygonContains(ringContainingSouthPole, point) returns the expected value");
    let ring0 = vec![
      [10f64, 10f64],
      [-10f64, 10f64],
      [-10f64, -10f64],
      [10f64, -10f64],
      [10f64, 10f64],
    ];
    let ring1 = vec![
      [0f64, 80f64],
      [120f64, 80f64],
      [-120f64, 80f64],
      [0f64, 80f64],
    ];
    let polygon = vec![ring0, ring1];

    assert_eq!(polygon_contains(&polygon, &[0f64, 90f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, -90f64]), true);
  }

  // tape("geoPolygonContains(ringContainingNorthPole, point) returns the expected value", function(test) {
  //   var ring0 = [[10, 10], [-10, 10], [-10, -10], [10, -10], [10, 10]].reverse(),
  //       ring1 = [[0, 80], [120, 80], [-120, 80], [0, 80]].reverse(),
  //       polygon = [ring0, ring1];
  //   test.equal(polygonContains(polygon, [0, -90]), 0);
  //   test.equal(polygonContains(polygon, [0, 90]), 1);
  //   test.end();
  // });

  #[test]
  fn ring_containing_north_pole() {
    println!("geoPolygonContains(ringContainingNorthPole, point) returns the expected value");
    let mut ring0 = vec![
      [10f64, 10f64],
      [-10f64, 10f64],
      [-10f64, -10f64],
      [10f64, -10f64],
      [10f64, 10f64],
    ];
    ring0.reverse();
    let mut ring1 = vec![
      [0f64, 80f64],
      [120f64, 80f64],
      [-120f64, 80f64],
      [0f64, 80f64],
    ];
    ring1.reverse();
    let polygon = vec![ring0, ring1];

    assert_eq!(polygon_contains(&polygon, &[0f64, -90f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, 90f64]), true);
  }

  // tape("geoPolygonContains(selfIntersectingNearOrigin, point) returns the expected value", function(test) {
  //   var polygon = [[[0, 0], [1, 0], [1, 3], [3, 3], [3, 1], [0, 1], [0, 0]]];
  //   test.equal(polygonContains(polygon, [15, 0.5]), 0);
  //   test.equal(polygonContains(polygon, [12, 2]), 0);
  //   test.equal(polygonContains(polygon, [0.5, 0.5]), 1);
  //   test.equal(polygonContains(polygon, [2, 2]), 1);
  //   test.end();
  // });

  #[test]
  fn self_intersecting_near_origin() {
    println!("geoPolygonContains(selfIntersectingNearOrigin, point) returns the expected value");
    let polygon = vec![vec![
      [0f64, 0f64],
      [1f64, 0f64],
      [1f64, 3f64],
      [3f64, 3f64],
      [3f64, 1f64],
      [0f64, 1f64],
      [0f64, 0f64],
    ]];
    assert_eq!(polygon_contains(&polygon, &[15f64, 0.5f64]), false);
    assert_eq!(polygon_contains(&polygon, &[12f64, 2f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0.5f64, 0.5f64]), true);
    assert_eq!(polygon_contains(&polygon, &[2f64, 2f64]), true);
  }

  // tape("geoPolygonContains(selfIntersectingNearSouthPole, point) returns the expected value", function(test) {
  //   var polygon = [[[-10, -80], [120, -80], [-120, -80], [10, -85], [10, -75], [-10, -75], [-10, -80]]];
  //   test.equal(polygonContains(polygon, [0, 0]), 0);
  //   test.equal(polygonContains(polygon, [0, -76]), 1);
  //   test.equal(polygonContains(polygon, [0, -89]), 1);
  //   test.end();
  // });

  #[test]
  fn self_intersecting_near_south_pole() {
    println!("geoPolygonContains(selfIntersectingNearSouthPole, point) returns the expected value");
    let polygon = vec![vec![
      [-10f64, -80f64],
      [120f64, -80f64],
      [-120f64, -80f64],
      [10f64, -85f64],
      [10f64, -75f64],
      [-10f64, 75f64],
      [-10f64, -80f64],
    ]];
    assert_eq!(polygon_contains(&polygon, &[0f64, 0f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, -76f64]), true);
    assert_eq!(polygon_contains(&polygon, &[0f64, -89f64]), true);
  }

  // tape("geoPolygonContains(selfIntersectingNearNorthPole, point) returns the expected value", function(test) {
  //   var polygon = [[[-10, 80], [-10, 75], [10, 75], [10, 85], [-120, 80], [120, 80], [-10, 80]]];
  //   test.equal(polygonContains(polygon, [0, 0]), 0);
  //   test.equal(polygonContains(polygon, [0, 76]), 1);
  //   test.equal(polygonContains(polygon, [0, 89]), 1);
  //   test.end();
  // });

  #[test]
  fn self_intersecting_near_north_pole() {
    println!("geoPolygonContains(selfIntersectingNearNorthPole, point) returns the expected value");
    let polygon = vec![vec![
      [-10f64, 80f64],
      [-10f64, 75f64],
      [10f64, 75f64],
      [10f64, 85f64],
      [-120f64, 80f64],
      [120f64, 80f64],
      [-10f64, 80f64],
    ]];
    assert_eq!(polygon_contains(&polygon, &[0f64, 0f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, 76f64]), true);
    assert_eq!(polygon_contains(&polygon, &[0f64, 89f64]), true);
  }

  // tape("geoPolygonContains(hemisphereTouchingTheSouthPole, point) returns the expected value", function(test) {
  //   var polygon = geoCircle().radius(90)().coordinates;
  //   test.equal(polygonContains(polygon, [0, 0]), 1);
  //   test.end();
  // });

  // #[test]
  // fn hemisphere_touching_the_south_pole() {
  //   /// circle
  // }

  // tape("geoPolygonContains(triangleTouchingTheSouthPole, point) returns the expected value", function(test) {
  //   var polygon = [[[180, -90], [-45, 0], [45, 0], [180, -90]]];
  //   test.equal(polygonContains(polygon, [-46, 0]), 0);
  //   test.equal(polygonContains(polygon, [0, 1]), 0);
  //   test.equal(polygonContains(polygon, [-90, -80]), 0);
  //   test.equal(polygonContains(polygon, [-44, 0]), 1);
  //   test.equal(polygonContains(polygon, [0, 0]), 1);
  //   test.equal(polygonContains(polygon, [0, -30]), 1);
  //   test.equal(polygonContains(polygon, [30, -80]), 1);
  //   test.end();
  // });

  #[test]
  fn triangle_touching_the_south_pole() {
    println!("geoPolygonContains(triangleTouchingTheSouthPole, point) returns the expected value");
    let polygon = vec![vec![
      [180f64, -90f64],
      [-45f64, 0f64],
      [45f64, 0f64],
      [180f64, -90f64],
    ]];
    assert_eq!(polygon_contains(&polygon, &[-46f64, 0f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, 1f64]), false);
    assert_eq!(polygon_contains(&polygon, &[-90f64, -80f64]), false);
    assert_eq!(polygon_contains(&polygon, &[-44f64, 0f64]), true);
    assert_eq!(polygon_contains(&polygon, &[0f64, 0f64]), true);
    assert_eq!(polygon_contains(&polygon, &[0f64, -30f64]), true);
    assert_eq!(polygon_contains(&polygon, &[30f64, -80f64]), true);
  }

  // tape("geoPolygonContains(triangleTouchingTheSouthPole2, point) returns the expected value", function(test) {
  //   var polygon = [[[-45, 0], [45, 0], [180, -90], [-45, 0]]];
  //   test.equal(polygonContains(polygon, [-46, 0]), 0);
  //   test.equal(polygonContains(polygon, [0, 1]), 0);
  //   test.equal(polygonContains(polygon, [-90, -80]), 0);
  //   test.equal(polygonContains(polygon, [-44, 0]), 1);
  //   test.equal(polygonContains(polygon, [0, 0]), 1);
  //   test.equal(polygonContains(polygon, [0, -30]), 1);
  //   test.equal(polygonContains(polygon, [30, -80]), 1);
  //   test.end();
  // });

  #[test]
  fn triangle_touching_the_south_pole2() {
    println!("geoPolygonContains(triangleTouchingTheSouthPole2, point) returns the expected value");
    let polygon = vec![vec![
      [-45f64, 0f64],
      [45f64, 0f64],
      [180f64, -90f64],
      [-45f64, 0f64],
    ]];
    assert_eq!(polygon_contains(&polygon, &[-46f64, 0f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, 1f64]), false);
    assert_eq!(polygon_contains(&polygon, &[-90f64, -80f64]), false);
    assert_eq!(polygon_contains(&polygon, &[-44f64, 0f64]), true);
    assert_eq!(polygon_contains(&polygon, &[0f64, 0f64]), true);
    assert_eq!(polygon_contains(&polygon, &[0f64, -30f64]), true);
    assert_eq!(polygon_contains(&polygon, &[30f64, -80f64]), true);
  }

  // tape("geoPolygonContains(triangleTouchingTheSouthPole3, point) returns the expected value", function(test) {
  //   var polygon = [[[180, -90], [-135, 0], [135, 0], [180, -90]]];
  //   test.equal(polygonContains(polygon, [180, 0]), 0);
  //   test.equal(polygonContains(polygon, [150, 0]), 0);
  //   test.equal(polygonContains(polygon, [180, -30]), 0);
  //   test.equal(polygonContains(polygon, [150, -80]), 0);
  //   test.equal(polygonContains(polygon, [0, 0]), 1);
  //   test.equal(polygonContains(polygon, [180, 1]), 1);
  //   test.equal(polygonContains(polygon, [-90, -80]), 1);
  //   test.end();
  // });

  #[test]
  fn triangle_touching_the_south_pole3() {
    println!("geoPolygonContains(triangleTouchingTheSouthPole2, point) returns the expected value");
    let polygon = vec![vec![
      [180f64, -90f64],
      [-135f64, 0f64],
      [135f64, 0f64],
      [180f64, -90f64],
    ]];
    assert_eq!(polygon_contains(&polygon, &[180f64, 0f64]), false);
    assert_eq!(polygon_contains(&polygon, &[150f64, 0f64]), false);
    assert_eq!(polygon_contains(&polygon, &[180f64, -30f64]), false);
    assert_eq!(polygon_contains(&polygon, &[150f64, -80f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, 0f64]), true);
    assert_eq!(polygon_contains(&polygon, &[180f64, 1f64]), true);
    assert_eq!(polygon_contains(&polygon, &[-90f64, -80f64]), true);
  }

  // tape("geoPolygonContains(triangleTouchingTheNorthPole, point) returns the expected value", function(test) {
  //   var polygon = [[[180, 90], [45, 0], [-45, 0], [180, 90]]];
  //   test.equal(polygonContains(polygon, [-90, 0]), 0);
  //   test.equal(polygonContains(polygon, [0, -1]), 0);
  //   test.equal(polygonContains(polygon, [0, -80]), 0);
  //   test.equal(polygonContains(polygon, [-90, 1]), 0);
  //   test.equal(polygonContains(polygon, [-90, 80]), 0);
  //   test.equal(polygonContains(polygon, [-44, 10]), 1);
  //   test.equal(polygonContains(polygon, [0, 10]), 1);
  //   test.equal(polygonContains(polygon, [30, 80]), 1);
  //   test.end();
  // });

  #[test]
  fn triangle_touching_the_north_pole() {
    println!("geoPolygonContains(triangleTouchingTheSouthPole2, point) returns the expected value");
    let polygon = vec![vec![
      [180f64, 90f64],
      [45f64, 0f64],
      [-45f64, 0f64],
      [180f64, 90f64],
    ]];
    assert_eq!(polygon_contains(&polygon, &[-90f64, 0f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, -1f64]), false);
    assert_eq!(polygon_contains(&polygon, &[0f64, -80f64]), false);
    assert_eq!(polygon_contains(&polygon, &[-90f64, 1f64]), false);
    assert_eq!(polygon_contains(&polygon, &[-90f64, 80f64]), false);
    assert_eq!(polygon_contains(&polygon, &[-44f64, 10f64]), true);
    assert_eq!(polygon_contains(&polygon, &[0f64, 10f64]), true);
    assert_eq!(polygon_contains(&polygon, &[30f64, 80f64]), true);
  }
}
