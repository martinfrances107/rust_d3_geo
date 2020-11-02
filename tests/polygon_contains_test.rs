mod polygon_contains_test {
    extern crate pretty_assertions;

    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::rc::Rc;

    use delaunator::Point;

    use rust_d3_geo::circle::circle::Circle;
    use rust_d3_geo::circle::CircleInArg;
    use rust_d3_geo::circle::CircleTrait;
    use rust_d3_geo::circle::FnValMaybe;
    use rust_d3_geo::circle::FnValMaybe2D;

    use rust_d3_geo::polygon_contains::contains;

    fn polygon_contains(polygon_p: &Vec<Vec<Point>>, point: &Point) -> bool {
        let point_radians = |p: Point| Point {
            x: p.x.to_radians(),
            y: p.y.to_radians(),
        };
        let ring_radians = |ring: Vec<Point>| {
            let mut rr = ring.into_iter().map(point_radians).collect::<Vec<Point>>();
            rr.pop();
            return rr;
        };

        let polygon = polygon_p.clone();
        let polygon_radians: Vec<Vec<Point>> = polygon.into_iter().map(ring_radians).collect();
        return contains(polygon_radians, &point_radians((*point).clone()));
    }

    #[test]
    fn empty_return_false() {
        println!("geoPolygonContains(empty, point) returns false");
        let polygon: Vec<Vec<Point>> = Vec::new();
        let contained = polygon_contains(&polygon, &Point { x: 0f64, y: 0f64 });
        assert_eq!(contained, false);
    }

    #[test]
    fn simple() {
        println!("geoPolygonContains(empty, point) returns the expecpted value");
        let ring: Vec<Point> = vec![
            Point { x: 0f64, y: 0f64 },
            Point { x: 0f64, y: 1f64 },
            Point { x: 1f64, y: 1f64 },
            Point { x: 1f64, y: 0f64 },
            Point { x: 0f64, y: 0f64 },
        ];
        let mut polygon: Vec<Vec<Point>> = Vec::new();
        polygon.push(ring);
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0.1f64, y: 2f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: 0.1f64,
                    y: 0.1f64
                }
            ),
            true
        );
    }

    #[test]
    fn small_circle() {
        println!("geoPolygonContains(smallCircle, point) returns the expected value");

        let mut circle = Circle::new();
        circle.radius(FnValMaybe::FloatValue(Rc::new(60.0)));

        let c = circle.circle(CircleInArg::None);
        let polygon = c.coordinates;
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: -180f64,
                    y: 0f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 1f64, y: 1f64 }),
            true
        );
    }

    #[test]
    fn wraps_longitudes() {
        println!("geoPolygonContains wraps longitudes");

        let mut circle = Circle::new();
        circle.center(FnValMaybe2D::FloatValue(Rc::new(Point {
            x: 300f64,
            y: 0f64,
        })));
        let c = circle.circle(CircleInArg::None);
        let polygon = c.coordinates;
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 300f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: -60f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: -420f64,
                    y: 0f64
                }
            ),
            true
        );
    }

    #[test]
    fn south_pole() {
        println!("geoPolygonContains(southPole, point) returns the expected value");
        let polygon = vec![vec![
            Point {
                x: -60f64,
                y: -80f64,
            },
            Point {
                x: 60f64,
                y: -80f64,
            },
            Point {
                x: 180f64,
                y: -80f64,
            },
            Point {
                x: -60f64,
                y: -80f64,
            },
        ]];
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: -85f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: -90f64 }),
            true
        );
    }

    #[test]
    fn north_pole() {
        println!("geoPolygonContains(southPole, point) returns the expected value");
        let polygon = vec![vec![
            Point { x: 60f64, y: 80f64 },
            Point {
                x: -60f64,
                y: 80f64,
            },
            Point {
                x: -180f64,
                y: 80f64,
            },
            Point { x: 60f64, y: 80f64 },
        ]];
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 85f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 90f64 }),
            true
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: -100f64,
                    y: 90f64
                }
            ),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: -90f64 }),
            false
        );
    }

    #[test]
    fn touching_pole() {
        println!("geoPolygonContains(touchingPole, Pole) returns true (issue #105)");
        let polygon = vec![vec![
            Point { x: 0f64, y: -30f64 },
            Point {
                x: 120f64,
                y: -30f64,
            },
            Point { x: 0f64, y: -90f64 },
            Point { x: 0f64, y: -30f64 },
        ]];
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: -90f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: -60f64,
                    y: -90f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: 60f64,
                    y: -90f64
                }
            ),
            false
        );
        let polygon = vec![vec![
            Point { x: 0f64, y: 30f64 },
            Point {
                x: -120f64,
                y: 30f64,
            },
            Point { x: 0f64, y: 90f64 },
            Point { x: 0f64, y: 30f64 },
        ]];
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 90f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: -60f64,
                    y: 90f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 60f64, y: 90f64 }),
            false
        );
    }

    #[test]
    fn south_hemisphere_poly() {
        println!("geoPolygonContains(southHemispherePoly) returns the expected value");
        let polygon = vec![vec![
            Point { x: 0f64, y: 0f64 },
            Point {
                x: 10f64,
                y: -40f64,
            },
            Point {
                x: -10f64,
                y: -40f64,
            },
            Point { x: 0f64, y: 0f64 },
        ]];
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: 0f64,
                    y: -40.2f64
                }
            ),
            true
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: -60f64,
                    y: -40.5f64
                }
            ),
            false
        );
    }

    #[test]
    fn large_near_origin() {
        println!("geoPolygonContains(largeNearOrigin, point) returns the expected value");
        let polygon = vec![vec![
            Point { x: 0f64, y: 0f64 },
            Point { x: 1f64, y: 0f64 },
            Point { x: 1f64, y: 1f64 },
            Point { x: 0f64, y: 1f64 },
            Point { x: 0f64, y: 0f64 },
        ]];
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: 0.1f64,
                    y: 0.1f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: 2.0f64,
                    y: 0.1f64
                }
            ),
            true
        );
    }

    #[test]
    fn large_near_south_pole() {
        println!("geoPolygonContains(largeNearSouthPole, point) returns the expected value");
        let ring: Vec<Point> = vec![
            Point {
                x: -60f64,
                y: 80f64,
            },
            Point { x: 60f64, y: 80f64 },
            Point {
                x: 180f64,
                y: 80f64,
            },
            Point {
                x: -60f64,
                y: 80f64,
            },
        ];
        let mut polygon: Vec<Vec<Point>> = Vec::new();
        polygon.push(ring);
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0.0, y: 85.0 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn large_near_north_pole() {
        println!("geoPolygonContains(largeNearSouthPole, point) returns the expected value");
        let ring: Vec<Point> = vec![
            Point {
                x: 60f64,
                y: -80f64,
            },
            Point {
                x: -60f64,
                y: -80f64,
            },
            Point {
                x: -180f64,
                y: -80f64,
            },
            Point {
                x: 60f64,
                y: -80f64,
            },
        ];
        let mut polygon: Vec<Vec<Point>> = Vec::new();
        polygon.push(ring);
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0.0, y: -85.0 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn large_circle() {
        println!("geoPolygonContains(largeCircle, point) returns the expected value");
        let mut circle = Circle::new();
        circle.radius(FnValMaybe::FloatValue(Rc::new(120.0)));
        let c = circle.circle(CircleInArg::None);
        let polygon = c.coordinates;
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: -180f64,
                    y: 0f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: -90f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn large_narrow_strip_hole() {
        println!("geoPolygonContains(largeNearSouthPole, point) returns the expected value");
        let ring: Vec<Point> = vec![
            Point {
                x: -170f64,
                y: -1f64,
            },
            Point { x: 0f64, y: -1f64 },
            Point {
                x: 170f64,
                y: -1f64,
            },
            Point { x: 170f64, y: 1f64 },
            Point { x: 0f64, y: 1f64 },
            Point {
                x: -170f64,
                y: 1f64,
            },
            Point {
                x: -170f64,
                y: -1f64,
            },
        ];
        let mut polygon: Vec<Vec<Point>> = Vec::new();
        polygon.push(ring);
        assert_eq!(polygon_contains(&polygon, &Point { x: 0.0, y: 0.0 }), false);
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 20f64 }),
            true
        );
    }

    #[test]
    fn large_narrow_equatorial_hole() {
        println!("eoPolygonContains(largeNarrowEquatorialHole, point) returns the expected value");
        let mut circle = Circle::new();
        circle.center(FnValMaybe2D::FloatValue(Rc::new(Point {
            x: 0f64,
            y: -90f64,
        })));

        circle.radius(FnValMaybe::FloatValue(Rc::new(90f64 - 0.1f64)));
        let c1 = circle.circle(CircleInArg::None);
        let ring1 = c1.coordinates[0].clone();
        println!("ring1 {:?}", ring1);

        circle.radius(FnValMaybe::FloatValue(Rc::new(90f64 + 0.1f64)));
        let c2 = circle.circle(CircleInArg::None);
        let mut ring2 = c2.coordinates[0].clone();
        ring2.reverse();

        let mut polygon: Vec<Vec<Point>> = Vec::new();
        polygon.push(ring1);
        polygon.push(ring2);

        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: -90f64 }),
            true
        );
    }

    #[test]
    fn large_narrow_equatorial_strip() {
        println!("geoPolygonContains(empty, point) returns false");

        let mut circle = Circle::new();
        circle.center(FnValMaybe2D::FloatValue(Rc::new(Point {
            x: 0f64,
            y: -90f64,
        })));
        circle.radius(FnValMaybe::FloatValue(Rc::new(90f64 + 0.1f64)));
        let c1 = circle.circle(CircleInArg::None);
        let ring1 = c1.coordinates[0].clone();

        circle.center(FnValMaybe2D::FloatValue(Rc::new(Point {
            x: 0f64,
            y: -90f64,
        })));
        circle.radius(FnValMaybe::FloatValue(Rc::new(90f64 - 0.1f64)));
        let c2 = circle.circle(CircleInArg::None);
        let mut ring2 = c2.coordinates[0].clone();
        ring2.reverse();

        let mut polygon: Vec<Vec<Point>> = Vec::new();
        polygon.push(ring1);
        polygon.push(ring2);

        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: -90f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn ring_near_origin() {
        println!("geoPolygonContains(ringNearOrigin, point) returns the expected value");
        let ring0 = vec![
            Point { x: 0f64, y: 0f64 },
            Point { x: 0f64, y: 1f64 },
            Point { x: 1f64, y: 1f64 },
            Point { x: 1f64, y: 0f64 },
            Point { x: 0f64, y: 0f64 },
        ];
        let ring1 = vec![
            Point {
                x: 0.4f64,
                y: 0.4f64,
            },
            Point {
                x: 0.6f64,
                y: 0.4f64,
            },
            Point {
                x: 0.6f64,
                y: 0.6f64,
            },
            Point {
                x: 0.4f64,
                y: 0.6f64,
            },
            Point {
                x: 0.4f64,
                y: 0.4f64,
            },
        ];
        let polygon = vec![ring0, ring1];

        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: 0.5f64,
                    y: 0.5f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: 0.1f64,
                    y: 0.5f64
                }
            ),
            true
        );
    }

    #[test]
    fn ring_equatorial() {
        println!("geoPolygonContains(ringEquatorial, point) returns the expected value");
        let ring0 = vec![
            Point { x: 0f64, y: -10f64 },
            Point {
                x: -120f64,
                y: -10f64,
            },
            Point {
                x: 120f64,
                y: -10f64,
            },
            Point { x: 0f64, y: -10f64 },
        ];
        let ring1 = vec![
            Point { x: 0f64, y: 10f64 },
            Point {
                x: 120f64,
                y: 10f64,
            },
            Point {
                x: -120f64,
                y: 10f64,
            },
            Point { x: 0f64, y: 10f64 },
        ];
        let polygon = vec![ring0, ring1];

        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 20f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn ring_excluding_both_poles() {
        println!("geoPolygonContains(ringExcludingBothPoles, point) returns the expected value");
        let mut ring0 = vec![
            Point { x: 10f64, y: 10f64 },
            Point {
                x: -10f64,
                y: 10f64,
            },
            Point {
                x: -10f64,
                y: -10f64,
            },
            Point {
                x: 10f64,
                y: -10f64,
            },
            Point { x: 10f64, y: 10f64 },
        ];
        ring0.reverse();
        let mut ring1 = vec![
            Point {
                x: 170f64,
                y: 10f64,
            },
            Point {
                x: 170f64,
                y: -10f64,
            },
            Point {
                x: -170f64,
                y: -10f64,
            },
            Point {
                x: -170f64,
                y: 10f64,
            },
            Point {
                x: 170f64,
                y: 10f64,
            },
        ];
        ring1.reverse();
        let polygon = vec![ring0, ring1];
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 90f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn ring_containing_both_poles() {
        println!("geoPolygonContains(ringExcludingBothPoles, point) returns the expected value");
        let ring0 = vec![
            Point { x: 10f64, y: 10f64 },
            Point {
                x: -10f64,
                y: 10f64,
            },
            Point {
                x: -10f64,
                y: -10f64,
            },
            Point {
                x: 10f64,
                y: -10f64,
            },
            Point { x: 10f64, y: 10f64 },
        ];
        let ring1 = vec![
            Point {
                x: 170f64,
                y: 10f64,
            },
            Point {
                x: 170f64,
                y: -10f64,
            },
            Point {
                x: -170f64,
                y: -10f64,
            },
            Point {
                x: -170f64,
                y: 10f64,
            },
            Point {
                x: 170f64,
                y: 10f64,
            },
        ];
        let polygon = vec![ring0, ring1];

        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 20f64 }),
            true
        );
    }

    #[test]
    fn ring_containing_south_pole() {
        println!("geoPolygonContains(ringContainingSouthPole, point) returns the expected value");
        let ring0 = vec![
            Point { x: 10f64, y: 10f64 },
            Point {
                x: -10f64,
                y: 10f64,
            },
            Point {
                x: -10f64,
                y: -10f64,
            },
            Point {
                x: 10f64,
                y: -10f64,
            },
            Point { x: 10f64, y: 10f64 },
        ];
        let ring1 = vec![
            Point { x: 0f64, y: 80f64 },
            Point {
                x: 120f64,
                y: 80f64,
            },
            Point {
                x: -120f64,
                y: 80f64,
            },
            Point { x: 0f64, y: 80f64 },
        ];
        let polygon = vec![ring0, ring1];

        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 90f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: -90f64 }),
            true
        );
    }

    #[test]
    fn ring_containing_north_pole() {
        println!("geoPolygonContains(ringContainingNorthPole, point) returns the expected value");
        let mut ring0 = vec![
            Point { x: 10f64, y: 10f64 },
            Point {
                x: -10f64,
                y: 10f64,
            },
            Point {
                x: -10f64,
                y: -10f64,
            },
            Point {
                x: 10f64,
                y: -10f64,
            },
            Point { x: 10f64, y: 10f64 },
        ];
        ring0.reverse();
        let mut ring1 = vec![
            Point { x: 0f64, y: 80f64 },
            Point {
                x: 120f64,
                y: 80f64,
            },
            Point {
                x: -120f64,
                y: 80f64,
            },
            Point { x: 0f64, y: 80f64 },
        ];
        ring1.reverse();
        let polygon = vec![ring0, ring1];

        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: -90f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 90f64 }),
            true
        );
    }

    #[test]
    fn self_intersecting_near_origin() {
        println!(
            "geoPolygonContains(selfIntersectingNearOrigin, point) returns the expected value"
        );
        let polygon = vec![vec![
            Point { x: 0f64, y: 0f64 },
            Point { x: 1f64, y: 0f64 },
            Point { x: 1f64, y: 3f64 },
            Point { x: 3f64, y: 3f64 },
            Point { x: 3f64, y: 1f64 },
            Point { x: 0f64, y: 1f64 },
            Point { x: 0f64, y: 0f64 },
        ]];
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: 15f64,
                    y: 0.5f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 12f64, y: 2f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: 0.5f64,
                    y: 0.5f64
                }
            ),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 2f64, y: 2f64 }),
            true
        );
    }

    #[test]
    fn self_intersecting_near_south_pole() {
        println!(
            "geoPolygonContains(selfIntersectingNearSouthPole, point) returns the expected value"
        );
        let polygon = vec![vec![
            Point {
                x: -10f64,
                y: -80f64,
            },
            Point {
                x: 120f64,
                y: -80f64,
            },
            Point {
                x: -120f64,
                y: -80f64,
            },
            Point {
                x: 10f64,
                y: -85f64,
            },
            Point {
                x: 10f64,
                y: -75f64,
            },
            Point {
                x: -10f64,
                y: 75f64,
            },
            Point {
                x: -10f64,
                y: -80f64,
            },
        ]];
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: -76f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: -89f64 }),
            true
        );
    }

    #[test]
    fn self_intersecting_near_north_pole() {
        println!(
            "geoPolygonContains(selfIntersectingNearNorthPole, point) returns the expected value"
        );
        let polygon = vec![vec![
            Point {
                x: -10f64,
                y: 80f64,
            },
            Point {
                x: -10f64,
                y: 75f64,
            },
            Point { x: 10f64, y: 75f64 },
            Point { x: 10f64, y: 85f64 },
            Point {
                x: -120f64,
                y: 80f64,
            },
            Point {
                x: 120f64,
                y: 80f64,
            },
            Point {
                x: -10f64,
                y: 80f64,
            },
        ]];
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 76f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 89f64 }),
            true
        );
    }

    #[test]
    fn hemisphere_touching_the_south_pole() {
        println!(
            "geoPolygonContains(hemisphereTouchingTheSouthPole, point) returns the expected value"
        );

        let mut circle = Circle::new();
        circle.radius(FnValMaybe::FloatValue(Rc::new(90f64)));

        let c = circle.circle(CircleInArg::None);
        let polygon = &c.coordinates;
        assert_eq!(polygon_contains(polygon, &Point { x: 0f64, y: 0f64 }), true);
    }

    #[test]
    fn triangle_touching_the_south_pole() {
        println!(
            "geoPolygonContains(triangleTouchingTheSouthPole, point) returns the expected value"
        );
        let polygon = vec![vec![
            Point {
                x: 180f64,
                y: -90f64,
            },
            Point { x: -45f64, y: 0f64 },
            Point { x: 45f64, y: 0f64 },
            Point {
                x: 180f64,
                y: -90f64,
            },
        ]];
        assert_eq!(
            polygon_contains(&polygon, &Point { x: -46f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 1f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: -90f64,
                    y: -80f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: -44f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: -30f64 }),
            true
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: 30f64,
                    y: -80f64
                }
            ),
            true
        );
    }

    #[test]
    fn triangle_touching_the_south_pole2() {
        println!(
            "geoPolygonContains(triangleTouchingTheSouthPole2, point) returns the expected value"
        );
        let polygon = vec![vec![
            Point { x: -45f64, y: 0f64 },
            Point { x: 45f64, y: 0f64 },
            Point {
                x: 180f64,
                y: -90f64,
            },
            Point { x: -45f64, y: 0f64 },
        ]];
        assert_eq!(
            polygon_contains(&polygon, &Point { x: -46f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 1f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: -90f64,
                    y: -80f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: -44f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: -30f64 }),
            true
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: 30f64,
                    y: -80f64
                }
            ),
            true
        );
    }

    #[test]
    fn triangle_touching_the_south_pole3() {
        println!(
            "geoPolygonContains(triangleTouchingTheSouthPole3, point) returns the expected value"
        );
        let polygon = vec![vec![
            Point {
                x: 180f64,
                y: -90f64,
            },
            Point {
                x: -135f64,
                y: 0f64,
            },
            Point { x: 135f64, y: 0f64 },
            Point {
                x: 180f64,
                y: -90f64,
            },
        ]];
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 180f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 150f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: 180f64,
                    y: -30f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: 150f64,
                    y: -80f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 180f64, y: 1f64 }),
            true
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: -90f64,
                    y: -80f64
                }
            ),
            true
        );
    }

    #[test]
    fn triangle_touching_the_north_pole() {
        println!(
            "geoPolygonContains(triangleTouchingTheSouthPole2, point) returns the expected value"
        );
        let polygon = vec![vec![
            Point {
                x: 180f64,
                y: 90f64,
            },
            Point { x: 45f64, y: 0f64 },
            Point { x: -45f64, y: 0f64 },
            Point {
                x: 180f64,
                y: 90f64,
            },
        ]];
        assert_eq!(
            polygon_contains(&polygon, &Point { x: -90f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: -1f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: -80f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: -90f64, y: 1f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: -90f64,
                    y: 80f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Point {
                    x: -44f64,
                    y: 10f64
                }
            ),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 0f64, y: 10f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Point { x: 30f64, y: 80f64 }),
            true
        );
    }
}
