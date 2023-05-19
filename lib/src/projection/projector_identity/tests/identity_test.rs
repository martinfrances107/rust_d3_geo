#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod identity {

    use geo::Geometry;
    use geo::LineString;
    use geo_types::Coord;

    use crate::clip::antimeridian::ClipAntimeridianC;
    use crate::identity::Identity;
    use crate::in_delta::in_delta;
    use crate::path::string::String;
    use crate::path_identity::builder::Builder as PathBuilder;
    use crate::projection::builder::template::NoPCNC;
    use crate::projection::builder::template::NoPCNU;
    use crate::projection::builder::template::ResampleNoneNoPCNC;
    use crate::projection::builder::template::PCNC;
    use crate::projection::builder_identity::Builder;
    use crate::projection::equality::projection_equal;
    use crate::projection::ClipExtentSet;
    use crate::projection::ReflectGet;
    use crate::projection::ReflectSet;
    use crate::projection::ScaleGet;
    use crate::projection::ScaleSet;
    use crate::projection::TranslateGet;
    use crate::projection::TranslateSet;
    use crate::projection::REFLECT;
    use crate::stream::Connected;
    use crate::stream::DrainStub;
    use crate::stream::Unconnected;

    #[test]
    fn returns_a_point() {
        let mut ib = Builder::<DrainStub<f64>, Identity<Unconnected>, f64>::default();
        ib.translate_set(&Coord { x: 0_f64, y: 0_f64 })
            .scale_set::<ClipAntimeridianC<
                ResampleNoneNoPCNC<DrainStub<f64>, Identity<Connected<DrainStub<f64>>>, f64>,
                f64,
            >>(1_f64);
        let identity = ib.build::<PCNC<DrainStub<f64>, f64>>();
        assert!(projection_equal(
            &identity,
            &(0f64, 0f64).into(),
            &(0f64, 0f64).into(),
            None
        ));
        assert!(projection_equal(
            &identity,
            &(-180f64, 0f64).into(),
            &(-180f64, 0f64).into(),
            None
        ));
        assert!(projection_equal(
            &identity,
            &(180f64, 0f64).into(),
            &(180f64, 0f64).into(),
            None
        ));
        assert!(projection_equal(
            &identity,
            &(30f64, 30f64).into(),
            &(30f64, 30f64).into(),
            None
        ));

        // These getter asserts have no direct equivalent in the javascript original.
        // but increase code coverage.
        assert!(in_delta(ib.scale(), 1_f64, f64::EPSILON));
        assert_eq!(ib.translate(), Coord { x: 0_f64, y: 0_f64 });
    }

    #[test]
    fn reflect_return_the_transformed_point() {
        println!("identity(point).reflectX(…) and reflectY() return the transformed point");
        let mut ib: Builder<DrainStub<f64>, _, _> = Builder::default();
        ib.translate_set(&Coord {
            x: 100_f64,
            y: 10_f64,
        })
        .scale_set::<ClipAntimeridianC<
            ResampleNoneNoPCNC<DrainStub<f64>, Identity<Connected<DrainStub<f64>>>, f64>,
            f64,
        >>(2_f64);

        assert!(projection_equal(
            &ib.build::<NoPCNU>(),
            &(3f64, 7f64).into(),
            &(106f64, 24f64).into(),
            None
        ));

        ib.reflect_x_set(REFLECT::Flipped);
        assert!(projection_equal(
            &ib.build::<NoPCNU>(),
            &(3f64, 7f64).into(),
            &(94f64, 24f64).into(),
            None
        ));

        ib.reflect_y_set(REFLECT::Flipped);
        assert!(projection_equal(
            &ib.build::<NoPCNU>(),
            &(3f64, 7f64).into(),
            &(94f64, -4f64).into(),
            None
        ));

        ib.reflect_x_set(REFLECT::Unflipped);
        assert!(projection_equal(
            &ib.build::<NoPCNU>(),
            &(3f64, 7f64).into(),
            &(106f64, -4f64).into(),
            None
        ));

        ib.reflect_y_set(REFLECT::Unflipped);
        assert!(projection_equal(
            &ib.build::<NoPCNU>(),
            &(3f64, 7f64).into(),
            &(106f64, 24f64).into(),
            None
        ));

        // These getter asserts have no direct equivalent in the javascript original.
        // but increase code coverage.
        assert!(!ib.is_x_reflected());
        assert!(!ib.is_y_reflected());
    }

    #[test]
    fn identity_returns_path() {
        print!("geoPath(identity) returns the path");

        let mut pb: Builder<String<f64>, _, _> = Builder::default();
        pb.translate_set(&Coord { x: 0_f64, y: 0_f64 })
            .scale_set::<ClipAntimeridianC<
                ResampleNoneNoPCNC<DrainStub<f64>, Identity<Connected<DrainStub<f64>>>, f64>,
                f64,
            >>(1_f64);

        let projector = pb.build();

        let mut path = PathBuilder::context_pathstring().build(projector);

        let ls = Geometry::LineString(LineString(vec![
            Coord { x: 0_f64, y: 0_f64 },
            Coord {
                x: 10_f64,
                y: 10_f64,
            },
        ]));

        assert_eq!("M0,0L10,10", path.object(&ls));

        let projection_builder2 = pb
            .translate_set(&Coord {
                x: 30_f64,
                y: 90_f64,
            })
            .scale_set::<ClipAntimeridianC<
                ResampleNoneNoPCNC<DrainStub<f64>, Identity<Connected<DrainStub<f64>>>, f64>,
                f64,
            >>(2_f64);
        projection_builder2.reflect_y_set(REFLECT::Flipped);
        let projector2 = projection_builder2.build::<NoPCNC<String<f64>>>();

        let mut path2 = PathBuilder::context_pathstring().build(projector2);

        assert_eq!("M30,90L50,70", path2.object(&ls));
    }

    #[test]
    fn respects_clip_extent() {
        print!("geoPath(identity) respects clipExtent");

        let mut pb: Builder<String<f64>, _, _> = Builder::default();

        let pb = pb.translate_set(&Coord { x: 0_f64, y: 0_f64 });
        let pb = pb.scale_set::<ClipAntimeridianC<
            ResampleNoneNoPCNC<DrainStub<f64>, Identity<Connected<DrainStub<f64>>>, f64>,
            f64,
        >>(1_f64);
        let pb = pb.clip_extent_set(&[
            Coord { x: 5_f64, y: 5_f64 },
            Coord {
                x: 40_f64,
                y: 80_f64,
            },
        ]);

        let projector = pb.build();

        let mut path = PathBuilder::context_pathstring().build(projector);

        let ls = Geometry::LineString(LineString(vec![
            Coord { x: 0_f64, y: 0_f64 },
            Coord {
                x: 10_f64,
                y: 10_f64,
            },
        ]));

        assert_eq!("M5,5L10,10", path.object(&ls));

        let mut pb2: Builder<String<f64>, _, _> = Builder::default();

        pb2.translate_set(&Coord {
            x: 30_f64,
            y: 90_f64,
        })
        .scale_set::<ClipAntimeridianC<
            ResampleNoneNoPCNC<DrainStub<f64>, Identity<Connected<DrainStub<f64>>>, f64>,
            f64,
        >>(2_f64)
        .reflect_y_set(REFLECT::Flipped);
        let pb2 = pb2.clip_extent_set(&[
            Coord {
                x: 35_f64,
                y: 76_f64,
            },
            Coord {
                x: 45_f64,
                y: 86_f64,
            },
        ]);

        let projector2 = pb2.build();

        let mut path2 = PathBuilder::context_pathstring().build(projector2);

        assert_eq!("M35,85L44,76", path2.object(&ls));
    }
}
