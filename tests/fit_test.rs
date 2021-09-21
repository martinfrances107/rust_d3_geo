#[cfg(not(tarpaulin_include))]
#[cfg(test)]

mod fit_test {

	use geo::polygon;
	use geo::Geometry;
	use rust_d3_geo::path::bounds_stream::BoundsStream;
	use rust_d3_geo::projection::builder::Builder as ProjectionBuilder;
	use std::f64::consts::PI;

	use rust_d3_geo::data_object::sphere::Sphere;
	use rust_d3_geo::data_object::DataObject;
	use rust_d3_geo::in_delta::in_delta;
	use rust_d3_geo::projection::equirectangular::EquirectangularRaw;
	use rust_d3_geo::projection::mercator::Mercator;
	use rust_d3_geo::projection::Fit;
	use rust_d3_geo::projection::Precision;
	use rust_d3_geo::projection::Raw;
	use rust_d3_geo::projection::Scale;
	use rust_d3_geo::projection::Translate;

	// use rust_d3_geo::projection::projection::Projection;

	#[test]
	fn test_fit_extent_sphere_equirectangular() {
		println!("projection.fitExtent(…) sphere equirectangular");
		let d_object = DataObject::Sphere(Sphere::default());
		let projection: ProjectionBuilder<
			BoundsStream<f64>,
			_,
			EquirectangularRaw<BoundsStream<f64>, f64>,
			_,
			f64,
		> = EquirectangularRaw::builder()
			.fit_extent([[50.0_f64, 50.0_f64], [950.0_f64, 950.0_f64]], d_object);
		assert!(in_delta(projection.get_scale(), 900. / (2. * PI), 1e-6));
		let translate = projection.get_translate();
		assert!(in_delta(translate.x, 500., 1e-6));
		assert!(in_delta(translate.y, 500., 1e-6));
	}

	// // // tape("projection.fitExtent(…) world equirectangular", function(test) {
	// // //   var projection = d3.geoEquirectangular();
	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// // //   test.inDelta(projection.scale(), 143.239449, 1e-6);
	// // //   test.inDelta(projection.translate(), [500, 492.000762], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitExtent(…) world azimuthalEqualArea", function(test) {
	// // //   var projection = d3.geoAzimuthalEqualArea();
	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// // //   test.inDelta(projection.scale(), 228.357229, 1e-6);
	// // //   test.inDelta(projection.translate(), [496.353618, 479.684353], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitExtent(…) world azimuthalEquidistant", function(test) {
	// // //   var projection = d3.geoAzimuthalEquidistant();
	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// // //   test.inDelta(projection.scale(), 153.559317, 1e-6);
	// // //   test.inDelta(projection.translate(), [485.272493, 452.093375], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitExtent(…) world conicConformal", function(test) {
	// // //   var projection = d3.geoConicConformal().clipAngle(30).parallels([30, 60]).rotate([0, -45]);
	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// // //   test.inDelta(projection.scale(), 626.111027, 1e-6);
	// // //   test.inDelta(projection.translate(), [444.395951, 410.223799], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitExtent(…) world conicEqualArea", function(test) {
	// // //   var projection = d3.geoConicEqualArea();
	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// // //   test.inDelta(projection.scale(), 145.862346, 1e-6);
	// // //   test.inDelta(projection.translate(), [500, 498.0114265], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitExtent(…) world conicEquidistant", function(test) {
	// // //   var projection = d3.geoConicEquidistant();
	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// // //   test.inDelta(projection.scale(), 123.085587, 1e-6);
	// // //   test.inDelta(projection.translate(), [500, 498.598401], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitExtent(…) world equirectangular", function(test) {
	// // //   var projection = d3.geoEquirectangular();
	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// // //   test.inDelta(projection.scale(), 143.239449, 1e-6);
	// // //   test.inDelta(projection.translate(), [500, 492.000762], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitSize(…) world equirectangular", function(test) {
	// // //   var projection = d3.geoEquirectangular();
	// // //   projection.fitSize([900, 900], world);
	// // //   test.inDelta(projection.scale(), 143.239449, 1e-6);
	// // //   test.inDelta(projection.translate(), [450, 442.000762], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitExtent(…) world gnomonic", function(test) {
	// // //   var projection = d3.geoGnomonic().clipAngle(45);
	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// // //   test.inDelta(projection.scale(), 450.348233, 1e-6);
	// // //   test.inDelta(projection.translate(), [500.115138, 556.522620], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitExtent(…) world mercator", function(test) {
	// // //   var projection = d3.geoMercator();
	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// // //   test.inDelta(projection.scale(), 143.239449, 1e-6);
	// // //   test.inDelta(projection.translate(), [500, 481.549457], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitExtent(…) world orthographic", function(test) {
	// // //   var projection = d3.geoOrthographic();
	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// // //   test.inDelta(projection.scale(), 451.406773, 1e-6);
	// // //   test.inDelta(projection.translate(), [503.769179, 498.593227], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitSize(…) world orthographic", function(test) {
	// // //   var projection = d3.geoOrthographic();
	// // //   projection.fitSize([900, 900], world);
	// // //   test.inDelta(projection.scale(), 451.406773, 1e-6);
	// // //   test.inDelta(projection.translate(), [453.769179, 448.593227], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitExtent(…) world stereographic", function(test) {
	// // //   var projection = d3.geoStereographic();
	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// // //   test.inDelta(projection.scale(), 162.934379, 1e-6);
	// // //   test.inDelta(projection.translate(), [478.546293, 432.922534], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitExtent(…) world transverseMercator", function(test) {
	// // //   var projection = d3.geoTransverseMercator();
	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// // //   test.inDelta(projection.scale(), 143.239449, 1e-6);
	// // //   test.inDelta(projection.translate(), [473.829551, 500], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitExtent(…) USA albersUsa", function(test) {
	// // //   var projection = d3.geoAlbersUsa();
	// // //   projection.fitExtent([[50, 50], [950, 950]], us);
	// // //   test.inDelta(projection.scale(), 1152.889035, 1e-6);
	// // //   test.inDelta(projection.translate(), [533.52541, 496.232028], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitExtent(…) null geometries - Feature", function(test) {
	// // //   var projection = d3.geoEquirectangular();
	// // //   projection.fitExtent([[50, 50], [950, 950]], {type: "Feature", geometry: null});
	// // //   var s = projection.scale(), t = projection.translate();
	// // //   test.assert(!s);
	// // //   test.assert(isNaN(t[0]));
	// // //   test.assert(isNaN(t[1]));
	// // //   test.end();
	// // // });

