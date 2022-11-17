#[cfg(not(tarpaulin_include))]
mod area {
    extern crate pretty_assertions;

    use geo::line_string;
    use geo::polygon;
    use geo::CoordFloat;
    use geo::Geometry;
    use geo::LineString;
    use geo::MultiLineString;
    use geo::MultiPolygon;
    use geo::Point;
    use geo::Polygon;
    use geo_types::Coord;

    use pretty_assertions::assert_eq;

    use rust_d3_array::range::range;
    use rust_d3_geo::area::Area;
    use rust_d3_geo::circle::generator::Generator as CircleGenerator;
    use rust_d3_geo::data_object::sphere::Sphere;
    use rust_d3_geo::graticule::generate as generate_graticule;
    use rust_d3_geo::in_delta::in_delta;

    fn stripes<T>(a: T, b: T) -> Polygon<T>
    where
        T: CoordFloat,
    {
        let mut exterior: Vec<(T, T)> = Vec::new();
        let mut interior: Vec<(T, T)> = Vec::new();
        for (i, d) in [a, b].iter().enumerate() {
            let mut stripe: Vec<(T, T)> = range(
                T::from(-180_f64).unwrap(),
                T::from(180_f64).unwrap(),
                T::from(0.1_f64).unwrap(),
            )
            .iter()
            .map(|x| (*x, *d))
            .collect();
            stripe.push(stripe[0]);

            if i == 0usize {
                exterior = stripe;
            } else {
                stripe.reverse();
                interior = stripe;
            }
        }

        Polygon::new(LineString::from(exterior), vec![LineString::from(interior)])
    }

    #[test]
    fn point() {
        println!("area: Point");

        let g = Geometry::Point(Point::new(0_f64, 0_f64));
        let area = Area::calc(&g);
        assert_eq!(area, 0_f64);
    }

    #[test]
    fn multipoint() {
        println!("area: Mutlipoint");
        let g = Geometry::MultiPoint(vec![(0_f64, 1_f64), (2_f64, 3_f64)].into());
        let area = Area::calc(&g);
        assert_eq!(area, 0_f64);
    }

    #[test]
    fn line_string() {
        println!("area: LineString");
        let g = Geometry::LineString(vec![(0_f64, 1_f64), (2_f64, 3_f64)].into());
        let area = Area::calc(&g);
        assert_eq!(area, 0_f64);
    }

    #[test]
    fn multiline_string() {
        println!("area: LineString");
        let g = MultiLineString(vec![
            line_string![(x:0_f64, y:1_f64), (x:2_f64, y:3_f64)],
            line_string![(x:4_f64, y:5_f64), (x:6_f64,y:7_f64)],
        ]);
        let area = Area::calc(&g);
        assert_eq!(area, 0_f64);
    }

    #[test]
    fn polygon_tiny() {
        println!("area: Polygon - tiny");

        #[allow(clippy::excessive_precision)]
        let g = polygon![
            (x:-64.66070178517852, y:18.33986913231323),
            (x:-64.66079715091509, y:18.33994007490749),
            (x:-64.66074946804680, y:18.33994007490749),
            (x:-64.66070178517852, y:18.33986913231323)
        ];
        let area = Area::calc(&g);
        assert!(in_delta(area, 4.890516e-13, 1e-13));
    }

    // This is only works with f32s
    //
    // Looking at src/area.rs
    //
    // if area_ring < T::zero() {
    //     self.area_sum = self.area_sum + T::TAU() + area_ring;
    // } else {
    //     self.area_sum = self.area_sum + area_ring;
    // }
    //
    // Given the tightly specified floats in the polygon.
    // The test is designed to make 'self.area_ring' equal exactly zero.
    // In rust and with f64s the round errors make the sum slightly negative
    // ... so the test fails because TAU has been added.
    //
    #[test]
    fn polygon_zero_area() {
        println!("area: Polygon - zero area");

        #[allow(clippy::excessive_precision)]
        let g = polygon![
        (x: 96.79142432523281, y:5.262704519048153),
        (x: 96.81065389253769, y:5.272455576551362),
        (x: 96.82988345984256, y:5.272455576551362),
        (x: 96.81065389253769, y:5.272455576551362),
        (x: 96.79142432523281, y:5.262704519048153)
            ];
        let area = Area::calc(&g);
        assert_eq!(area, 0_f32);
    }

    #[test]
    fn polygon_semilune() {
        println!("area: Polygon - semilune");

        let g = polygon![
            (x:0_f64, y:0_f64),
            (x:0_f64, y:90_f64),
            (x:90_f64, y:0_f64),
            (x:0_f64, y:0_f64)
        ];

        let area = Area::calc(&g);
        assert!(in_delta(area, std::f64::consts::PI / 2_f64, 1e-6));
    }

