use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use std::cell::RefCell;
use std::rc::Rc;

use crate::stream::StreamPostClipNode;
use crate::stream::StreamResampleNode;
use crate::stream::StreamResampleTrait;
use crate::{cartesian::cartesian, stream::Stream, stream::StreamDummy, stream::StreamInTrait};
// use crate::math::epsilon;
use crate::stream::StreamSimpleNode;

use crate::Transform;

use super::resample_none::ResampleNone;
const MAXDEPTH: u8 = 16u8; // maximum depth of subdivision

#[derive(Clone)]
pub struct Resample<T>
where
    T: CoordFloat,
{
    project: Rc<Box<dyn Transform<C = Coordinate<T>, TcC = Coordinate<T>>>>,
    delta2: T,

    // first point
    lambda00: T,
    x00: T,
    y00: T,
    a00: T,
    b00: T,
    c00: T,

    // previous point
    lambda0: T,
    x0: T,
    y0: T,
    a0: T,
    b0: T,
    c0: T,

    cos_min_distance: T,
    stream: StreamSimpleNode<T>,

    use_line_point: bool,
    use_line_start: bool,
    use_line_end: bool,
}

impl<T> Resample<T>
where
    T: CoordFloat + FloatConst + std::default::Default + 'static,
{
    #[inline]
    pub fn gen_node(
        project: Rc<Box<dyn Transform<C = Coordinate<T>, TcC = Coordinate<T>>>>,
        delta2: Option<T>,
    ) -> StreamResampleNode<T> {
        match delta2 {
            None => ResampleNone::gen_node(project),
            Some(delta2) => {
                Rc::new(RefCell::new(Self {
                    project: project.clone(),
                    delta2,

                    lambda00: T::zero(),
                    x00: T::zero(),
                    y00: T::zero(),
                    a00: T::zero(),
                    b00: T::zero(),
                    c00: T::zero(), // first point

                    lambda0: T::zero(),
                    x0: T::zero(),
                    y0: T::zero(),
                    a0: T::zero(),
                    b0: T::zero(),
                    c0: T::zero(), // previous point
                    cos_min_distance: (T::from(30f64).unwrap().to_radians()).cos(), // cos(minimum angular distance)

                    stream: Rc::new(RefCell::new(StreamDummy::default())),
                    use_line_point: true,
                    use_line_end: true,
                    use_line_start: true,
                }))
            }
        }
    }
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
        self.resample_line_to(
            self.x0,
            self.y0,
            self.lambda0,
            self.a0,
            self.b0,
            self.c0,
            self.x00,
            self.y00,
            self.lambda00,
            self.a00,
            self.b00,
            self.c00,
            MAXDEPTH,
            self.stream.clone(),
        );
        self.use_line_end = true;

        let mut stream = self.stream.borrow_mut();
        stream.line_end();
    }

    fn line_point(&mut self, p: Coordinate<T>) {
        let c = cartesian(&p);
        let project_ptr = self.project.clone();
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
        self.resample_line_to(
            self.x0,
            self.y0,
            self.lambda0,
            self.a0,
            self.b0,
            self.c0,
            self.x0,
            self.y0,
            self.lambda0,
            self.a0,
            self.b0,
            self.c0,
            MAXDEPTH,
            self.stream.clone(),
        );
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
        stream: StreamSimpleNode<T>,
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

                let project_ptr = self.project.clone();
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
                    let s = stream.clone();
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

                    self.resample_line_to(
                        x2, y2, lambda2, a, b, c, x1, y1, lambda1, a1, b1, c1, depth, stream,
                    );
                    // }
                }
            }
        }
    }
}

impl<T> StreamResampleTrait<T> for Resample<T>
where
    T: CoordFloat + FloatConst,
{
    fn stream_postclip_in(&mut self, _stream_clip_in: StreamPostClipNode<T>) {
        panic!("Must override.");
    }
}

impl<T> StreamInTrait<T> for Resample<T> where T: CoordFloat + FloatConst {}
impl<T> Stream for Resample<T>
where
    T: CoordFloat + FloatConst + std::default::Default + 'static,
{
    type C = Coordinate<T>;
    #[inline]
    fn point(&mut self, p: Coordinate<T>, _m: Option<u8>) {
        if self.use_line_point {
            self.line_point(p);
        } else {
            self.ring_point(p);
        }
    }

    fn line_start(&mut self) {
        if self.use_line_start {
            let mut stream = self.stream.borrow_mut();
            self.x0 = T::nan();
            self.use_line_point = true;
            stream.line_start();
        } else {
            self.ring_start();
        }
    }

    fn line_end(&mut self) {
        match self.use_line_end {
            true => {
                let mut stream = self.stream.borrow_mut();
                self.use_line_point = false;
                stream.line_end();
            }

            false => {
                self.ring_end();
            }
        }
    }
}
