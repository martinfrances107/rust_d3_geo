#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod path_string_test {

    use std::f64::consts::PI;
    use std::fmt::Display;
    use std::ops::AddAssign;

    use approx::AbsDiffEq;
    use geo::line_string;
    use geo::point;
    use geo::CoordFloat;
    use geo::Coordinate;
    use geo::Geometry;
    use geo::GeometryCollection;
    use geo::MultiPolygon;
    use geo::Polygon;
    use num_traits::AsPrimitive;
    use num_traits::FloatConst;
    use pretty_assertions::assert_eq;

	use rust_d3_geo::clip::buffer::Buffer;
	use rust_d3_geo::identity::Identity;
	use rust_d3_geo::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
	use rust_d3_geo::clip::antimeridian::line::Line as LineAntimeridian;
	use rust_d3_geo::clip::antimeridian::pv::PV as PVAntimeridian;
    use rust_d3_geo::projection::PrecisionBypass;
    use rust_d3_geo::projection::PrecisionSet;
    use rust_d3_geo::projection::ProjectionRawBase;
	// use rust_d3_geo::projection::builder::template::ResampleNoClipC;
    // use rust_d3_geo::projection::builder::template::ResampleNoClipU;
    use rust_d3_geo::projection::builder::template::ResampleNoneClipC;
    use rust_d3_geo::projection::builder::template::ResampleNoneClipU;
    use rust_d3_geo::projection::builder::template::ResampleNoneNoClipC;
	use rust_d3_geo::stream::Connected;
	use rust_d3_geo::stream::Unconnected;
	use rust_d3_geo::projection::builder::template::ResampleNoneNoClipU;
    use rust_d3_geo::circle::generator::Generator as CircleGenerator;
    use rust_d3_geo::path::builder::Builder as PathBuilder;
    use rust_d3_geo::path::string::String as PathString;
    use rust_d3_geo::path::PointRadiusTrait;
    use rust_d3_geo::projection::equirectangular::Equirectangular;
    use rust_d3_geo::projection::orthographic::Orthographic;
    use rust_d3_geo::projection::projector::Projector;
    // use rust_d3_geo::projection::Precision;
    use rust_d3_geo::projection::Scale;
    use rust_d3_geo::projection::Translate;
    // use rust_d3_geo::stream::Stream;
    use rust_d3_geo::stream::Streamable;

    #[inline]
    fn equirectangular<T>(
    ) -> Projector<
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
		T>
    where
        T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        Equirectangular::builder()
            .scale(T::from(900f64 / PI).unwrap())
            .precision_bypass()
            .build()
    }

    #[inline]
    fn test_path<'a,  T>(
        projection: Projector<PathString<T>,
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
    ) -> String
    where
        // DRAIN: Stream<EP = DRAIN, T = T>,
        T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        PathBuilder::context_pathstring()
            .build(projection)
            .object(&object)
    }

    #[test]
    fn point_renders_a_point() {
        println!("geoPath.point(…) renders a point");
        let object = Geometry::Point(point!(x: -63_f64, y:18_f64));
        let eq = equirectangular::<f64>();
        assert_eq!(
            test_path(eq, object),
            "M165,160m0,4.5a4.5,4.5 0 1,1 0,-9a4.5,4.5 0 1,1 0,9z"
        );
    }

    #[test]
    fn point_renders_a_point_of_given_radius() {
        println!("geoPath.point(…) renders a point of a given radius");

        let mut builder: PathBuilder<
			PathString<f64>,
			InterpolateAntimeridian<
				PathString<f64>,
				ResampleNoneNoClipC<PathString<f64>, Equirectangular<PathString<f64>, f64>, f64>,
				f64,
			>,
			LineAntimeridian<Buffer<f64>, Buffer<f64>, Connected<Buffer<f64>>, f64>,
			LineAntimeridian<
				PathString<f64>,
				ResampleNoneNoClipC<PathString<f64>, Equirectangular<PathString<f64>, f64>, f64>,
				Connected<ResampleNoneNoClipC<PathString<f64>, Equirectangular<PathString<f64>, f64>, f64>>,
				f64,
			>,
			LineAntimeridian<
				PathString<f64>,
				ResampleNoneNoClipC<PathString<f64>, Equirectangular<PathString<f64>, f64>, f64>,
				Unconnected,
				f64,
			>,
			Identity<PathString<f64>, PathString<f64>, PathString<f64>, Connected<PathString<f64>>, f64>,
			Identity<PathString<f64>, PathString<f64>, PathString<f64>, Unconnected, f64>,
			Equirectangular<PathString<f64>, f64>,
			PVAntimeridian<f64>,
			ResampleNoneNoClipC<PathString<f64>, Equirectangular<PathString<f64>, f64>, f64>,
			ResampleNoneNoClipU<PathString<f64>, Equirectangular<PathString<f64>, f64>, f64>,
			f64,
		> = PathBuilder::context_pathstring();

        builder.point_radius(10_f64);

        let eq = equirectangular::<f64>();
        let mut path = builder.build(eq);
        let object = Geometry::Point(point!(x: -63_f64, y:18_f64));

        assert_eq!(
            path.object(&object),
            "M165,160m0,10a10,10 0 1,1 0,-20a10,10 0 1,1 0,20z"
        );
    }

    #[test]
    fn renders_multipoint() {
        println!("geoPath(MultiPoint) renders a point");
        let object = Geometry::MultiPoint(
            vec![
                point![x:-63_f64, y:18_f64],
                point![x:-62_f64, y:18_f64],
                point![x:-62_f64, y:17_f64],
            ]
            .into(),
        );
        let eq = equirectangular::<f64>();
        assert_eq!(test_path(eq, object),
			"M165,160m0,4.5a4.5,4.5 0 1,1 0,-9a4.5,4.5 0 1,1 0,9zM170,160m0,4.5a4.5,4.5 0 1,1 0,-9a4.5,4.5 0 1,1 0,9zM170,165m0,4.5a4.5,4.5 0 1,1 0,-9a4.5,4.5 0 1,1 0,9z");
    }

    #[test]
    fn renders_a_line_string() {
        let object = Geometry::LineString(line_string![
            (x:-63_f64, y:18_f64), (x:-62_f64, y:18_f64), (x:-62_f64, y:17_f64)
        ]);
        let eq = equirectangular::<f64>();
        assert_eq!(test_path(eq, object), "M165,160L170,160L170,165");
    }

    #[test]
    fn renders_a_polygon() {
        let object = Geometry::Polygon(Polygon::new(
            line_string![
                (x:-63_f64, y:18_f64), (x:-62_f64, y:18_f64), (x:-62_f64, y:17_f64)
            ],
            vec![],
        ));
        let eq = equirectangular::<f64>();
        assert_eq!(test_path(eq, object), "M165,160L170,160L170,165Z");
    }

    #[test]
    // This has no equivalent in JS testing, looking down the js functions, it is a hole in the test stratergy.
    // the values for everythig after the first z where copied from a modified javascript test.
    fn renders_a_multipolygon() {
        println!("");
        let object = Geometry::MultiPolygon(MultiPolygon(vec![
            Polygon::new(
                line_string![
                    (x:-63_f64, y:18_f64), (x:-62_f64, y:18_f64), (x:-62_f64, y:17_f64)
                ],
                vec![],
            ),
            Polygon::new(
                line_string![
                    (x:0_f64, y:0_f64), (x:0_f64, y:1_f64), (x:1_f64, y:1_f64), (x:0_f64, y:1_f64),(x:0_f64, y:0_f64)
                ],
                vec![],
            ),
        ]));

        let eq = equirectangular::<f64>();
        assert_eq!(
            test_path(eq, object),
            "M165,160L170,160L170,165ZM480,250L480,245L485,245L480,245Z"
        );
    }

    #[test]
    fn render_a_simple_multi_polygon() {
        let gc = CircleGenerator::default().radius(10_f64).precision(80_f64);

        let mut p_vec = vec![];

        let lat = 0;
        for long in (0..=40).step_by(40) {
            let poly = gc
                .clone()
                .center(&Coordinate {
                    x: long as f64,
                    y: lat as f64,
                })
                .circle();
            p_vec.push(poly);
        }
        let object = Geometry::MultiPolygon(MultiPolygon(p_vec));

        let ortho = Orthographic::<PathString<f64>, f64>::builder()
            .scale(240_f64)
            .translate(&Coordinate {
                x: 300_f64,
                y: 300_f64,
            })
            .build();

        let s = PathBuilder::context_pathstring()
            .build(ortho)
            .object(&object);

        assert_eq!(s, "M258.95758280091974,307.23688550569096L285.7461180926677,260.83778132003164L336.0920959633045,279.16221867996836L326.7885352917479,331.9253331742774L273.21146470825204,331.92533317427734ZM420.4850175467511,307.23688550569096L431.56777751410493,283.93695684484305L441.0062261462913,260.83778132003164L479.5734827274837,279.16221867996836L477.116799820893,305.57863634335655L472.44654177381744,331.9253331742774L431.40412457473724,331.92533317427734Z");

    }

    #[test]
    fn renders_a_geometry_collection() {
        let object = Geometry::GeometryCollection(GeometryCollection(vec![Geometry::Polygon(
            Polygon::new(
                line_string![
                    (x:-63_f64, y:18_f64), (x:-62_f64, y:18_f64), (x:-62_f64, y:17_f64)
                ],
                vec![],
            ),
        )]));
        let eq = equirectangular::<f64>();
        assert_eq!(test_path(eq, object), "M165,160L170,160L170,165Z");
    }

    // Missing Feature, FeatureCollection test.

    #[test]
    fn line_string_then_point() {
        println!(
            "geoPath(LineString) then geoPath(Point) does not treat the point as part of a line"
        );
        let line_object = Geometry::LineString(line_string![
            (x:-63_f64, y:18_f64), (x:-62_f64, y:18_f64), (x:-62_f64, y:17_f64)
        ]);
        let point_object = Geometry::Point(point!(x: -63_f64, y:18_f64));
        let eq = equirectangular::<f64>();

        assert_eq!(
            test_path(eq.clone(), line_object),
            "M165,160L170,160L170,165"
        );

        assert_eq!(
            test_path(eq, point_object),
            "M165,160m0,4.5a4.5,4.5 0 1,1 0,-9a4.5,4.5 0 1,1 0,9z"
        );
    }
}
