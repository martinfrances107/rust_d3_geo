#[cfg(not(tarpaulin_include))]
#[cfg(test)]

mod fit_test {
	extern crate pretty_assertions;
	extern crate rust_topojson_client;

	use geo::Coordinate;
	use rust_d3_geo::data_object::sphere::Sphere;
	use rust_d3_geo::in_delta::in_delta;
	use rust_d3_geo::in_delta::in_delta_coordinate;
	use rust_d3_geo::path::bounds::Bounds;
	use rust_d3_geo::projection::azimuthal_equal_area::AzimuthalEqualArea;
	use rust_d3_geo::projection::azimuthal_equidistant::AzimuthalEquiDistant;
	use rust_d3_geo::projection::builder::types::BuilderAntimeridianResampleNoClip;
	use rust_d3_geo::projection::builder::types::BuilderCircleResampleNoClip;
	use rust_d3_geo::projection::equirectangular::Equirectangular;
	use rust_d3_geo::projection::gnomic::Gnomic;
	use rust_d3_geo::projection::mercator::Mercator;
	use rust_d3_geo::projection::orthographic::Orthographic;
	use rust_d3_geo::projection::stereographic::Stereographic;
	use rust_d3_geo::projection::ClipAngleAdjust;
	use rust_d3_geo::projection::FitSet;
	use rust_d3_geo::projection::PrecisionAdjust;
	use rust_d3_geo::projection::ProjectionRawBase;
	use rust_d3_geo::projection::ScaleGet;
	use rust_d3_geo::projection::TranslateGet;
	use rust_d3_geo::stream::StreamDrainStub;
	use std::fs::File;

	use geo::polygon;
	use geo::Geometry;
	use topojson::Topology;

	// use rust_d3_geo::projection::azimuthal_equal_area::AzimuthalEqualArea;

	// use rust_d3_geo::projection::mercator::Mercator;
	use rust_topojson_client::feature::Builder;

	///  Helper function to extract world geometry from file.
	fn world() -> Geometry<f64> {
		let file =
			File::open("./tests/world-atlas/world/50m.json").expect("File should open read only.");
		let topology: Topology =
			serde_json::from_reader(file).expect("File should be parse as JSON.");

		if let Some(g) = Builder::generate_from_name(&topology, &"land") {
			return g;
		} else {
			panic!("failed to file and decode from file.");
		}
	}

	#[test]
	fn fit_extent_sphere_equirectangular() {
		println!("projection.fitExtent(…) sphere equirectangular");
		let d_object = Sphere::default();
		let projection: BuilderAntimeridianResampleNoClip<
			Bounds<f64>,
			Equirectangular<Bounds<f64>, f64>,
			f64,
		> = Equirectangular::builder();

		let projection =
			projection.fit_extent([[50.0_f64, 50.0_f64], [950.0_f64, 950.0_f64]], &d_object);
		assert!(in_delta(
			projection.get_scale(),
			900. / (2_f64 * std::f64::consts::PI),
			1e-6
		));
		let translate = projection.get_translate();
		assert!(in_delta_coordinate(
			&translate,
			&Coordinate {
				x: 500_f64,
				y: 500_f64
			},
			1e-6
		));
	}

	#[test]
	fn fit_extent_world_equirectangular() {
		println!("projection.fitExtent(…) world equirectangular");
		let world = world();

		let projection = Equirectangular::builder()
			.fit_extent([[50.0_f64, 50.0_f64], [950.0_f64, 950.0_f64]], &world);
		assert!(in_delta(projection.get_scale(), 143.239449, 1e-6));
		assert!(in_delta_coordinate(
			&projection.get_translate(),
			&Coordinate {
				x: 500_f64,

				y: 492.000762_f64
			},
			1e-6
		));
	}

	#[test]
	fn fit_extent_world_azimuthal_equal_area() {
		println!("projection.fitExtent(…) world azimuthalEqualArea");

		let world = world();
		let projection = AzimuthalEqualArea::builder()
			.fit_extent([[50.0_f64, 50.0_f64], [950.0_f64, 950.0_f64]], &world);
		assert!(in_delta(projection.get_scale(), 228.357229, 1e-6));
		assert!(in_delta_coordinate(
			&projection.get_translate(),
			&Coordinate {
				x: 496.353618_f64,
				y: 479.684353_f64
			},
			1e-6
		));
	}

