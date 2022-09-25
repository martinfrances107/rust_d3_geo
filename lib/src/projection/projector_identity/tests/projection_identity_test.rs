#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod identity_test {
    use geo::Coordinate;
    use geo::Geometry;
    use geo::LineString;

    use crate::path::builder::Builder as PathBuilder;
    use crate::path::context::Context;
    use crate::path_test_context::CanvasRenderingContext2d;
    use crate::projection::builder::template::ClipU;
    use crate::projection::builder::template::NoClipC;
    use crate::projection::builder::template::NoClipU;
    use crate::projection::builder_identity::Builder;
    use crate::projection::projection_equal::projection_equal;
    use crate::projection::projector_identity::Projector;
    use crate::projection::Build;
    use crate::projection::ClipExtentSet;
    use crate::projection::ReflectSet;
    use crate::projection::ScaleSet;
    use crate::projection::TranslateSet;
    use crate::stream::StreamDrainStub;

    // it("identity(point) returns the point", () => {
    //   const identity = geoIdentity().translate([0, 0]).scale(1);
    //   assertProjectionEqual(identity, [   0,   0], [   0,   0]);
    //   assertProjectionEqual(identity, [-180,   0], [-180,   0]);
    //   assertProjectionEqual(identity, [ 180,   0], [ 180,   0]);
    //   assertProjectionEqual(identity, [  30,  30], [  30,  30]);
    // });

    #[test]
    fn test_returns_a_point() {
        let identity: Projector<StreamDrainStub<f64>, _, _, _> = Builder::default()
            .translate_set(&Coordinate {
                x: 100_f64,
                y: 10_f64,
            })
            .scale_set(2_f64)
            .build::<NoClipC<StreamDrainStub<f64>>>();
        assert!(projection_equal(
            &identity,
            &(0f64, 0f64).into(),
            &(100f64, 10f64).into(),
            None
        ));
        assert!(projection_equal(
            &identity,
            &(-180f64, 0f64).into(),
            &(-260f64, 10f64).into(),
            None
        ));
        assert!(projection_equal(
            &identity,
            &(180f64, 0f64).into(),
            &(460f64, 10f64).into(),
            None
        ));
        assert!(projection_equal(
            &identity,
            &(30f64, 30f64).into(),
            &(160f64, 70f64).into(),
            None
        ));
    }

    // it("identity(point).reflectX(…) and reflectY() return the transformed point", () => {
    //   const identity = geoIdentity().translate([100, 10]).scale(2)
    //     .reflectX(false).reflectY(false);
    //   assertProjectionEqual(identity, [   3,   7], [ 106,  24]);
    //   assertProjectionEqual(identity.reflectX(true), [   3,   7], [ 94,  24]);
    //   assertProjectionEqual(identity.reflectY(true), [   3,   7], [ 94,  -4]);
    //   assertProjectionEqual(identity.reflectX(false), [   3,   7], [ 106,  -4]);
    //   assertProjectionEqual(identity.reflectY(false), [   3,   7], [ 106,  24]);
    // });
    #[test]
    fn test_reflect() {
        println!("identity(point).reflectX(…) and reflectY() return the transformed point");
        let identity: Builder<StreamDrainStub<f64>, _, _> = Builder::default()
            .translate_set(&Coordinate {
                x: 100_f64,
                y: 10_f64,
            })
            .scale_set(2_f64);
        assert!(projection_equal(
            &identity
                .clone()
                .reflect_x_set(true)
                .build::<NoClipU<StreamDrainStub<f64>>>(),
            &(3f64, 7f64).into(),
            &(94f64, 24f64).into(),
            None
        ));
        assert!(projection_equal(
            &identity
                .clone()
                .reflect_y_set(true)
                .build::<NoClipU<StreamDrainStub<f64>>>(),
            &(3f64, 7f64).into(),
            &(94f64, -4f64).into(),
            None
        ));
        assert!(projection_equal(
            &identity
                .clone()
                .reflect_x_set(false)
                .build::<NoClipU<StreamDrainStub<f64>>>(),
            &(3f64, 7f64).into(),
            &(106f64, -4f64).into(),
            None
        ));
        assert!(projection_equal(
            &identity
                .reflect_y_set(false)
                .build::<NoClipU<StreamDrainStub<f64>>>(),
            &(3f64, 7f64).into(),
            &(106f64, 24f64).into(),
            None
        ));
    }

    // it("geoPath(identity) returns the path", () => {
    //   const identity = geoIdentity().translate([0, 0]).scale(1),
    //     path = geoPath().projection(identity);
    //   assert.strictEqual(path({type:"LineString", coordinates: [[0,0], [10,10]]}), "M0,0L10,10");
    //   identity.translate([30,90]).scale(2).reflectY(true);
    //   assert.strictEqual(path({type:"LineString", coordinates: [[0,0], [10,10]]}), "M30,90L50,70");
    // });
    #[test]
    fn identity_returns_path() {
        print!("geoPath(identity) returns the path");

        let identity: Builder<Context, ClipU<Context, f64>, f64> = Builder::default()
            .translate_set(&Coordinate {
                x: 100_f64,
                y: 10_f64,
            })
            .scale_set(2_f64)
            .clip_extent_set(&[(5_f64, 5_f64).into(), (40_f64, 80_f64).into()]);
        let crc2d = CanvasRenderingContext2d::default();

        let context = Context::new(crc2d);
        // let pb = PathBuilder::new(context);

        // let mut path = pb.build(equirectangular());

        // let path = PathBuilder::new(context)
        //     .build(identity)
        //     .object(&Geometry::LineString(LineString::new(vec![
        //         (0f64, 0f64).into(),
        //         (10f64, 10f64).into(),
        //     ])));

        // assert_eq!(path.object());

        // let identities = identity.clip_extent_set(&[(0, 0).into(), (0.0).into()]);
    }

    // it("geoPath(identity) respects clipExtent", () => {
    //   const identity = geoIdentity().translate([0, 0]).scale(1),
    //     path = geoPath().projection(identity);
    //   identity.clipExtent([[5,5], [40, 80]]);
    //   assert.strictEqual(path({type:"LineString", coordinates: [[0,0], [10,10]]}), "M5,5L10,10");
    //   identity.translate([30,90]).scale(2).reflectY(true).clipExtent([[35,76], [45, 86]]);
    //   assert.strictEqual(path({type:"LineString", coordinates: [[0,0], [10,10]]}), "M35,85L44,76");
    // });

    #[test]
    fn respects_clip_extent() {
        print!("geoPath(identity) respects clipExtent");
        // let identity: Builder<
        //     StreamDrainStub<f64>,
        //     NoClipC<StreamDrainStub<f64>>,
        //     NoClipU<StreamDrainStub<f64>>,
        // > = Builder::default()
        //     .translate_set(&Coordinate {
        //         x: 100_f64,
        //         y: 10_f64,
        //     })
        //     .scale_set(2_f64)
        //     .clip_extent_set(&[(5_f64, 5_f64).into(), (40_f64, 80_f64).into()]);

        // let path = PathBuilder::context_pathstring()
        //     .build(identity)
        //     .object(&Geometry::LineString(LineString::new(vec![/* Coords*/])));

        // assert_eq!(path.object());

        // let identities = identity.clip_extent_set(&[(0, 0).into(), (0.0).into()]);
    }
}
