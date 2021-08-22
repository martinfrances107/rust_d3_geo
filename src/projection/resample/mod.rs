pub mod none;
pub mod resample;
pub mod stream_node_resample_factory;

use std::fmt::Debug;

use geo::CoordFloat;
use geo::Coordinate;

use crate::stream::Stream;

use super::stream_node::StreamNode;

use none::None;
use resample::Resample;

use super::Raw as ProjectionRaw;

#[derive(Debug, Clone)]
pub enum ResampleNode<PR, SINK, T>
where
    PR: ProjectionRaw<T>,
    T: CoordFloat,
{
    RN(StreamNode<None<PR, T>, SINK, T>),
    R(StreamNode<Resample<PR, T>, SINK, T>),
}

// impl<PR, SINK, T> Default for ResampleNode<PR, SINK, T>
// where
//     PR: ProjectionRaw<T>,
//     SINK: Default + Stream<T = T>,
//     T: CoordFloat,
// {
//     #[inline]
//     fn default() -> Self {
//         let stream_node = StreamNode {
//             raw: None::default(),
//             sink: Rc::new(RefCell::new(SINK::default())),
//             pd: PhantomData::<T>,
//         };
//         ResampleNode::RN(stream_node)
//     }
// }

// impl<PR, T> Clone for ResampleEnum<PR, T>
// where
//     PR: ProjectionRaw<T>,
//     T: CoordFloat,
// {
//     #[inline]
//     fn clone(&self) -> Self {
//         match self {
//             ResampleEnum::RN(rn) => ResampleEnum::RN(*rn),
//             ResampleEnum::R(r) => ResampleEnum::R(*r),
//         }
//     }
// }

impl<'a, PR, SINK, T> Stream for ResampleNode<PR, SINK, T>
where
    PR: ProjectionRaw<T>,
    SINK: Stream<T = T>,
    T: CoordFloat,
{
    type T = T;

    fn sphere(&mut self) {
        match self {
            ResampleNode::RN(n) => n.sphere(),
            ResampleNode::R(r) => r.sphere(),
        };
    }
    fn polygon_start(&mut self) {
        match self {
            ResampleNode::RN(n) => n.polygon_start(),
            ResampleNode::R(r) => r.polygon_start(),
        };
    }
    fn polygon_end(&mut self) {
        match self {
            ResampleNode::RN(n) => n.polygon_end(),
            ResampleNode::R(r) => r.polygon_end(),
        };
    }
    fn point(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        match self {
            ResampleNode::RN(n) => n.point(p, m),
            ResampleNode::R(r) => r.point(p, m),
        };
    }
    fn line_start(&mut self) {
        match self {
            ResampleNode::RN(n) => n.line_start(),
            ResampleNode::R(r) => r.line_start(),
        };
    }
    fn line_end(&mut self) {
        match self {
            ResampleNode::RN(n) => n.line_end(),
            ResampleNode::R(r) => r.line_end(),
        };
    }
}

// #[inline]
// pub fn gen_resample_factory<PR, SINK, T>(
//     projection_raw: PR,
//     delta2: T,
// ) -> StreamNodeFactory<ResampleNode<PR, SINK, T>, SINK, T>
// where
//     PR: ProjectionRaw<T>,
//     SINK: Stream<T = T>,
//     T: CoordFloat,
// {
//     if delta2.is_zero() {
//         StreamNodeFactory::new(ResampleNode::RN(StreamNode{
//             raw, ResampleNone::new(self.projection_raw),
//             sink, SINK::default,
//             (None::new(projection_raw))))
//     } else {
//         StreamNodeFactory::new(ResampleNode::R(Resample::new(projection_raw, delta2)))
//     }
// }