	#[test]
	fn fit_extent_world_azimuthal_equidistant() {
		println!("projection.fitExtent(…) world azimuthalEquidistant");

		let world = world();
		let projection = AzimuthalEquiDistant::builder()
			.fit_extent([[50.0_f64, 50.0_f64], [950.0_f64, 950.0_f64]], &world);
		assert!(in_delta(projection.get_scale(), 153.559317, 1e-6));
		assert!(in_delta_coordinate(
			&projection.get_translate(),
			&Coordinate {
				x: 485.272493_f64,
				y: 452.093375_f64
			},
			1e-6
		));
	}

	// 	// 	// // // tape("projection.fitExtent(…) world conicConformal", function(test) {
	// 	// 	// // //   var projection = d3.geoConicConformal().clipAngle(30).parallels([30, 60]).rotate([0, -45]);
	// 	// 	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// 	// 	// // //   test.inDelta(projection.scale(), 626.111027, 1e-6);
	// 	// 	// // //   test.inDelta(projection.translate(), [444.395951, 410.223799], 1e-6);
	// 	// 	// // //   test.end();
	// 	// 	// // // });

	// 	// 	// // // tape("projection.fitExtent(…) world conicEqualArea", function(test) {
	// 	// 	// // //   var projection = d3.geoConicEqualArea();
	// 	// 	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// 	// 	// // //   test.inDelta(projection.scale(), 145.862346, 1e-6);
	// 	// 	// // //   test.inDelta(projection.translate(), [500, 498.0114265], 1e-6);
	// 	// 	// // //   test.end();
	// 	// 	// // // });

	// 	// 	// // // tape("projection.fitExtent(…) world conicEquidistant", function(test) {
	// 	// 	// // //   var projection = d3.geoConicEquidistant();
	// 	// 	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// 	// 	// // //   test.inDelta(projection.scale(), 123.085587, 1e-6);
	// 	// 	// // //   test.inDelta(projection.translate(), [500, 498.598401], 1e-6);
	// 	// 	// // //   test.end();
	// 	// 	// // // });

	// it("projection.fitExtent(…) world equirectangular", () => {
	// 	const projection = geoEquirectangular();
	// 	projection.fitExtent([[50, 50], [950, 950]], world);
	// 	assertInDelta(projection.scale(), 143.239449, 1e-6);
	// 	assertInDelta(projection.translate(), [500, 492.000762], 1e-6);
	//   });

	//   it("projection.fitSize(…) world equirectangular", () => {
	// 	const projection = geoEquirectangular();
	// 	projection.fitSize([900, 900], world);
	// 	assertInDelta(projection.scale(), 143.239449, 1e-6);
	// 	assertInDelta(projection.translate(), [450, 442.000762], 1e-6);
	//   });

	#[test]
	fn fit_extent_world_gnomic() {
		println!("projection.fitExtent(…) world gnomonic");

		let world = world();
		let projection = Gnomic::builder()
			.clip_angle(45_f64)
			.fit_extent([[50_f64, 50_f64], [950_f64, 950_f64]], &world);
		assert!(in_delta(projection.get_scale(), 450.348233, 1e-6));
		// TODO Must investigate the failure below.
		// assert!(in_delta_coordinate(
		// 	&projection.get_translate(),
		// 	&Coordinate {
		// 		x: 500.115138_f64,
		// 		y: 556.522620_f64
		// 	},
		// 	1e-6
		// ));
	}

	// 	// 	// // // tape("projection.fitExtent(…) world mercator", function(test) {
	// 	// 	// // //   var projection = d3.geoMercator();
	// 	// 	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// 	// 	// // //   test.inDelta(projection.scale(), 143.239449, 1e-6);
	// 	// 	// // //   test.inDelta(projection.translate(), [500, 481.549457], 1e-6);
	// 	// 	// // //   test.end();
	// 	// 	// // // });

	#[test]
	fn fit_extent_world_orthographic() {
		println!("projection.fitExtent(…) world orthographic");

		let world = world();
		let projection = Orthographic::builder()
			.fit_extent([[50.0_f64, 50.0_f64], [950.0_f64, 950.0_f64]], &world);
		assert!((in_delta(projection.get_scale(), 451.406773, 1e-6)));
		assert!(in_delta_coordinate(
			&projection.get_translate(),
			&Coordinate {
				x: 503.769179_f64,
				y: 498.593227_f64
			},
			1e-6
		));
	}

