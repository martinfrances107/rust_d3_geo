use core::f64::consts::PI;

use geo::CoordFloat;
use geo::Geometry;
use geo::LineString;
use geo::Polygon;
use geo_types::Coord;

use num_traits::FloatConst;
use pretty_assertions::assert_eq;

use d3_geo_rs::data_object::sphere::Sphere;
use d3_geo_rs::path::area::Area;
use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::projection::equirectangular::Equirectangular;
use d3_geo_rs::projection::projector_common::types::ProjectorAntimeridianResampleNoneNoClip;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::PrecisionBypass;
use d3_geo_rs::projection::RawBase;
use d3_geo_rs::projection::ScaleSet;
use d3_geo_rs::stream::Streamable;

#[inline]
fn projector<T>(
) -> ProjectorAntimeridianResampleNoneNoClip<Area<T>, Equirectangular<T>, T>
where
    T: 'static + CoordFloat + Default + FloatConst,
{
    let mut ba = Equirectangular::<T>::builder();
    ba.scale_set(T::from(900f64 / PI).unwrap());

    let builder = ba.precision_bypass();
    builder.build()
}

#[inline]
fn area<T>(
    projection: ProjectorAntimeridianResampleNoneNoClip<
        Area<T>,
        Equirectangular<T>,
        T,
    >,
    object: impl Streamable<T = T>,
) -> T
where
    T: 'static + CoordFloat + FloatConst,
{
    let builder = PathBuilder::new(Area::default());

    builder.build(projection).area(&object)
}

#[test]
fn polygon_with_no_holes() {
    println!("geoPath.area(…) of a polygon with no holes");
    let object = Geometry::Polygon(Polygon::new(
        LineString::from(vec![
            Coord { x: 100_f64, y: 0. },
            Coord { x: 100., y: 1. }, //  [101, 1], [101, 0], [100, 0]
            Coord { x: 101., y: 1. },
            Coord { x: 101., y: 0. },
            Coord { x: 100., y: 0. },
        ]),
        vec![],
    ));
    let eq = projector();
    assert_eq!(area(eq, object), 25_f64);
}

#[test]
fn polygon_with_holes() {
    println!("geoPath.area(…) of a polygon with holes");
    let object = Geometry::Polygon(Polygon::new(
        LineString::from(vec![
            Coord { x: 100_f64, y: 0. },
            Coord { x: 100., y: 1. }, //  [101, 1], [101, 0], [100, 0]
            Coord { x: 101., y: 1. },
            Coord { x: 101., y: 0. },
            Coord { x: 100., y: 0. },
        ]),
        vec![LineString::from(vec![
            Coord { x: 100.2, y: 0.2 },
            Coord { x: 100.8, y: 0.2 },
            Coord { x: 100.8, y: 0.8 },
            Coord { x: 100.2, y: 0.8 },
            Coord { x: 100.2, y: 0.2 },
        ])],
    ));
    let eq = projector();
    assert_eq!(area(eq, object), 16_f64);
}

#[test]
fn area_of_a_sphere() {
    println!("geoPath.area(…) of a sphere");
    let eq = projector::<f64>();
    let object = Sphere::default();
    assert_eq!(area(eq, object), 1620000_f64);
}
