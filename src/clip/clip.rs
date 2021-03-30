use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;
use std::ops::AddAssign;

use crate::clip::ClipTraitRaw;

// use super::antimeridian::line::Line as AntimeridianLine;
// use super::circle::line::Line as CircleLine;
use super::buffer::ClipBuffer;
use super::clip_base::ClipBase;
use super::ClipRaw;
use super::ClipSinkEnum;
use super::LineEnum;

use crate::clip::LineSinkEnum;
use crate::stream::Stream;
use crate::stream::StreamDst;

#[derive(Clone, Debug)]
pub struct Clip<T>
where
    T: CoordFloat + Default + FloatConst,
{
    raw: ClipRaw<T>,
    base: ClipBase<T>,
}

impl<T> Clip<T>
where
    T: CoordFloat + Default + FloatConst,
{
    #[inline]
    pub fn stream_in(&mut self, stream: ClipSinkEnum<T>)
    where
        T: CoordFloat + FloatConst,
    {
        self.base.sink = stream;
        match &mut self.base.line {
            LineEnum::Antimeridian(line) => {
                line.stream_in(LineSinkEnum::CSE(self.base.sink.clone()));
            }
            LineEnum::Circle(line) => {
                line.stream_in(LineSinkEnum::CSE(self.base.sink.clone()));
            }
        }
    }
}

impl<T> Clip<T>
where
    T: CoordFloat + Default + FloatConst,
{
    pub fn new(raw: ClipRaw<T>, start: Coordinate<T>) -> Self {
        // let mut line = raw.line.clone();

        let ring_buffer = ClipBuffer::default();
        match raw {
            ClipRaw::Antimeridian(ref r) => {
                let line;
                match &r.base.line {
                    LineEnum::Antimeridian(l) => {
                        line = l.clone();
                        let mut ring_sink = line.clone();
                        ring_sink.stream_in(LineSinkEnum::CB(ring_buffer));
                        Self {
                            raw,
                            base: ClipBase {
                                line: LineEnum::Antimeridian(line.clone()),
                                ring_sink: LineEnum::Antimeridian(ring_sink),
                                start,
                                ..ClipBase::default()
                            },
                        }
                    }
                    LineEnum::Circle(_) => {
                        panic!("mismatch ");
                    }
                }
            }

            ClipRaw::Circle(r) => match r.base.line {
                LineEnum::Antimeridian(ref l) => {
                    let line = l.clone();
                    let mut ring_sink = line.clone();
                    ring_sink.stream_in(LineSinkEnum::CB(ring_buffer));
                    Self {
                        raw: ClipRaw::Circle(r.clone()),
                        base: ClipBase {
                            line: LineEnum::Antimeridian(line),
                            ring_sink: LineEnum::Antimeridian(ring_sink),
                            start,
                            ..ClipBase::default()
                        },
                    }
                }
                LineEnum::Circle(ref l) => {
                    let line = l.clone();
                    let mut ring_sink = line.clone();
                    ring_sink.stream_in(LineSinkEnum::CB(ring_buffer.clone()));
                    Self {
                        raw: ClipRaw::Circle(r),
                        base: ClipBase {
                            line: LineEnum::Circle(line),
                            ring_sink: LineEnum::Circle(ring_sink),
                            start,
                            ..ClipBase::default()
                        },
                    }
                }
            },
        }
    }
}

