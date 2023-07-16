#[cfg(not(tarpaulin_include))]
mod polygon_contains {
    extern crate pretty_assertions;

    use geo::CoordFloat;
    use geo::LineString;
    use geo::Polygon;
    use geo_types::Coord;
    use pretty_assertions::assert_eq;

    use d3_geo_rs::circle::generator::Generator as CircleGenerator;
    use d3_geo_rs::polygon_contains::polygon_contains as contains;

    #[inline]
    fn point_radians<T>(p: &Coord<T>) -> Coord<T>
    where
        T: CoordFloat,
    {
        Coord {
            x: p.x.to_radians(),
            y: p.y.to_radians(),
        }
    }

    fn ring_radians<T>(ring: &LineString<T>) -> LineString<T>
    where
        T: CoordFloat,
    {
        let mut v: Vec<Coord<T>> = ring
            .0
            .iter()
            .map(|x| point_radians(x))
            .collect::<Vec<Coord<T>>>();
        v.pop();
        LineString(v)
    }

    fn polygon_contains<T>(polygon_p: &Polygon<T>, point: &Coord<T>) -> bool
    where
        T: CoordFloat + num_traits::float::FloatConst,
    {
        // Combined in a vector of line strings.
        // exterior first, followed by all the interior line strings.
        let (e, i) = polygon_p.clone().into_inner();
        let combined = [vec![e], i].concat();

        let polygon_radians: Vec<LineString<T>> =
            combined.iter().map(|x| ring_radians(x)).collect();
        contains(&polygon_radians, &point_radians(point))
    }

    #[test]
    fn empty_return_false() {
        println!("geoPolygonContains(empty, point) returns false");
        let polygon: Polygon<f64> = Polygon::new(LineString(vec![]), vec![]);
        let contained = polygon_contains(&polygon, &Coord { x: 0f64, y: 0f64 });
        assert_eq!(contained, false);
    }

    #[test]
    fn simple() {
        println!("geoPolygonContains(simple, point) returns the expected value");
        let polygon: Polygon<f64> = Polygon::new(
            LineString(vec![
                Coord { x: 0f64, y: 0f64 },
                Coord { x: 0f64, y: 1f64 },
                Coord { x: 1f64, y: 1f64 },
                Coord { x: 1f64, y: 0f64 },
                Coord { x: 0f64, y: 0f64 },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0.1f64, y: 2f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
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

        let mut circle = CircleGenerator::default();
        circle.radius_set(60.0);
        let polygon = circle.circle();

        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: -180f64,
                    y: 0f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 1f64, y: 1f64 }),
            true
        );
    }

