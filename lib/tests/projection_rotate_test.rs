// #[cfg(not(tarpaulin_include))]
// #[cfg(test)]

// mod projection_rotate_test {

// 	use geo::Coordinate;
// 	use geo::Geometry;
// 	use geo::LineString;
// 	use geo::Polygon;
// 	use pretty_assertions::assert_eq;

// 	use rust_d3_geo::path::builder::Builder as PathBuilder;
// 	use rust_d3_geo::projection::mercator::Mercator;
// 	use rust_d3_geo::projection::Build;
// 	use rust_d3_geo::projection::ProjectionRawBase;
// 	use rust_d3_geo::projection::Rotate;
// 	use rust_d3_geo::projection::Scale;
// 	use rust_d3_geo::projection::Translate;

// 	#[test]
// 	fn degenerate_polygon_should_not_break() {
// 		let projection = Mercator::builder()
// 			.rotate(&[-134.3_f32, 25.776_f32, 0_f32])
// 			.scale(750_f32)
// 			.translate(&Coordinate { x: 0_f32, y: 0_f32 })
// 			.build();

// 		let path_builder = PathBuilder::context_pathstring();

// 		let object = Geometry::Polygon(Polygon::new(
// 			LineString::from(vec![
// 				Coordinate {
// 					x: 125.67351590459046,
// 					y: -14.17673705310531,
// 				},
// 				Coordinate {
// 					x: 125.67351590459046,
// 					y: -14.173276873687367,
// 				},
// 				Coordinate {
// 					x: 125.67351590459046,
// 					y: -14.173276873687367,
// 				},
// 				Coordinate {
// 					x: 125.67351590459046,
// 					y: -14.169816694269425,
// 				},
// 				Coordinate {
// 					x: 125.67351590459046,
// 					y: -14.17673705310531,
// 				},
// 			]),
// 			vec![],
// 		));

// 		let s = path_builder.build(projection).object(&object);
// 		assert_eq!(s, "M-111.644162,-149.157654L-111.647235,-149.203744L-111.647235,-149.203744L-111.650307,-149.249835Z");
// 	}
// }