	// // // tape("projection.fitExtent(…) null geometries - MultiPoint", function(test) {
	// // //   var projection = d3.geoEquirectangular();
	// // //   projection.fitExtent([[50, 50], [950, 950]], {type: "MultiPoint", coordinates: []});
	// // //   var s = projection.scale(), t = projection.translate();
	// // //   test.assert(!s);
	// // //   test.assert(isNaN(t[0]));
	// // //   test.assert(isNaN(t[1]));
	// // //   test.end();
	// // // });

	// // // tape("projection.fitExtent(…) null geometries - MultiLineString", function(test) {
	// // //   var projection = d3.geoEquirectangular();
	// // //   projection.fitExtent([[50, 50], [950, 950]], {type: "MultiLineString", coordinates: []});
	// // //   var s = projection.scale(), t = projection.translate();
	// // //   test.assert(!s);
	// // //   test.assert(isNaN(t[0]));
	// // //   test.assert(isNaN(t[1]));
	// // //   test.end();
	// // // });

	// // // tape("projection.fitExtent(…) null geometries - MultiPolygon", function(test) {
	// // //   var projection = d3.geoEquirectangular();
	// // //   projection.fitExtent([[50, 50], [950, 950]], {type: "MultiPolygon", coordinates: []});
	// // //   var s = projection.scale(), t = projection.translate();
	// // //   test.assert(!s);
	// // //   test.assert(isNaN(t[0]));
	// // //   test.assert(isNaN(t[1]));
	// // //   test.end();
	// // // });

	// // // tape("projection.fitExtent(…) custom projection", function(test) {
	// // //   var projection = d3.geoProjection(function(x, y) { return [x, Math.pow(y, 3)]; });
	// // //   projection.fitExtent([[50, 50], [950, 950]], world);
	// // //   test.inDelta(projection.scale(), 128.903525, 1e-6);
	// // //   test.inDelta(projection.translate(), [500, 450.414357], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitSize(…) ignore clipExtent - world equirectangular", function(test) {
	// // //   var p1 = d3.geoEquirectangular().fitSize([1000, 1000], world),
	// // //       s1 = p1.scale(),
	// // //       t1 = p1.translate(),
	// // //       c1 = p1.clipExtent(),
	// // //       p2 = d3.geoEquirectangular().clipExtent([[100, 200], [700, 600]]).fitSize([1000, 1000], world),
	// // //       s2 = p2.scale(),
	// // //       t2 = p2.translate(),
	// // //       c2 = p2.clipExtent();
	// // //   test.inDelta(s1, s2, 1e-6);
	// // //   test.inDelta(t1, t2, 1e-6);
	// // //   test.equal(c1, null);
	// // //   test.deepEqual(c2, [[100, 200], [700, 600]]);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitExtent(…) chaining - world transverseMercator", function(test) {
	// // //   var projection = d3.geoTransverseMercator().fitExtent([[50, 50], [950, 950]], world).scale(500);
	// // //   test.equal(projection.scale(), 500);
	// // //   test.inDelta(projection.translate(), [473.829551, 500], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitSize(…) resampling - world mercator", function(test) {
	// // //   var box = {"type": "Polygon", "coordinates": [[[-135, 45], [-45, 45], [-45, -45], [-135, -45], [-135, 45]]]},
	// // //       p1 = d3.geoMercator().precision(0.1).fitSize([1000, 1000], box),
	// // //       p2 = d3.geoMercator().precision(0).fitSize([1000, 1000], box),
	// // //       t1 = p1.translate(),
	// // //       t2 = p2.translate();
	// // //   test.equal(p1.precision(), 0.1);
	// // //   test.equal(p2.precision(), 0);
	// // //   test.inDelta(p1.scale(), 436.218018, 1e-6);
	// // //   test.inDelta(p2.scale(), 567.296328, 1e-6);
	// // //   test.inDelta(t1[0], 1185.209661, 1e-6);
	// // //   test.inDelta(t2[0], 1391.106989, 1e-6);
	// // //   test.inDelta(t1[1], 500, 1e-6);
	// // //   test.inDelta(t1[1], t2[1], 1e-6);
	// // //   test.end();
	// // // });

