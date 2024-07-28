#[cfg(feature = "wgpu")]
mod wgpu_buffers {
    use d3_geo_rs::path::builder::Builder as PathBuilder;
    use d3_geo_rs::path::points_wgpu::PointsWGPU;
    use d3_geo_rs::path::points_wgpu::Vertex;
    use d3_geo_rs::projection::equirectangular::Equirectangular;
    use d3_geo_rs::projection::projector_common::types::ProjectorAntimeridianResampleNoneNoClip;
    use d3_geo_rs::projection::Build;
    use d3_geo_rs::projection::PrecisionBypass;
    use d3_geo_rs::projection::RawBase;
    use d3_geo_rs::projection::ScaleSet;
    use d3_geo_rs::stream::Streamable;
    use geo::point;
    use geo::CoordFloat;
    use geo::Geometry;
    use num_traits::FloatConst;

    type Projector<T> = ProjectorAntimeridianResampleNoneNoClip<
        PointsWGPU<T>,
        Equirectangular<T>,
        T,
    >;

    #[inline]
    fn equirectangular<T>() -> Projector<T>
    where
        T: 'static + CoordFloat + Default + FloatConst,
    {
        Equirectangular::builder()
            .scale_set(T::from(900f64 / core::f64::consts::PI).unwrap())
            .precision_bypass()
            .build()
    }

    #[inline]
    fn path<T>(
        projection: Projector<T>,
        object: &impl Streamable<T = T>,
    ) -> Vec<Vertex<T>>
    where
        T: 'static + CoordFloat + FloatConst,
    {
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
                pos: [165_f32, 160_f32],
            },
            Vertex {
                pos: [170_f32, 160_f32],
            },
            Vertex {
                pos: [170_f32, 165_f32],
            },
        ];
        assert_eq!(actual, expected);
    }
}
