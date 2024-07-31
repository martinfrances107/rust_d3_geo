#[cfg(feature = "wgpu")]
mod wgpu_buffers {

    use geo::point;
    use geo::Geometry;

    use d3_geo_rs::in_delta::in_delta;
    use d3_geo_rs::path::builder::Builder as PathBuilder;
    use d3_geo_rs::path::wgpu::points::Points as PointsWGPU;
    use d3_geo_rs::path::wgpu::Vertex;
    use d3_geo_rs::projection::equirectangular::Equirectangular;
    use d3_geo_rs::projection::projector_common::types::ProjectorAntimeridianResampleNoneNoClip;
    use d3_geo_rs::projection::Build;
    use d3_geo_rs::projection::PrecisionBypass;
    use d3_geo_rs::projection::RawBase;
    use d3_geo_rs::projection::ScaleSet;
    use d3_geo_rs::stream::Streamable;

    type Projector = ProjectorAntimeridianResampleNoneNoClip<
        PointsWGPU,
        Equirectangular<f32>,
        f32,
    >;

    #[inline]
    fn equirectangular() -> Projector {
        Equirectangular::builder()
            .scale_set(900_f32 / core::f32::consts::PI)
            .precision_bypass()
            .build()
    }

    #[inline]
    fn path(
        projection: Projector,
        object: &impl Streamable<T = f32>,
    ) -> Vec<Vertex> {
        let context = PointsWGPU::default();
        let pb = PathBuilder::new(context);

        pb.build(projection).object(object)
    }

    #[test]
    fn renders_multipoint() {
        // data points taken from renders_multipoint
        println!("Render points as a GPU array buffer");
        let object = Geometry::MultiPoint(
            vec![
                point![x:-63_f32, y:18_f32],
                point![x:-62_f32, y:18_f32],
                point![x:-62_f32, y:17_f32],
            ]
            .into(),
        );

        let eq = equirectangular();

        // Build a pipe line where the endpoint is a WGPU array buffer.
        let actual = path(eq, &object);
        let expected = vec![
            Vertex {
                pos: [165_f32, 160_f32, 0_f32],
            },
            Vertex {
                pos: [170_f32, 160_f32, 0_f32],
            },
            Vertex {
                pos: [170_f32, 165_f32, 0_f32],
            },
        ];

        let results = actual.iter().zip(&expected);
        for (a, b) in results {
            assert!(in_delta(a.pos[0], b.pos[0], 1e-4));
        }
    }
}
