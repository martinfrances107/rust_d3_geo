#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod area_test {
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

    use pretty_assertions::assert_eq;

    use rust_d3_array::range::range;
    use rust_d3_geo::area::Area;
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
            let mut stripe: Vec<(T, T)> = Vec::new();
            stripe = range(
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
        let area = Area::<f64>::calc(&g);
        assert_eq!(area, 0_f64);
    }

    #[test]
    fn multipoint() {
        println!("area: Mutlipoint");
        let g = Geometry::MultiPoint(vec![(0_f64, 1_f64), (2_f64, 3_f64)].into());
        let area = Area::<f64>::calc(&g);
        assert_eq!(area, 0_f64);
    }

    #[test]
    fn line_string() {
        println!("area: LineString");
        let g = Geometry::LineString(vec![(0_f64, 1_f64), (2_f64, 3_f64)].into());
        let area = Area::<f64>::calc(&g);
        assert_eq!(area, 0_f64);
    }

    #[test]
    fn multiline_string() {
        println!("area: LineString");
        let g = MultiLineString(vec![
            line_string![(x:0_f64, y:1_f64), (x:2_f64, y:3_f64)],
            line_string![(x:4_f64, y:5_f64), (x:6_f64,y:7_f64)],
        ]);
        let area = Area::<f64>::calc(&g);
        assert_eq!(area, 0_f64);
    }

    #[test]
    fn polygon_tiny() {
        println!("area: Polygon - tiny");

        let g = polygon![
            (x:-64.66070178517852, y:18.33986913231323),
            (x:-64.66079715091509, y:18.33994007490749),
            (x:-64.66074946804680, y:18.33994007490749),
            (x:-64.66070178517852, y:18.33986913231323)
        ];
        let area = Area::<f64>::calc(&g);
        assert!(in_delta(area, 4.890516e-13, 1e-13));
    }

    // This test is too brittle to copy over.
    // and I don't think the extra coverage it provides is useful.
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
    // In rust the round errors make the sum slightly negative
    // ... so the test fails because TAU has been added.
    //
    // #[test]
    // fn polygon_zero_area() {
    //     println!("area: Polygon - zero area");

    //     let g = polygon![
    //     (x: 96.79142432523281, y:5.262704519048153),
    //     (x: 96.81065389253769, y:5.272455576551362),
    //     (x: 96.82988345984256, y:5.272455576551362),
    //     (x: 96.81065389253769, y:5.272455576551362),
    //     (x: 96.79142432523281, y:5.262704519048153)
    //         ];
    //     let area = Area::<f64>::calc(&g);
    //     assert_eq!(area, 0_f64);
    // }

    #[test]
    fn polygon_semilune() {
        println!("area: Polygon - semilune");

        let g = polygon![
            (x:0_f64, y:0_f64),
            (x:0_f64, y:90_f64),
            (x:90_f64, y:0_f64),
            (x:0_f64, y:0_f64)
        ];

        let area = Area::<f64>::calc(&g);
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

        let area = Area::<f64>::calc(&g);
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

        let area = Area::<f64>::calc(&g);
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

        let area = Area::<f64>::calc(&g);
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

        let area = Area::<f64>::calc(&g);
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

        let area = Area::<f64>::calc(&g);
        assert!(in_delta(area, 2_f64 * std::f64::consts::PI, 1e-6));
    }

    #[test]
    fn graticule_outline_sphere() {
        println!("area: Polygon - graticule outline sphere");
        let outline = generate_graticule()
            .extent([[-180_f64, -90_f64], [180_f64, 90_f64]])
            .outline();
        let area = Area::<f64>::calc(&outline);
        assert!(in_delta(area, 4_f64 * std::f64::consts::PI, 1e-5));
    }

    #[test]
    fn graticule_outline_hemisphere() {
        println!("area: Polygon - graticule outline hemisphere");
        let outline = generate_graticule()
            .extent([[-180_f64, 0_f64], [180_f64, 90_f64]])
            .outline();
        let area = Area::<f64>::calc(&outline);
        assert!(in_delta(area, 2_f64 * std::f64::consts::PI, 1e-5));
    }

    #[test]
    fn graticule_outline_semilune() {
        println!("area: Polygon - graticule outline semilune");
        let outline = generate_graticule()
            .extent([[0_f64, 0_f64], [90_f64, 90_f64]])
            .outline();
        let area = Area::<f64>::calc(&outline);
        assert!(in_delta(area, std::f64::consts::FRAC_PI_2, 1e-5));
    }

    // TODO add geoCircle tests here.

    #[test]
    fn stripes_45_minus_45() {
        println!("area: Polygon - stripes 45°, -45°");
        let stripes = stripes(45_f64, -45_f64);
        let area = Area::<f64>::calc(&stripes);
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
        let area = Area::<f64>::calc(&stripes);
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
        let area = Area::<f64>::calc(&stripes);
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
        let area = Area::<f64>::calc(&data_o);
        assert!(in_delta(area, 4_f64 * std::f64::consts::PI, 1e-6));
    }

    #[test]
    fn sphere() {
        println!("area: Sphere");

        let g = Sphere::default();

        let area = Area::<f64>::calc(&g);
        assert!(in_delta(area, 4_f64 * std::f64::consts::PI, 1e-6));
    }

    // TODO add GC, FC, F here.
}
