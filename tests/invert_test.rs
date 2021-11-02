#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod invert_test {

    use geo::Coordinate;

    use rust_d3_geo::clip::antimeridian::pv::PV;
    use rust_d3_geo::clip::circle::pv::PV as PVCircle;
    use rust_d3_geo::projection::azimuthal_equal_area::AzimuthalEqualArea;
    use rust_d3_geo::projection::equirectangular::EquirectangularRaw;
    use rust_d3_geo::projection::gnomic::Gnomic;
    use rust_d3_geo::projection::mercator::Mercator;
    use rust_d3_geo::projection::orthographic::Orthographic;
    use rust_d3_geo::projection::projection::Projection;
    use rust_d3_geo::projection::projection_equal::projection_equal;
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
    fn test_azimuthal_equal_area() {
        let a = AzimuthalEqualArea::<StreamDrainStub<f64>, f64>::builder().build();
        symetric_invert(a);
    }

    #[test]
    fn test_equirectangular() {
        let e: Projection<
            StreamDrainStub<f64>,
            EquirectangularRaw<StreamDrainStub<f64>, f64>,
            PV<f64>,
            f64,
        > = EquirectangularRaw::<StreamDrainStub<f64>, f64>::builder().build();
        symetric_invert(e);
    }

    #[test]
    fn test_gnomic() {
        let g: Projection<
            StreamDrainStub<f64>,
            Gnomic<StreamDrainStub<f64>, f64>,
            PVCircle<f64>,
            f64,
        > = Gnomic::<StreamDrainStub<f64>, f64>::builder().build();
        symetric_invert(g);
    }

    #[test]
    fn test_orthographic() {
        let o: Projection<
            StreamDrainStub<f64>,
            Orthographic<StreamDrainStub<f64>, f64>,
            PVCircle<f64>,
            f64,
        > = Orthographic::<StreamDrainStub<f64>, f64>::builder().build();
        symetric_invert(o);
    }

    #[test]
    fn test_mercator() {
        let m: Projection<StreamDrainStub<f64>, Mercator<StreamDrainStub<f64>, f64>, PV<f64>, f64> =
            Mercator::<StreamDrainStub<f64>, f64>::builder().build();
        symetric_invert(m);
    }

    #[test]
    fn test_stereographic() {
        let s: Projection<
            StreamDrainStub<f64>,
            Stereographic<StreamDrainStub<f64>, f64>,
            PVCircle<f64>,
            f64,
        > = Stereographic::<StreamDrainStub<f64>, f64>::builder().build();
        symetric_invert(s);
    }
}
