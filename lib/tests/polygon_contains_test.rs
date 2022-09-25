#[cfg(not(tarpaulin_include))]
mod polygon_contains_test {
    extern crate pretty_assertions;

    use geo::{CoordFloat, Coordinate, LineString, Polygon};
    use pretty_assertions::assert_eq;

    use rust_d3_geo::circle::generator::Generator as CircleGenerator;
    use rust_d3_geo::polygon_contains::polygon_contains as contains;

    #[inline]
    fn point_radians<T>(p: &Coordinate<T>) -> Coordinate<T>
    where
        T: CoordFloat,
    {
        Coordinate {
            x: p.x.to_radians(),
            y: p.y.to_radians(),
        }
    }

    fn ring_radians<T>(ring: &LineString<T>) -> LineString<T>
    where
        T: CoordFloat,
    {
        let mut v: Vec<Coordinate<T>> = ring
            .0
            .iter()
            .map(|x| point_radians(&x))
            .collect::<Vec<Coordinate<T>>>();
        v.pop();
        let out = LineString(v);
        out
    }

    fn polygon_contains<T>(polygon_p: &Polygon<T>, point: &Coordinate<T>) -> bool
    where
        T: CoordFloat + num_traits::float::FloatConst,
    {
        // Combined in a vector of linestrings.
        // exterior first, followed by all the interior linestrings.
        let (e, i) = polygon_p.clone().into_inner();
        let combined = [vec![e], i].concat();

        let polygon_radians: Vec<LineString<T>> =
            combined.iter().map(|x| ring_radians(&x)).collect();
        return contains(&polygon_radians, &point_radians(&(*point)));
    }

    #[test]
    fn empty_return_false() {
        println!("geoPolygonContains(empty, point) returns false");
        let polygon: Polygon<f64> = Polygon::new(LineString(vec![]), vec![]);
        let contained = polygon_contains(&polygon, &Coordinate { x: 0f64, y: 0f64 });
        assert_eq!(contained, false);
    }

    #[test]
    fn simple() {
        println!("geoPolygonContains(simple, point) returns the expected value");
        let polygon: Polygon<f64> = Polygon::new(
            LineString(vec![
                Coordinate { x: 0f64, y: 0f64 },
                Coordinate { x: 0f64, y: 1f64 },
                Coordinate { x: 1f64, y: 1f64 },
                Coordinate { x: 1f64, y: 0f64 },
                Coordinate { x: 0f64, y: 0f64 },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0.1f64, y: 2f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
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

        let mut circle = CircleGenerator::default().radius_set(60.0);
        let polygon = circle.circle();

        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: -180f64,
                    y: 0f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 1f64, y: 1f64 }),
            true
        );
    }

