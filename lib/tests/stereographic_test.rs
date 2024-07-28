use d3_geo_rs::projection::ScaleSet;
use geo_types::Coord;

use d3_geo_rs::projection::equality::projection_equal;
use d3_geo_rs::projection::stereographic::Stereographic;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::RawBase;
use d3_geo_rs::projection::TranslateSet;
use d3_geo_rs::stream::DrainStub;

#[test]
fn stereographic() {
    let stereo = Stereographic::builder::<DrainStub<f64>>()
        .translate_set(&Coord { x: 0f64, y: 0f64 })
        .scale_set(1f64)
        .build();

    assert!(projection_equal(
        &stereo,
        &Coord { x: 0_f64, y: 0_f64 },
        &Coord { x: 0_f64, y: 0_f64 },
        None
    ));
    assert!(projection_equal(
        &stereo,
        &Coord {
            x: -90_f64,
            y: 0_f64
        },
        &Coord {
            x: -1_f64,
            y: 0_f64
        },
        None
    ));
    assert!(projection_equal(
        &stereo,
        &Coord {
            x: 90_f64,
            y: 0_f64
        },
        &Coord { x: 1_f64, y: 0_f64 },
        None
    ));
    assert!(projection_equal(
        &stereo,
        &Coord {
            x: 0_f64,
            y: -90_f64
        },
        &Coord { x: 0_f64, y: 1_f64 },
        None
    ));
    assert!(projection_equal(
        &stereo,
        &Coord {
            x: 0_f64,
            y: 90_f64
        },
        &Coord {
            x: 0_f64,
            y: -1_f64
        },
        None
    ));
}
