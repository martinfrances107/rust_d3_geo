#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod clip_maco_integration_test {
    use std::cell::RefCell;
    use std::fmt::Display;
    use std::ops::AddAssign;
    use std::rc::Rc;

    use clip_ops_macro_derive::ClipOps;
    use geo::CoordFloat;
    use geo::Coordinate;
    use num_traits::AsPrimitive;
    use num_traits::FloatConst;

    use rust_d3_geo::centroid::stream::Stream;
    use rust_d3_geo::clip::antimeridian::line::Line;
    use rust_d3_geo::clip::buffer::Buffer;
    // use rust_d3_geo::clip::clip_base::ClipBase;
    use rust_d3_geo::clip::line_elem::LineElem;
    use rust_d3_geo::clip::point_visible_trait::PointVisible;
    use rust_d3_geo::clip::ClipOps;
    use rust_d3_geo::clip::LCB;
    use rust_d3_geo::stream::Stream;

    #[derive(ClipOps)]
    pub struct ClipTest<T>
    where
        T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        pub x: u32,
        // pub s: SINK,
        base: ClipBase<Line<T>, T>,
    }

    impl<SINK, T> PointVisible for StreamNode<ClipTest<SINK, T>, T>
    where
        SINK: Stream<SC = Coordinate<T>>,
        T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        type T = T;

        #[inline]
        fn point_visible(&self, _p: &Coordinate<T>, _z: Option<u8>) -> bool {
            true
        }
    }

    impl<T> Stream for StreamNode<ClipTest<SINK, T>, T>
    where
        T: AddAssign + AsPrimitive<T> + CoordFloat + Display + FloatConst,
    {
        type SC = Coordinate<T>;
        fn point(&mut self, _p: &Self::SC, _m: Option<u8>) {}
        fn sphere(&mut self) {}
        fn line_start(&mut self) {}
        fn line_end(&mut self) {}
        fn polygon_start(&mut self) {}
        fn polygon_end(&mut self) {}
    }

    #[test]
    fn test_clip_macro() {
        let start = LineElem {
            p: Coordinate {
                x: -f64::PI(),
                y: -f64::PI() / 2.0,
            },
            m: None,
        };
        let line = Line::default();
        let ring_buffer = Rc::new(RefCell::new(Buffer::default()));
        let mut ring_sink: Box<Line<Buffer<f64>, f64>> = Box::new(Line::default());
        ring_sink.link_to_stream(ring_buffer.clone());

        let a = ClipTest {
            x: 1,
            s: CentroidStream::default(),
            base: ClipBase::new(line, ring_buffer, ring_sink, start),
        };
        assert_eq!(42, a.hello_macro());
    }
}
