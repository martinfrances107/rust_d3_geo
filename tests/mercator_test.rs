#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod mercator_tests {
	use geo::Coordinate;
	use geo::Geometry;

	use rust_d3_geo::clip::antimeridian::line::Line;
	// use rust_d3_geo::clip::circle::line::Line;
	use rust_d3_geo::clip::antimeridian::pv::PV;
	use rust_d3_geo::data_object::sphere::Sphere;
	use rust_d3_geo::data_object::DataObject;
	use rust_d3_geo::path::builder::Builder as PathBuilder;
	use rust_d3_geo::path::context_stream::ContextStream;
	use rust_d3_geo::path::ResultEnum;
	use rust_d3_geo::projection::mercator::Mercator;
	use rust_d3_geo::projection::mercator_builder::MercatorBuilder;
	use rust_d3_geo::projection::projection::Projection;
	use rust_d3_geo::projection::ClipExtent;
	use rust_d3_geo::projection::Fit;
	use rust_d3_geo::projection::Precision;
	use rust_d3_geo::projection::Raw;
	use rust_d3_geo::projection::Rotate;
	use rust_d3_geo::projection::Scale;
	use rust_d3_geo::projection::Translate;

	// #[test]
	// fn test_clip_extent_defaults_to_automatic() {
	// 	println!("mercator.clipExtent(null) sets the default automatic clip extent");
	// 	let projection: Projection<
	// 		ContextStream<f64>,
	// 		Line<f64>,
	// 		Mercator<ContextStream<f64>, f64>,
	// 		PV<f64>,
	// 		f64,
	// 	> = Mercator::builder()
	// 		.translate(&Coordinate { x: 0_f64, y: 0_f64 })
	// 		.scale(1_f64)
	// 		.clip_extent(None)
	// 		.precision(&0_f64)
	// 		.build();
	// 	// panic!("stop here");
	// 	let path_builder: PathBuilder<Line<f64>, Mercator<ContextStream<f64>, f64>, PV<f64>, f64> =
	// 		PathBuilder::context_pathstring();

	// 	let object = DataObject::Sphere(Sphere::default());

	// 	match path_builder.build(projection).object(object) {
	// 		Some(r) => match r {
	// 			ResultEnum::String(s) => {
	// 				assert_eq!(s, "M3.141593,-3.141593L3.141593,0L3.141593,3.141593L3.141593,3.141593L-3.141593,3.141593L-3.141593,3.141593L-3.141593,0L-3.141593,-3.141593L-3.141593,-3.141593L3.141593,-3.141593Z");
	// 			}
	// 			_ => todo!("must handle "),
	// 		},
	// 		None => panic!("Expecting an string."),
	// 	}
	// }

	#[test]
	fn test_rotate_does_not_affect_automatic_clip_extent() {
		println!("geoPath(MultiPoint) renders a point");

		let pb = Mercator::builder()
			.translate(&Coordinate { x: 0_f64, y: 0_f64 })
			.scale(1_f64)
			.clip_extent(None)
			.precision(&0_f64);
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

		// let pb = pb
		// 	.rotate([0_f64, 90_f64, 0_f64])
		// 	.fit_extent([[0_f64, 0_f64], [960_f64, 600_f64]], object);
		// assert_eq!(pb.get_scale(), 35781690.650920525_f64);
		// assert_eq!(
		// 	pb.get_translate(),
		// 	Coordinate {
		// 		x: 75115911.95344563_f64,
		// 		y: 2586046.4116968135_f64
		// 	}
		// );
	}
}
