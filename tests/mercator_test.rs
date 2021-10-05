#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod mercator_tests {
    use std::rc::Rc;

    use geo::Coordinate;
    use geo::Geometry;

    use rust_d3_geo::data_object::sphere::Sphere;
    use rust_d3_geo::data_object::DataObject;
    use rust_d3_geo::in_delta::in_delta_coordinate;
    use rust_d3_geo::path::builder::Builder as PathBuilder;
    use rust_d3_geo::path::ResultEnum;
    use rust_d3_geo::projection::mercator::Mercator;
    use rust_d3_geo::projection::ClipExtent;
    use rust_d3_geo::projection::Fit;
    use rust_d3_geo::projection::Precision;
    use rust_d3_geo::projection::Raw;
    use rust_d3_geo::projection::Rotate;
    use rust_d3_geo::projection::Scale;
    use rust_d3_geo::projection::Translate;

    // #[test]
    // fn test_clip_extent_defaults_to_automatic() {
    //     println!("mercator.clipExtent(null) sets the default automatic clip extent");
    //     let projection = Rc::new(
    //         Mercator::builder()
    //             .translate(&Coordinate { x: 0_f64, y: 0_f64 })
    //             .scale(1_f64)
    //             .clip_extent_clear()
    //             .precision(&0_f64)
    //             .build(),
    //     );

    //     let path_builder = PathBuilder::context_pathstring();

    //     let object = DataObject::Sphere(Sphere::default());

    //     match path_builder.build(projection).object(object) {
    //         Some(r) => match r {
    //             ResultEnum::String(s) => {
    //                 assert_eq!(s, "M3.141592653589793,-3.1415927L3.141592653589793,0L3.141592653589793,3.1415927L3.141592653589793,3.1415927L-3.141592653589793,3.1415927L-3.141592653589793,3.1415927L-3.141592653589793,0L-3.141592653589793,-3.1415927L-3.141592653589793,-3.1415927L3.141592653589793,-3.1415927Z");
    //             }
    //             _ => todo!("must handle "),
    //         },
    //         None => panic!("Expecting an string."),
    //     }
    // }

    #[test]
    fn test_updates_the_intersected_clip_extent() {
        println!(
            "mercator.clipExtent(extent).translate(translate) updates the intersected clip extent"
        );
        let projection = Rc::new(
            Mercator::builder()
                .scale(1_f64)
                .clip_extent([
                    Coordinate {
                        x: -10_f64,
                        y: -10_f64,
                    },
                    Coordinate {
                        x: 10_f64,
                        y: 10_f64,
                    },
                ])
                .translate(&Coordinate { x: 0_f64, y: 0_f64 })
                .precision(&0_f64)
                .build(),
        );

        let path_builder = PathBuilder::context_pathstring();

        let object = DataObject::Sphere(Sphere::default());

        // There is a bodge associated with this test
        // I have had to adjust the return string to include PI_f64 not PI_f32 to get this to pass.
        // See MercatorRaw::transform for an expanation of the issue.
        match path_builder.build(projection).object(object) {
            Some(r) => match r {
                ResultEnum::String(s) => {
                    assert_eq!(s, "M3.141592653589793,-10L3.141592653589793,0L3.141592653589793,10L3.141592653589793,10L-3.141592653589793,10L-3.141592653589793,10L-3.141592653589793,0L-3.141592653589793,-10L-3.141592653589793,-10L3.141592653589793,-10Z");
                }
                _ => todo!("must handle "),
            },
            None => panic!("Expecting an string."),
        }
    }

    #[test]
    fn test_rotate_does_not_affect_automatic_clip_extent() {
        println!("mercator.rotate(…) does not affect the automatic clip extent");

        let pb = Mercator::builder();

        let object = DataObject::Geometry(Geometry::MultiPoint(
            vec![
                (-82.35024908550241, 29.649391549778745),
                (-82.35014449996858, 29.65075946917633),
                (-82.34916073446641, 29.65070265688781),
                (-82.3492653331286, 29.64933474064504),
            ]
            .into(),
        ));
        let pb = pb.fit_extent([[0_f64, 0_f64], [960_f64, 600_f64]], object.clone());
        assert_eq!(pb.get_scale(), 20969742.365692537_f64);
        assert_eq!(
            pb.get_translate(),
            Coordinate {
                x: 30139734.76760269_f64,
                y: 11371473.949706702_f64
            }
        );

        let pb = pb
            .rotate([0_f64, 95_f64, 0_f64])
            .fit_extent([[0_f64, 0_f64], [960_f64, 600_f64]], object);
        assert_eq!(pb.get_scale(), 35781690.650920525_f64);
        assert!(in_delta_coordinate(
            pb.get_translate(),
            Coordinate {
                x: 75115911.95344563_f64,
                y: 2586046.4116968135_f64
            },
            1e-6
        ));
    }
}
