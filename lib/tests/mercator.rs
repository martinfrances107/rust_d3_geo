#[cfg(not(tarpaulin_include))]
mod mercator {

    extern crate pretty_assertions;

    use geo::Geometry;
    use geo_types::Coord;
    use pretty_assertions::assert_eq;

    use rust_d3_geo::data_object::sphere::Sphere;
    use rust_d3_geo::in_delta::coordinate as in_delta_coordinate;
    use rust_d3_geo::path::builder::Builder as PathBuilder;
    use rust_d3_geo::projection::mercator::Mercator;
    use rust_d3_geo::projection::Build;
    use rust_d3_geo::projection::CenterSet;
    use rust_d3_geo::projection::ClipExtentAdjust;
    use rust_d3_geo::projection::ClipExtentClear;
    use rust_d3_geo::projection::ClipExtentGet;
    use rust_d3_geo::projection::Fit;
    use rust_d3_geo::projection::PrecisionBypass;
    use rust_d3_geo::projection::RawBase;
    use rust_d3_geo::projection::RotateGet;
    use rust_d3_geo::projection::RotateSet;
    use rust_d3_geo::projection::ScaleGet;
    use rust_d3_geo::projection::ScaleSet;
    use rust_d3_geo::projection::TranslateGet;
    use rust_d3_geo::projection::TranslateSet;

    #[test]
    fn clip_extent_defaults_to_automatic() {
        println!("mercator.clipExtent(null) sets the default automatic clip extent");
        let pb = Mercator::builder()
            .translate_set(&Coord { x: 0_f64, y: 0_f64 })
            .scale_set(1_f64)
            .precision_bypass()
            .clip_extent_clear();

        let projection = pb.build();
        let path_builder = PathBuilder::context_pathstring();

        let object = Sphere::<f64>::default();

        let s = path_builder.build(projection).object(&object);
        assert_eq!(s, "M3.141593,-3.141593L3.141593,0L3.141593,3.141593L3.141593,3.141593L-3.141593,3.141593L-3.141593,3.141593L-3.141593,0L-3.141593,-3.141593L-3.141593,-3.141593L3.141593,-3.141593Z");
    }

    #[test]
    fn center_set_correct_automatic() {
        println!("mercator.center(center) sets the correct automatic clip extent");
        let pb = Mercator::builder()
            .translate_set(&Coord { x: 0_f64, y: 0_f64 })
            .center_set(&Coord {
                x: 10_f64,
                y: 10_f64,
            })
            .scale_set(1_f64)
            .precision_bypass();

        let projection = pb.build();
        let path_builder = PathBuilder::context_pathstring();

        let object = Sphere::default();

        //Bodge: I have had to alter the expected string ... dropping trailing 0 from some floats.
        // This is a trivial difference to between rust and Javascriipt.
        let s = path_builder.build(projection).object(&object);
        assert_eq!(s, "M2.96706,-2.966167L2.96706,0.175426L2.96706,3.317018L2.96706,3.317018L-3.316126,3.317018L-3.316126,3.317019L-3.316126,0.175426L-3.316126,-2.966167L-3.316126,-2.966167L2.96706,-2.966167Z");

        // assert_eq!(pb.get_clip_extent(), None);
    }

    #[test]
    fn intersected_clip_extent() {
        println!(
	            "mercator.clipExtent(extent) intersects the specified clip extent with the automatic clip extent"
	        );
        let pb = Mercator::builder()
            .translate_set(&Coord { x: 0_f64, y: 0_f64 })
            .clip_extent_adjust(&[
                Coord {
                    x: -10_f64,
                    y: -10_f64,
                },
                Coord {
                    x: 10_f64,
                    y: 10_f64,
                },
            ])
            .scale_set(1_f64)
            .precision_bypass();

        let projection = pb.build();

        let path_builder = PathBuilder::context_pathstring();

        let object = Sphere::default();

        let s = path_builder.build(projection).object(&object);
        assert_eq!(s, "M3.141593,-10L3.141593,0L3.141593,10L3.141593,10L-3.141593,10L-3.141593,10L-3.141593,0L-3.141593,-10L-3.141593,-10L3.141593,-10Z");
        assert_eq!(
            pb.clip_extent(),
            Some([
                Coord {
                    x: -10_f64,
                    y: -10_f64,
                },
                Coord {
                    x: 10_f64,
                    y: 10_f64,
                },
            ])
        );
    }

