use geo::{CoordFloat, Coordinate};
use num_traits::FloatConst;

use crate::path::PathResultEnum;
use crate::stream::Stream;
use crate::stream::StreamClipLine;
// use crate::stream::StreamClipLineNode;
use crate::clip::ClipBuffer;
// use crate::stream::StreamClone;
use crate::stream::StreamPathResult;
// use crate::stream::StreamPathResultTrait;
use crate::clip::ClipSinkEnum;
use crate::stream::stream_path_result_node_stub::StreamPathResultNodeStub;
use crate::stream::{Clean, CleanEnum, StreamClean};

use super::intersect::intersect;

#[derive(Clone)]
pub struct Line<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    clean: CleanEnum,
    lambda0: T,
    phi0: T,
    sign0: T,
    // stream: Box<dyn StreamPathResult<C = Coordinate<T>, Out = Option<PathResultEnum<T>>>>,
    stream: LineSinkEnum<T>,
}

// impl<T: CoordFloat + FloatConst + Default + 'static> StreamClone for Line<T> {
//     type RetType = Box<dyn StreamPathResult<C = Coordinate<T>, Out = PathResultEnum<T>>>;
//     // fn box_clone(&self) -> Box<dyn StreamPathResult<C = Coordinate<T>, Out = PathResultEnum<T>>> {
//     fn box_clone(&self) -> Self::RetType {
//         Box::new(self)
//     }
// }

// impl<T> Clone for Line<T>
// where
//     T: CoordFloat + FloatConst + Default + 'static,
// {
//     fn clone(&self) -> Self {
//         Self {
//             stream: self.stream.box_clone(),
//             ..*self
//         }
//     }
// }

impl<T> Default for Line<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    #[inline]
    fn default() -> Self {
        Line {
            clean: CleanEnum::IntersectionsOrEmpty,
            lambda0: T::nan(),
            phi0: T::nan(),
            sign0: T::nan(),
            // stream: Box::new(StreamPathResultNodeStub::default()),
            stream: LineSinkEnum::CB(ClipBuffer::default()),
        }
    }
}
impl<T> StreamClipLine for Line<T>
where
    T: CoordFloat + FloatConst + Default + 'static,
{
    // #[inline]
    // fn box_clone(&self) -> Box<dyn StreamClipLine<C = Self::C, BitCB = Self::BitCB>> {
    //     Box::new(self.clone())
    // }
}

use crate::clip::LineSinkEnum;
impl<T> Line<T>
where
    T: CoordFloat + Default + FloatConst,
{
    // type BitSink = Box<dyn StreamPathResult<Out = Option<PathResultEnum<T>>, C = Coordinate<T>>>;

    #[inline]
    pub fn stream_in(
        &mut self,
        // stream: Box<(dyn StreamPathResult<C = Coordinate<T>, Out = Option<PathResultEnum<T>>>)>,
        stream: LineSinkEnum<T>,
    ) {
        self.stream = stream;
    }
}

// impl<T> BufferInTrait for Line<T>
// where
//     T: CoordFloat + FloatConst,
// {
//     // type BitSink = Box<dyn StreamPathResult<Out = Option<PathResultEnum<T>>, C = Coordinate<T>>>;
//     type BitCB = ClipBuffer<T>;
//     #[inline]
//     fn buffer_in(&mut self, &buffer: Self::BitCB) {
//         // self.stream = stream;
//     }
// }

impl<T> Clean for Line<T>
where
    T: CoordFloat + FloatConst + Default,
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

impl<T> StreamClean<T> for Line<T> where T: CoordFloat + FloatConst + Default + 'static {}

