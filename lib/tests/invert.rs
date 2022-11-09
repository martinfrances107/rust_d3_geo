#[cfg(not(tarpaulin_include))]
mod invert {

    use geo::Coordinate;

    use rust_d3_geo::projection::azimuthal_equal_area::AzimuthalEqualArea;
    use rust_d3_geo::projection::azimuthal_equidistant::AzimuthalEquiDistant;
    use rust_d3_geo::projection::conic_equal_area::ConicEqualArea;
    use rust_d3_geo::projection::equirectangular::Equirectangular;
    use rust_d3_geo::projection::gnomic::Gnomic;
    use rust_d3_geo::projection::mercator::Mercator;
    use rust_d3_geo::projection::mercator_transverse::MercatorTransverse;
    use rust_d3_geo::projection::orthographic::Orthographic;
    use rust_d3_geo::projection::projection_equal::projection_equal;
    use rust_d3_geo::projection::stereographic::Stereographic;
    use rust_d3_geo::projection::Build;
    use rust_d3_geo::projection::ProjectionRawBase;
    use rust_d3_geo::stream::StreamDrainStub;
    use rust_d3_geo::Transform;

    fn symetric_invert<'a, PM>(pm: PM)
    where
        PM: Transform<T = f64>,
    {
        for p in [
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
            assert!(projection_equal(&pm, p, &pm.transform(p), None));
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
    fn conic_equal_area() {
        let c = ConicEqualArea::<StreamDrainStub<f64>, f64>::builder().build();
        symetric_invert(c);
    }

    #[test]
    fn equirectangular() {
        let e = Equirectangular::<StreamDrainStub<f64>, f64>::builder().build();
        symetric_invert(e);
    }

    #[test]
    fn gnomic() {
        let g = Gnomic::<StreamDrainStub<f64>, f64>::builder().build();
        symetric_invert(g);
    }

    #[test]
    fn orthographic() {
        let o = Orthographic::<StreamDrainStub<f64>, f64>::builder().build();
        symetric_invert(o);
    }

    #[test]
    fn mercator() {
        let m = Mercator::<StreamDrainStub<f64>>::builder().build();
        symetric_invert(m);
    }

    #[test]
    fn mercator_traverse() {
        let m = MercatorTransverse::<StreamDrainStub<f64>>::builder().build();
        symetric_invert(m);
    }

    #[test]
    fn stereographic() {
        let s = Stereographic::<StreamDrainStub<f64>, f64>::builder().build();
        symetric_invert(s);
    }
}