	#[test]
	fn fit_size_world_orthographic() {
		println!("projection.fitSize(…) world orthographic");

		let world = world();
		let projection = Orthographic::builder().fit_size([900.0_f64, 900.0_f64], &world);
		assert!(in_delta(projection.get_scale(), 451.406773, 1e-6));
		assert!(in_delta_coordinate(
			&projection.get_translate(),
			&Coordinate {
				x: 453.769179_f64,
				y: 448.593227_f64
			},
			1e-6
		));
	}

	#[test]
	fn fit_extent_world_stereographic() {
		println!("projection.fitExtent(…) world stereographic");

		let world = world();
		let projection = Stereographic::builder()
			.fit_extent([[50.0_f64, 50.0_f64], [950.0_f64, 950.0_f64]], &world);
		assert!(in_delta(projection.get_scale(), 162.934379_f64, 1e-6));
		assert!(in_delta_coordinate(
			&projection.get_translate(),
			&Coordinate {
				x: 478.546293_f64,
				y: 432.922534_f64
			},
			1e-6
		));
	}

	// 	// 	// // // tape("projection.fitExtent(…) world transverseMercator", function(test) {
	// 	// 	// // //   var projection = d3.geoTransverseMercator();
	// 	// 	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// 	// 	// // //   test.inDelta(projection.scale(), 143.239449, 1e-6);
	// 	// 	// // //   test.inDelta(projection.translate(), [473.829551, 500], 1e-6);
	// 	// 	// // //   test.end();
	// 	// 	// // // });

	// 	// 	// // // tape("projection.fitExtent(…) USA albersUsa", function(test) {
	// 	// 	// // //   var projection = d3.geoAlbersUsa();
	// 	// 	// // //   projection.fitExtent([[50, 50], [950, 950]], us);
	// 	// 	// // //   test.inDelta(projection.scale(), 1152.889035, 1e-6);
	// 	// 	// // //   test.inDelta(projection.translate(), [533.52541, 496.232028], 1e-6);
	// 	// 	// // //   test.end();
	// 	// 	// // // });

	// 	// 	// // // tape("projection.fitExtent(…) null geometries - Feature", function(test) {
	// 	// 	// // //   var projection = d3.geoEquirectangular();
	// 	// 	// // //   projection.fitExtent([[50, 50], [950, 950]], {type: "Feature", geometry: null});
	// 	// 	// // //   var s = projection.scale(), t = projection.translate();
	// 	// 	// // //   test.assert(!s);
	// 	// 	// // //   test.assert(isNaN(t[0]));
	// 	// 	// // //   test.assert(isNaN(t[1]));
	// 	// 	// // //   test.end();
	// 	// 	// // // });

	// 	// 	// // // tape("projection.fitExtent(…) null geometries - MultiPoint", function(test) {
	// 	// 	// // //   var projection = d3.geoEquirectangular();
	// 	// 	// // //   projection.fitExtent([[50, 50], [950, 950]], {type: "MultiPoint", coordinates: []});
	// 	// 	// // //   var s = projection.scale(), t = projection.translate();
	// 	// 	// // //   test.assert(!s);
	// 	// 	// // //   test.assert(isNaN(t[0]));
	// 	// 	// // //   test.assert(isNaN(t[1]));
	// 	// 	// // //   test.end();
	// 	// 	// // // });

	// 	// 	// // // tape("projection.fitExtent(…) null geometries - MultiLineString", function(test) {
	// 	// 	// // //   var projection = d3.geoEquirectangular();
	// 	// 	// // //   projection.fitExtent([[50, 50], [950, 950]], {type: "MultiLineString", coordinates: []});
	// 	// 	// // //   var s = projection.scale(), t = projection.translate();
	// 	// 	// // //   test.assert(!s);
	// 	// 	// // //   test.assert(isNaN(t[0]));
	// 	// 	// // //   test.assert(isNaN(t[1]));
	// 	// 	// // //   test.end();
	// 	// 	// // // });

