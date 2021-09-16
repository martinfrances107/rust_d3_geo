// #[cfg(not(tarpaulin_include))]
// #[cfg(test)]
// mod path_area_test {
// 	#[cfg(not(tarpaulin_include))]
// 	#[cfg(test)]
// 	mod path_string_test {

// 		use rust_d3_geo::stream::Stream;
// 		use std::f64::consts::PI;
// 		use std::fmt::Display;
// 		use std::ops::AddAssign;

// 		use geo::CoordFloat;
// 		use geo::Coordinate;
// 		use geo::Geometry;
// 		use geo::LineString;
// 		use geo::Polygon;
// 		use num_traits::AsPrimitive;
// 		use num_traits::FloatConst;

// 		use rust_d3_geo::clip::antimeridian::line::Line;
// 		use rust_d3_geo::clip::antimeridian::pv::PV;
// 		use rust_d3_geo::data_object::sphere::Sphere;
// 		use rust_d3_geo::data_object::DataObject;
// 		use rust_d3_geo::path::builder::Builder as PathBuilder;
// 		use rust_d3_geo::path::context_stream::ContextStream;
// 		use rust_d3_geo::path::ResultEnum;
// 		use rust_d3_geo::projection::equirectangular::EquirectangularRaw;
// 		use rust_d3_geo::projection::projection::Projection;
// 		use rust_d3_geo::projection::Precision;
// 		use rust_d3_geo::projection::Raw;
// 		use rust_d3_geo::projection::Scale;

// 		#[inline]
// 		fn equirectangular<DRAIN, T>(
// 		) -> Projection<DRAIN, Line<T>, EquirectangularRaw<DRAIN, T>, PV<T>, T>
// 		where
// 			DRAIN: Stream<T = T> + Default,
// 			T: AsPrimitive<T> + CoordFloat + Display + FloatConst,
// 		{
// 			EquirectangularRaw::builder()
// 				.scale(T::from(900f64 / PI).unwrap())
// 				.precision(&T::zero())
// 				.build()
// 		}

// 		#[inline]
// 		fn test_path<'a, DRAIN, T>(
// 			projection: Projection<
// 				ContextStream<T>,
// 				Line<T>,
// 				EquirectangularRaw<DRAIN, T>,
// 				PV<T>,
// 				T,
// 			>,
// 			object: DataObject<T>,
// 		) -> T
// 		where
// 			DRAIN: Stream<T = T>,
// 			T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
// 		{
// 			let builder = PathBuilder::context_pathstring();
// 			let string = builder.build(projection).string(&object);
// 			match string {
// 				Some(p) => match p {
// 					ResultEnum::String(a) => return a,
// 					_ => panic!("Expecting a string."),
// 				},
// 				None => {
// 					panic!("Expecting an string result.");
// 				}
// 			}
// 		}

// 		#[test]
// 		fn test_point_renders_a_point() {
// 			println!("geoPath.point(…) renders a point");
// 			let object = DataObject::Geometry(Geometry::Point(
// 				point!()
// 			));
// 			let eq = equirectangular::<ContextStream<f64>, f64>();
// 			assert_eq!(test_area(eq, object), 25.0);
// 		}

// 	}
