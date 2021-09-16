// #[cfg(not(tarpaulin_include))]
// #[cfg(test)]
// mod mercator_tests {
// 	use geo::Coordinate;

// 	use rust_d3_geo::clip::antimeridian::line::Line;
// 	// use rust_d3_geo::clip::circle::line::Line;
// 	use rust_d3_geo::clip::antimeridian::pv::PV;
// 	use rust_d3_geo::data_object::sphere::Sphere;
// 	use rust_d3_geo::data_object::DataObject;
// 	use rust_d3_geo::path::builder::Builder as PathBuilder;
// 	use rust_d3_geo::path::context_stream::ContextStream;
// 	use rust_d3_geo::path::ResultEnum;
// 	use rust_d3_geo::projection::mercator::Mercator;
// 	use rust_d3_geo::projection::projection::Projection;
// 	use rust_d3_geo::projection::ClipExtent;
// 	use rust_d3_geo::projection::Precision;
// 	use rust_d3_geo::projection::Raw;
// 	use rust_d3_geo::projection::Scale;
// 	use rust_d3_geo::projection::Translate;

// 	#[test]
// 	fn test_clip_extent_defaults_to_automatic() {
// 		println!("mercator.clipExtent(null) sets the default automatic clip extent");
// 		let projection: Projection<
// 			ContextStream<f64>,
// 			Line<f64>,
// 			Mercator<ContextStream<f64>, f64>,
// 			PV<f64>,
// 			f64,
// 		> = Mercator::builder()
// 			.translate(&Coordinate { x: 0_f64, y: 0_f64 })
// 			.scale(1_f64)
// 			.clip_extent(None)
// 			.precision(&0_f64)
// 			.build();
// 		// panic!("stop here");
// 		let path_builder: PathBuilder<Line<f64>, Mercator<ContextStream<f64>, f64>, PV<f64>, f64> =
// 			PathBuilder::context_pathstring();

// 		let object = DataObject::Sphere(Sphere::default());

// 		match path_builder.build(projection).object(object) {
// 			Some(r) => match r {
// 				ResultEnum::String(s) => {
// 					assert_eq!(s, "M3.141593,-3.141593L3.141593,0L3.141593,3.141593L3.141593,3.141593L-3.141593,3.141593L-3.141593,3.141593L-3.141593,0L-3.141593,-3.141593L-3.141593,-3.141593L3.141593,-3.141593Z");
// 				}
// 				_ => todo!("must handle "),
// 			},
// 			None => panic!("Expecting an string."),
// 		}
// 	}

// 	// #[test]
// 	// fn test_reflect_x_defaults_to_false() {
// 	// 	println!("mercator.rotate(…) does not affect the automatic clip extent");

// 	// 	DataObject::
// 	// }
// }