    #[test]
    fn polygon_lune() {
        println!("area: Polygon - lune");

        let g = polygon![
            (x:0_f64, y:0_f64),
            (x:0_f64, y:90_f64),
            (x:90_f64, y:0_f64),
            (x:0_f64, y:-90_f64),
            (x:0_f64, y:0_f64)
        ];

        let area = Area::calc(&g);
        assert!(in_delta(area, std::f64::consts::PI, 1e-6));
    }

    #[test]
    fn polygon_hemispheres_north() {
        println!("area: Polygon - hemispheres north");

        let g = polygon![
            (x:0_f64, y:0_f64),
            (x:-90_f64, y:0_f64),
            (x:180_f64, y:0_f64),
            (x:90_f64, y:0_f64),
            (x:0_f64, y:0_f64)
        ];

        let area = Area::calc(&g);
        assert!(in_delta(area, 2_f64 * std::f64::consts::PI, 1e-6));
    }

    #[test]
    fn polygon_hemispheres_south() {
        println!("area: Polygon - hemispheres south");

        let g = polygon![
            (x:0_f64, y:0_f64),
            (x:90_f64, y:0_f64),
            (x:180_f64, y:0_f64),
            (x:-90_f64, y:0_f64),
            (x:0_f64, y:0_f64)
        ];

        let area = Area::calc(&g);
        assert!(in_delta(area, 2_f64 * std::f64::consts::PI, 1e-6));
    }

    #[test]
    fn polygon_hemispheres_east() {
        println!("area: Polygon - hemispheres east");

        let g = polygon![
            (x:0_f64, y:0_f64),
            (x:0_f64, y:90_f64),
            (x:180_f64, y:0_f64),
            (x:0_f64, y:-90_f64),
            (x:0_f64, y:0_f64)
        ];

        let area = Area::calc(&g);
        assert!(in_delta(area, 2_f64 * std::f64::consts::PI, 1e-6));
    }

    #[test]
    fn polygon_hemispheres_west() {
        println!("area: Polygon - hemispheres west");

        let g = polygon![
            (x:0_f64, y:0_f64),
            (x:0_f64, y:-90_f64),
            (x:180_f64, y:0_f64),
            (x:0_f64, y:90_f64),
            (x:0_f64, y:0_f64)
        ];

        let area = Area::calc(&g);
        assert!(in_delta(area, 2_f64 * std::f64::consts::PI, 1e-6));
    }

    #[test]
    fn graticule_outline_sphere() {
        println!("area: Polygon - graticule outline sphere");
        let mut graticule = generate_graticule();
        graticule.extent_set([[-180_f64, -90_f64], [180_f64, 90_f64]]);
        let outline = graticule.outline();
        let area = Area::calc(&outline);
        assert!(in_delta(area, 4_f64 * std::f64::consts::PI, 1e-5));
    }

    #[test]
    fn graticule_outline_hemisphere() {
        println!("area: Polygon - graticule outline hemisphere");
        let mut graticule = generate_graticule();
        graticule.extent_set([[-180_f64, 0_f64], [180_f64, 90_f64]]);
        let outline = graticule.outline();
        let area = Area::calc(&outline);
        assert!(in_delta(area, 2_f64 * std::f64::consts::PI, 1e-5));
    }

    #[test]
    fn graticule_outline_semilune() {
        println!("area: Polygon - graticule outline semilune");
        let mut graticule = generate_graticule();
        graticule.extent_set([[0_f64, 0_f64], [90_f64, 90_f64]]);
        let outline = graticule.outline();
        let area = Area::calc(&outline);
        assert!(in_delta(area, std::f64::consts::FRAC_PI_2, 1e-5));
    }

    #[test]
    fn circle_hemisphere() {
        println!("area: Polygon - circles hemisphere");
        let circle = CircleGenerator::default().radius_set(90_f64).circle();
        let area = Area::calc(&circle);
        assert!(in_delta(area, 2_f64 * std::f64::consts::PI, 1e-5));
    }

    #[test]
    fn circle_plus_60() {
        println!("area: Polygon - circles 60°");
        let circle = CircleGenerator::default()
            .radius_set(60_f64)
            .precision_set(0.1_f64)
            .circle();
        let area = Area::calc(&circle);
        assert!(in_delta(area, std::f64::consts::PI, 1e-5));
    }