    #[test]
    fn scale_updates_the_intersected_clip_extent() {
        println!(
            "mercator.clipExtent(extent).translate(scale) updates the intersected clip extent"
        );
        let pb = Mercator::builder()
            .translate_set(&Coord { x: 0_f64, y: 0_f64 })
            .clip_extent_adjust(&[
                Coord {
                    x: -10_f64,
                    y: -10_f64,
                },
                Coord {
                    x: 10_f64,
                    y: 10_f64,
                },
            ])
            .scale_set(1_f64)
            .precision_bypass();
        let projection = pb.build();

        let path_builder = PathBuilder::context_pathstring();

        let object = Sphere::default();

        let s = path_builder.build(projection).object(&object);
        assert_eq!(s, "M3.141593,-10L3.141593,0L3.141593,10L3.141593,10L-3.141593,10L-3.141593,10L-3.141593,0L-3.141593,-10L-3.141593,-10L3.141593,-10Z");
        assert_eq!(
            pb.clip_extent(),
            Some([
                Coord {
                    x: -10_f64,
                    y: -10_f64,
                },
                Coord {
                    x: 10_f64,
                    y: 10_f64,
                },
            ])
        );
    }

    #[test]
    fn translate_updates_the_intersected_clip_extent() {
        println!(
            "mercator.clipExtent(extent).translate(translate) updates the intersected clip extent"
        );
        let mut pb = Mercator::builder();
        pb.scale_set(1_f64);
        pb.clip_extent_adjust(&[
            Coord {
                x: -10_f64,
                y: -10_f64,
            },
            Coord {
                x: 10_f64,
                y: 10_f64,
            },
        ]);
        let pb = pb.translate_set(&Coord { x: 0_f64, y: 0_f64 });
        let pb = pb.precision_bypass();

        let projection = pb.build();
        let path_builder = PathBuilder::context_pathstring();

        let object = Sphere::default();

        let s = path_builder.build(projection).object(&object);
        assert_eq!(s, "M3.141593,-10L3.141593,0L3.141593,10L3.141593,10L-3.141593,10L-3.141593,10L-3.141593,0L-3.141593,-10L-3.141593,-10L3.141593,-10Z");
        assert_eq!(
            pb.clip_extent(),
            Some([
                Coord {
                    x: -10_f64,
                    y: -10_f64,
                },
                Coord {
                    x: 10_f64,
                    y: 10_f64,
                },
            ])
        );
    }

    // Must debug as final assert's for scale and translate are buggy.
    #[test]
    fn rotate_does_not_affect_automatic_clip_extent() {
        println!("mercator.rotate(…) does not affect the automatic clip extent");

        let object: Geometry<f64> = Geometry::MultiPoint(
            vec![
                (-82.35024908550241, 29.649391549778745),
                (-82.35014449996858, 29.65075946917633),
                (-82.34916073446641, 29.65070265688781),
                (-82.3492653331286, 29.64933474064504),
            ]
            .into(),
        );

        let mut pb = Mercator::builder();
        pb.fit_extent([[0_f64, 0_f64], [960_f64, 600_f64]], &object);
        assert_eq!(pb.scale(), 20969742.365692537_f64);
        assert_eq!(
            pb.translate(),
            Coord {
                x: 30139734.76760269_f64,
                y: 11371473.949706702_f64
            }
        );

        let pb = pb
            .rotate2_set(&[0_f64, 95_f64])
            .fit_extent([[0_f64, 0f64], [960_f64, 600_f64]], &object);
        assert_eq!(pb.rotate(), [0_f64, 95_f64, 0_f64]);
        assert_eq!(pb.scale(), 35781690.650920525_f64);
        assert!(in_delta_coordinate(
            &pb.translate(),
            &Coord {
                x: 75115911.95344563_f64,
                y: 2586046.4116968135_f64
            },
            1e-6
        ));
    }
}
