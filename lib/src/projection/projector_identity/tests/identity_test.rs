#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod identity_test {

    use geo::Coordinate;
    use geo::Geometry;
    use geo::LineString;

    use crate::path::string::String;
    use crate::path_identity::builder::Builder as PathBuilder;
    use crate::projection::builder::template::NoPCNC;
    use crate::projection::builder::template::NoPCNU;
    use crate::projection::builder::template::PCNC;
    use crate::projection::builder_identity::Builder;
    use crate::projection::projection_equal::projection_equal;
    use crate::projection::projector_identity::Projector;
    use crate::projection::ClipExtentSet;
    use crate::projection::ReflectSet;
    use crate::projection::ScaleSet;
    use crate::projection::TranslateSet;
    use crate::stream::StreamDrainStub;

    #[test]
    fn test_returns_a_point() {
        let identity: Projector<StreamDrainStub<f64>, _, _, _> = Builder::default()
            .translate_set(&Coordinate { x: 0_f64, y: 0_f64 })
            .scale_set(1_f64)
            .build::<PCNC<StreamDrainStub<f64>, f64>>();
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
    }

    #[test]
    fn test_reflect() {
        println!("identity(point).reflectX(…) and reflectY() return the transformed point");
        let mut identity: Builder<StreamDrainStub<f64>, _, _> = Builder::default()
            .translate_set(&Coordinate {
                x: 100_f64,
                y: 10_f64,
            })
            .scale_set(2_f64);

        assert!(projection_equal(
            &identity.clone().build::<NoPCNU<StreamDrainStub<f64>>>(),
            &(3f64, 7f64).into(),
            &(106f64, 24f64).into(),
            None
        ));

        identity = identity.reflect_x_set(true);
        assert!(projection_equal(
            &identity.clone().build::<NoPCNU<StreamDrainStub<f64>>>(),
            &(3f64, 7f64).into(),
            &(94f64, 24f64).into(),
            None
        ));

        identity = identity.reflect_y_set(true);
        assert!(projection_equal(
            &identity.clone().build::<NoPCNU<StreamDrainStub<f64>>>(),
            &(3f64, 7f64).into(),
            &(94f64, -4f64).into(),
            None
        ));

        identity = identity.reflect_x_set(false);
        assert!(projection_equal(
            &identity.clone().build::<NoPCNU<StreamDrainStub<f64>>>(),
            &(3f64, 7f64).into(),
            &(106f64, -4f64).into(),
            None
        ));

        identity = identity.reflect_y_set(false);
        assert!(projection_equal(
            &identity.build::<NoPCNU<StreamDrainStub<f64>>>(),
            &(3f64, 7f64).into(),
            &(106f64, 24f64).into(),
            None
        ));
    }

    #[test]
    fn identity_returns_path() {
        print!("geoPath(identity) returns the path");

        let projection_builder: Builder<String<f64>, _, _> = Builder::default()
            .translate_set(&Coordinate { x: 0_f64, y: 0_f64 })
            .scale_set(1_f64);

        let projector = projection_builder.build();

        let mut path = PathBuilder::context_pathstring().build(projector);

        let ls = Geometry::LineString(LineString(vec![
            Coordinate { x: 0_f64, y: 0_f64 },
            Coordinate {
                x: 10_f64,
                y: 10_f64,
            },
        ]));

        assert_eq!("M0,0L10,10", path.object(&ls));

        let projection_buidler2 = projection_builder
            .translate_set(&Coordinate {
                x: 30_f64,
                y: 90_f64,
            })
            .scale_set(2_f64)
            .reflect_y_set(true);
        let projector2 = projection_buidler2.build::<NoPCNC<String<f64>>>();

        let mut path2 = PathBuilder::context_pathstring().build(projector2);

        assert_eq!("M30,90L50,70", path2.object(&ls));
    }

    #[test]
    fn respects_clip_extent() {
        print!("geoPath(identity) respects clipExtent");

        let projection_builder: Builder<String<f64>, _, _> = Builder::default()
            .translate_set(&Coordinate { x: 0_f64, y: 0_f64 })
            .scale_set(1_f64)
            .clip_extent_set(&[
                Coordinate { x: 5_f64, y: 5_f64 },
                Coordinate {
                    x: 40_f64,
                    y: 80_f64,
                },
            ]);

        let projector = projection_builder.build();

        let mut path = PathBuilder::context_pathstring().build(projector);

        let ls = Geometry::LineString(LineString(vec![
            Coordinate { x: 0_f64, y: 0_f64 },
            Coordinate {
                x: 10_f64,
                y: 10_f64,
            },
        ]));

        assert_eq!("M5,5L10,10", path.object(&ls));

        let projection_builder2: Builder<String<f64>, _, _> = Builder::default()
            .translate_set(&Coordinate {
                x: 30_f64,
                y: 90_f64,
            })
            .scale_set(2_f64)
            .reflect_y_set(true)
            .clip_extent_set(&[
                Coordinate {
                    x: 35_f64,
                    y: 76_f64,
                },
                Coordinate {
                    x: 45_f64,
                    y: 86_f64,
                },
            ]);

        let projector2 = projection_builder2.build();

        let mut path2 = PathBuilder::context_pathstring().build(projector2);

        assert_eq!("M35,85L44,76", path2.object(&ls));
    }
}