    #[test]
    fn circle_plus_60_north() {
        println!("area: Polygon - circles 60° North");
        let circle = CircleGenerator::default()
            .radius_set(60_f64)
            .precision_set(0.1_f64)
            .center_set(&Coord {
                x: 0_f64,
                y: 90_f64,
            })
            .circle();
        let area = Area::calc(&circle);
        assert!(in_delta(area, std::f64::consts::PI, 1e-5));
    }

    #[test]
    fn circle_plus_45() {
        println!("area: Polygon - circles 45°");
        let circle = CircleGenerator::default()
            .radius_set(45_f64)
            .precision_set(0.1_f64)
            .circle();
        let area = Area::calc(&circle);
        assert!(in_delta(
            area,
            (2_f64 - (2_f64).sqrt()) * std::f64::consts::PI,
            1e-5
        ));
    }

    #[test]
    fn circle_plus_45_north() {
        println!("area: Polygon - circles 45° North");
        let circle = CircleGenerator::default()
            .radius_set(45_f64)
            .precision_set(0.1_f64)
            .center_set(&Coord {
                x: 0_f64,
                y: 90_f64,
            })
            .circle();
        let area = Area::calc(&circle);
        assert!(in_delta(
            area,
            (2_f64 - (2_f64).sqrt()) * std::f64::consts::PI,
            1e-5
        ));
    }

    #[test]
    fn circle_plus_45_south() {
        println!("area: Polygon - circles 45° South");
        let circle = CircleGenerator::default()
            .radius_set(45_f64)
            .precision_set(0.1_f64)
            .center_set(&Coord {
                x: 0_f64,
                y: -90_f64,
            })
            .circle();
        let area = Area::calc(&circle);
        assert!(in_delta(
            area,
            (2_f64 - (2_f64).sqrt()) * std::f64::consts::PI,
            1e-5
        ));
    }

    #[test]
    fn circle_plus_135() {
        println!("area: Polygon - circles 45° South");
        let circle = CircleGenerator::default()
            .radius_set(135_f64)
            .precision_set(0.1_f64)
            .circle();
        let area = Area::calc(&circle);
        assert!(in_delta(
            area,
            (2_f64 + (2_f64).sqrt()) * std::f64::consts::PI,
            1e-5
        ));
    }

    #[test]
    fn circle_plus_135_north() {
        println!("area: Polygon - circles 45° South");
        let circle = CircleGenerator::default()
            .radius_set(135_f64)
            .precision_set(0.1_f64)
            .center_set(&Coord {
                x: 0_f64,
                y: 90_f64,
            })
            .circle();
        let area = Area::calc(&circle);
        assert!(in_delta(
            area,
            (2_f64 + (2_f64).sqrt()) * std::f64::consts::PI,
            1e-5
        ));
    }

    #[test]
    fn circle_plus_135_south() {
        println!("area: Polygon - circles 45° South");
        let circle = CircleGenerator::default()
            .radius_set(135_f64)
            .precision_set(0.1_f64)
            .center_set(&Coord {
                x: 0_f64,
                y: -90_f64,
            })
            .circle();
        let area = Area::calc(&circle);
        assert!(in_delta(
            area,
            (2_f64 + (2_f64).sqrt()) * std::f64::consts::PI,
            1e-5
        ));
    }

    #[test]
    fn circles_tiny() {
        println!("area: Polygon - circles tiny");
        let circle = CircleGenerator::default()
            .radius_set(1e-6_f64)
            .precision_set(0.1_f64)
            .circle();
        let area = Area::calc(&circle);
        assert!(in_delta(area, 0_f64, 1e-5));
    }

    #[test]
    fn circles_huge() {
        println!("area: Polygon - circles tiny");
        let circle = CircleGenerator::default()
            .radius_set(180_f64 - 1e-6_f64)
            .precision_set(0.1_f64)
            .circle();
        let area = Area::calc(&circle);
        assert!(in_delta(area, 4_f64 * std::f64::consts::PI, 1e-5));
    }

    #[test]
    fn circles_60_with_45_hole() {
        println!("area: Polygon - circles 60° with 45° hole");
        let ring1 = CircleGenerator::default()
            .precision_set(0.1)
            .radius_set(60_f64)
            .circle()
            .exterior()
            .clone();
        let ring2 = CircleGenerator::default()
            .precision_set(0.1)
            .radius_set(45_f64)
            .circle()
            .exterior()
            .clone();

        let rev_vec = ring2.into_iter().rev().collect();
        let ring2_rev = LineString(rev_vec);

        let polygon = Polygon::new(ring1, vec![ring2_rev]);
        assert!(in_delta(
            Area::calc(&polygon),
            (2_f64.sqrt() - 1_f64) * std::f64::consts::PI,
            1e-5
        ));
    }

