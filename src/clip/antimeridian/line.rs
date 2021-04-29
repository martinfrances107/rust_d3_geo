use std::fmt::Display;
use std::ops::AddAssign;

use geo::{CoordFloat, Coordinate};
use num_traits::AsPrimitive;
use num_traits::FloatConst;

// use crate::path::PathResultEnum;
use crate::clip::clip_sink_enum::ClipSinkEnum;
use crate::clip::line_sink_enum::LineSinkEnum;
use crate::clip::ClipBuffer;
use crate::path::path_context_stream::PathContextStream;
use crate::stream::stream_dst::StreamDst;
use crate::stream::Stream;
use crate::stream::{Clean, CleanEnum};

use super::intersect::intersect;

#[derive(Clone, Debug)]
pub struct Line<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    clean: CleanEnum,
    lambda0: T,
    phi0: T,
    sign0: T,
    stream: LineSinkEnum<T>,
}

impl<T> Default for Line<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    #[inline]
    fn default() -> Self {
        Line {
            clean: CleanEnum::IntersectionsOrEmpty,
            lambda0: T::nan(),
            phi0: T::nan(),
            sign0: T::nan(),
            stream: LineSinkEnum::CB(ClipBuffer::default()),
        }
    }
}

impl<T> Line<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    #[inline]
    pub fn stream_in(&mut self, stream: LineSinkEnum<T>) {
        self.stream = stream;
    }

    #[inline]
    pub fn get_stream(&self) -> LineSinkEnum<T> {
        self.stream.clone()
    }
}

impl<T> Clean for Line<T>
where
    T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst,
{
    #[inline]
    fn clean(&self) -> CleanEnum {
        match self.clean {
            // if intersections, rejoin first and last segments
            CleanEnum::IntersectionsOrEmpty => CleanEnum::IntersectionsRejoin,
            CleanEnum::NoIntersections => CleanEnum::NoIntersections,
            CleanEnum::IntersectionsRejoin => CleanEnum::IntersectionsOrEmpty,
        }
    }
}

