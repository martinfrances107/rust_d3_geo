use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

// use crate::stream::StreamClone;
use crate::stream::StreamPostClipTrait;
// use crate::stream::StreamResampleNode;
// use crate::stream::StreamResampleTrait;
use super::StreamResampleTrait;
use crate::stream::StreamClone;
use crate::{cartesian::cartesian, stream::stream_dummy::StreamDummy, stream::Stream};
// use crate::math::epsilon;
// use super::resample_none::ResampleNone;
// use super::ResampleEnum;
use crate::stream::CompareIntersection;
// use crate::stream::StreamSimpleNode;
use crate::stream::StreamSrc;
use crate::Transform;
use crate::TransformIdentity;
const MAXDEPTH: u8 = 16u8; // maximum depth of subdivision

pub struct Resample<T>
where
    T: CoordFloat + FloatConst,
{
    pub project: Box<dyn Transform<TcC = Coordinate<T>>>,
    pub delta2: T,

    // first point
    pub lambda00: T,
    pub x00: T,
    pub y00: T,
    pub a00: T,
    pub b00: T,
    pub c00: T,

    // previous point
    pub lambda0: T,
    pub x0: T,
    pub y0: T,
    pub a0: T,
    pub b0: T,
    pub c0: T,

    pub cos_min_distance: T,
    pub stream: Box<
        dyn StreamPostClipTrait<
            SpostctStream = StreamSrc<T>,
            C = Coordinate<T>,
            SctC = Coordinate<T>,
            SctT = T,
            SctOC = Option<Coordinate<T>>,
            SctCi = CompareIntersection<T>,
            SctStream = Box<dyn Stream<C = Coordinate<T>>>,
        >,
    >,

    pub use_line_point: bool,
    pub use_line_start: bool,
    pub use_line_end: bool,
}

impl<T> Clone for Resample<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    fn clone(&self) -> Self {
        Self {
            project: self.project.box_clone(),
            stream: self.stream.box_clone(),
            ..*self
        }
    }
}

impl<T> Default for Resample<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    fn default() -> Resample<T> {
        Self {
            project: Box::new(TransformIdentity::<T>::default()),
            delta2: T::zero(),

            // first point
            lambda00: T::zero(),
            x00: T::zero(),
            y00: T::zero(),
            a00: T::zero(),
            b00: T::zero(),
            c00: T::zero(),

            // previous point
            lambda0: T::zero(),
            x0: T::zero(),
            y0: T::zero(),
            a0: T::zero(),
            b0: T::zero(),
            c0: T::zero(),

            cos_min_distance: T::zero(),
            stream: Box::new(StreamDummy::default()),

            use_line_point: false,
            use_line_start: false,
            use_line_end: false,
        }
    }
}

impl<T> Resample<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    pub fn new(project: Box<dyn Transform<TcC = Coordinate<T>>>) -> Self {
        Self {
            project,
            ..Self::default()
        }
    }
}

// impl<T> StreamClone for Resample<T>
// where
//     T: CoordFloat + FloatConst + 'static,
// {
//     type C = Coordinate<T>;
//     fn box_clone(
//         &self,
//     ) -> Box<dyn StreamResampleTrait<C = Coordinate<T>, SRTsci = StreamPostClipNode<T>>> {
//         Box::new(*self.clone())
//     }
// }

impl<T> StreamResampleTrait for Resample<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    type SRTsci = Box<
        dyn StreamPostClipTrait<
            SpostctStream = StreamSrc<T>,
            C = Coordinate<T>,
            SctC = Coordinate<T>,
            SctT = T,
            SctOC = Option<Coordinate<T>>,
            SctCi = CompareIntersection<T>,
            SctStream = Box<dyn Stream<C = Coordinate<T>>>,
        >,
    >;
    fn stream_postclip_in(
        &mut self,
        _stream_clip_in: Box<
            dyn StreamPostClipTrait<
                SpostctStream = StreamSrc<T>,
                C = Coordinate<T>,
                SctC = Coordinate<T>,
                SctT = T,
                SctOC = Option<Coordinate<T>>,
                SctCi = CompareIntersection<T>,
                SctStream = Box<dyn Stream<C = Coordinate<T>>>,
            >,
        >,
    ) {
        panic!("Must override.");
    }
}

// impl<T> StreamResampleTrait for Box<dyn StreamResampleTrait<SRTsci = StreamPostClipNode<T>>>
// where
//     T: CoordFloat + FloatConst + 'static,
// {
//     type SRTsci = StreamPostClipNode<T>;
//     fn stream_postclip_in(&mut self, _stream_clip_in: StreamPostClipNode<T>) {
//         // No-op.
//     }
// }