    #[test]
    fn circles_45_with_hole_0_0_and_0_90() {
        println!("area: Polygon - circles 45° holes at [0°, 0°] and [0°, 90°]");
        let ring1 = CircleGenerator::default()
            .precision_set(0.1)
            .radius_set(45_f64)
            .center_set(&Coord { x: 0_f64, y: 0_f64 })
            .circle()
            .exterior()
            .clone();
        let rev1_vec: Vec<Coord<f64>> = ring1.into_iter().rev().collect();
        let ring1_rev = LineString(rev1_vec);

        let ring2 = CircleGenerator::default()
            .precision_set(0.1)
            .radius_set(45_f64)
            .center_set(&Coord {
                x: 0_f64,
                y: 90_f64,
            })
            .circle()
            .exterior()
            .clone();

        let rev2_vec = ring2.into_iter().rev().collect();
        let ring2_rev = LineString(rev2_vec);

        let polygon = Polygon::new(ring1_rev, vec![ring2_rev]);
        assert!(in_delta(
            Area::calc(&polygon),
            2_f64 * 2_f64.sqrt() * std::f64::consts::PI,
            1e-5
        ));
    }

    #[test]
    fn circles_45_with_hole_0_90_and_0_0() {
        println!("area: Polygon - circles 45° holes at [0°, 90°] and [0°, 0°]");
        let ring1 = CircleGenerator::default()
            .precision_set(0.1)
            .radius_set(45_f64)
            .center_set(&Coord {
                x: 0_f64,
                y: 90_f64,
            })
            .circle()
            .exterior()
            .clone();
        let rev1_vec: Vec<Coord<f64>> = ring1.into_iter().rev().collect();
        let ring1_rev = LineString(rev1_vec);

        let ring2 = CircleGenerator::default()
            .precision_set(0.1)
            .radius_set(45_f64)
            .center_set(&Coord { x: 0_f64, y: 0_f64 })
            .circle()
            .exterior()
            .clone();

        let rev2_vec: Vec<Coord<f64>> = ring2.into_iter().rev().collect();
        let ring2_rev = LineString(rev2_vec);

        let polygon = Polygon::new(ring1_rev, vec![ring2_rev]);
        assert!(in_delta(
            Area::calc(&polygon),
            2_f64 * 2_f64.sqrt() * std::f64::consts::PI,
            1e-5
        ));
    }

    #[test]
    fn stripes_45_minus_45() {
        println!("area: Polygon - stripes 45°, -45°");
        let stripes = stripes(45_f64, -45_f64);
        let area = Area::calc(&stripes);
        assert!(in_delta(
            area,
            std::f64::consts::PI * 2_f64 * 2f64.sqrt(),
            1e-5
        ));
    }

    #[test]
    fn stripes_minus_45_plus_45() {
        println!("area: Polygon - stripes 45°, 45°");
        let stripes = stripes(-45_f64, 45_f64);
        let area = Area::calc(&stripes);
        assert!(in_delta(
            area,
            std::f64::consts::PI * 2_f64 * (2_f64 - 2f64.sqrt()),
            1e-5
        ));
    }

    #[test]
    fn stripes_45_30() {
        println!("area: Polygon - stripes 45°, 30°");
        let stripes = stripes(45_f64, 30_f64);
        let area = Area::calc(&stripes);
        assert!(in_delta(
            area,
            std::f64::consts::PI * (2f64.sqrt() - 1_f64),
            1e-5
        ));
    }

    #[test]
    fn two_hemispheres() {
        println!("area: MultiPolygon two hemispheres");
        let mp = MultiPolygon(vec![
            Polygon::new(
                line_string![
                    (x: 0_f64, y:0_f64),
                     (x:-90_f64, y:0_f64),
                      (x: 180_f64, y: 0_f64),
                       (x:90_f64, y:0_f64),
                        (x:0_f64, y:0_f64)
                ],
                vec![],
            ),
            Polygon::new(
                line_string![
                    (x: 0_f64, y:0_f64),
                    (x:90_f64, y:0_f64),
                     (x: 180_f64, y: 0_f64),
                      (x:-90_f64, y:0_f64),
                       (x:0_f64, y:0_f64)
                ],
                vec![],
            ),
        ]);
        let data_o = Geometry::MultiPolygon(mp);
        let area = Area::calc(&data_o);
        assert!(in_delta(area, 4_f64 * std::f64::consts::PI, 1e-6));
    }

    #[test]
    fn sphere() {
        println!("area: Sphere");

        let g = Sphere::default();

        let area = Area::calc(&g);
        assert!(in_delta(area, 4_f64 * std::f64::consts::PI, 1e-6));
    }

    // TODO add GC, FC, F here.
}
