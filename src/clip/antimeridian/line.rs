use geo::CoordFloat;
use num_traits::FloatConst;

use super::intersect::intersect;
use crate::stream::StreamNode;
use crate::{stream::Stream, transform_stream::StreamProcessor};

use std::cell::RefCell;
use std::rc::Rc;

// Return indicator :-
// There were intersections or the line was empty.
const INTERSECTION_OR_LINE_EMPTY: u8 = 0u8;
const NO_INTERSECTIONS: u8 = 1u8;
// There were intersectoins and the first and last sections should be rejoined.
const INTERSECTION_REJOIN: u8 = 2u8;

// use crate::clip::ClipLine;
pub struct Line<T>
where
    T: CoordFloat + FloatConst,
{
    clean: Option<u8>,
    lambda0: T,
    phi0: T,
    sign0: T,
    stream: StreamNode<T>,
}

impl<T> Line<T>
where
    T: CoordFloat + FloatConst + 'static,
{
    #[inline]
    pub fn new() -> StreamProcessor<T> {
        Box::new(|stream_ptr: StreamNode<T>| {
            let stream = stream_ptr;
            Rc::new(RefCell::new(Box::new(Line {
                clean: None, // no intersections
                lambda0: T::nan(),
                phi0: T::nan(),
                sign0: T::nan(),
                stream,
            })))
        })
    }

    fn clean(&mut self) -> Option<u8> {
        return match self.clean {
            Some(clean) => Some(2u8 - clean), // if intersections, rejoin first and last segments
            None => None,
        };
    }
}

impl<T: CoordFloat + FloatConst> Stream<T> for Line<T> {
    fn line_start(&mut self) {
        let mut s = self.stream.borrow_mut();
        s.line_start();
        self.clean = Some(NO_INTERSECTIONS);
    }

    fn point(&mut self, mut lambda1: T, phi1: T, _m: Option<u8>) {
        let mut s = self.stream.borrow_mut();
        let sign1 = match lambda1.is_sign_positive() {
            true => T::PI(),
            false => -T::PI(),
        };
        let delta = (lambda1 - self.lambda0).abs();

        if (delta - T::PI()).abs() < T::epsilon() {
            // Line crosses a pole.
            let f_2 = T::from(2f64).unwrap();
            self.phi0 = (self.phi0 + phi1) / f_2;
            match (self.phi0 + phi1 / f_2).is_sign_positive() {
                true => {
                    s.point(self.lambda0, T::FRAC_PI_2(), None);
                }
                false => {
                    s.point(self.lambda0, -T::FRAC_PI_2(), None);
                }
            }
            s.point(self.sign0, self.phi0, None);
            s.line_end();
            s.line_start();
            s.point(sign1, self.phi0, None);
            s.point(lambda1, self.phi0, None);
            self.clean = Some(INTERSECTION_OR_LINE_EMPTY);
        } else if self.sign0 != sign1 && delta >= T::PI() {
            // Line crosses antimeridian.
            if (self.lambda0 - self.sign0).abs() < T::epsilon() {
                self.lambda0 = self.lambda0 - self.sign0 * T::epsilon(); // handle degeneracies
            }
            if (lambda1 - sign1).abs() < T::epsilon() {
                lambda1 = lambda1 - sign1 * T::epsilon();
            }
            self.phi0 = intersect(self.lambda0, self.phi0, lambda1, phi1);
            s.point(self.sign0, self.phi0, None);
            s.line_end();
            //  self.stream.line_start();
            s.point(sign1, self.phi0, None);
            self.clean = Some(INTERSECTION_OR_LINE_EMPTY);
        }
        self.lambda0 = lambda1;
        self.phi0 = phi1;
        s.point(self.lambda0, self.phi0, None);
        self.sign0 = sign1;
    }

    fn line_end(&mut self) {
        let mut s = self.stream.borrow_mut();
        s.line_end();
        self.lambda0 = T::nan();
        self.phi0 = T::nan();
    }
}
