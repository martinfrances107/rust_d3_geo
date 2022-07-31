#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod reflect_tests {

	extern crate pretty_assertions;

	use geo::Coordinate;
	use pretty_assertions::assert_eq;

	use rust_d3_geo::clip::buffer::Buffer;
	use rust_d3_geo::clip::circle::interpolate::Interpolate as InterpolateCircle;
	use rust_d3_geo::clip::circle::line::Line as LineCircle;
	use rust_d3_geo::clip::circle::pv::PV as PVCircle;
	use rust_d3_geo::identity::Identity;
	use rust_d3_geo::in_delta::in_delta;
	use rust_d3_geo::projection::builder::template::ResampleNoClipC;
	use rust_d3_geo::projection::builder::template::ResampleNoClipU;
	use rust_d3_geo::projection::builder::Builder;
	use rust_d3_geo::projection::builder_mercator::Builder as MercatorBuilder;
	use rust_d3_geo::projection::gnomic::Gnomic;
	use rust_d3_geo::projection::mercator::Mercator;
	use rust_d3_geo::projection::projection_equal::projection_equal;
	use rust_d3_geo::projection::AngleGet;
	use rust_d3_geo::projection::AngleSet;
	use rust_d3_geo::projection::Build;
	use rust_d3_geo::projection::ProjectionRawBase;
	use rust_d3_geo::projection::ReflectGet;
	use rust_d3_geo::projection::ReflectSet;
	use rust_d3_geo::projection::ScaleSet;
	use rust_d3_geo::projection::TranslateSet;
	use rust_d3_geo::stream::Connected;
	use rust_d3_geo::stream::StreamDrainStub;
	use rust_d3_geo::stream::Unconnected;
	use rust_d3_geo::Transform;

	type GB = Builder<
		StreamDrainStub<f64>,
		InterpolateCircle<f64>,
		LineCircle<Buffer<f64>, Connected<Buffer<f64>>, f64>,
		LineCircle<
			ResampleNoClipC<StreamDrainStub<f64>, Gnomic<StreamDrainStub<f64>, f64>, f64>,
			Connected<
				ResampleNoClipC<StreamDrainStub<f64>, Gnomic<StreamDrainStub<f64>, f64>, f64>,
			>,
			f64,
		>,
		LineCircle<
			ResampleNoClipC<StreamDrainStub<f64>, Gnomic<StreamDrainStub<f64>, f64>, f64>,
			Unconnected,
			f64,
		>,
		Identity<StreamDrainStub<f64>, Unconnected>,
		Gnomic<StreamDrainStub<f64>, f64>,
		PVCircle<f64>,
		ResampleNoClipC<StreamDrainStub<f64>, Gnomic<StreamDrainStub<f64>, f64>, f64>,
		ResampleNoClipU<StreamDrainStub<f64>, Gnomic<StreamDrainStub<f64>, f64>, f64>,
		f64,
	>;

	#[test]
	fn test_reflect_x_defaults_to_false() {
		println!("projection.reflectX(…) defaults to false");

		let builder: GB = Gnomic::builder()
			.scale_set(1f64)
			.translate_set(&Coordinate { x: 0_f64, y: 0_f64 });

		assert_eq!(builder.is_x_reflected(), false);
		assert_eq!(builder.is_y_reflected(), false);

		let projection = builder.build();
		assert!(projection_equal(
			&projection,
			&Coordinate { x: 0_f64, y: 0_f64 },
			&Coordinate { x: 0_f64, y: 0_f64 },
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: 10_f64,
				y: 0_f64
			},
			&Coordinate {
				x: 0.17632698070846498_f64,
				y: 0_f64
			},
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: 0_f64,
				y: 10_f64
			},
			&Coordinate {
				x: 0_f64,
				y: -0.17632698070846498_f64
			},
			None
		));
	}

	#[test]
	fn test_reflect_mirrors_x_after_processing() {
		println!("projection.reflectX(…) mirrors x after projecting");
		let mut builder: GB = Gnomic::builder()
			.scale_set(1_f64)
			.translate_set(&Coordinate { x: 0_f64, y: 0_f64 })
			.reflect_x_set(true);

		assert_eq!(builder.is_x_reflected(), true);

		let projection = builder.build();

		assert!(projection_equal(
			&projection,
			&Coordinate { x: 0_f64, y: 0_f64 },
			&Coordinate { x: 0_f64, y: 0_f64 },
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: 10_f64,
				y: 0_f64
			},
			&Coordinate {
				x: -0.17632698070846498_f64,
				y: 0_f64
			},
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: 0_f64,
				y: 10_f64
			},
			&Coordinate {
				x: 0_f64,
				y: -0.17632698070846498_f64
			},
			None
		));

		builder = builder.reflect_x_set(false).reflect_y_set(true);
		let projection = builder.build();
		assert_eq!(builder.is_x_reflected(), false);
		assert_eq!(builder.is_y_reflected(), true);

		assert!(projection_equal(
			&projection,
			&Coordinate { x: 0_f64, y: 0_f64 },
			&Coordinate { x: 0_f64, y: 0_f64 },
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: 10_f64,
				y: 0_f64
			},
			&Coordinate {
				x: 0.17632698070846498_f64,
				y: 0_f64
			},
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: 0_f64,
				y: 10_f64
			},
			&Coordinate {
				x: 0_f64,
				y: 0.17632698070846498_f64
			},
			None
		));
	}

	#[test]
	fn reflect_x_works_with_projection_angle() {
		println!("projection.reflectX(…) works with projection.angle()");
		let builder: MercatorBuilder<StreamDrainStub<f32>, _, _, _, _, _, _, _, _, _, f32> =
			Mercator::builder()
				.scale_set(1_f32)
				.translate_set(&Coordinate {
					x: 10_f32,
					y: 20_f32,
				})
				.reflect_x_set(true)
				.angle_set(45_f32);

		assert_eq!(builder.is_x_reflected(), true);
		assert!(in_delta(45_f32, builder.angle(), 1e-6));
		let p = builder.build();
		assert_eq!(
			p.transform(&Coordinate { x: 0_f32, y: 0_f32 }),
			Coordinate {
				x: 10_f32,
				y: 20_f32
			}
		);
		assert_eq!(
			p.transform(&Coordinate {
				x: 10_f32,
				y: 0_f32
			}),
			Coordinate {
				x: 9.87658658_f32,
				y: 20.12341341_f32
			}
		);
		assert_eq!(
			p.transform(&Coordinate {
				x: 0_f32,
				y: 10_f32
			}),
			Coordinate {
				x: 9.87595521_f32,
				y: 19.87595521_f32
			}
		);
	}
}
