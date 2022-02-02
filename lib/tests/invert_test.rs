#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod invert_test {

    use geo::Coordinate;

    use rust_d3_geo::clip::antimeridian::line::Line;
    use rust_d3_geo::clip::antimeridian::pv::PV;
    use rust_d3_geo::clip::circle::line::Line as LineCircle;
    use rust_d3_geo::clip::circle::pv::PV as PVCircle;
    use rust_d3_geo::projection::azimuthal_equal_area::AzimuthalEqualArea;
    use rust_d3_geo::projection::azimuthal_equidistant::AzimuthalEquiDistant;
    use rust_d3_geo::projection::equirectangular::Equirectangular;
    use rust_d3_geo::projection::gnomic::Gnomic;
    use rust_d3_geo::projection::mercator::Mercator;
    use rust_d3_geo::projection::orthographic::Orthographic;
    use rust_d3_geo::projection::projection_equal::projection_equal;
    use rust_d3_geo::projection::projector::Projector;
    use rust_d3_geo::projection::stereographic::Stereographic;
    use rust_d3_geo::projection::Raw;
    use rust_d3_geo::stream::StreamDrainStub;
    use rust_d3_geo::Transform;

    fn symetric_invert<'a, PM>(pm: PM)
    where
        PM: Transform<T = f64>,
    {
        for p in vec![
            &Coordinate {
                x: 0.0f64,
                y: 0.0f64,
            },
            &Coordinate {
                x: 30.3f64,
                y: 24.1f64,
            },
            &Coordinate {
                x: -10f64,
                y: 42f64,
            },
            &Coordinate {
                x: -2.0f64,
                y: -5.0f64,
            },
        ] {
            assert!(projection_equal(&pm, &p, &pm.transform(&p), None));
        }
    }

    #[test]
    fn azimuthal_equal_area() {
        let a = AzimuthalEqualArea::<StreamDrainStub<f64>, f64>::builder().build();
        symetric_invert(a);
    }

    #[test]
    fn azimuthal_equidistant() {
        let a = AzimuthalEquiDistant::<StreamDrainStub<f64>, f64>::builder().build();
        symetric_invert(a);
    }

    #[test]
    fn equirectangular() {
        let e: Projector<
            StreamDrainStub<f64>,
            Line<f64>,
            Equirectangular<StreamDrainStub<f64>, f64>,
            PV<f64>,
            f64,
        > = Equirectangular::<StreamDrainStub<f64>, f64>::builder().build();
        symetric_invert(e);
    }

    #[test]
    fn gnomic() {
        let g: Projector<
            StreamDrainStub<f64>,
            LineCircle<f64>,
            Gnomic<StreamDrainStub<f64>, f64>,
            PVCircle<f64>,
            f64,
        > = Gnomic::<StreamDrainStub<f64>, f64>::builder().build();
        symetric_invert(g);
    }

    #[test]
    fn orthographic() {
        let o: Projector<
            StreamDrainStub<f64>,
            LineCircle<f64>,
            Orthographic<StreamDrainStub<f64>, f64>,
            PVCircle<f64>,
            f64,
        > = Orthographic::<StreamDrainStub<f64>, f64>::builder().build();
        symetric_invert(o);
    }

    #[test]
    fn mercator() {
        let m: Projector<
            StreamDrainStub<f64>,
            Line<f64>,
            Mercator<StreamDrainStub<f64>, f64>,
            PV<f64>,
            f64,
        > = Mercator::<StreamDrainStub<f64>, f64>::builder().build();
        symetric_invert(m);
    }

    #[test]
    fn stereographic() {
        let s: Projector<
            StreamDrainStub<f64>,
            LineCircle<f64>,
            Stereographic<StreamDrainStub<f64>, f64>,
            PVCircle<f64>,
            f64,
        > = Stereographic::<StreamDrainStub<f64>, f64>::builder().build();
        symetric_invert(s);
    }
}