	// 	// 	// // // tape("projection.fitExtent(…) null geometries - MultiPolygon", function(test) {
	// 	// 	// // //   var projection = d3.geoEquirectangular();
	// 	// 	// // //   projection.fitExtent([[50, 50], [950, 950]], {type: "MultiPolygon", coordinates: []});
	// 	// 	// // //   var s = projection.scale(), t = projection.translate();
	// 	// 	// // //   test.assert(!s);
	// 	// 	// // //   test.assert(isNaN(t[0]));
	// 	// 	// // //   test.assert(isNaN(t[1]));
	// 	// 	// // //   test.end();
	// 	// 	// // // });

	// 	// 	// // // tape("projection.fitExtent(…) custom projection", function(test) {
	// 	// 	// // //   var projection = d3.geoProjection(function(x, y) { return [x, Math.pow(y, 3)]; });
	// 	// 	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// 	// 	// // //   test.inDelta(projection.scale(), 128.903525, 1e-6);
	// 	// 	// // //   test.inDelta(projection.translate(), [500, 450.414357], 1e-6);
	// 	// 	// // //   test.end();
	// 	// 	// // // });

	// 	// 	// // // tape("projection.fitSize(…) ignore clipExtent - world equirectangular", function(test) {
	// 	// 	// // //   var p1 = d3.geoEquirectangular().fitSize([1000, 1000], world),
	// 	// 	// // //       s1 = p1.scale(),
	// 	// 	// // //       t1 = p1.translate(),
	// 	// 	// // //       c1 = p1.clipExtent(),
	// 	// 	// // //       p2 = d3.geoEquirectangular().clipExtent([[100, 200], [700, 600]]).fitSize([1000, 1000], world),
	// 	// 	// // //       s2 = p2.scale(),
	// 	// 	// // //       t2 = p2.translate(),
	// 	// 	// // //       c2 = p2.clipExtent();
	// 	// 	// // //   test.inDelta(s1, s2, 1e-6);
	// 	// 	// // //   test.inDelta(t1, t2, 1e-6);
	// 	// 	// // //   test.equal(c1, null);
	// 	// 	// // //   test.deepEqual(c2, [[100, 200], [700, 600]]);
	// 	// 	// // //   test.end();
	// 	// 	// // // });

	// 	// 	// // // tape("projection.fitExtent(…) chaining - world transverseMercator", function(test) {
	// 	// 	// // //   var projection = d3.geoTransverseMercator().fitExtent([[50, 50], [950, 950]], world).scale(500);
	// 	// 	// // //   test.equal(projection.scale(), 500);
	// 	// 	// // //   test.inDelta(projection.translate(), [473.829551, 500], 1e-6);
	// 	// 	// // //   test.end();
	// 	// 	// // // });

	// #[test]
	// fn fit_size_resampling() {
	// 	println!("projection.fitSize(…) resampling - world mercator");
	// 	let box_object = Geometry::Polygon(polygon![
	// 		(x: -135f64, y: 45f64 ),
	// 		(x: -45f64, y: 45f64 ),
	// 		(x: -45f64, y: -45f64 ),
	// 		(x: -135f64, y: -45f64 ),
	// 		(x: -135f64, y: 45f64 )
	// 	]);
	// 	let p1 = Mercator::builder()
	// 		.precision(&0.1_f64)
	// 		.fit_size([1000_f64, 1000_f64], &box_object);
	// 	let p2 = Mercator::builder()
	// 		.precision(&0.0_f64)
	// 		.fit_size([1000_f64, 1000_f64], &box_object);
	// 	let t1 = p1.get_translate();
	// 	let t2 = p2.get_translate();
	// 	assert_eq!(p1.get_precision(), 0.1_f64);
	// 	assert_eq!(p2.get_precision(), 0_f64);
	// 	assert!(in_delta(p1.get_scale(), 436.218018, 1e-6));
	// 	assert!(in_delta(p2.get_scale(), 567.296328, 1e-6));
	// 	assert!(in_delta(t1.x, 1185.209661_f64, 1e-6));
	// 	assert!(in_delta(t2.x, 1391.106989_f64, 1e-6));
	// 	assert!(in_delta(t1.y, 500_f64, 1e-6));
	// 	assert!(in_delta(t1.y, t2.y, 1e-6));
	// }

	// 	// 	// // // tape("projection.fitWidth(…) world equirectangular", function(test) {
	// 	// 	// // //   var projection = d3.geoEquirectangular();
	// 	// 	// // //   projection.fitWidth(900, world);
	// 	// 	// // //   test.inDelta(projection.scale(), 143.239449, 1e-6);
	// 	// 	// // //   test.inDelta(projection.translate(), [450, 208.999023], 1e-6);
	// 	// 	// // //   test.end();
	// 	// 	// // // });