impl<T> Resample<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    #[inline]

    fn ring_start(&mut self) {
        self.line_start();
        self.use_line_point = false;
        self.use_line_end = false;
    }

    fn ring_point(&mut self, p: Coordinate<T>) {
        self.lambda00 = p.x;
        self.line_point(Coordinate {
            x: self.lambda00,
            y: p.y,
        });
        self.x00 = self.x0;
        self.y00 = self.y0;
        self.a00 = self.a0;
        self.b00 = self.b0;
        self.c00 = self.c0;
        self.use_line_point = true;
    }

    fn ring_end(&mut self) {
        // self.resample_line_to(
        //     self.x0,
        //     self.y0,
        //     self.lambda0,
        //     self.a0,
        //     self.b0,
        //     self.c0,
        //     self.x00,
        //     self.y00,
        //     self.lambda00,
        //     self.a00,
        //     self.b00,
        //     self.c00,
        //     MAXDEPTH,
        //     self.stream,
        // );
        // self.use_line_end = true;

        // // let mut stream = self.stream.borrow_mut();
        // self.stream.line_end();
    }

    fn line_point(&mut self, p: Coordinate<T>) {
        let c = cartesian(&p);
        let project_ptr = self.project.box_clone();
        let project = &*project_ptr;
        let p = project.transform(&p);
        self.x0 = p.x;
        self.y0 = p.y;
        self.lambda0 = p.x;
        self.a0 = c[0];
        self.b0 = c[1];
        self.c0 = c[2];
        // let s_p = self.stream.as_ref();
        // let mut s = s_p.borrow_mut();
        // self.resample_line_to(
        //     self.x0,
        //     self.y0,
        //     self.lambda0,
        //     self.a0,
        //     self.b0,
        //     self.c0,
        //     self.x0,
        //     self.y0,
        //     self.lambda0,
        //     self.a0,
        //     self.b0,
        //     self.c0,
        //     MAXDEPTH,
        //     self.stream,
        // );
        // stream.point(x0, y0);
    }

    #[allow(clippy::many_single_char_names)]
    fn resample_line_to(
        &mut self,
        x0: T,
        y0: T,
        lambda0: T,
        a0: T,
        b0: T,
        c0: T,
        x1: T,
        y1: T,
        lambda1: T,
        a1: T,
        b1: T,
        c1: T,
        depth_p: u8,
        stream: Box<
            dyn StreamPostClipTrait<
                C = Coordinate<T>,
                SctC = Coordinate<T>,
                SctT = T,
                SctStream = Box<(dyn Stream<C = Coordinate<T>> + 'static)>,
                SctOC = Option<Coordinate<T>>,
                SpostctStream = StreamSrc<T>,
                SctCi = CompareIntersection<T>,
            >,
        >,
    ) {
        let mut depth = depth_p;
        let dx = x1 - x0;
        let dy = y1 - y0;
        let d2 = dx * dx + dy * dy;

        // if (d2 > 4 * delta2 && depth--) {
        if d2 > T::from(4f64).unwrap() * self.delta2 {
            depth -= 1u8;
            if depth > 0u8 {
                let mut a = a0 + a1;
                let mut b = b0 + b1;
                let mut c = c0 + c1;
                let m = (a * a + b * b + c * c).sqrt();
                c = c / m;
                let phi2 = c.asin();
                let lambda2;
                if (c.abs() - T::one()).abs() < T::epsilon()
                    || (lambda0 - lambda1).abs() < T::epsilon()
                {
                    lambda2 = (lambda0 + lambda1) / T::from(2).unwrap();
                } else {
                    lambda2 = b.atan2(a);
                };

                let project_ptr = self.project.box_clone();
                let project = &*project_ptr;
                let p = project.transform(&Coordinate {
                    x: lambda2,
                    y: phi2,
                });

                let x2 = p.x;
                let y2 = p.y;
                let dx2 = x2 - x0;
                let dy2 = y2 - y0;
                let dz = dy * dx2 - dx * dy2;
                // Three condtions :-
                // perpendicular projected distance
                // midpoint close to an end
                // angular distance
                if dz * dz / d2 > self.delta2
                    || ((dx * dx2 + dy * dy2) / d2 - T::from(0.5).unwrap()).abs()
                        > T::from(0.3).unwrap()
                    || a0 * a1 + b0 * b1 + c0 * c1 < self.cos_min_distance
                {
                    a = a / m;
                    b = b / m;
                    // let stream_p: RefCell<_> = RefCell::new(stream);
                    // let self_p: RefCell<_> = RefCell::new(self);
                    // {
                    // let mut s = stream.borrow_mut();
                    // let self_p1 = self_p.borrow_mut();
                    // &*project_ptr.borrow();
                    let s = stream;
                    self.resample_line_to(
                        x0, y0, lambda0, a0, b0, c0, x2, y2, lambda2, a, b, c, depth, s,
                    );
                    // }
                    // {
                    // let mut s2 = stream.borrow_mut();
                    // s2.borrow_mut().point(x2, y2, None);
                    // }
                    // {
                    // let mut s3 = stream_p.borrow_mut();
                    // let self_p2 = self_p.borrow_mut();

                    // self.resample_line_to(
                    //     x2, y2, lambda2, a, b, c, x1, y1, lambda1, a1, b1, c1, depth, stream,
                    // );
                    // }
                }
            }
        }
    }
}
impl<T> StreamClone for Resample<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type RetType = Box<dyn Stream<C = Coordinate<T>>>;
    #[inline]
    fn box_clone(&self) -> Self::RetType {
        Box::new(self.clone())
    }
}

impl<T> Stream for Resample<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    type C = Coordinate<T>;
    #[inline]
    fn point(&mut self, p: Self::C, _m: Option<u8>) {
        if self.use_line_point {
            self.line_point(p);
        } else {
            self.ring_point(p);
        }
    }

    fn line_start(&mut self) {
        if self.use_line_start {
            // let mut stream = self.stream.borrow_mut();
            self.x0 = T::nan();
            self.use_line_point = true;
            self.stream.line_start();
        } else {
            self.ring_start();
        }
    }

    fn line_end(&mut self) {
        match self.use_line_end {
            true => {
                // let mut stream = self.stream.borrow_mut();
                self.use_line_point = false;
                self.stream.line_end();
            }

            false => {
                self.ring_end();
            }
        }
    }
}