impl<T> Stream<T> for Clip<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    type C = Coordinate<T>;
    fn get_dst(&self) -> StreamDst<T> {
        match &self.base.sink {
            ClipSinkEnum::Blank => {
                panic!("calling get_dst on a blank");
            }
            ClipSinkEnum::Resample(r) => r.get_dst(),
            ClipSinkEnum::Src(s) => s.get_dst(),
        }
    }
    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        match self.base.use_ring {
            true => {
                self.base.ring.push(*p);
                self.base.ring_sink.point(p, None);
            }
            false => {
                let pv = match &self.raw {
                    ClipRaw::Antimeridian(r) => r.point_visible(p, None),
                    ClipRaw::Circle(r) => r.point_visible(p, None),
                };
                if pv {
                    match &mut self.base.sink {
                        ClipSinkEnum::Blank => {
                            panic!("ClickSinkEnum - actively using an unconnected blank");
                        }
                        ClipSinkEnum::Src(sink) => {
                            sink.point(p, m);
                        }
                        ClipSinkEnum::Resample(sink) => {
                            sink.point(p, m);
                        }
                    }
                }
            }
        }
    }

    fn line_start(&mut self) {
        self.base.use_ring = false;
        self.base.line.line_start();
    }

    fn line_end(&mut self) {
        // if self.use_ring_end {
        //     self.ring_end();
        // } else {
        //     // put somethignhere.
        // }
        self.base.use_ring = true;
        // is this correct!!!
        self.base.line.line_end();
    }

    fn polygon_start(&mut self) {
        self.base.use_ring = true;
        self.base.use_ring_start = true;
        self.base.use_ring_end = true;
        self.base.segments.clear();
        self.base.polygon.clear();
    }

    fn polygon_end(&mut self) {
        self.base.use_ring = false;
        self.base.use_ring_start = false;
        self.base.use_ring_end = false;
        // segments = merge(segments);
        // let start_inside = contains(&self.polygon, &self.start);
        let start_inside = false;

        if !self.base.polygon_started {
            match &mut self.base.sink {
                ClipSinkEnum::Blank => {
                    panic!("ClickSinkEnum - actively using an unconnected blank");
                }
                ClipSinkEnum::Src(s) => s.polygon_start(),
                ClipSinkEnum::Resample(s) => s.polygon_start(),
            };
            self.base.polygon_started = true;

        // rejoin(
        //     &self.segments,
        //     self.compare_intersection,
        //     start_inside,
        //     self.interpolate,
        //     self.sink,
        // );
        } else if start_inside {
            if !self.base.polygon_started {
                match &mut self.base.sink {
                    ClipSinkEnum::Blank => {
                        panic!("ClickSinkEnum - actively using an unconnected blank");
                    }
                    ClipSinkEnum::Src(s) => s.polygon_start(),
                    ClipSinkEnum::Resample(s) => s.polygon_start(),
                };
                self.base.polygon_started = true;
            }
            match &mut self.base.sink {
                ClipSinkEnum::Blank => {
                    panic!("ClickSinkEnum - actively using an unconnected blank");
                }
                ClipSinkEnum::Src(s) => s.line_start(),
                ClipSinkEnum::Resample(s) => s.line_start(),
            };
            // (self.interpolate)(None, None, 1f64, self.sink);
            {
                // match &mut self.base.sink {
                //     ClipSinkEnum::Src(mut s) => s.line_end(),
                //     ClipSinkEnum::Resample(mut s) => s.line_end(),
                // };
            }
        }
        if self.base.polygon_started {
            match &mut self.base.sink {
                ClipSinkEnum::Blank => {
                    panic!("ClickSinkEnum - actively using an unconnected blank");
                }
                ClipSinkEnum::Src(s) => s.polygon_end(),
                ClipSinkEnum::Resample(s) => s.polygon_end(),
            };
            self.base.polygon_started = false;
        }
        self.base.segments.clear();
        self.base.polygon.clear();
    }

    fn sphere(&mut self) {
        match &mut self.base.sink {
            ClipSinkEnum::Blank => {
                panic!("ClickSinkEnum - actively using an unconnected blank");
            }
            ClipSinkEnum::Src(s) => {
                s.polygon_start();
                s.line_start();
            }
            ClipSinkEnum::Resample(s) => {
                s.polygon_start();
                s.line_start();
            }
        };
        // (self.interpolate)(None, None, T::one(), &mut sink as &mut dyn Stream<T>);
        match &mut self.base.sink {
            ClipSinkEnum::Blank => {
                panic!("ClickSinkEnum - actively using an unconnected blank");
            }
            ClipSinkEnum::Src(s) => {
                s.line_end();
                s.polygon_end();
            }
            ClipSinkEnum::Resample(s) => {
                s.line_end();
                s.polygon_end();
            }
        };
    }
}
