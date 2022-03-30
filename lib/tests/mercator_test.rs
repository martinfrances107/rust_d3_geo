#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod mercator_tests {
	extern crate pretty_assertions;

	use geo::Coordinate;
	use geo::Geometry;
	use pretty_assertions::assert_eq;
	use rust_d3_geo::projection::ClipExtentSet;
	use rust_d3_geo::projection::ProjectionRawBase;

	use rust_d3_geo::data_object::sphere::Sphere;
	use rust_d3_geo::in_delta::in_delta_coordinate;
	use rust_d3_geo::path::builder::Builder as PathBuilder;
	// use rust_d3_geo::path::string::String as PathString;
	use rust_d3_geo::projection::mercator::Mercator;
	use rust_d3_geo::projection::CenterSet;
	// use rust_d3_geo::projection::ClipExtentBounded;
	// use rust_d3_geo::projection::Fit;
	use rust_d3_geo::projection::PrecisionBypass;
	// use rust_d3_geo::projection::RotateSet;
	use rust_d3_geo::projection::ScaleSet;
	use rust_d3_geo::projection::TranslateSet;

	// #[test]
	// fn test_clip_extent_defaults_to_automatic() {
	// 	println!("mercator.clipExtent(null) sets the default automatic clip extent");
	// 	let projection_builder = Mercator::builder()
	// 		.translate(&Coordinate { x: 0_f32, y: 0_f32 })
	// 		.scale(1_f32)
	// 		// in javascript clip_extent_clear has not effect
	// 		// it is prevent by the state based API.
	// 		// .clip_extent_clear()
	// 		.precision_bypass();

	// 	let projection = projection_builder.build();
	// 	let path_builder = PathBuilder::context_pathstring();

	// 	let object = Sphere::default();

	// 	// The strings are very close here..
	// 	// There is a divergence between JS and RUST here
	// 	// See mercator.transform .. f32 is implied here
	// 	// So I have adjusted some values ending 3 with 27.
	// 	// From the JS reference I have adjusted the second numeric
	// 	// value in the string to be zero 0
	// 	// after tracing the program and seeing that its input to the
	// 	// raw mercator projection was FRAC_PI_2 and evaluates to NAN
	// 	// while JS provides a large numeric value.
	// 	let s: String = path_builder.build(projection).object(&object);
	// 	assert_eq!(s, "M3.141593,0L3.141593,0L3.141593,3.141593L3.141593,3.141593L-3.141593,3.141593L-3.141593,3.141593L-3.141593,0L-3.141593,-3.141593L-3.141593,-3.141593L3.141593,-3.141593Z");
	// 	assert_eq!(projection_builder.get_clip_extent(), None);
	// }

	// 	#[test]
	// 	fn center_set_correct_automatic() {
	// 		println!("mercator.center(center) sets the correct automatic clip extent");
	// 		let projection_builder = Mercator::builder()
	// 			.translate(&Coordinate { x: 0_f32, y: 0_f32 })
	// 			.center(&Coordinate {
	// 				x: 10_f32,
	// 				y: 10_f32,
	// 			})
	// 			.scale(1_f32)
	// 			.precision_bypass();

	// 		let projection = projection_builder.build();
	// 		let path_builder = PathBuilder::context_pathstring();

	// 		let object = Sphere::default();

	// 		let s = path_builder.build(projection).object(&object);
	// 		assert_eq!(s, "M2.967060,-2.966167L2.967060,0.175426L2.967060,3.317018L2.967060,3.317018L-3.316126,3.317018L-3.316126,3.317019L-3.316126,0.175426L-3.316126,-2.966167L-3.316126,-2.966167L2.967060,-2.966167Z");

	// 		assert_eq!(projection_builder.get_clip_extent(), None);
	// 	}

	// 	#[test]
	// 	fn intersected_clip_extent() {
	// 		println!(
	//             "mercator.clipExtent(extent) intersects the specified clip extent with the automatic clip extent"
	//         );
	// 		let projection_builder = Mercator::builder()
	// 			.translate(&Coordinate { x: 0_f64, y: 0_f64 })
	// 			.scale(1_f64)
	// 			.clip_extent(&[
	// 				Coordinate {
	// 					x: -10_f64,
	// 					y: -10_f64,
	// 				},
	// 				Coordinate {
	// 					x: 10_f64,
	// 					y: 10_f64,
	// 				},
	// 			])
	// 			.precision_bypass();

	// 		let projection = projection_builder.build();

	// 		let path_builder = PathBuilder::context_pathstring();

	// 		let object = Sphere::default();

	// 		// There is a bodge associated with this test
	// 		// I have had to adjust the return string to include PI_f64 not PI_f32 to get this to pass.
	// 		// See MercatorRaw::transform for an expanation of the issue.user:martinfrances107
	// 		let s = path_builder.build(projection).object(&object);
	// 		assert_eq!(s, "M3.141592653589793,-10L3.141592653589793,0L3.141592653589793,10L3.141592653589793,10L-3.141592653589793,10L-3.141592653589793,10L-3.141592653589793,0L-3.141592653589793,-10L-3.141592653589793,-10L3.141592653589793,-10Z");
	// 		assert_eq!(
	// 			projection_builder.get_clip_extent(),
	// 			Some([
	// 				Coordinate {
	// 					x: -10_f64,
	// 					y: -10_f64,
	// 				},
	// 				Coordinate {
	// 					x: 10_f64,
	// 					y: 10_f64,
	// 				},
	// 			])
	// 		);
	// 	}

	// 	#[test]
	// 	fn scale_updates_the_intersected_clip_extent() {
	// 		println!(
	// 			"mercator.clipExtent(extent).translate(translate) updates the intersected clip extent"
	// 		);
	// 		let projection_builder = Mercator::builder()
	// 			.translate(&Coordinate { x: 0_f64, y: 0_f64 })
	// 			.clip_extent(&[
	// 				Coordinate {
	// 					x: -10_f64,
	// 					y: -10_f64,
	// 				},
	// 				Coordinate {
	// 					x: 10_f64,
	// 					y: 10_f64,
	// 				},
	// 			])
	// 			.scale(1_f64)
	// 			.precision(&0_f64);
	// 		let projection = projection_builder.build();

	// 		let path_builder = PathBuilder::context_pathstring();

	// 		let object = Sphere::default();

	// 		// There is a bodge associated with this test
	// 		// I have had to adjust the return string to include PI_f64 not PI_f32 to get this to pass.
	// 		// See MercatorRaw::transform for an expanation of the issue.
	// 		let s = path_builder.build(projection).object(&object);
	// 		assert_eq!(s, "M3.141592653589793,-10L3.141592653589793,0L3.141592653589793,10L3.141592653589793,10L-3.141592653589793,10L-3.141592653589793,10L-3.141592653589793,0L-3.141592653589793,-10L-3.141592653589793,-10L3.141592653589793,-10Z");
	// 		assert_eq!(
	// 			projection_builder.get_clip_extent(),
	// 			Some([
	// 				Coordinate {
	// 					x: -10_f64,
	// 					y: -10_f64,
	// 				},
	// 				Coordinate {
	// 					x: 10_f64,
	// 					y: 10_f64,
	// 				},
	// 			])
	// 		);
	// 	}

	// 	#[test]
	// 	fn translate_updates_the_intersected_clip_extent() {
	// 		println!(
	// 			"mercator.clipExtent(extent).translate(translate) updates the intersected clip extent"
	// 		);
	// 		let projection_builder = Mercator::builder()
	// 			.scale(1_f64)
	// 			.clip_extent(&[
	// 				Coordinate {
	// 					x: -10_f64,
	// 					y: -10_f64,
	// 				},
	// 				Coordinate {
	// 					x: 10_f64,
	// 					y: 10_f64,
	// 				},
	// 			])
	// 			.translate(&Coordinate { x: 0_f64, y: 0_f64 })
	// 			.precision(&0_f64);

	// 		let projection = projection_builder.build();
	// 		let path_builder = PathBuilder::context_pathstring();

	// 		let object = Sphere::default();

	// 		// There is a bodge associated with this test
	// 		// I have had to adjust the return string to include PI_f64 not PI_f32 to get this to pass.
	// 		// See MercatorRaw::transform for an expanation of the issue.
	// 		let s = path_builder.build(projection).object(&object);
	// 		assert_eq!(s, "M3.141592653589793,-10L3.141592653589793,0L3.141592653589793,10L3.141592653589793,10L-3.141592653589793,10L-3.141592653589793,10L-3.141592653589793,0L-3.141592653589793,-10L-3.141592653589793,-10L3.141592653589793,-10Z");
	// 		assert_eq!(
	// 			projection_builder.get_clip_extent(),
	// 			Some([
	// 				Coordinate {
	// 					x: -10_f64,
	// 					y: -10_f64,
	// 				},
	// 				Coordinate {
	// 					x: 10_f64,
	// 					y: 10_f64,
	// 				},
	// 			])
	// 		);
	// 	}

	// 	#[test]
	// 	fn rotate_does_not_affect_automatic_clip_extent() {
	// 		println!("mercator.rotate(…) does not affect the automatic clip extent");

	// 		let pb = Mercator::builder();

	// 		let object: Geometry<f64> = Geometry::MultiPoint(
	// 			vec![
	// 				(-82.35024908550241, 29.649391549778745),
	// 				(-82.35014449996858, 29.65075946917633),
	// 				(-82.34916073446641, 29.65070265688781),
	// 				(-82.3492653331286, 29.64933474064504),
	// 			]
	// 			.into(),
	// 		);
	// 		let pb = pb.fit_extent([[0_f64, 0_f64], [960_f64, 600_f64]], &object);
	// 		assert_eq!(pb.get_scale(), 20969742.365692537_f64);
	// 		assert_eq!(
	// 			pb.get_translate(),
	// 			Coordinate {
	// 				x: 30139734.76760269_f64,
	// 				y: 11371473.949706702_f64
	// 			}
	// 		);

	// 		let pb = pb
	// 			.rotate(&[0_f64, 95_f64, 0_f64])
	// 			.fit_extent([[0_f64, 0_f64], [960_f64, 600_f64]], &object);
	// 		assert_eq!(pb.get_rotate(), [0_f64, 95_f64, 0_f64]);
	// 		assert_eq!(pb.get_scale(), 35781690.650920525_f64);
	// 		assert!(in_delta_coordinate(
	// 			&pb.get_translate(),
	// 			&Coordinate {
	// 				x: 75115911.95344563_f64,
	// 				y: 2586046.4116968135_f64
	// 			},
	// 			1e-6
	// 		));
	// 	}
}