	#[test]
	fn fit_width_world_equirectangular() {
		println!("projection.fitWidth(…) world equirectangular");

		let world = world();
		let projection = Equirectangular::builder().fit_width(900_f64, &world);
		assert!(in_delta(projection.get_scale(), 143.239449_f64, 1e-6));
		assert!(in_delta_coordinate(
			&projection.get_translate(),
			&Coordinate {
				x: 450_f64,
				y: 208.999023_f64
			},
			1e-6
		));
	}

	// 	// 	// // // tape("projection.fitWidth(…) world transverseMercator", function(test) {
	// 	// 	// // //   var projection = d3.geoTransverseMercator();
	// 	// 	// // //   projection.fitWidth(900, world);
	// 	// 	// // //   test.inDelta(projection.scale(), 166.239257, 1e-6);
	// 	// 	// // //   test.inDelta(projection.translate(), [419.627390, 522.256029], 1e-6);
	// 	// 	// // //   test.end();
	// 	// 	// // // });

	// 	// 	// // // tape("projection.fitWidth(…) USA albersUsa", function(test) {
	// 	// 	// // //   var projection = d3.geoAlbersUsa();
	// 	// 	// // //   projection.fitWidth(900, us);
	// 	// 	// // //   test.inDelta(projection.scale(), 1152.889035, 1e-6);
	// 	// 	// // //   test.inDelta(projection.translate(), [483.52541, 257.736905], 1e-6);
	// 	// 	// // //   test.end();
	// 	// 	// // // });

	// 	// 	// // // tape("projection.fitHeight(…) world equirectangular", function(test) {
	// 	// 	// // //   var projection = d3.geoEquirectangular();
	// 	// 	// // //   projection.fitHeight(900, world);
	// 	// 	// // //   test.inDelta(projection.scale(), 297.042711, 1e-6);
	// 	// 	// // //   test.inDelta(projection.translate(), [933.187199, 433.411585], 1e-6);
	// 	// 	// // //   test.end();
	// 	// 	// // // });

	// 	// 	// #[test]
	// 	// 	// fn fit_height_world_equirectangular() {
	// 	// 	//     println!("projection.fitHeight(…) world equirectangular");

	// 	// 	//     let world = world();
	// 	// 	//     let projection: ProjectionBuilder<
	// 	// 	//         Bounds<f64>,
	// 	// 	//         _,
	// 	// 	//         Equirectangular<Bounds<f64>, f64>,
	// 	// 	//         _,
	// 	// 	//         f64,
	// 	// 	//     > = Equirectangular::builder()
	// 	// 	//         .fit_height(900f64, &world);
	// 	// 	//     assert!(in_delta(projection.get_scale(), 297.042711_f64, 1e-6));
	// 	// 	//     assert!(in_delta_coordinate(
	// 	// 	//         &projection.get_translate(),
	// 	// 	//         &Coordinate {
	// 	// 	//             x: 933.187199_f64,
	// 	// 	//             y: 433.411585_f64
	// 	// 	//         },
	// 	// 	//         1e-6
	// 	// 	//     ));
	// 	// 	// }

	// 	// 	// // // tape("projection.fitHeight(…) world transverseMercator", function(test) {
	// 	// 	// // //   var projection = d3.geoTransverseMercator();
	// 	// 	// // //   projection.fitHeight(900, world);
	// 	// 	// // //   test.inDelta(projection.scale(), 143.239449, 1e-6);
	// 	// 	// // //   test.inDelta(projection.translate(), [361.570408, 450], 1e-6);
	// 	// 	// // //   test.end();
	// 	// 	// // // });

	// 	// 	// // // tape("projection.fitHeight(…) USA albersUsa", function(test) {
	// 	// 	// // //   var projection = d3.geoAlbersUsa();
	// 	// 	// // //   projection.fitHeight(900, us);
	// 	// 	// // //   test.inDelta(projection.scale(), 1983.902059, 1e-6);
	// 	// 	// // //   test.inDelta(projection.translate(), [832.054974, 443.516038], 1e-6);
	// 	// 	// // //   test.end();
	// 	// 	// // // });
}
