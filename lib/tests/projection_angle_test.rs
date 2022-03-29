#[cfg(not(tarpaulin_include))]
#[cfg(test)]

mod angle_test {

	use geo::Coordinate;
	use pretty_assertions::assert_eq;

	use rust_d3_geo::in_delta::in_delta;
	use rust_d3_geo::projection::gnomic::Gnomic;
	use rust_d3_geo::projection::projection_equal::projection_equal;
	use rust_d3_geo::projection::AngleGet;
	use rust_d3_geo::projection::AngleSet;
	use rust_d3_geo::projection::ProjectionRawBase;
	use rust_d3_geo::projection::ScaleSet;
	use rust_d3_geo::projection::TranslateSet;
	use rust_d3_geo::stream::StreamDrainStub;

	#[test]
	fn angle_defaults_to_zero() {
		println!("projection.angle(…) defaults to zero");
		let pb = Gnomic::<StreamDrainStub<f64>, f64>::builder()
			.scale(1_f64)
			.translate(&Coordinate { x: 0_f64, y: 0_f64 });
		assert_eq!(pb.get_angle(), 0_f64);
		let projection = pb.build();

		assert!(projection_equal(
			&projection,
			&Coordinate { x: 0f64, y: 0f64 },
			&Coordinate { x: 0f64, y: 0f64 },
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
				x: -10_f64,
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
		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: 0_f64,
				y: -10_f64
			},
			&Coordinate {
				x: 0_f64,
				y: 0.17632698070846498_f64
			},
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: 10_f64,
				y: 10_f64
			},
			&Coordinate {
				x: 0.17632698070846495_f64,
				y: -0.17904710860483972_f64
			},
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: 10_f64,
				y: -10_f64
			},
			&Coordinate {
				x: 0.17632698070846495_f64,
				y: 0.17904710860483972_f64
			},
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: -10_f64,
				y: 10_f64
			},
			&Coordinate {
				x: -0.17632698070846495_f64,
				y: -0.17904710860483972_f64
			},
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: -10_f64,
				y: -10_f64
			},
			&Coordinate {
				x: -0.17632698070846495_f64,
				y: 0.17904710860483972_f64
			},
			None
		));
	}

	#[test]
	fn angle_rotates_by_plus_30() {
		println!("projection.angle(…) defaults to zero");
		let pb = Gnomic::<StreamDrainStub<f64>, f64>::builder()
			.scale(1_f64)
			.translate(&Coordinate { x: 0_f64, y: 0_f64 })
			.angle(30_f64);

		// this rounds to 29.9999999 not 30!!
		// assert_eq!(pb.get_angle(), 30_f64);
		assert!(in_delta(pb.get_angle(), 30_f64, 1e-6));
		let projection = pb.build();

		assert!(projection_equal(
			&projection,
			&Coordinate { x: 0f64, y: 0f64 },
			&Coordinate { x: 0f64, y: 0f64 },
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: 10_f64,
				y: 0_f64
			},
			&Coordinate {
				x: 0.1527036446661393_f64,
				y: -0.08816349035423247_f64
			},
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: -10_f64,
				y: 0_f64
			},
			&Coordinate {
				x: -0.1527036446661393_f64,
				y: 0.08816349035423247_f64
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
				x: -0.08816349035423247_f64,
				y: -0.1527036446661393_f64
			},
			None
		));
		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: 0_f64,
				y: -10_f64
			},
			&Coordinate {
				x: 0.08816349035423247_f64,
				y: 0.1527036446661393_f64
			},
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: 10_f64,
				y: 10_f64
			},
			&Coordinate {
				x: 0.06318009036371944_f64,
				y: -0.24322283488017502_f64
			},
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: 10_f64,
				y: -10_f64
			},
			&Coordinate {
				x: 0.24222719896855913_f64,
				y: 0.0668958541717101
			},
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: -10_f64,
				y: 10_f64
			},
			&Coordinate {
				x: -0.24222719896855913_f64,
				y: -0.0668958541717101
			},
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: -10_f64,
				y: -10_f64
			},
			&Coordinate {
				x: -0.06318009036371944_f64,
				y: 0.24322283488017502
			},
			None
		));
	}

	#[test]
	fn angle_rotates_by_minus_30() {
		println!("projection.angle(…) defaults to zero");
		let pb = Gnomic::<StreamDrainStub<f64>, f64>::builder()
			.scale(1_f64)
			.translate(&Coordinate { x: 0_f64, y: 0_f64 })
			.angle(-30_f64);

		// this rounds to 29.9999999 not 30!!
		assert!(in_delta(pb.get_angle(), -30_f64, 1e-6));
		let projection = pb.build();

		assert!(projection_equal(
			&projection,
			&Coordinate { x: 0f64, y: 0f64 },
			&Coordinate { x: 0f64, y: 0f64 },
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: 10_f64,
				y: 0_f64
			},
			&Coordinate {
				x: 0.1527036446661393_f64,
				y: 0.08816349035423247_f64
			},
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: -10_f64,
				y: 0_f64
			},
			&Coordinate {
				x: -0.1527036446661393_f64,
				y: -0.08816349035423247_f64
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
				x: 0.08816349035423247_f64,
				y: -0.1527036446661393_f64
			},
			None
		));
		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: 0_f64,
				y: -10_f64
			},
			&Coordinate {
				x: -0.08816349035423247_f64,
				y: 0.1527036446661393_f64
			},
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: 10_f64,
				y: 10_f64
			},
			&Coordinate {
				x: 0.24222719896855913_f64,
				y: -0.0668958541717101_f64
			},
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: 10_f64,
				y: -10_f64
			},
			&Coordinate {
				x: 0.06318009036371944_f64,
				y: 0.24322283488017502_f64,
			},
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: -10_f64,
				y: 10_f64
			},
			&Coordinate {
				x: -0.06318009036371944_f64,
				y: -0.24322283488017502_f64
			},
			None
		));

		assert!(projection_equal(
			&projection,
			&Coordinate {
				x: -10_f64,
				y: -10_f64
			},
			&Coordinate {
				x: -0.24222719896855913_f64,
				y: 0.0668958541717101_f64
			},
			None
		));
	}

	#[test]
	fn wraps_360() {
		println!("projection.angle(…) wraps around 360°");
		let pb = Gnomic::<StreamDrainStub<f64>, f64>::builder()
			.scale(1_f64)
			.translate(&Coordinate { x: 0_f64, y: 0_f64 })
			.angle(360_f64);

		assert!(in_delta(pb.get_angle(), 0_f64, 1e-6));
	}
	// TODO add geoIdentity test
}
