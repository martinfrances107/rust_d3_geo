// #[cfg(not(tarpaulin_include))]
// #[cfg(test)]

// mod projection_rotate_test {

//     use std::rc::Rc;

//     use geo::polygon;
//     use geo::Coordinate;
//     use geo::Geometry;
//     use geo::LineString;
//     use geo::Polygon;
//     use pretty_assertions::assert_eq;

//     use rust_d3_geo::clip::antimeridian::line::Line;
//     use rust_d3_geo::clip::antimeridian::pv::PV;
//     use rust_d3_geo::data_object::DataObject;
//     use rust_d3_geo::in_delta::in_delta;
//     use rust_d3_geo::path::builder::Builder as PathBuilder;
//     use rust_d3_geo::path::context_stream::ContextStream;
//     use rust_d3_geo::path::ResultEnum;
//     use rust_d3_geo::projection::builder::Builder;
//     use rust_d3_geo::projection::gnomic::Gnomic;
//     use rust_d3_geo::projection::mercator::Mercator;
//     use rust_d3_geo::projection::projection::Projection;
//     use rust_d3_geo::projection::projection_equal::projection_equal;
//     use rust_d3_geo::projection::Angle;
//     use rust_d3_geo::projection::Raw;
//     use rust_d3_geo::projection::Rotate;
//     use rust_d3_geo::projection::Scale;
//     use rust_d3_geo::projection::Translate;
//     use rust_d3_geo::stream::StreamDrainStub;

//     // import {geoMercator, geoPath} from "../../src/index.js";
//     // import {assertPathEqual} from "../asserts.js";

//     // it("a rotation of a degenerate polygon should not break", () => {
//     //   const projection = geoMercator().rotate([-134.300, 25.776]).scale(750).translate([0, 0]);
//     //   assertPathEqual(geoPath(projection)({
//     //     "type": "Polygon",
//     //     "coordinates": [
//     //       [
//     //         [125.67351590459046, -14.17673705310531],
//     //         [125.67351590459046, -14.173276873687367],
//     //         [125.67351590459046, -14.173276873687367],
//     //         [125.67351590459046, -14.169816694269425],
//     //         [125.67351590459046, -14.17673705310531]
//     //       ]
//     //     ]
//     //   }), "M-111.644162,-149.157654L-111.647235,-149.203744L-111.647235,-149.203744L-111.650307,-149.249835Z");
//     // });

//     #[test]
//     fn degenerate_polygon_should_not_break() {
//         //   const projection = geoMercator().rotate([-134.300, 25.776]).scale(750).translate([0, 0]);
//         let projection = Rc::new(
//             Mercator::builder()
//                 .rotate(&[-134.30_f64, 25.776_f64, 0_f64])
//                 .scale(750_f64)
//                 .translate(&Coordinate { x: 0_f64, y: 0_f64 })
//                 .build(),
//         );

//         let path_builder = PathBuilder::context_pathstring();

//         let object = DataObject::Geometry(Geometry::Polygon(Polygon::new(
//             LineString::from(vec![
//                 Coordinate {
//                     x: 125.67351590459046,
//                     y: -14.17673705310531,
//                 },
//                 Coordinate {
//                     x: 125.67351590459046,
//                     y: -14.173276873687367,
//                 },
//                 Coordinate {
//                     x: 125.67351590459046,
//                     y: -14.173276873687367,
//                 },
//                 Coordinate {
//                     x: 125.67351590459046,
//                     y: -14.169816694269425,
//                 },
//                 Coordinate {
//                     x: 125.67351590459046,
//                     y: -14.17673705310531,
//                 },
//             ]),
//             vec![],
//         )));

//         match path_builder.build(projection).object(&object) {
//             Some(r) => match r {
//                 ResultEnum::String(s) => {
//                     assert_eq!(s, "M-111.644162,-149.157654L-111.647235,-149.203744L-111.647235,-149.203744L-111.650307,-149.249835Z");
//                 }
//                 _ => assert!(false, "Incorrect Result"),
//             },
//             None => assert!(false, "Expecting an string."),
//         }
//     }
// }