impl<T: AddAssign + AsPrimitive<T> + CoordFloat + Default + Display + FloatConst> Stream<T>
    for Line<T>
{
    type C = Coordinate<T>;
    fn sphere(&mut self) {
        todo!("is this called")
    }
    fn polygon_start(&mut self) {
        todo!("is this called")
    }

    fn polygon_end(&mut self) {
        todo!("is this called")
    }

    fn get_dst(&self) -> StreamDst<T> {
        match &self.stream {
            LineSinkEnum::CB(stream) => stream.get_dst(),
            LineSinkEnum::CSE(_stream) => {
                todo!("not sure what todo here.")
            }
        }
    }
    fn line_start(&mut self) {
        match &mut self.stream {
            LineSinkEnum::CSE(stream) => match stream {
                ClipSinkEnum::Resample(stream) => stream.line_start(),
                ClipSinkEnum::Src(stream) => {
                    // match stream {
                    //     StreamDst::Context2D(_CanvasRenderingContext2d) => {
                    //         // TODo must implement.
                    //     }
                    //     StreamDst::PathContextStream(pcs) => match pcs {
                    //         PathContextStream::PC(_pc) => {
                    //             // implement!("work ..");
                    //             // pc.line_start()
                    //         }
                    //         PathContextStream::PS(ps) => {
                    //             ps.line_start();
                    //         }
                    //     },
                    //     StreamDst::Circle(c) => c.line_start(),
                    //     StreamDst::PAS(pas) => pas.line_start(),
                    //     StreamDst::CS(cs) => cs.line_start(),
                    //     StreamDst::LS(ls) => ls.line_start(),
                    // }
                }
                ClipSinkEnum::Blank => {
                    panic!("ClickSinkEnum - actively using an unconnected blank");
                }
            },
            LineSinkEnum::CB(stream) => stream.line_start(),
        }
        self.clean = CleanEnum::NoIntersections;
    }

    fn point(&mut self, p: &Self::C, _m: Option<u8>) {
        let mut lambda1 = p.x;
        let phi1 = p.y;
        // let mut s = self.stream.borrow_mut();
        let sign1 = match lambda1.is_sign_positive() {
            true => T::PI(),
            false => -T::PI(),
        };
        let delta = (lambda1 - self.lambda0).abs();

        if (delta - T::PI()).abs() < T::epsilon() {
            // Line crosses a pole.
            let f_2 = T::from(2f64).unwrap();
            self.phi0 = (self.phi0 + phi1) / f_2;
            match &mut self.stream {
                LineSinkEnum::CB(stream) => {
                    match (self.phi0 + phi1 / f_2).is_sign_positive() {
                        true => {
                            stream.point(
                                &Coordinate {
                                    x: self.lambda0,
                                    y: T::FRAC_PI_2(),
                                },
                                None,
                            );
                        }
                        false => {
                            stream.point(
                                &Coordinate {
                                    x: self.lambda0,
                                    y: -T::FRAC_PI_2(),
                                },
                                None,
                            );
                        }
                    }
                    stream.point(
                        &Coordinate {
                            x: self.sign0,
                            y: self.phi0,
                        },
                        None,
                    );
                    stream.line_end();
                    stream.line_start();
                    stream.point(
                        &Coordinate {
                            x: sign1,
                            y: self.phi0,
                        },
                        None,
                    );
                    stream.point(
                        &Coordinate {
                            x: lambda1,
                            y: self.phi0,
                        },
                        None,
                    );
                }
                LineSinkEnum::CSE(stream) => match stream {
                    ClipSinkEnum::Blank => {
                        panic!("ClickSinkEnum - actively using an unconnected blank");
                    }
                    ClipSinkEnum::Src(stream) => {
                        match (self.phi0 + phi1 / f_2).is_sign_positive() {
                            true => {
                                stream.point(
                                    &Coordinate {
                                        x: self.lambda0,
                                        y: T::FRAC_PI_2(),
                                    },
                                    None,
                                );
                            }
                            false => {
                                stream.point(
                                    &Coordinate {
                                        x: self.lambda0,
                                        y: -T::FRAC_PI_2(),
                                    },
                                    None,
                                );
                            }
                        }
                        stream.point(
                            &Coordinate {
                                x: self.sign0,
                                y: self.phi0,
                            },
                            None,
                        );
                        stream.line_end();
                        stream.line_start();
                        stream.point(
                            &Coordinate {
                                x: sign1,
                                y: self.phi0,
                            },
                            None,
                        );
                        stream.point(
                            &Coordinate {
                                x: lambda1,
                                y: self.phi0,
                            },
                            None,
                        );
                    }
                    ClipSinkEnum::Resample(stream) => {
                        match (self.phi0 + phi1 / f_2).is_sign_positive() {
                            true => {
                                stream.point(
                                    &Coordinate {
                                        x: self.lambda0,
                                        y: T::FRAC_PI_2(),
                                    },
                                    None,
                                );
                            }
                            false => {
                                stream.point(
                                    &Coordinate {
                                        x: self.lambda0,
                                        y: -T::FRAC_PI_2(),
                                    },
                                    None,
                                );
                            }
                        }
                        stream.point(
                            &Coordinate {
                                x: self.sign0,
                                y: self.phi0,
                            },
                            None,
                        );
                        stream.line_end();
                        stream.line_start();
                        stream.point(
                            &Coordinate {
                                x: sign1,
                                y: self.phi0,
                            },
                            None,
                        );
                        stream.point(
                            &Coordinate {
                                x: lambda1,
                                y: self.phi0,
                            },
                            None,
                        );
                    }
                },
            }

            self.clean = CleanEnum::IntersectionsOrEmpty;
        } else if self.sign0 != sign1 && delta >= T::PI() {
            // Line crosses antimeridian.
            if (self.lambda0 - self.sign0).abs() < T::epsilon() {
                self.lambda0 = self.lambda0 - self.sign0 * T::epsilon(); // handle degeneracies
            }
            if (lambda1 - sign1).abs() < T::epsilon() {
                lambda1 = lambda1 - sign1 * T::epsilon();
            }
            self.phi0 = intersect(self.lambda0, self.phi0, lambda1, phi1);
            match &mut self.stream {
                LineSinkEnum::CB(stream) => {
                    stream.point(
                        &Coordinate {
                            x: self.sign0,
                            y: self.phi0,
                        },
                        None,
                    );
                    stream.line_end();
                    stream.line_start();
                    stream.point(
                        &Coordinate {
                            x: sign1,
                            y: self.phi0,
                        },
                        None,
                    );
                }
                LineSinkEnum::CSE(stream) => match stream {
                    ClipSinkEnum::Blank => {
                        panic!("ClickSinkEnum - actively using an unconnected blank");
                    }
                    ClipSinkEnum::Src(stream) => {
                        stream.point(
                            &Coordinate {
                                x: self.sign0,
                                y: self.phi0,
                            },
                            None,
                        );
                        stream.line_end();
                        stream.line_start();
                        stream.point(
                            &Coordinate {
                                x: sign1,
                                y: self.phi0,
                            },
                            None,
                        );
                    }
                    ClipSinkEnum::Resample(stream) => {
                        stream.point(
                            &Coordinate {
                                x: self.sign0,
                                y: self.phi0,
                            },
                            None,
                        );
                        stream.line_end();
                        stream.line_start();
                        stream.point(
                            &Coordinate {
                                x: sign1,
                                y: self.phi0,
                            },
                            None,
                        );
                    }
                },
            }

            match &mut self.stream {
                LineSinkEnum::CB(stream) => {
                    stream.point(
                        &Coordinate {
                            x: self.sign0,
                            y: self.phi0,
                        },
                        None,
                    );
                    stream.line_end();
                    stream.line_start();
                    stream.point(
                        &Coordinate {
                            x: sign1,
                            y: self.phi0,
                        },
                        None,
                    );
                }
                LineSinkEnum::CSE(stream) => match stream {
                    ClipSinkEnum::Blank => {
                        panic!("ClickSinkEnum - actively using an unconnected blank");
                    }
                    ClipSinkEnum::Src(stream) => {
                        stream.point(
                            &Coordinate {
                                x: self.sign0,
                                y: self.phi0,
                            },
                            None,
                        );
                        stream.line_end();
                        stream.line_start();
                        stream.point(
                            &Coordinate {
                                x: sign1,
                                y: self.phi0,
                            },
                            None,
                        );
                    }
                    ClipSinkEnum::Resample(stream) => {
                        stream.point(
                            &Coordinate {
                                x: self.sign0,
                                y: self.phi0,
                            },
                            None,
                        );
                        stream.line_end();
                        stream.line_start();
                        stream.point(
                            &Coordinate {
                                x: sign1,
                                y: self.phi0,
                            },
                            None,
                        );
                    }
                },
            }

            self.clean = CleanEnum::IntersectionsOrEmpty;
        }
        self.lambda0 = lambda1;
        self.phi0 = phi1;
        match &mut self.stream {
            LineSinkEnum::CB(stream) => stream.point(
                &Coordinate {
                    x: self.lambda0,
                    y: self.phi0,
                },
                None,
            ),
            LineSinkEnum::CSE(stream) => match stream {
                ClipSinkEnum::Blank => {
                    panic!("ClickSinkEnum - actively using an unconnected blank");
                }
                ClipSinkEnum::Src(stream) => stream.point(
                    &Coordinate {
                        x: self.lambda0,
                        y: self.phi0,
                    },
                    None,
                ),
                ClipSinkEnum::Resample(stream) => stream.point(
                    &Coordinate {
                        x: self.lambda0,
                        y: self.phi0,
                    },
                    None,
                ),
            },
        }
        self.sign0 = sign1;
    }

    fn line_end(&mut self) {
        // self.stream.line_end();
        match self.stream.clone() {
            LineSinkEnum::CB(mut stream) => stream.line_end(),
            LineSinkEnum::CSE(stream) => match stream {
                ClipSinkEnum::Blank => {
                    panic!("ClickSinkEnum - actively using an unconnected blank");
                }
                ClipSinkEnum::Src(mut stream) => stream.line_end(),
                ClipSinkEnum::Resample(mut stream) => stream.line_end(),
            },
        }

        self.lambda0 = T::nan();
        self.phi0 = T::nan();
    }
}
