#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod path_area_test {

	use std::f64::consts::PI;
	use std::fmt::Display;
	use std::ops::AddAssign;

	use approx::AbsDiffEq;
	use geo::CoordFloat;
	use geo::Coordinate;
	use geo::Geometry;
	use geo::LineString;
	use geo::Polygon;
	use num_traits::AsPrimitive;
	use num_traits::FloatConst;
	use pretty_assertions::assert_eq;

	use rust_d3_geo::clip::antimeridian::interpolate::Interpolate as InterpolateAntimeridian;
	use rust_d3_geo::clip::antimeridian::line::Line as LineAntimeridian;
	use rust_d3_geo::clip::antimeridian::pv::PV as PVAntimeridian;
	use rust_d3_geo::clip::buffer::Buffer;
	use rust_d3_geo::data_object::sphere::Sphere;
	use rust_d3_geo::path::area::Area;
	use rust_d3_geo::path::builder::Builder as PathBuilder;
	use rust_d3_geo::projection::builder::template::NoClipC;
	use rust_d3_geo::projection::builder::template::NoClipU;
	use rust_d3_geo::projection::builder::template::ResampleNoneNoClipC;
	use rust_d3_geo::projection::builder::template::ResampleNoneNoClipU;
	use rust_d3_geo::projection::builder::types::BuilderAntimeridianResampleNoClip;
	use rust_d3_geo::projection::equirectangular::Equirectangular;
	use rust_d3_geo::projection::projector::types::ProjectorAntimeridianResampleNoneNoClip;
	use rust_d3_geo::projection::PrecisionBypass;
	use rust_d3_geo::projection::ProjectionRawBase;
	use rust_d3_geo::projection::ScaleAdjust;
	use rust_d3_geo::stream::Connected;
	use rust_d3_geo::stream::Streamable;
	use rust_d3_geo::stream::Unconnected;

	#[inline]
	fn equirectangular<T>(
	) -> ProjectorAntimeridianResampleNoneNoClip<Area<T>, Equirectangular<Area<T>, T>, T>
	where
		T: AbsDiffEq<Epsilon = T> + AsPrimitive<T> + CoordFloat + Display + FloatConst,
	{
		let ba: BuilderAntimeridianResampleNoClip<Area<T>, Equirectangular<Area<T>, T>, T> =
			Equirectangular::<Area<T>, T>::builder().scale(T::from(900f64 / PI).unwrap());

		let builder = ba.precision_bypass();
		let out = builder.build();

		out
	}

	#[inline]
	fn test_area<'a, T>(
		projection: ProjectorAntimeridianResampleNoneNoClip<
			Area<T>,
			Equirectangular<Area<T>, T>,
			T,
		>,
		object: impl Streamable<T = T>,
	) -> T
	where
		T: AbsDiffEq<Epsilon = T> + AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
	{
		let builder: PathBuilder<
			Area<T>,
			InterpolateAntimeridian<T>,
			LineAntimeridian<Buffer<T>, Buffer<T>, Connected<Buffer<T>>, T>,
			LineAntimeridian<
				Area<T>,
				ResampleNoneNoClipC<Area<T>, Equirectangular<Area<T>, T>, T>,
				Connected<ResampleNoneNoClipC<Area<T>, Equirectangular<Area<T>, T>, T>>,
				T,
			>,
			LineAntimeridian<
				Area<T>,
				ResampleNoneNoClipC<Area<T>, Equirectangular<Area<T>, T>, T>,
				Unconnected,
				T,
			>,
			NoClipC<Area<T>, T>,
			NoClipU<Area<T>, T>,
			Equirectangular<Area<T>, T>,
			PVAntimeridian<T>,
			ResampleNoneNoClipC<Area<T>, Equirectangular<Area<T>, T>, T>,
			ResampleNoneNoClipU<Area<T>, Equirectangular<Area<T>, T>, T>,
			T,
		> = PathBuilder::new(Area::default());
		builder.build(projection).area(&object)
	}

	#[test]
	fn polygon_with_no_holes() {
		println!("geoPath.area(…) of a polygon with no holes");
		let object = Geometry::Polygon(Polygon::new(
			LineString::from(vec![
				Coordinate { x: 100_f64, y: 0. },
				Coordinate { x: 100., y: 1. }, //  [101, 1], [101, 0], [100, 0]
				Coordinate { x: 101., y: 1. },
				Coordinate { x: 101., y: 0. },
				Coordinate { x: 100., y: 0. },
			]),
			vec![],
		));
		let eq = equirectangular::<f64>();
		assert_eq!(test_area(eq, object), 25_f64);
	}

	#[test]
	fn polygon_with_holes() {
		println!("geoPath.area(…) of a polygon with holes");
		let object = Geometry::Polygon(Polygon::new(
			LineString::from(vec![
				Coordinate { x: 100_f64, y: 0. },
				Coordinate { x: 100., y: 1. }, //  [101, 1], [101, 0], [100, 0]
				Coordinate { x: 101., y: 1. },
				Coordinate { x: 101., y: 0. },
				Coordinate { x: 100., y: 0. },
			]),
			vec![
				// [100.2, 0.2], [100.8, 0.2], [100.8, 0.8], [100.2, 0.8], [100.2, 0.2]
				LineString::from(vec![
					Coordinate { x: 100.2, y: 0.2 },
					Coordinate { x: 100.8, y: 0.2 },
					Coordinate { x: 100.8, y: 0.8 },
					Coordinate { x: 100.2, y: 0.8 },
					Coordinate { x: 100.2, y: 0.2 },
				]),
			],
		));
		let eq = equirectangular::<f64>();
		assert_eq!(test_area(eq, object), 16_f64);
	}

	#[test]
	fn area_of_a_sphere() {
		println!("geoPath.area(…) of a sphere");
		let eq = equirectangular::<f64>();
		let object = Sphere::default();
		assert_eq!(test_area(eq, object), 1620000_f64);
	}
}