impl<T: CoordFloat + FloatConst + Default + 'static> Stream for Line<T> {
    type C = Coordinate<T>;
    fn line_start(&mut self) {
        // self.stream.line_start();
        match self.stream.clone() {
            LineSinkEnum::CSE(stream) => {
                match stream {
                    ClipSinkEnum::Resample(mut stream) => stream.line_start(),
                    ClipSinkEnum::Src(mut stream) => stream.line_start(),
                }
                // stream.line_start()
            }
            LineSinkEnum::CB(mut stream) => stream.line_start(),
        }
        self.clean = CleanEnum::NoIntersections;
    }

    fn point(&mut self, p: Self::C, _m: Option<u8>) {
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
            // match (self.phi0 + phi1 / f_2).is_sign_positive() {
            //     true => {
            //         self.stream.point(
            //             Coordinate {
            //                 x: self.lambda0,
            //                 y: T::FRAC_PI_2(),
            //             },
            //             None,
            //         );
            //     }
            //     false => {
            //         self.stream.point(
            //             Coordinate {
            //                 x: self.lambda0,
            //                 y: -T::FRAC_PI_2(),
            //             },
            //             None,
            //         );
            //     }
            // }
            // self.stream.point(
            //     Coordinate {
            //         x: self.sign0,
            //         y: self.phi0,
            //     },
            //     None,
            // );
            // self.stream.line_end();
            // self.stream.line_start();
            // self.stream.point(
            //     Coordinate {
            //         x: sign1,
            //         y: self.phi0,
            //     },
            //     None,
            // );
            // self.stream.point(
            //     Coordinate {
            //         x: lambda1,
            //         y: self.phi0,
            //     },
            //     None,
            // );

            match self.stream.clone() {
                LineSinkEnum::CB(mut stream) => {
                    match (self.phi0 + phi1 / f_2).is_sign_positive() {
                        true => {
                            stream.point(
                                Coordinate {
                                    x: self.lambda0,
                                    y: T::FRAC_PI_2(),
                                },
                                None,
                            );
                        }
                        false => {
                            stream.point(
                                Coordinate {
                                    x: self.lambda0,
                                    y: -T::FRAC_PI_2(),
                                },
                                None,
                            );
                        }
                    }
                    stream.point(
                        Coordinate {
                            x: self.sign0,
                            y: self.phi0,
                        },
                        None,
                    );
                    stream.line_end();
                    stream.line_start();
                    stream.point(
                        Coordinate {
                            x: sign1,
                            y: self.phi0,
                        },
                        None,
                    );
                    stream.point(
                        Coordinate {
                            x: lambda1,
                            y: self.phi0,
                        },
                        None,
                    );
                }
                LineSinkEnum::CSE(stream) => match stream {
                    ClipSinkEnum::Src(mut stream) => {
                        match (self.phi0 + phi1 / f_2).is_sign_positive() {
                            true => {
                                stream.point(
                                    Coordinate {
                                        x: self.lambda0,
                                        y: T::FRAC_PI_2(),
                                    },
                                    None,
                                );
                            }
                            false => {
                                stream.point(
                                    Coordinate {
                                        x: self.lambda0,
                                        y: -T::FRAC_PI_2(),
                                    },
                                    None,
                                );
                            }
                        }
                        stream.point(
                            Coordinate {
                                x: self.sign0,
                                y: self.phi0,
                            },
                            None,
                        );
                        stream.line_end();
                        stream.line_start();
                        stream.point(
                            Coordinate {
                                x: sign1,
                                y: self.phi0,
                            },
                            None,
                        );
                        stream.point(
                            Coordinate {
                                x: lambda1,
                                y: self.phi0,
                            },
                            None,
                        );
                    }
                    ClipSinkEnum::Resample(mut stream) => {
                        match (self.phi0 + phi1 / f_2).is_sign_positive() {
                            true => {
                                stream.point(
                                    Coordinate {
                                        x: self.lambda0,
                                        y: T::FRAC_PI_2(),
                                    },
                                    None,
                                );
                            }
                            false => {
                                stream.point(
                                    Coordinate {
                                        x: self.lambda0,
                                        y: -T::FRAC_PI_2(),
                                    },
                                    None,
                                );
                            }
                        }
                        stream.point(
                            Coordinate {
                                x: self.sign0,
                                y: self.phi0,
                            },
                            None,
                        );
                        stream.line_end();
                        stream.line_start();
                        stream.point(
                            Coordinate {
                                x: sign1,
                                y: self.phi0,
                            },
                            None,
                        );
                        stream.point(
                            Coordinate {
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
            // self.stream.point(
            //     Coordinate {
            //         x: self.sign0,
            //         y: self.phi0,
            //     },
            //     None,
            // );
            // self.stream.line_end();
            // //  self.stream.line_start();
            // self.stream.point(
            //     Coordinate {
            //         x: sign1,
            //         y: self.phi0,
            //     },
            //     None,
            // );
            match self.stream.clone() {
                LineSinkEnum::CB(mut stream) => {
                    stream.point(
                        Coordinate {
                            x: self.sign0,
                            y: self.phi0,
                        },
                        None,
                    );
                    stream.line_end();
                    //  self.stream.line_start();
                    stream.point(
                        Coordinate {
                            x: sign1,
                            y: self.phi0,
                        },
                        None,
                    );
                }
                LineSinkEnum::CSE(stream) => match stream {
                    ClipSinkEnum::Src(mut stream) => {
                        stream.point(
                            Coordinate {
                                x: self.sign0,
                                y: self.phi0,
                            },
                            None,
                        );
                        stream.line_end();
                        //  self.stream.line_start();
                        stream.point(
                            Coordinate {
                                x: sign1,
                                y: self.phi0,
                            },
                            None,
                        );
                    }
                    ClipSinkEnum::Resample(mut stream) => {
                        stream.point(
                            Coordinate {
                                x: self.sign0,
                                y: self.phi0,
                            },
                            None,
                        );
                        stream.line_end();
                        //  self.stream.line_start();
                        stream.point(
                            Coordinate {
                                x: sign1,
                                y: self.phi0,
                            },
                            None,
                        );
                    }
                },
            }

            match self.stream.clone() {
                LineSinkEnum::CB(mut stream) => {
                    stream.point(
                        Coordinate {
                            x: self.sign0,
                            y: self.phi0,
                        },
                        None,
                    );
                    stream.line_end();
                    //  self.stream.line_start();
                    stream.point(
                        Coordinate {
                            x: sign1,
                            y: self.phi0,
                        },
                        None,
                    );
                }
                LineSinkEnum::CSE(stream) => match stream {
                    ClipSinkEnum::Src(mut stream) => {
                        stream.point(
                            Coordinate {
                                x: self.sign0,
                                y: self.phi0,
                            },
                            None,
                        );
                        stream.line_end();
                        //  self.stream.line_start();
                        stream.point(
                            Coordinate {
                                x: sign1,
                                y: self.phi0,
                            },
                            None,
                        );
                    }
                    ClipSinkEnum::Resample(mut stream) => {
                        stream.point(
                            Coordinate {
                                x: self.sign0,
                                y: self.phi0,
                            },
                            None,
                        );
                        stream.line_end();
                        //  self.stream.line_start();
                        stream.point(
                            Coordinate {
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
        match self.stream.clone() {
            LineSinkEnum::CB(mut stream) => stream.point(
                Coordinate {
                    x: self.lambda0,
                    y: self.phi0,
                },
                None,
            ),
            LineSinkEnum::CSE(stream) => match stream {
                ClipSinkEnum::Src(mut stream) => stream.point(
                    Coordinate {
                        x: self.lambda0,
                        y: self.phi0,
                    },
                    None,
                ),
                ClipSinkEnum::Resample(mut stream) => stream.point(
                    Coordinate {
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
                ClipSinkEnum::Src(mut stream) => stream.line_end(),
                ClipSinkEnum::Resample(mut stream) => stream.line_end(),
            },
        }

        self.lambda0 = T::nan();
        self.phi0 = T::nan();
    }
}