    #[test]
    fn wraps_longitudes() {
        println!("geoPolygonContains wraps longitudes");

        let mut circle = CircleGenerator::default().center_set(&Coordinate { x: 300f64, y: 0f64 });
        let c = circle.circle();
        let polygon = c;

        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 300f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: -60f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(
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

        let polygon = Polygon::new(
            LineString(vec![
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
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: -85f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: -90f64 }),
            true
        );
    }

    #[test]
    fn north_pole() {
        println!("geoPolygonContains(northPole, point) returns the expected value");
        let polygon = Polygon::new(
            LineString(vec![
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
            ]),
            vec![],
        );

        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 85f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 90f64 }),
            true
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: -100f64,
                    y: 90f64
                }
            ),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: -90f64 }),
            false
        );
    }

    #[test]
    fn touching_pole() {
        println!("geoPolygonContains(touchingPole, Pole) returns true (issue #105)");
        let polygon = Polygon::new(
            LineString(vec![
                Coordinate { x: 0f64, y: -30f64 },
                Coordinate {
                    x: 120f64,
                    y: -30f64,
                },
                Coordinate { x: 0f64, y: -90f64 },
                Coordinate { x: 0f64, y: -30f64 },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: -90f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: -60f64,
                    y: -90f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: 60f64,
                    y: -90f64
                }
            ),
            false
        );
        let polygon = Polygon::new(
            LineString(vec![
                Coordinate { x: 0f64, y: 30f64 },
                Coordinate {
                    x: -120f64,
                    y: 30f64,
                },
                Coordinate { x: 0f64, y: 90f64 },
                Coordinate { x: 0f64, y: 30f64 },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 90f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: -60f64,
                    y: 90f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 60f64, y: 90f64 }),
            false
        );
    }

    #[test]
    fn south_hemisphere_poly() {
        println!("geoPolygonContains(southHemispherePoly) returns the expected value");
        let polygon = Polygon::new(
            LineString(vec![
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
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: 0f64,
                    y: -40.2f64
                }
            ),
            true
        );
        assert_eq!(
            polygon_contains(
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
        let polygon = Polygon::new(
            LineString(vec![
                Coordinate { x: 0f64, y: 0f64 },
                Coordinate { x: 1f64, y: 0f64 },
                Coordinate { x: 1f64, y: 1f64 },
                Coordinate { x: 0f64, y: 1f64 },
                Coordinate { x: 0f64, y: 0f64 },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: 0.1f64,
                    y: 0.1f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(
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
        let polygon = Polygon::new(
            LineString(vec![
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
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: 0.0f64,
                    y: 85.0f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn large_near_north_pole() {
        println!("geoPolygonContains(largeNearNorthPole, point) returns the expected value");
        let polygon = Polygon::new(
            LineString(vec![
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
            ]),
            vec![],
        );

        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: 0.0f64,
                    y: -85.0f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn large_circle() {
        println!("geoPolygonContains(largeCircle, point) returns the expected value");
        let mut circle = CircleGenerator::default().radius_set(120.0);
        let c = circle.circle();
        let polygon = c;
        println!("polygon {:#?}", polygon);
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: -180f64,
                    y: 0f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: -90f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn large_narrow_strip_hole() {
        println!("geoPolygonContains(largeNarrowStripHole, point) returns the expected value");
        let polygon = Polygon::new(
            LineString(vec![
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
            ]),
            vec![],
        );

        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0.0, y: 0.0 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 20f64 }),
            true
        );
    }

    #[test]
    fn large_narrow_equatorial_hole() {
        println!("geoPolygonContains(largeNarrowEquatorialHole, point) returns the expected value");
        let mut circle_gen = CircleGenerator::default()
            .center_set(&Coordinate { x: 0f64, y: -90f64 })
            .radius_set(90f64 - 0.1f64);
        let ring0: LineString<f64> = circle_gen.circle().exterior().clone();

        let ring1 = circle_gen
            .radius_set(90f64 + 0.1f64)
            .circle()
            .exterior()
            .clone();

        let rev_vec: Vec<Coordinate<f64>> = ring1.into_iter().rev().collect();
        let ring1_rev = LineString(rev_vec);

        let polygon = Polygon::new(ring0, vec![ring1_rev]);

        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: -90f64 }),
            true
        );
    }

    #[test]
    fn large_narrow_equatorial_strip() {
        println!("geoPolygonContains(largeNarrowEquatorialHole, point) returns the expected value");

        let mut circle = CircleGenerator::default()
            .center_set(&Coordinate { x: 0f64, y: -90f64 })
            .radius_set(90f64 + 0.1f64);

        let ring1 = circle.circle().exterior().clone();

        let mut circle = CircleGenerator::default()
            .center_set(&Coordinate { x: 0f64, y: -90f64 })
            .radius_set(90f64 - 0.1f64);
        let c2 = circle.circle().exterior().clone();
        let rev_vec = c2.into_iter().rev().collect();
        let ring2 = LineString(rev_vec);

        let polygon = Polygon::new(ring1, vec![ring2]);
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: -90f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
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
        let polygon = Polygon::new(LineString(ring0), vec![LineString(ring1)]);

        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: 0.5f64,
                    y: 0.5f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(
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
        let polygon = Polygon::new(LineString(ring0), vec![LineString(ring1)]);

        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 20f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
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
        let polygon = Polygon::new(LineString(ring0), vec![LineString(ring1)]);
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 90f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
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
        let polygon = Polygon::new(LineString(ring0), vec![LineString(ring1)]);

        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 20f64 }),
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
        let polygon = Polygon::new(LineString(ring0), vec![LineString(ring1)]);

        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 90f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: -90f64 }),
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
        let polygon = Polygon::new(LineString(ring0), vec![LineString(ring1)]);

        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: -90f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 90f64 }),
            true
        );
    }

    #[test]
    fn self_intersecting_near_origin() {
        println!(
            "geoPolygonContains(selfIntersectingNearOrigin, point) returns the expected value"
        );
        let polygon = Polygon::new(
            LineString(vec![
                Coordinate { x: 0f64, y: 0f64 },
                Coordinate { x: 1f64, y: 0f64 },
                Coordinate { x: 1f64, y: 3f64 },
                Coordinate { x: 3f64, y: 3f64 },
                Coordinate { x: 3f64, y: 1f64 },
                Coordinate { x: 0f64, y: 1f64 },
                Coordinate { x: 0f64, y: 0f64 },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: 15f64,
                    y: 0.5f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 12f64, y: 2f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: 0.5f64,
                    y: 0.5f64
                }
            ),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 2f64, y: 2f64 }),
            true
        );
    }

    #[test]
    fn self_intersecting_near_south_pole() {
        println!(
            "geoPolygonContains(selfIntersectingNearSouthPole, point) returns the expected value"
        );
        let polygon = Polygon::new(
            LineString(vec![
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
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: -76f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: -89f64 }),
            true
        );
    }

    #[test]
    fn self_intersecting_near_north_pole() {
        println!(
            "geoPolygonContains(selfIntersectingNearNorthPole, point) returns the expected value"
        );
        let polygon = Polygon::new(
            LineString(vec![
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
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 76f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 89f64 }),
            true
        );
    }

    #[test]
    fn hemisphere_touching_the_south_pole() {
        println!(
            "geoPolygonContains(hemisphereTouchingTheSouthPole, point) returns the expected value"
        );

        let mut circle = CircleGenerator::default().radius_set(90f64);

        let c = circle.circle();
        let polygon = &c;
        assert_eq!(
            polygon_contains(polygon, &Coordinate { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn triangle_touching_the_south_pole() {
        println!(
            "geoPolygonContains(triangleTouchingTheSouthPole, point) returns the expected value"
        );
        let polygon = Polygon::new(
            LineString(vec![
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
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: -46f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 1f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: -90f64,
                    y: -80f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: -44f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: -30f64 }),
            true
        );
        assert_eq!(
            polygon_contains(
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
        let polygon = Polygon::new(
            LineString(vec![
                Coordinate { x: -45f64, y: 0f64 },
                Coordinate { x: 45f64, y: 0f64 },
                Coordinate {
                    x: 180f64,
                    y: -90f64,
                },
                Coordinate { x: -45f64, y: 0f64 },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: -46f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 1f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: -90f64,
                    y: -80f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: -44f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: -30f64 }),
            true
        );
        assert_eq!(
            polygon_contains(
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
        let polygon = Polygon::new(
            LineString(vec![
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
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 180f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 150f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: 180f64,
                    y: -30f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: 150f64,
                    y: -80f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 180f64, y: 1f64 }),
            true
        );
        assert_eq!(
            polygon_contains(
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
        let polygon = Polygon::new(
            LineString(vec![
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
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: -90f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: -1f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: -80f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: -90f64, y: 1f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: -90f64,
                    y: 80f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coordinate {
                    x: -44f64,
                    y: 10f64
                }
            ),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 0f64, y: 10f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coordinate { x: 30f64, y: 80f64 }),
            true
        );
    }
}
