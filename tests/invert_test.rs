// #[cfg(not(tarpaulin_include))]
// #[cfg(test)]
// mod invert_test {

//     use geo::Coordinate;
//     use rust_d3_geo::projection::azimuthal_equal_area::AzimuthalEqualAreaRaw;
//     use rust_d3_geo::projection::equirectangular::EquirectangularRaw;
//     use rust_d3_geo::projection::gnomic::GnomicRaw;
//     use rust_d3_geo::projection::mecator::MecatorRaw;
//     use rust_d3_geo::projection::projection_trait::ProjectionTrait;
//     use rust_d3_geo::projection::orthographic::OrthographicRaw;
//     use rust_d3_geo::projection::projection_equal::projection_equal;
//     use rust_d3_geo::projection::stereographic::StereographicRaw;
//     use rust_d3_geo::Transform;

//     fn symetric_invert<'a, PM>(pm: PM)
//     where
//         PM: Transform<C=Coordinate<f64>> + ProjectionTrait<'a>
//     {
//         for p in vec![
//             &Coordinate {
//                 x: 0.0f64,
//                 y: 0.0f64,
//             },
//             &Coordinate {
//                 x: 30.3f64,
//                 y: 24.1f64,
//             },
//             &Coordinate {
//                 x: -10f64,
//                 y: 42f64,
//             },
//             &Coordinate {
//                 x: -2.0f64,
//                 y: -5.0f64,
//             },
//         ] {
//             assert!(projection_equal(&pm, &p, &pm.transform(&p), None));
//         }
//     }

//     #[test]
//     fn test_azimuthal_equal_area() {
//         symetric_invert(AzimuthalEqualAreaRaw::<f64>::gen_projection_mutator());
//     }

//     #[test]
//     fn test_equirectangular() {
//         symetric_invert(EquirectangularRaw::<f64>::gen_projection_mutator());
//     }

//     #[test]
//     fn test_gnomic() {
//         symetric_invert(GnomicRaw::<f64>::gen_projection_mutator());
//     }

//     #[test]
//     fn test_orthographic() {
//         symetric_invert(OrthographicRaw::<f64>::gen_projection_mutator());
//     }

//     #[test]
//     fn test_mecator() {
//         symetric_invert(MecatorRaw::<f64>::gen_projection_mutator());
//     }

//     #[test]
//     fn test_stereographic() {
//         symetric_invert(StereographicRaw::<f64>::gen_projection_mutator());
//     }
// }
