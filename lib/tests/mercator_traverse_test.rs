#[cfg(not(tarpaulin_include))]
mod mercator_tranverse_tests {

    extern crate pretty_assertions;

    use geo::Coordinate;
    use geo::Geometry;
    use pretty_assertions::assert_eq;

    use rust_d3_geo::data_object::sphere::Sphere;
    use rust_d3_geo::in_delta::in_delta_coordinate;
    use rust_d3_geo::path::builder::Builder as PathBuilder;
    use rust_d3_geo::path::string::String;
    use rust_d3_geo::projection::builder_mercator::ScaleReclip;
    use rust_d3_geo::projection::mercator_transverse::MercatorTransverse;
    use rust_d3_geo::projection::CenterGet;
    use rust_d3_geo::projection::CenterSet;
    use rust_d3_geo::projection::ClipExtentClear;
    use rust_d3_geo::projection::PrecisionBypass;
    use rust_d3_geo::projection::ProjectionRawBase;
    use rust_d3_geo::projection::ScaleSet;
    use rust_d3_geo::projection::TranslateSet;

    // it("transverseMercator.clipExtent(null) sets the default automatic clip extent", () => {
    //   const projection = geoTransverseMercator().translate([0, 0]).scale(1).clipExtent(null).precision(0);
    //   assertPathEqual(geoPath(projection)({type: "Sphere"}), "M3.141593,3.141593L0,3.141593L-3.141593,3.141593L-3.141593,-3.141593L-3.141593,-3.141593L0,-3.141593L3.141593,-3.141593L3.141593,3.141593Z");
    //   assert.strictEqual(projection.clipExtent(), null);
    // });

    #[test]
    fn clip_extent_defaults_to_automatic() {
        println!("transverseMercator.clipExtent(null) sets the default automatic clip extent");
        // let pb = MercatorTransverse::builder();
        // .translate_adjust(&Coordinate { x: 0_f32, y: 0_f32 })
        // pb.scale_reclip(1_f32);

        // let pb = pb.clip_extent_clear();
        // let pb = pb.precision_bypass();

        // let projection = pb.build();
        // let path_builder = PathBuilder::context_pathstring();

        let object = Sphere::<f32>::default();

        // let s = path_builder.build(projection).object(&object);
        // assert_eq!(s, "M3.141593,3.141593L0,3.141593L-3.141593,3.141593L-3.141593,-3.141593L-3.141593,-3.141593L0,-3.141593L3.141593,-3.141593L3.141593,3.141593Z");
    }

    // it("transverseMercator.center(center) sets the correct automatic clip extent", () => {
    //   const projection = geoTransverseMercator().translate([0, 0]).scale(1).center([10, 10]).precision(0);
    //   assertPathEqual(geoPath(projection)({type: "Sphere"}), "M2.966167,3.316126L-0.175426,3.316126L-3.317018,3.316126L-3.317019,-2.967060L-3.317019,-2.967060L-0.175426,-2.967060L2.966167,-2.967060L2.966167,3.316126Z");
    //   assert.strictEqual(projection.clipExtent(), null);
    // });

    #[test]
    fn center_set_the_automatic_clip_extent() {
        println!("transverseMercator.center(center) sets the correct automatic clip extent");
        // let pb = MercatorTransverse::<String<f32>, f32>::builder();
        // pb.translate_set(&Coordinate { x: 0_f32, y: 0_f32 });

        // let pb = pb.scale_set(1_f32);
        // let pb = pb.center_set(&Coordinate {
        //     x: 10_f32,
        //     y: 10_f32,
        // });
        // pb.precision_bypass();

        // let projection = pb.build();
        // let path_builder = PathBuilder::context_pathstring();

        // let object = Sphere::<f32>::default();

        // let s = path_builder.build(projection).object(&object);
        // assert_eq!(s, "M2.966167,3.316126L-0.175426,3.316126L-3.317018,3.316126L-3.317019,-2.967060L-3.317019,-2.967060L-0.175426,-2.967060L2.966167,-2.967060L2.966167,3.316126Z");
    }

