#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod polygon_contains_test {
    extern crate pretty_assertions;

    use geo::Coordinate;
    use pretty_assertions::assert_eq;

    use rust_d3_geo::circle::generator::Generator as CircleGenerator;
    use rust_d3_geo::polygon_contains::polygon_contains as contains;

    fn polygon_contains<T>(polygon_p: &Vec<Vec<Coordinate<f64>>>, point: &Coordinate<f64>) -> bool {
        let point_radians = |p: Coordinate<f64>| Coordinate {
            x: p.x.to_radians(),
            y: p.y.to_radians(),
        };
        let ring_radians = |ring: Vec<_>| {
            let mut rr = ring
                .into_iter()
                .map(point_radians)
                .collect::<Vec<Coordinate<f64>>>();
            rr.pop();
            return rr;
        };

        let polygon = polygon_p.clone();
        let polygon_radians: Vec<Vec<Coordinate<f64>>> =
            polygon.into_iter().map(ring_radians).collect();
        return contains(&polygon_radians, &point_radians((*point).clone()));
    }

    #[test]
    fn empty_return_false() {
        println!("geoPolygonContains(empty, point) returns false");
        let polygon: Vec<Vec<Coordinate<f64>>> = Vec::new();
        let contained = polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 0f64 });
        assert_eq!(contained, false);
    }

    #[test]
    fn simple() {
        println!("geoPolygonContains(empty, point) returns the expecpted value");
        let ring = vec![
            Coordinate { x: 0f64, y: 0f64 },
            Coordinate { x: 0f64, y: 1f64 },
            Coordinate { x: 1f64, y: 1f64 },
            Coordinate { x: 1f64, y: 0f64 },
            Coordinate { x: 0f64, y: 0f64 },
        ];
        let mut polygon: Vec<Vec<Coordinate<f64>>> = Vec::new();
        polygon.push(ring);
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0.1f64, y: 2f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
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

        let mut circle = CircleGenerator::default().radius(60.0);
        let polygon = circle.circle();

        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: -180f64,
                    y: 0f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 1f64, y: 1f64 }),
            true
        );
    }

    #[test]
    fn wraps_longitudes() {
        println!("geoPolygonContains wraps longitudes");

        let mut circle = CircleGenerator::default().center(&Coordinate { x: 300f64, y: 0f64 });
        let c = circle.circle();
        let polygon = c;

        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 300f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: -60f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
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
            Coordinate {
                x: -60f64,
                y: -80f64,
            },
            Coordinate {
                x: 60f64,
                y: -80f64,
            },
            Coordinate {
                x: 180f64,
                y: -80f64,
            },
            Coordinate {
                x: -60f64,
                y: -80f64,
            },
        ]];
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: -85f64 }),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: -90f64 }),
            true
        );
    }

    #[test]
    fn north_pole() {
        println!("geoPolygonContains(northPole, point) returns the expected value");
        let polygon = vec![vec![
            Coordinate { x: 60f64, y: 80f64 },
            Coordinate {
                x: -60f64,
                y: 80f64,
            },
            Coordinate {
                x: -180f64,
                y: 80f64,
            },
            Coordinate { x: 60f64, y: 80f64 },
        ]];
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 85f64 }),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 90f64 }),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: -100f64,
                    y: 90f64
                }
            ),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: -90f64 }),
            false
        );
    }

    #[test]
    fn touching_pole() {
        println!("geoPolygonContains(touchingPole, Pole) returns true (issue #105)");
        let polygon = vec![vec![
            Coordinate { x: 0f64, y: -30f64 },
            Coordinate {
                x: 120f64,
                y: -30f64,
            },
            Coordinate { x: 0f64, y: -90f64 },
            Coordinate { x: 0f64, y: -30f64 },
        ]];
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: -90f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: -60f64,
                    y: -90f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: 60f64,
                    y: -90f64
                }
            ),
            false
        );
        let polygon = vec![vec![
            Coordinate { x: 0f64, y: 30f64 },
            Coordinate {
                x: -120f64,
                y: 30f64,
            },
            Coordinate { x: 0f64, y: 90f64 },
            Coordinate { x: 0f64, y: 30f64 },
        ]];
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 90f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: -60f64,
                    y: 90f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 60f64, y: 90f64 }),
            false
        );
    }

    #[test]
    fn south_hemisphere_poly() {
        println!("geoPolygonContains(southHemispherePoly) returns the expected value");
        let polygon = vec![vec![
            Coordinate { x: 0f64, y: 0f64 },
            Coordinate {
                x: 10f64,
                y: -40f64,
            },
            Coordinate {
                x: -10f64,
                y: -40f64,
            },
            Coordinate { x: 0f64, y: 0f64 },
        ]];
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: 0f64,
                    y: -40.2f64
                }
            ),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
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
            Coordinate { x: 0f64, y: 0f64 },
            Coordinate { x: 1f64, y: 0f64 },
            Coordinate { x: 1f64, y: 1f64 },
            Coordinate { x: 0f64, y: 1f64 },
            Coordinate { x: 0f64, y: 0f64 },
        ]];
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: 0.1f64,
                    y: 0.1f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
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
        let ring = vec![
            Coordinate {
                x: -60f64,
                y: 80f64,
            },
            Coordinate { x: 60f64, y: 80f64 },
            Coordinate {
                x: 180f64,
                y: 80f64,
            },
            Coordinate {
                x: -60f64,
                y: 80f64,
            },
        ];
        let mut polygon = Vec::new();
        polygon.push(ring);
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: 0.0f64,
                    y: 85.0f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn large_near_north_pole() {
        println!("geoPolygonContains(largeNearNorthPole, point) returns the expected value");
        let ring = vec![
            Coordinate {
                x: 60f64,
                y: -80f64,
            },
            Coordinate {
                x: -60f64,
                y: -80f64,
            },
            Coordinate {
                x: -180f64,
                y: -80f64,
            },
            Coordinate {
                x: 60f64,
                y: -80f64,
            },
        ];
        let mut polygon = Vec::new();
        polygon.push(ring);
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: 0.0f64,
                    y: -85.0f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn large_circle() {
        println!("geoPolygonContains(largeCircle, point) returns the expected value");
        let mut circle = CircleGenerator::default().radius(120.0);
        let c = circle.circle();
        let polygon = c;
        println!("polygon {:#?}", polygon);
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: -180f64,
                    y: 0f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: -90f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn large_narrow_strip_hole() {
        println!("geoPolygonContains(largeNarrowStripHole, point) returns the expected value");
        let ring = vec![
            Coordinate {
                x: -170f64,
                y: -1f64,
            },
            Coordinate { x: 0f64, y: -1f64 },
            Coordinate {
                x: 170f64,
                y: -1f64,
            },
            Coordinate { x: 170f64, y: 1f64 },
            Coordinate { x: 0f64, y: 1f64 },
            Coordinate {
                x: -170f64,
                y: 1f64,
            },
            Coordinate {
                x: -170f64,
                y: -1f64,
            },
        ];
        let mut polygon = Vec::new();
        polygon.push(ring);
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0.0, y: 0.0 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 20f64 }),
            true
        );
    }

    #[test]
    fn large_narrow_equatorial_hole() {
        println!("geoPolygonContains(largeNarrowEquatorialHole, point) returns the expected value");
        let mut circle_gen = CircleGenerator::default()
            .center(&Coordinate { x: 0f64, y: -90f64 })
            .radius(90f64 - 0.1f64);
        // let ring0 = circle_gen.circle(&CircleInArg::None).clone().coordinates[0];
        let temp = &circle_gen.circle()[0];
        let ring0 = temp.clone();

        let out = &circle_gen.radius(90f64 + 0.1f64).circle()[0];
        let mut ring1 = out.clone();

        ring1.reverse();

        let polygon: Vec<Vec<Coordinate<f64>>> = vec![ring0, ring1.clone()];

        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: -90f64 }),
            true
        );
    }

    #[test]
    fn large_narrow_equatorial_strip() {
        println!("geoPolygonContains(largeNarrowEquatorialHole, point) returns the expected value");

        let mut circle = CircleGenerator::default()
            .center(&Coordinate { x: 0f64, y: -90f64 })
            .radius(90f64 + 0.1f64);

        let c1 = circle.circle();
        let ring1 = c1[0].clone();

        let mut circle = CircleGenerator::default()
            .center(&Coordinate { x: 0f64, y: -90f64 })
            .radius(90f64 - 0.1f64);
        let c2_temp = &circle.circle()[0];
        let c2 = c2_temp.clone();

        // let temp = &circle_gen.circle().coordinates[0];
        // let ring0 = temp.clone();

        let mut ring2 = c2.clone();
        ring2.reverse();

        let mut polygon: Vec<Vec<Coordinate<f64>>> = Vec::new();
        polygon.push(ring1);
        polygon.push(ring2);

        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: -90f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn ring_near_origin() {
        println!("geoPolygonContains(ringNearOrigin, point) returns the expected value");
        let ring0 = vec![
            Coordinate { x: 0f64, y: 0f64 },
            Coordinate { x: 0f64, y: 1f64 },
            Coordinate { x: 1f64, y: 1f64 },
            Coordinate { x: 1f64, y: 0f64 },
            Coordinate { x: 0f64, y: 0f64 },
        ];
        let ring1 = vec![
            Coordinate {
                x: 0.4f64,
                y: 0.4f64,
            },
            Coordinate {
                x: 0.6f64,
                y: 0.4f64,
            },
            Coordinate {
                x: 0.6f64,
                y: 0.6f64,
            },
            Coordinate {
                x: 0.4f64,
                y: 0.6f64,
            },
            Coordinate {
                x: 0.4f64,
                y: 0.4f64,
            },
        ];
        let polygon = vec![ring0, ring1];

        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: 0.5f64,
                    y: 0.5f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
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
            Coordinate { x: 0f64, y: -10f64 },
            Coordinate {
                x: -120f64,
                y: -10f64,
            },
            Coordinate {
                x: 120f64,
                y: -10f64,
            },
            Coordinate { x: 0f64, y: -10f64 },
        ];
        let ring1 = vec![
            Coordinate { x: 0f64, y: 10f64 },
            Coordinate {
                x: 120f64,
                y: 10f64,
            },
            Coordinate {
                x: -120f64,
                y: 10f64,
            },
            Coordinate { x: 0f64, y: 10f64 },
        ];
        let polygon = vec![ring0, ring1];

        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 20f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn ring_excluding_both_poles() {
        println!("geoPolygonContains(ringExcludingBothPoles, point) returns the expected value");
        let mut ring0 = vec![
            Coordinate { x: 10f64, y: 10f64 },
            Coordinate {
                x: -10f64,
                y: 10f64,
            },
            Coordinate {
                x: -10f64,
                y: -10f64,
            },
            Coordinate {
                x: 10f64,
                y: -10f64,
            },
            Coordinate { x: 10f64, y: 10f64 },
        ];
        ring0.reverse();
        let mut ring1 = vec![
            Coordinate {
                x: 170f64,
                y: 10f64,
            },
            Coordinate {
                x: 170f64,
                y: -10f64,
            },
            Coordinate {
                x: -170f64,
                y: -10f64,
            },
            Coordinate {
                x: -170f64,
                y: 10f64,
            },
            Coordinate {
                x: 170f64,
                y: 10f64,
            },
        ];
        ring1.reverse();
        let polygon = vec![ring0, ring1];
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 90f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn ring_containing_both_poles() {
        println!("geoPolygonContains(ringExcludingBothPoles, point) returns the expected value");
        let ring0 = vec![
            Coordinate { x: 10f64, y: 10f64 },
            Coordinate {
                x: -10f64,
                y: 10f64,
            },
            Coordinate {
                x: -10f64,
                y: -10f64,
            },
            Coordinate {
                x: 10f64,
                y: -10f64,
            },
            Coordinate { x: 10f64, y: 10f64 },
        ];
        let ring1 = vec![
            Coordinate {
                x: 170f64,
                y: 10f64,
            },
            Coordinate {
                x: 170f64,
                y: -10f64,
            },
            Coordinate {
                x: -170f64,
                y: -10f64,
            },
            Coordinate {
                x: -170f64,
                y: 10f64,
            },
            Coordinate {
                x: 170f64,
                y: 10f64,
            },
        ];
        let polygon = vec![ring0, ring1];

        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 20f64 }),
            true
        );
    }

    #[test]
    fn ring_containing_south_pole() {
        println!("geoPolygonContains(ringContainingSouthPole, point) returns the expected value");
        let ring0 = vec![
            Coordinate { x: 10f64, y: 10f64 },
            Coordinate {
                x: -10f64,
                y: 10f64,
            },
            Coordinate {
                x: -10f64,
                y: -10f64,
            },
            Coordinate {
                x: 10f64,
                y: -10f64,
            },
            Coordinate { x: 10f64, y: 10f64 },
        ];
        let ring1 = vec![
            Coordinate { x: 0f64, y: 80f64 },
            Coordinate {
                x: 120f64,
                y: 80f64,
            },
            Coordinate {
                x: -120f64,
                y: 80f64,
            },
            Coordinate { x: 0f64, y: 80f64 },
        ];
        let polygon = vec![ring0, ring1];

        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 90f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: -90f64 }),
            true
        );
    }

    #[test]
    fn ring_containing_north_pole() {
        println!("geoPolygonContains(ringContainingNorthPole, point) returns the expected value");
        let mut ring0 = vec![
            Coordinate { x: 10f64, y: 10f64 },
            Coordinate {
                x: -10f64,
                y: 10f64,
            },
            Coordinate {
                x: -10f64,
                y: -10f64,
            },
            Coordinate {
                x: 10f64,
                y: -10f64,
            },
            Coordinate { x: 10f64, y: 10f64 },
        ];
        ring0.reverse();
        let mut ring1 = vec![
            Coordinate { x: 0f64, y: 80f64 },
            Coordinate {
                x: 120f64,
                y: 80f64,
            },
            Coordinate {
                x: -120f64,
                y: 80f64,
            },
            Coordinate { x: 0f64, y: 80f64 },
        ];
        ring1.reverse();
        let polygon = vec![ring0, ring1];

        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: -90f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 90f64 }),
            true
        );
    }

    #[test]
    fn self_intersecting_near_origin() {
        println!(
            "geoPolygonContains(selfIntersectingNearOrigin, point) returns the expected value"
        );
        let polygon = vec![vec![
            Coordinate { x: 0f64, y: 0f64 },
            Coordinate { x: 1f64, y: 0f64 },
            Coordinate { x: 1f64, y: 3f64 },
            Coordinate { x: 3f64, y: 3f64 },
            Coordinate { x: 3f64, y: 1f64 },
            Coordinate { x: 0f64, y: 1f64 },
            Coordinate { x: 0f64, y: 0f64 },
        ]];
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: 15f64,
                    y: 0.5f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 12f64, y: 2f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: 0.5f64,
                    y: 0.5f64
                }
            ),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 2f64, y: 2f64 }),
            true
        );
    }

    #[test]
    fn self_intersecting_near_south_pole() {
        println!(
            "geoPolygonContains(selfIntersectingNearSouthPole, point) returns the expected value"
        );
        let polygon = vec![vec![
            Coordinate {
                x: -10f64,
                y: -80f64,
            },
            Coordinate {
                x: 120f64,
                y: -80f64,
            },
            Coordinate {
                x: -120f64,
                y: -80f64,
            },
            Coordinate {
                x: 10f64,
                y: -85f64,
            },
            Coordinate {
                x: 10f64,
                y: -75f64,
            },
            Coordinate {
                x: -10f64,
                y: 75f64,
            },
            Coordinate {
                x: -10f64,
                y: -80f64,
            },
        ]];
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: -76f64 }),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: -89f64 }),
            true
        );
    }

    #[test]
    fn self_intersecting_near_north_pole() {
        println!(
            "geoPolygonContains(selfIntersectingNearNorthPole, point) returns the expected value"
        );
        let polygon = vec![vec![
            Coordinate {
                x: -10f64,
                y: 80f64,
            },
            Coordinate {
                x: -10f64,
                y: 75f64,
            },
            Coordinate { x: 10f64, y: 75f64 },
            Coordinate { x: 10f64, y: 85f64 },
            Coordinate {
                x: -120f64,
                y: 80f64,
            },
            Coordinate {
                x: 120f64,
                y: 80f64,
            },
            Coordinate {
                x: -10f64,
                y: 80f64,
            },
        ]];
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 76f64 }),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 89f64 }),
            true
        );
    }

    #[test]
    fn hemisphere_touching_the_south_pole() {
        println!(
            "geoPolygonContains(hemisphereTouchingTheSouthPole, point) returns the expected value"
        );

        let mut circle = CircleGenerator::default().radius(90f64);

        let c = circle.circle();
        let polygon = &c;
        assert_eq!(
            polygon_contains::<f64>(polygon, &Coordinate { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn triangle_touching_the_south_pole() {
        println!(
            "geoPolygonContains(triangleTouchingTheSouthPole, point) returns the expected value"
        );
        let polygon = vec![vec![
            Coordinate {
                x: 180f64,
                y: -90f64,
            },
            Coordinate { x: -45f64, y: 0f64 },
            Coordinate { x: 45f64, y: 0f64 },
            Coordinate {
                x: 180f64,
                y: -90f64,
            },
        ]];
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: -46f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 1f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: -90f64,
                    y: -80f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: -44f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: -30f64 }),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
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
            Coordinate { x: -45f64, y: 0f64 },
            Coordinate { x: 45f64, y: 0f64 },
            Coordinate {
                x: 180f64,
                y: -90f64,
            },
            Coordinate { x: -45f64, y: 0f64 },
        ]];
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: -46f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 1f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: -90f64,
                    y: -80f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: -44f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: -30f64 }),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
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
            Coordinate {
                x: 180f64,
                y: -90f64,
            },
            Coordinate {
                x: -135f64,
                y: 0f64,
            },
            Coordinate { x: 135f64, y: 0f64 },
            Coordinate {
                x: 180f64,
                y: -90f64,
            },
        ]];
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 180f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 150f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: 180f64,
                    y: -30f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: 150f64,
                    y: -80f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 180f64, y: 1f64 }),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
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
            "geoPolygonContains(triangleTouchingTheNorthPole, point) returns the expected value"
        );
        let polygon = vec![vec![
            Coordinate {
                x: 180f64,
                y: 90f64,
            },
            Coordinate { x: 45f64, y: 0f64 },
            Coordinate { x: -45f64, y: 0f64 },
            Coordinate {
                x: 180f64,
                y: 90f64,
            },
        ]];
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: -90f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: -1f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: -80f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: -90f64, y: 1f64 }),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: -90f64,
                    y: 80f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains::<f64>(
                &polygon,
                &Coordinate {
                    x: -44f64,
                    y: 10f64
                }
            ),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 0f64, y: 10f64 }),
            true
        );
        assert_eq!(
            polygon_contains::<f64>(&polygon, &Coordinate { x: 30f64, y: 80f64 }),
            true
        );
    }
}
