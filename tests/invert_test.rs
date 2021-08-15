#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod invert_test {

    use geo::Coordinate;
    use rust_d3_geo::clip::antimeridian::line::Line;
    use rust_d3_geo::clip::antimeridian::pv::PV;
    use rust_d3_geo::projection::azimuthal_equal_area::AzimuthalEqualArea;
    use rust_d3_geo::projection::equirectangular::EquirectangularRaw;
    use rust_d3_geo::projection::gnomic::Gnomic;
    use rust_d3_geo::projection::mecator::Mecator;
    use rust_d3_geo::projection::orthographic::Orthographic;
    use rust_d3_geo::projection::projection::Projection;
    use rust_d3_geo::projection::projection_equal::projection_equal;
    use rust_d3_geo::projection::stereographic::Stereographic;
    use rust_d3_geo::stream::StreamDrainStub;
    use rust_d3_geo::Transform;

    fn symetric_invert<'a, PM>(pm: PM)
    where
        PM: Transform<C = Coordinate<f64>>,
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
        let a: Projection<StreamDrainStub<f64>, Line<f64>, AzimuthalEqualArea<f64>, PV<f64>, f64> =
            AzimuthalEqualArea::<f64>::gen_projection_builder().build();
        symetric_invert(a);
    }

    #[test]
    fn test_equirectangular() {
        let e: Projection<StreamDrainStub<f64>, Line<f64>, EquirectangularRaw<f64>, PV<f64>, f64> =
            EquirectangularRaw::<f64>::gen_projection_builder().build();
        symetric_invert(e);
    }

    #[test]
    fn test_gnomic() {
        let g: Projection<StreamDrainStub<f64>, Line<f64>, Gnomic<f64>, PV<f64>, f64> =
            Gnomic::<f64>::gen_projection_builder().build();
        symetric_invert(g);
    }

    #[test]
    fn test_orthographic() {
        let o: Projection<StreamDrainStub<f64>, Line<f64>, Orthographic<f64>, PV<f64>, f64> =
            Orthographic::<f64>::gen_projection_builder().build();
        symetric_invert(o);
    }

    #[test]
    fn test_mecator() {
        let m: Projection<StreamDrainStub<f64>, Line<f64>, Mecator<f64>, PV<f64>, f64> =
            Mecator::<f64>::gen_projection_mutator().build();
        symetric_invert(m);
    }

    #[test]
    fn test_stereographic() {
        let s: Projection<StreamDrainStub<f64>, Line<f64>, Stereographic<f64>, PV<f64>, f64> =
            Stereographic::<f64>::gen_projection_builder().build();
        symetric_invert(s);
    }
}