    // it("transverseMercator.clipExtent(extent) intersects the specified clip extent with the automatic clip extent", () => {
    //   const projection = geoTransverseMercator().translate([0, 0]).scale(1).clipExtent([[-10, -10], [10, 10]]).precision(0);
    //   assertPathEqual(geoPath(projection)({type: "Sphere"}), "M10,3.141593L0,3.141593L-10,3.141593L-10,-3.141593L-10,-3.141593L0,-3.141593L10,-3.141593L10,3.141593Z");
    //   assert.deepStrictEqual(projection.clipExtent(), [[-10, -10], [10, 10]]);
    // });

    #[test]
    fn clip_extent_intersects() {
        println!("transverseMercator.clipExtent(extent) intersects the specified clip extent with the automatic clip extent");
        // let pb = MercatorTransverse::builder()
        //     .translate_set(&Coordinate { x: 0_f32, y: 0_f32 })
        //     .scale_set(1_f32)
        //     .clip_extent(&[
        //         Coordinate {
        //             x: -10_f32,
        //             y: -10_f32,
        //         },
        //         Coordinate {
        //             x: 10_f32,
        //             y: 10_f32,
        //         },
        //     ])
        //     .precision_bypass();

        // let projection = pb.build();
        // let path_builder = PathBuilder::context_pathstring();

        // let object = Sphere::<f32>::default();

        // let s = path_builder.build(projection).object(&object);
        // assert_eq!(s, "M10,3.141593L0,3.141593L-10,3.141593L-10,-3.141593L-10,-3.141593L0,-3.141593L10,-3.141593L10,3.141593Z");
    }

    // it("transverseMercator.clipExtent(extent).scale(scale) updates the intersected clip extent", () => {
    //   const projection = geoTransverseMercator().translate([0, 0]).clipExtent([[-10, -10], [10, 10]]).scale(1).precision(0);
    //   assertPathEqual(geoPath(projection)({type: "Sphere"}), "M10,3.141593L0,3.141593L-10,3.141593L-10,-3.141593L-10,-3.141593L0,-3.141593L10,-3.141593L10,3.141593Z");
    //   assert.deepStrictEqual(projection.clipExtent(), [[-10, -10], [10, 10]]);
    // });

    // it("transverseMercator.clipExtent(extent).translate(translate) updates the intersected clip extent", () => {
    //   const projection = geoTransverseMercator().scale(1).clipExtent([[-10, -10], [10, 10]]).translate([0, 0]).precision(0);
    //   assertPathEqual(geoPath(projection)({type: "Sphere"}), "M10,3.141593L0,3.141593L-10,3.141593L-10,-3.141593L-10,-3.141593L0,-3.141593L10,-3.141593L10,3.141593Z");
    //   assert.deepStrictEqual(projection.clipExtent(), [[-10, -10], [10, 10]]);
    // });

    // it("transverseMercator.rotate(…) does not affect the automatic clip extent", () => {
    //   const projection = geoTransverseMercator(), object = {
    //     type: "MultiPoint",
    //     coordinates: [
    //       [-82.35024908550241, 29.649391549778745],
    //       [-82.35014449996858, 29.65075946917633],
    //       [-82.34916073446641, 29.65070265688781],
    //       [-82.3492653331286, 29.64933474064504]
    //     ]
    //   };
    //   projection.fitExtent([[0, 0], [960, 600]], object);
    //   assert.deepStrictEqual(projection.scale(), 15724992.330511674);
    //   assert.deepStrictEqual(projection.translate(), [20418843.897824813, 21088401.790971387]);
    //   projection.rotate([0, 95]).fitExtent([[0, 0], [960, 600]], object);
    //   assert.deepStrictEqual(projection.scale(), 15724992.330511674);
    //   assert.deepStrictEqual(projection.translate(), [20418843.897824813, 47161426.43770847]);
    // });
}
