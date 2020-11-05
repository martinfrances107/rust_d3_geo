use std::cell::RefCell;
use std::f64;
use std::rc::Rc;

// use crate::stream::GeoStream;
use crate::transform_stream::StreamProcessor;
use crate::transform_stream::TransformStream;
// use crate::transform_stream::TransformStreamIdentity;

use super::intersect::intersect;

// Return indicator :-
// There were intersections or the line was empty.
const INTERSECTION_OR_LINE_EMPTY: u8 = 0u8;
const NO_INTERSECTIONS: u8 = 1u8;
// There were intersectoins and the first and last sections should be rejoined.
const INTERSECTION_REJOIN: u8 = 2u8;

// use crate::clip::ClipLine;

#[derive(Clone)]
pub struct Line {
    clean: Option<u8>,
    lambda0: f64,
    phi0: f64,
    sign0: f64,
    stream: Rc<RefCell<Box<dyn TransformStream>>>,
}

impl Line {
    pub fn new() -> StreamProcessor {
        return Box::new(|stream_ptr: Rc<RefCell<Box<dyn TransformStream>>>| {
            let stream = stream_ptr.clone();
            return Rc::new(RefCell::new(Box::new(Line {
                clean: None, // no intersections
                lambda0: f64::NAN,
                phi0: f64::NAN,
                sign0: f64::NAN,
                stream,
            })));
        });
    }

    fn clean(&mut self) -> Option<u8> {
        return match self.clean {
            Some(clean) => Some(2u8 - clean), // if intersections, rejoin first and last segments
            None => None,
        };
    }
}

impl TransformStream for Line {
    fn line_start(&mut self) {
        let mut stream = self.stream.borrow_mut();
        stream.line_start();
        self.clean = Some(NO_INTERSECTIONS);
    }

    fn point(&mut self, mut lambda1: f64, phi1: f64, _m: Option<u8>) {
        let mut stream = self.stream.borrow_mut();
        let sign1 = match lambda1 > 0f64 {
            true => f64::consts::PI,
            false => -f64::consts::PI,
        };
        let delta = (lambda1 - self.lambda0).abs();

        if (delta - f64::consts::PI).abs() < f64::EPSILON {
            // Line crosses a pole.
            let f_2 = 2f64;
            self.phi0 = (self.phi0 + phi1) / f_2;
            match (self.phi0 + phi1 / f_2) > 0f64 {
                true => {
                    stream.point(self.lambda0, f64::consts::FRAC_PI_2, None);
                }
                false => {
                    stream.point(self.lambda0, -f64::consts::FRAC_PI_2, None);
                }
            }
            stream.point(self.sign0, self.phi0, None);
            stream.line_end();
            stream.line_start();
            stream.point(sign1, self.phi0, None);
            stream.point(lambda1, self.phi0, None);
            self.clean = Some(INTERSECTION_OR_LINE_EMPTY);
        } else if self.sign0 != sign1 && delta >= f64::consts::PI {
            // Line crosses antimeridian.
            if (self.lambda0 - self.sign0).abs() < f64::EPSILON {
                self.lambda0 = self.lambda0 - self.sign0 * f64::EPSILON; // handle degeneracies
            }
            if (lambda1 - sign1).abs() < f64::EPSILON {
                lambda1 = lambda1 - sign1 * f64::EPSILON;
            }
            self.phi0 = intersect(self.lambda0, self.phi0, lambda1, phi1);
            stream.point(self.sign0, self.phi0, None);
            stream.line_end();
            //  self.stream.line_start();
            stream.point(sign1, self.phi0, None);
            self.clean = Some(INTERSECTION_OR_LINE_EMPTY);
        }
        self.lambda0 = lambda1;
        self.phi0 = phi1;
        stream.point(self.lambda0, self.phi0, None);
        self.sign0 = sign1;
    }

    fn line_end(&mut self) {
        let mut stream = self.stream.borrow_mut();
        stream.line_end();
        self.lambda0 = f64::NAN;
        self.phi0 = f64::NAN;
    }
}
