extern crate pretty_assertions;

use geo::polygon;
use geo::Geometry;
use geo_types::Coord;
use pretty_assertions::assert_eq;
use regex::Regex;

use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::projection::azimuthal_equal_area::AzimuthalEqualArea;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::ClipAngleAdjust;
use d3_geo_rs::projection::RawBase;
use d3_geo_rs::projection::RotateSet;
use d3_geo_rs::projection::TranslateSet;

#[test]
fn projection_clip_angle_degenerate_polygons() {
    println!("projection.clipAngle() deals with degenerate polygons");

    let round_down = Regex::new(r"\.\d+").unwrap();

    let poly = polygon![(x: -120., y:-30.),(x:0., y:-30.),(x:0., y:-90.),(x:0., y:-30.),(x:120., y:-30.),(x:-120., y:-30.)];
    let d = Geometry::Polygon(poly);

    let projector = AzimuthalEqualArea::builder()
        .translate_set(&Coord {
            x: 0.5_f64,
            y: 0.5_f64,
        })
        .rotate2_set(&[0_f64, -90_f64])
        .clip_angle(170_f64)
        .build();

    let path_builder = PathBuilder::pathstring();

    let s = path_builder.build(projector).object(&d);
    let rounded = round_down.replace_all(&s, "");
    assert_eq!(rounded, "M0,249L0,238L0,216L21,219L45,219L71,215L98,207L127,193L141,184L155,173L168,161L181,148L192,133L202,117L211,100L218,83L224,65L228,48L230,30L231,13L229,-17L222,-45L212,-70L200,-90L187,-107L179,-127L167,-147L151,-168L130,-188L104,-206L89,-213L73,-220L55,-225L37,-229L19,-232L0,-233L-18,-232L-36,-229L-54,-225L-72,-220L-88,-213L-103,-206L-129,-188L-150,-168L-166,-147L-178,-127L-186,-107L-186,-107L-199,-90L-211,-70L-221,-45L-228,-17L-230,13L-229,30L-227,48L-223,65L-217,83L-210,100L-201,117L-191,133L-180,148L-167,161L-154,173L-140,184L-126,193L-97,207L-70,215L-44,219L-20,219L0,216L0,238L0,249L0,249L-25,247L-51,243L-76,236L-100,227L-123,215L-145,201L-165,185L-184,166L-200,146L-214,124L-226,101L-235,77L-242,52L-246,26L-248,0L-246,-25L-242,-51L-235,-76L-226,-100L-214,-123L-200,-145L-184,-165L-165,-184L-145,-200L-123,-214L-100,-226L-76,-235L-51,-242L-25,-246L0,-248L26,-246L52,-242L77,-235L101,-226L124,-214L146,-200L166,-184L185,-165L201,-145L215,-123L227,-100L236,-76L243,-51L247,-25L249,0L247,26L243,52L236,77L227,101L215,124L201,146L185,166L166,185L146,201L124,215L101,227L77,236L52,243L26,247Z")
}