    #[test]
    fn wraps_longitudes() {
        println!("geoPolygonContains wraps longitudes");

        let mut circle = CircleGenerator::default();
        circle.center_set(&Coord { x: 300f64, y: 0f64 });
        let c = circle.circle();
        let polygon = c;

        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 300f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: -60f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
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
                Coord {
                    x: -60f64,
                    y: -80f64,
                },
                Coord {
                    x: 60f64,
                    y: -80f64,
                },
                Coord {
                    x: 180f64,
                    y: -80f64,
                },
                Coord {
                    x: -60f64,
                    y: -80f64,
                },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: -85f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: -90f64 }),
            true
        );
    }

    #[test]
    fn north_pole() {
        println!("geoPolygonContains(northPole, point) returns the expected value");
        let polygon = Polygon::new(
            LineString(vec![
                Coord { x: 60f64, y: 80f64 },
                Coord {
                    x: -60f64,
                    y: 80f64,
                },
                Coord {
                    x: -180f64,
                    y: 80f64,
                },
                Coord { x: 60f64, y: 80f64 },
            ]),
            vec![],
        );

        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 85f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 90f64 }),
            true
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: -100f64,
                    y: 90f64
                }
            ),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: -90f64 }),
            false
        );
    }

    #[test]
    fn touching_pole() {
        println!("geoPolygonContains(touchingPole, Pole) returns true (issue #105)");
        let polygon = Polygon::new(
            LineString(vec![
                Coord { x: 0f64, y: -30f64 },
                Coord {
                    x: 120f64,
                    y: -30f64,
                },
                Coord { x: 0f64, y: -90f64 },
                Coord { x: 0f64, y: -30f64 },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: -90f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: -60f64,
                    y: -90f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: 60f64,
                    y: -90f64
                }
            ),
            false
        );
        let polygon = Polygon::new(
            LineString(vec![
                Coord { x: 0f64, y: 30f64 },
                Coord {
                    x: -120f64,
                    y: 30f64,
                },
                Coord { x: 0f64, y: 90f64 },
                Coord { x: 0f64, y: 30f64 },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 90f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: -60f64,
                    y: 90f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 60f64, y: 90f64 }),
            false
        );
    }

    #[test]
    fn south_hemisphere_poly() {
        println!("geoPolygonContains(southHemispherePoly) returns the expected value");
        let polygon = Polygon::new(
            LineString(vec![
                Coord { x: 0f64, y: 0f64 },
                Coord {
                    x: 10f64,
                    y: -40f64,
                },
                Coord {
                    x: -10f64,
                    y: -40f64,
                },
                Coord { x: 0f64, y: 0f64 },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: 0f64,
                    y: -40.2f64
                }
            ),
            true
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
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
                Coord { x: 0f64, y: 0f64 },
                Coord { x: 1f64, y: 0f64 },
                Coord { x: 1f64, y: 1f64 },
                Coord { x: 0f64, y: 1f64 },
                Coord { x: 0f64, y: 0f64 },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: 0.1f64,
                    y: 0.1f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
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
                Coord {
                    x: -60f64,
                    y: 80f64,
                },
                Coord { x: 60f64, y: 80f64 },
                Coord {
                    x: 180f64,
                    y: 80f64,
                },
                Coord {
                    x: -60f64,
                    y: 80f64,
                },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: 0.0f64,
                    y: 85.0f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn large_near_north_pole() {
        println!("geoPolygonContains(largeNearNorthPole, point) returns the expected value");
        let polygon = Polygon::new(
            LineString(vec![
                Coord {
                    x: 60f64,
                    y: -80f64,
                },
                Coord {
                    x: -60f64,
                    y: -80f64,
                },
                Coord {
                    x: -180f64,
                    y: -80f64,
                },
                Coord {
                    x: 60f64,
                    y: -80f64,
                },
            ]),
            vec![],
        );

        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: 0.0f64,
                    y: -85.0f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn large_circle() {
        println!("geoPolygonContains(largeCircle, point) returns the expected value");
        let mut circle = CircleGenerator::default();
        circle.radius_set(120.0);
        let c = circle.circle();
        let polygon = c;
        println!("polygon {polygon:#?}");
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: -180f64,
                    y: 0f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: -90f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn large_narrow_strip_hole() {
        println!("geoPolygonContains(largeNarrowStripHole, point) returns the expected value");
        let polygon = Polygon::new(
            LineString(vec![
                Coord {
                    x: -170f64,
                    y: -1f64,
                },
                Coord { x: 0f64, y: -1f64 },
                Coord {
                    x: 170f64,
                    y: -1f64,
                },
                Coord { x: 170f64, y: 1f64 },
                Coord { x: 0f64, y: 1f64 },
                Coord {
                    x: -170f64,
                    y: 1f64,
                },
                Coord {
                    x: -170f64,
                    y: -1f64,
                },
            ]),
            vec![],
        );

        assert_eq!(polygon_contains(&polygon, &Coord { x: 0.0, y: 0.0 }), false);
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 20f64 }),
            true
        );
    }

    #[test]
    fn large_narrow_equatorial_hole() {
        println!("geoPolygonContains(largeNarrowEquatorialHole, point) returns the expected value");
        let mut circle_gen = CircleGenerator::default();
        circle_gen
            .center_set(&Coord { x: 0f64, y: -90f64 })
            .radius_set(90f64 - 0.1f64);
        let ring0: LineString<f64> = circle_gen.circle().exterior().clone();

        let ring1 = circle_gen
            .radius_set(90f64 + 0.1f64)
            .circle()
            .exterior()
            .clone();

        let rev_vec: Vec<Coord<f64>> = ring1.into_iter().rev().collect();
        let ring1_rev = LineString(rev_vec);

        let polygon = Polygon::new(ring0, vec![ring1_rev]);

        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: -90f64 }),
            true
        );
    }

    #[test]
    fn large_narrow_equatorial_strip() {
        println!("geoPolygonContains(largeNarrowEquatorialHole, point) returns the expected value");

        let mut circle = CircleGenerator::default();
        circle.center_set(&Coord { x: 0f64, y: -90f64 });
        circle.radius_set(90f64 + 0.1f64);

        let ring1 = circle.circle().exterior().clone();

        let mut circle = CircleGenerator::default();
        circle
            .center_set(&Coord { x: 0f64, y: -90f64 })
            .radius_set(90f64 - 0.1f64);
        let c2 = circle.circle().exterior().clone();
        let rev_vec = c2.into_iter().rev().collect();
        let ring2 = LineString(rev_vec);

        let polygon = Polygon::new(ring1, vec![ring2]);
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: -90f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn ring_near_origin() {
        println!("geoPolygonContains(ringNearOrigin, point) returns the expected value");
        let ring0 = vec![
            Coord { x: 0f64, y: 0f64 },
            Coord { x: 0f64, y: 1f64 },
            Coord { x: 1f64, y: 1f64 },
            Coord { x: 1f64, y: 0f64 },
            Coord { x: 0f64, y: 0f64 },
        ];
        let ring1 = vec![
            Coord {
                x: 0.4f64,
                y: 0.4f64,
            },
            Coord {
                x: 0.6f64,
                y: 0.4f64,
            },
            Coord {
                x: 0.6f64,
                y: 0.6f64,
            },
            Coord {
                x: 0.4f64,
                y: 0.6f64,
            },
            Coord {
                x: 0.4f64,
                y: 0.4f64,
            },
        ];
        let polygon = Polygon::new(LineString(ring0), vec![LineString(ring1)]);

        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: 0.5f64,
                    y: 0.5f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
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
            Coord { x: 0f64, y: -10f64 },
            Coord {
                x: -120f64,
                y: -10f64,
            },
            Coord {
                x: 120f64,
                y: -10f64,
            },
            Coord { x: 0f64, y: -10f64 },
        ];
        let ring1 = vec![
            Coord { x: 0f64, y: 10f64 },
            Coord {
                x: 120f64,
                y: 10f64,
            },
            Coord {
                x: -120f64,
                y: 10f64,
            },
            Coord { x: 0f64, y: 10f64 },
        ];
        let polygon = Polygon::new(LineString(ring0), vec![LineString(ring1)]);

        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 20f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn ring_excluding_both_poles() {
        println!("geoPolygonContains(ringExcludingBothPoles, point) returns the expected value");
        let mut ring0 = vec![
            Coord { x: 10f64, y: 10f64 },
            Coord {
                x: -10f64,
                y: 10f64,
            },
            Coord {
                x: -10f64,
                y: -10f64,
            },
            Coord {
                x: 10f64,
                y: -10f64,
            },
            Coord { x: 10f64, y: 10f64 },
        ];
        ring0.reverse();
        let mut ring1 = vec![
            Coord {
                x: 170f64,
                y: 10f64,
            },
            Coord {
                x: 170f64,
                y: -10f64,
            },
            Coord {
                x: -170f64,
                y: -10f64,
            },
            Coord {
                x: -170f64,
                y: 10f64,
            },
            Coord {
                x: 170f64,
                y: 10f64,
            },
        ];
        ring1.reverse();
        let polygon = Polygon::new(LineString(ring0), vec![LineString(ring1)]);
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 90f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 0f64 }),
            true
        );
    }

    #[test]
    fn ring_containing_both_poles() {
        println!("geoPolygonContains(ringExcludingBothPoles, point) returns the expected value");
        let ring0 = vec![
            Coord { x: 10f64, y: 10f64 },
            Coord {
                x: -10f64,
                y: 10f64,
            },
            Coord {
                x: -10f64,
                y: -10f64,
            },
            Coord {
                x: 10f64,
                y: -10f64,
            },
            Coord { x: 10f64, y: 10f64 },
        ];
        let ring1 = vec![
            Coord {
                x: 170f64,
                y: 10f64,
            },
            Coord {
                x: 170f64,
                y: -10f64,
            },
            Coord {
                x: -170f64,
                y: -10f64,
            },
            Coord {
                x: -170f64,
                y: 10f64,
            },
            Coord {
                x: 170f64,
                y: 10f64,
            },
        ];
        let polygon = Polygon::new(LineString(ring0), vec![LineString(ring1)]);

        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 20f64 }),
            true
        );
    }

    #[test]
    fn ring_containing_south_pole() {
        println!("geoPolygonContains(ringContainingSouthPole, point) returns the expected value");
        let ring0 = vec![
            Coord { x: 10f64, y: 10f64 },
            Coord {
                x: -10f64,
                y: 10f64,
            },
            Coord {
                x: -10f64,
                y: -10f64,
            },
            Coord {
                x: 10f64,
                y: -10f64,
            },
            Coord { x: 10f64, y: 10f64 },
        ];
        let ring1 = vec![
            Coord { x: 0f64, y: 80f64 },
            Coord {
                x: 120f64,
                y: 80f64,
            },
            Coord {
                x: -120f64,
                y: 80f64,
            },
            Coord { x: 0f64, y: 80f64 },
        ];
        let polygon = Polygon::new(LineString(ring0), vec![LineString(ring1)]);

        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 90f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: -90f64 }),
            true
        );
    }

    #[test]
    fn ring_containing_north_pole() {
        println!("geoPolygonContains(ringContainingNorthPole, point) returns the expected value");
        let mut ring0 = vec![
            Coord { x: 10f64, y: 10f64 },
            Coord {
                x: -10f64,
                y: 10f64,
            },
            Coord {
                x: -10f64,
                y: -10f64,
            },
            Coord {
                x: 10f64,
                y: -10f64,
            },
            Coord { x: 10f64, y: 10f64 },
        ];
        ring0.reverse();
        let mut ring1 = vec![
            Coord { x: 0f64, y: 80f64 },
            Coord {
                x: 120f64,
                y: 80f64,
            },
            Coord {
                x: -120f64,
                y: 80f64,
            },
            Coord { x: 0f64, y: 80f64 },
        ];
        ring1.reverse();
        let polygon = Polygon::new(LineString(ring0), vec![LineString(ring1)]);

        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: -90f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 90f64 }),
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
                Coord { x: 0f64, y: 0f64 },
                Coord { x: 1f64, y: 0f64 },
                Coord { x: 1f64, y: 3f64 },
                Coord { x: 3f64, y: 3f64 },
                Coord { x: 3f64, y: 1f64 },
                Coord { x: 0f64, y: 1f64 },
                Coord { x: 0f64, y: 0f64 },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: 15f64,
                    y: 0.5f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 12f64, y: 2f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: 0.5f64,
                    y: 0.5f64
                }
            ),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 2f64, y: 2f64 }),
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
                Coord {
                    x: -10f64,
                    y: -80f64,
                },
                Coord {
                    x: 120f64,
                    y: -80f64,
                },
                Coord {
                    x: -120f64,
                    y: -80f64,
                },
                Coord {
                    x: 10f64,
                    y: -85f64,
                },
                Coord {
                    x: 10f64,
                    y: -75f64,
                },
                Coord {
                    x: -10f64,
                    y: 75f64,
                },
                Coord {
                    x: -10f64,
                    y: -80f64,
                },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: -76f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: -89f64 }),
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
                Coord {
                    x: -10f64,
                    y: 80f64,
                },
                Coord {
                    x: -10f64,
                    y: 75f64,
                },
                Coord { x: 10f64, y: 75f64 },
                Coord { x: 10f64, y: 85f64 },
                Coord {
                    x: -120f64,
                    y: 80f64,
                },
                Coord {
                    x: 120f64,
                    y: 80f64,
                },
                Coord {
                    x: -10f64,
                    y: 80f64,
                },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 76f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 89f64 }),
            true
        );
    }

    #[test]
    fn hemisphere_touching_the_south_pole() {
        println!(
            "geoPolygonContains(hemisphereTouchingTheSouthPole, point) returns the expected value"
        );

        let mut circle = CircleGenerator::default();
        circle.radius_set(90f64);

        let c = circle.circle();
        let polygon = &c;
        assert_eq!(polygon_contains(polygon, &Coord { x: 0f64, y: 0f64 }), true);
    }

    #[test]
    fn triangle_touching_the_south_pole() {
        println!(
            "geoPolygonContains(triangleTouchingTheSouthPole, point) returns the expected value"
        );
        let polygon = Polygon::new(
            LineString(vec![
                Coord {
                    x: 180f64,
                    y: -90f64,
                },
                Coord { x: -45f64, y: 0f64 },
                Coord { x: 45f64, y: 0f64 },
                Coord {
                    x: 180f64,
                    y: -90f64,
                },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: -46f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 1f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: -90f64,
                    y: -80f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: -44f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: -30f64 }),
            true
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
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
                Coord { x: -45f64, y: 0f64 },
                Coord { x: 45f64, y: 0f64 },
                Coord {
                    x: 180f64,
                    y: -90f64,
                },
                Coord { x: -45f64, y: 0f64 },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: -46f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 1f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: -90f64,
                    y: -80f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: -44f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: -30f64 }),
            true
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
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
                Coord {
                    x: 180f64,
                    y: -90f64,
                },
                Coord {
                    x: -135f64,
                    y: 0f64,
                },
                Coord { x: 135f64, y: 0f64 },
                Coord {
                    x: 180f64,
                    y: -90f64,
                },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 180f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 150f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: 180f64,
                    y: -30f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: 150f64,
                    y: -80f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 0f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 180f64, y: 1f64 }),
            true
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
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
                Coord {
                    x: 180f64,
                    y: 90f64,
                },
                Coord { x: 45f64, y: 0f64 },
                Coord { x: -45f64, y: 0f64 },
                Coord {
                    x: 180f64,
                    y: 90f64,
                },
            ]),
            vec![],
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: -90f64, y: 0f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: -1f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: -80f64 }),
            false
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: -90f64, y: 1f64 }),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: -90f64,
                    y: 80f64
                }
            ),
            false
        );
        assert_eq!(
            polygon_contains(
                &polygon,
                &Coord {
                    x: -44f64,
                    y: 10f64
                }
            ),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 0f64, y: 10f64 }),
            true
        );
        assert_eq!(
            polygon_contains(&polygon, &Coord { x: 30f64, y: 80f64 }),
            true
        );
    }
}