	#[test]
	fn fit_size_resampling() {
		println!("projection.fitSize(…) resampling - world mercator");
		let d_object = DataObject::Geometry(Geometry::Polygon(polygon![
			(x:  0f64, y: 0f64 ),
			(x:  3f64, y: 0f64 ),
			(x:  3f64, y: 3f64 ),
			(x:  0f64, y: 3f64 ),
			(x:  0f64, y: 0f64 )
		]));
		let p1 = Mercator::builder()
			.precision(&0.1_f64)
			.fit_size([1000_f64, 1000_f64], d_object.clone());
		let p2 = Mercator::builder()
			.precision(&0.0_f64)
			.fit_size([1000_f64, 1000_f64], d_object);
		let t1 = p1.get_translate();
		let t2 = p2.get_translate();
		assert_eq!(p1.get_precision(), 0.1_f64);
		assert_eq!(p2.get_precision(), 0_f64);
		// assert!(in_delta(p1.get_scale(), 436.218018, 1e-6));
		// assert!(in_delta(p2.get_scale(), 567.296328, 1e-6));
		// assert!(in_delta(t1.x, 1185.209661_f64, 1e-6));
		// assert!(in_delta(t2.x, 1391.106989_f64, 1e-6));
		// assert!(in_delta(t1.y, 500_f64, 1e-6));
		// assert!(in_delta(t1.y, t2.y, 1e-6));
	}

	// // // tape("projection.fitWidth(…) world equirectangular", function(test) {
	// // //   var projection = d3.geoEquirectangular();
	// // //   projection.fitWidth(900, world);
	// // //   test.inDelta(projection.scale(), 143.239449, 1e-6);
	// // //   test.inDelta(projection.translate(), [450, 208.999023], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitWidth(…) world transverseMercator", function(test) {
	// // //   var projection = d3.geoTransverseMercator();
	// // //   projection.fitWidth(900, world);
	// // //   test.inDelta(projection.scale(), 166.239257, 1e-6);
	// // //   test.inDelta(projection.translate(), [419.627390, 522.256029], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitWidth(…) USA albersUsa", function(test) {
	// // //   var projection = d3.geoAlbersUsa();
	// // //   projection.fitWidth(900, us);
	// // //   test.inDelta(projection.scale(), 1152.889035, 1e-6);
	// // //   test.inDelta(projection.translate(), [483.52541, 257.736905], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitHeight(…) world equirectangular", function(test) {
	// // //   var projection = d3.geoEquirectangular();
	// // //   projection.fitHeight(900, world);
	// // //   test.inDelta(projection.scale(), 297.042711, 1e-6);
	// // //   test.inDelta(projection.translate(), [933.187199, 433.411585], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitHeight(…) world transverseMercator", function(test) {
	// // //   var projection = d3.geoTransverseMercator();
	// // //   projection.fitHeight(900, world);
	// // //   test.inDelta(projection.scale(), 143.239449, 1e-6);
	// // //   test.inDelta(projection.translate(), [361.570408, 450], 1e-6);
	// // //   test.end();
	// // // });

	// // // tape("projection.fitHeight(…) USA albersUsa", function(test) {
	// // //   var projection = d3.geoAlbersUsa();
	// // //   projection.fitHeight(900, us);
	// // //   test.inDelta(projection.scale(), 1983.902059, 1e-6);
	// // //   test.inDelta(projection.translate(), [832.054974, 443.516038], 1e-6);
	// // //   test.end();
	// // // });
}
