#[cfg(not(tarpaulin_include))]
mod projection_rotate {

    use geo::Geometry;
    use geo::LineString;
    use geo::Polygon;
    use geo_types::Coord;
    use pretty_assertions::assert_eq;

    use d3_geo_rs::path::builder::Builder as PathBuilder;
    use d3_geo_rs::projection::mercator::Mercator;
    use d3_geo_rs::projection::Build;
    use d3_geo_rs::projection::RawBase;
    use d3_geo_rs::projection::RotateSet;
    use d3_geo_rs::projection::ScaleSet;
    use d3_geo_rs::projection::TranslateSet;

    #[test]
    fn degenerate_polygon_should_not_break() {
        println!("a rotation of a degenerate polygon should not break");
        let projection = Mercator::builder()
            .rotate2_set(&[-134.3_f64, 25.776_f64])
            .scale_set(750_f64)
            .translate_set(&Coord { x: 0_f64, y: 0_f64 })
            .build();

        let path_builder = PathBuilder::pathstring();

        let object = Geometry::Polygon(Polygon::new(
            LineString::from(vec![
                Coord {
                    x: 125.67351590459046,
                    y: -14.17673705310531,
                },
                Coord {
                    x: 125.67351590459046,
                    y: -14.173276873687367,
                },
                Coord {
                    x: 125.67351590459046,
                    y: -14.173276873687367,
                },
                Coord {
                    x: 125.67351590459046,
                    y: -14.169816694269425,
                },
                Coord {
                    x: 125.67351590459046,
                    y: -14.17673705310531,
                },
            ]),
            vec![],
        ));

        let s = path_builder.build(projection).object(&object);
        assert_eq!(s, "M-111.644162,-149.157654L-111.647235,-149.203744L-111.647235,-149.203744L-111.650307,-149.249835Z");
    }
}
