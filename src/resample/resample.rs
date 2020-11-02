use delaunator::Point;

use std::cell::RefCell;
use std::rc::Rc;

use crate::cartesian::cartesian;
// use crate::math::epsilon;
use crate::transform_stream::StreamProcessor;
use crate::transform_stream::TransformStream;
use crate::Transform;

// use crate::stream::Stream;

// import {cartesian} from "../cartesian.js";
// import {transformer} from "../transform.js";

const MAXDEPTH: u8 = 16u8; // maximum depth of subdivision

// #[derive(Clone)]
pub struct Resample {
    project: Rc<RefCell<Box<dyn Transform>>>,
    delta2: f64,

    // first point
    lambda00: f64,
    x00: f64,
    y00: f64,
    a00: f64,
    b00: f64,
    c00: f64,

    // previous point
    lambda0: f64,
    x0: f64,
    y0: f64,
    a0: f64,
    b0: f64,
    c0: f64,

    cos_min_distance: f64,
    stream: Rc<RefCell<Box<dyn TransformStream>>>,
    // s: &'a Box<dyn TransformStream>,
    use_line_point: bool,
    use_line_start: bool,
    use_line_end: bool,
}

impl Resample {
    pub fn new(project: Rc<RefCell<Box<dyn Transform>>>, delta2: f64) -> StreamProcessor {
        return Box::new(move |stream: Rc<RefCell<Box<dyn TransformStream>>>| {
            return Rc::new(RefCell::new(Box::new(Self {
                project: project.clone(),
                delta2,

                lambda00: 0f64,
                x00: 0f64,
                y00: 0f64,
                a00: 0f64,
                b00: 0f64,
                c00: 0f64, // first point

                lambda0: 0f64,
                x0: 0f64,
                y0: 0f64,
                a0: 0f64,
                b0: 0f64,
                c0: 0f64,                                     // previous point
                cos_min_distance: (30f64.to_radians()).cos(), // cos(minimum angular distance)

                stream,
                use_line_point: true,
                use_line_end: true,
                use_line_start: true,
            })));
        });
    }

    fn ring_start(&mut self) {
        self.line_start();
        self.use_line_point = false;
        self.use_line_end = false;
    }

    fn ring_point(&mut self, lambda: f64, phi: f64) {
        self.lambda00 = lambda;
        self.line_point(self.lambda00, phi);
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

    fn line_point(&mut self, lambda: f64, phi: f64) {
        let c = cartesian(&Point { x: lambda, y: phi });
        let project_ptr = self.project.clone();
        let project = &*project_ptr.borrow();
        let p = project.transform(&Point { x: lambda, y: phi });
        self.x0 = p.x;
        self.y0 = p.y;
        self.lambda0 = lambda;
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
        x0: f64,
        y0: f64,
        lambda0: f64,
        a0: f64,
        b0: f64,
        c0: f64,
        x1: f64,
        y1: f64,
        lambda1: f64,
        a1: f64,
        b1: f64,
        c1: f64,
        depth_p: u8,
        stream: Rc<RefCell<Box<dyn TransformStream>>>,
    ) {
        let mut depth = depth_p;
        let dx = x1 - x0;
        let dy = y1 - y0;
        let d2 = dx * dx + dy * dy;

        // if (d2 > 4 * delta2 && depth--) {
        if d2 > 4f64 * self.delta2 {
            depth -= 1u8;
            if depth > 0u8 {
                let mut a = a0 + a1;
                let mut b = b0 + b1;
                let mut c: f64 = c0 + c1;
                let m: f64 = (a * a + b * b + c * c).sqrt();
                c = c / m;
                let phi2 = c.asin();
                let lambda2;
                if (c.abs() - 1f64).abs() < f64::EPSILON || (lambda0 - lambda1).abs() < f64::EPSILON
                {
                    lambda2 = (lambda0 + lambda1) / 2f64;
                } else {
                    lambda2 = b.atan2(a);
                };

                let project_ptr = self.project.clone();
                let project = &*project_ptr.borrow();
                let p = project.transform(&Point {
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
                    || ((dx * dx2 + dy * dy2) / d2 - 0.5f64).abs() > 0.3f64
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
                    let mut s2 = stream.borrow_mut();
                    s2.point(x2, y2, None);
                    // }
                    // {
                    // let mut s3 = stream_p.borrow_mut();
                    // let self_p2 = self_p.borrow_mut();
                    self.resample_line_to(
                        x2,
                        y2,
                        lambda2,
                        a,
                        b,
                        c,
                        x1,
                        y1,
                        lambda1,
                        a1,
                        b1,
                        c1,
                        depth,
                        stream.clone(),
                    );
                    // }
                }
            }
        }
    }
}

impl TransformStream for Resample {
    fn point(&mut self, x: f64, y: f64, _m: Option<u8>) {
        if self.use_line_point {
            self.line_point(x, y);
        } else {
            self.ring_point(x, y);
        }
    }

    fn line_start(&mut self) {
        if self.use_line_start {
            let mut stream = self.stream.borrow_mut();
            self.x0 = f64::NAN;
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
