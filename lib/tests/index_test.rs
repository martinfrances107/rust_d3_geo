#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod index_test {

    extern crate pretty_assertions;

    use std::f64::consts::PI;
    use std::ops::AddAssign;
	use std::fmt::Display;

    use approx::AbsDiffEq;
    use geo::CoordFloat;
    use geo::line_string;
    use geo::Coordinate;
    use geo::Geometry;
    use geo::LineString;
    use geo::Polygon;
    use num_traits::AsPrimitive;
    use num_traits::FloatConst;
    use pretty_assertions::assert_eq;

    use rust_d3_geo::clip::buffer::Buffer;
    use rust_d3_geo::identity::Identity;
    use rust_d3_geo::path::builder::Builder as PathBuilder;
    use rust_d3_geo::path::string::String as PathString;
    use rust_d3_geo::projection::ProjectionRawBase;
    use rust_d3_geo::projection::builder::template::ResampleNoneNoClipC;
	use rust_d3_geo::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
	use rust_d3_geo::clip::antimeridian::line::Line as LineAntimeridian;
	use rust_d3_geo::clip::antimeridian::pv::PV as PVAntimeridian;
    use rust_d3_geo::projection::builder::template::ResampleNoneNoClipU;
    use rust_d3_geo::projection::equirectangular::Equirectangular;
    use rust_d3_geo::projection::projector::Projector;
    use rust_d3_geo::projection::PrecisionBypass;
    use rust_d3_geo::projection::Scale;
    use rust_d3_geo::stream::Connected;
    use rust_d3_geo::stream::Stream;
    use rust_d3_geo::stream::Streamable;
    use rust_d3_geo::stream::Unconnected;

    #[inline]
    fn equirectangular<
		EP: Stream<EP=EP, T=T> + Default,
        T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + AddAssign + CoordFloat + Display + FloatConst,
    >() -> Projector<
		EP,
		InterpolateAntimeridian<
			EP,
			ResampleNoneNoClipC<EP, Equirectangular<EP, T>, T>,
			T,
		>,
		LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
		LineAntimeridian<
			EP,
			ResampleNoneNoClipC<EP, Equirectangular<EP, T>, T>,
			Connected<ResampleNoneNoClipC<EP, Equirectangular<EP, T>, T>>,
			T,
		>,
		LineAntimeridian<
			EP,
			ResampleNoneNoClipC<EP, Equirectangular<EP, T>, T>,
			Unconnected,
			T,
		>,
		Identity<EP, EP, EP, Connected<EP>, T>,
		Identity<EP, EP, EP, Unconnected, T>,
		Equirectangular<EP, T>,
		PVAntimeridian<T>,
		ResampleNoneNoClipC<EP, Equirectangular<EP, T>, T>,
		ResampleNoneNoClipU<EP, Equirectangular<EP, T>, T>,
		T> {
        Equirectangular::builder()
            .scale(T::from(900f64 / PI).unwrap())
            .precision_bypass()
            .build()
    }

    #[inline]
    fn test_path<'a, T: AsPrimitive<T> + AddAssign + AbsDiffEq<Epsilon=T> + CoordFloat + FloatConst  + Display>(
        projection: Projector<
            PathString<T>,
            InterpolateAntimeridian<
                PathString<T>,
                ResampleNoneNoClipC<PathString<T>, Equirectangular<PathString<T>, T>, T>,
                T,
            >,
            LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
            LineAntimeridian<
                PathString<T>,
                ResampleNoneNoClipC<PathString<T>, Equirectangular<PathString<T>, T>, T>,
                Connected<ResampleNoneNoClipC<PathString<T>, Equirectangular<PathString<T>, T>, T>>,
                T,
            >,
            LineAntimeridian<
                PathString<T>,
                ResampleNoneNoClipC<PathString<T>, Equirectangular<PathString<T>, T>, T>,
                Unconnected,
                T,
            >,
            Identity<PathString<T>, PathString<T>, PathString<T>, Connected<PathString<T>>, T>,
            Identity<PathString<T>, PathString<T>, PathString<T>, Unconnected, T>,
            Equirectangular<PathString<T>, T>,
            PVAntimeridian<T>,
            ResampleNoneNoClipC<PathString<T>, Equirectangular<PathString<T>, T>, T>,
            ResampleNoneNoClipU<PathString<T>, Equirectangular<PathString<T>, T>, T>,
            T>,
        object: impl Streamable<T = T>,
    ) -> String {
        let pb = PathBuilder::context_pathstring();

        pb.build(projection).object(&object)
    }

    // function testPath(projection, object) {
    //   var context = testContext();

    //   d3_geo.geoPath()
    //       .projection(projection)
    //       .context(context)
    //       (object);

    //   return context.result();
    // }

    // tape("geoPath.projection() defaults to null", function(test) {
    //   var path = d3_geo.geoPath();
    //   test.strictEqual(path.projection(), null);
    //   test.end();
    // });

    // tape("geoPath.context() defaults to null", function(test) {
    //   var path = d3_geo.geoPath();
    //   test.strictEqual(path.context(), null);
    //   test.end();
    // });

    // tape("d3.geoPath(projection) sets the initial projection", function(test) {
    //   var projection = d3_geo.geoAlbers(), path = d3_geo.geoPath(projection);
    //   test.strictEqual(path.projection(), projection);
    //   test.end();
    // });

    // tape("d3.geoPath(projection, context) sets the initial projection and context", function(test) {
    //   var context = testContext(), projection = d3_geo.geoAlbers(), path = d3_geo.geoPath(projection, context);
    //   test.strictEqual(path.projection(), projection);
    //   test.strictEqual(path.context(), context);
    //   test.end();
    // });

    // tape("geoPath(Point) renders a point", function(test) {
    //   test.deepEqual(testPath(equirectangular, {
    //     type: "Point",
    //     coordinates: [-63, 18]
    //   }), [
    //     {type: "moveTo", x: 170, y: 160},
    //     {type: "arc", x: 165, y: 160, r: 4.5}
    //   ]);
    //   test.end();
    // });

    // #[test]
    // fn test_path_point_renders_a_point() {
    // todo!(
    // 	"Must mock CanvasRederingConext2D otherwise this test is identical to path_string_test.rs"
    // );
    // 	println!("geoPath(Point) renders a point");
    // 	let object = DataObject::Geometry(Geometry::Point(Point::new(-63.0_f64, 18.0_f64)));
    // 	assert_eq!(test_path(equirectangular(), object), "M170,160,A165,160,");
    // }

    //     // tape("geoPath(MultiPoint) renders a point", function(test) {
    //     //   test.deepEqual(testPath(equirectangular, {
    //     //     type: "MultiPoint",
    //     //     coordinates: [[-63, 18], [-62, 18], [-62, 17]]
    //     //   }), [
    //     //     {type: "moveTo", x: 170, y: 160}, {type: "arc", x: 165, y: 160, r: 4.5},
    //     //     {type: "moveTo", x: 175, y: 160}, {type: "arc", x: 170, y: 160, r: 4.5},
    //     //     {type: "moveTo", x: 175, y: 165}, {type: "arc", x: 170, y: 165, r: 4.5}
    //     //   ]);
    //     //   test.end();
    //     // });

    // #[test]
    // fn test_render_a_multipoint() {
    // 	println!("geoPath(MultiPoint) renders a point");
    // 	let object = DataObject::Geometry(Geometry::MultiPoint(
    // 		vec![(-63_f64, 18_f64), (-62_f64, 18_f64), (-62_f64, 17_f64)].into(),
    // 	));
    // 	assert_eq!(test_path(equirectangular(), object), "happ");
    // }

    #[test]
    fn render_line_string() {
        println!("geoPath(LineString) renders a line string");
        let object = Geometry::LineString(line_string![
			(x: -63_f64, y: 18_f64),(x: -62_f64, y: 18_f64), (x: -62_f64, y:17_f64) ]);

        assert_eq!(
            test_path(equirectangular(), object),
            ["M165,160", "L170,160", "L170,165"].join("")
        );
    }

    #[test]
    fn render_a_polygon() {
        println!("geoPath(Polygon) renders a polygon");
        let exterior: LineString<f64> = LineString(vec![
            Coordinate {
                x: -63_f64,
                y: 18_f64,
            },
            Coordinate {
                x: -62_f64,
                y: 18_f64,
            },
            Coordinate {
                x: -62_f64,
                y: 17_f64,
            },
        ]);
        let interiors = vec![];
        let object = Geometry::Polygon(Polygon::new(exterior, interiors));
        assert_eq!(
            test_path(equirectangular(), object),
            ["M165,160", "L170,160", "L170,165", "Z"].join("")
        );
    }
    //     // tape("geoPath(Polygon) renders a polygon", function(test) {
    //     //   test.deepEqual(testPath(equirectangular, {
    //     //     type: "Polygon",
    //     //     coordinates: [[[-63, 18], [-62, 18], [-62, 17], [-63, 18]]]
    //     //   }), [
    //     //     {type: "moveTo", x: 165, y: 160},
    //     //     {type: "lineTo", x: 170, y: 160},
    //     //     {type: "lineTo", x: 170, y: 165},
    //     //     {type: "closePath"}
    //     //   ]);
    //     //   test.end();
    //     // });

    //     // tape("geoPath(GeometryCollection) renders a geometry collection", function(test) {
    //     //   test.deepEqual(testPath(equirectangular, {
    //     //     type: "GeometryCollection",
    //     //     geometries: [{
    //     //       type: "Polygon",
    //     //       coordinates: [[[-63, 18], [-62, 18], [-62, 17], [-63, 18]]]
    //     //     }]
    //     //   }), [
    //     //     {type: "moveTo", x: 165, y: 160},
    //     //     {type: "lineTo", x: 170, y: 160},
    //     //     {type: "lineTo", x: 170, y: 165},
    //     //     {type: "closePath"}
    //     //   ]);
    //     //   test.end();
    //     // });

    //     // tape("geoPath(Feature) renders a feature", function(test) {
    //     //   test.deepEqual(testPath(equirectangular, {
    //     //     type: "Feature",
    //     //     geometry: {
    //     //       type: "Polygon",
    //     //       coordinates: [[[-63, 18], [-62, 18], [-62, 17], [-63, 18]]]
    //     //     }
    //     //   }), [
    //     //     {type: "moveTo", x: 165, y: 160},
    //     //     {type: "lineTo", x: 170, y: 160},
    //     //     {type: "lineTo", x: 170, y: 165},
    //     //     {type: "closePath"}
    //     //   ]);
    //     //   test.end();
    //     // });

    //     // tape("geoPath(FeatureCollection) renders a feature collection", function(test) {
    //     //   test.deepEqual(testPath(equirectangular, {
    //     //     type: "FeatureCollection",
    //     //     features: [{
    //     //       type: "Feature",
    //     //       geometry: {
    //     //         type: "Polygon",
    //     //         coordinates: [[[-63, 18], [-62, 18], [-62, 17], [-63, 18]]]
    //     //       }
    //     //     }]
    //     //   }), [
    //     //     {type: "moveTo", x: 165, y: 160},
    //     //     {type: "lineTo", x: 170, y: 160},
    //     //     {type: "lineTo", x: 170, y: 165},
    //     //     {type: "closePath"}
    //     //   ]);
    //     //   test.end();
    //     // });

    // #[test]
    // fn test_wrap_longitude_outside_180() {
    // todo!(
    // 	"Must mock CanvasRederingConext2D otherwise this test is identical to path_string_test.rs"
    // );
    // 	println!("geoPath(Point) renders a point");
    // 	let object = DataObject::Geometry(Geometry::Point(Point::new(180_f64 + 1e-6_f64, 0_f64)));
    // 	assert_eq!(
    // 		test_path(equirectangular(), object),
    // 		["M-415,250", "A-420,250,4.5"].join("")
    // 	);
    // }

    //     // tape("geoPath(…) observes the correct winding order of a tiny polygon", function(test) {
    //     //   test.deepEqual(testPath(equirectangular, {
    //     //     type: "Polygon",
    //     //     coordinates: [[
    //     //       [-0.06904102953339501, 0.346043661846373],
    //     //       [-6.725674252975136e-15, 0.3981303360336475],
    //     //       [-6.742247658534323e-15, -0.08812465346531581],
    //     //       [-0.17301258217724075, -0.12278150669440671],
    //     //       [-0.06904102953339501, 0.346043661846373]
    //     //     ]]
    //     //   }), [
    //     //     {type: "moveTo", x: 480, y: 248},
    //     //     {type: "lineTo", x: 480, y: 248},
    //     //     {type: "lineTo", x: 480, y: 250},
    //     //     {type: "lineTo", x: 479, y: 251},
    //     //     {type: "closePath"}
    //     //   ]);
    //     //   test.end();
    //     // });

    //     // tape("geoPath.projection(null)(…) does not transform coordinates", function(test) {
    //     //   test.deepEqual(testPath(null, {
    //     //     type: "Polygon",
    //     //     coordinates: [[[-63, 18], [-62, 18], [-62, 17], [-63, 18]]]
    //     //   }), [
    //     //     {type: "moveTo", x: -63, y: 18},
    //     //     {type: "lineTo", x: -62, y: 18},
    //     //     {type: "lineTo", x: -62, y: 17},
    //     //     {type: "closePath"}
    //     //   ]);
    //     //   test.end();
    //     // });

    //     // tape("geoPath.context(null)(null) returns null", function(test) {
    //     //   var path = d3_geo.geoPath();
    //     //   test.strictEqual(path(), null);
    //     //   test.strictEqual(path(null), null);
    //     //   test.strictEqual(path(undefined), null);
    //     //   test.end();
    //     // });

    //     // tape("geoPath.context(null)(Unknown) returns null", function(test) {
    //     //   var path = d3_geo.geoPath();
    //     //   test.strictEqual(path({type: "Unknown"}), null);
    //     //   test.strictEqual(path({type: "__proto__"}), null);
    //     //   test.end();
    //     // });

    //     // tape("geoPath(LineString) then geoPath(Point) does not treat the point as part of a line", function(test) {
    //     //   var context = testContext(), path = d3_geo.geoPath().projection(equirectangular).context(context);
    //     //   path({
    //     //     type: "LineString",
    //     //     coordinates: [[-63, 18], [-62, 18], [-62, 17]]
    //     //   });
    //     //   test.deepEqual(context.result(), [
    //     //     {type: "moveTo", x: 165, y: 160},
    //     //     {type: "lineTo", x: 170, y: 160},
    //     //     {type: "lineTo", x: 170, y: 165}
    //     //   ]);
    //     //   path({
    //     //     type: "Point",
    //     //     coordinates: [-63, 18]
    //     //   });
    //     //   test.deepEqual(context.result(), [
    //     //     {type: "moveTo", x: 170, y: 160},
    //     //     {type: "arc", x: 165, y: 160, r: 4.5}
    //     //   ]);
    //     //   test.end();
    //     // });
}
