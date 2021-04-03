use derivative::Derivative;
use geo::CoordFloat;
use geo::Coordinate;
use num_traits::FloatConst;
use rust_d3_array::merge::merge;
use std::ops::AddAssign;

use crate::path::PathResultEnum;
use crate::polygon_contains::contains;
use crate::stream::stream_dst::StreamDst;
use crate::stream::Clean;
use crate::stream::CleanEnum;
use crate::stream::Stream;

use super::buffer::ClipBuffer;
use super::buffer::LineElem;
use super::clip_base::ClipBase;
use super::clip_raw::ClipRaw;
use super::clip_sink_enum::ClipSinkEnum;
use super::line_enum::LineEnum;
use super::line_sink_enum::LineSinkEnum;
use super::ClipTraitRaw;

#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone)]
pub struct Clip<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    raw: ClipRaw<T>,
    base: ClipBase<T>,
    #[derivative(Debug = "ignore")]
    point_fn: fn(&mut Self, p: &Coordinate<T>, m: Option<u8>),
    #[derivative(Debug = "ignore")]
    line_start_fn: fn(&mut Self),
    #[derivative(Debug = "ignore")]
    line_end_fn: fn(&mut Self),
}

impl<T> Clip<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
{
    pub fn new(raw: ClipRaw<T>, start: Coordinate<T>) -> Self {
        let ring_buffer = LineSinkEnum::CB(ClipBuffer::default());
        match raw {
            ClipRaw::Antimeridian(r) => match &r.base.line {
                LineEnum::Antimeridian(l) => {
                    let mut ring_sink = LineEnum::Antimeridian(l.clone());
                    ring_sink.stream_in(ring_buffer);
                    Self {
                        raw: ClipRaw::Antimeridian(r.clone()),
                        base: ClipBase {
                            line: LineEnum::Antimeridian(l.clone()),
                            ring_sink,
                            start,
                            ..ClipBase::default()
                        },
                        point_fn: Self::point_default,
                        line_start_fn: Self::line_start_default,
                        line_end_fn: Self::line_end_default,
                    }
                }
                LineEnum::Circle(_) => {
                    panic!("mismatch ");
                }
            },

            ClipRaw::Circle(r) => match r.base.line {
                LineEnum::Antimeridian(ref l) => {
                    let line = l.clone();
                    let mut ring_sink = line.clone();
                    ring_sink.stream_in(ring_buffer);
                    Self {
                        raw: ClipRaw::Circle(r.clone()),
                        base: ClipBase {
                            line: LineEnum::Antimeridian(line),
                            ring_sink: LineEnum::Antimeridian(ring_sink),
                            start,
                            ..ClipBase::default()
                        },
                        point_fn: Self::point_default,
                        line_start_fn: Self::line_start_default,
                        line_end_fn: Self::line_end_default,
                    }
                }
                LineEnum::Circle(ref l) => {
                    let line = l.clone();
                    let mut ring_sink = line.clone();
                    ring_sink.stream_in(ring_buffer);
                    Self {
                        raw: ClipRaw::Circle(r),
                        base: ClipBase {
                            line: LineEnum::Circle(line),
                            ring_sink: LineEnum::Circle(ring_sink),
                            start,
                            ..ClipBase::default()
                        },
                        point_fn: Self::point_default,
                        line_start_fn: Self::line_start_default,
                        line_end_fn: Self::line_end_default,
                    }
                }
            },
        }
    }
}

impl<T> Clip<T>
where
    T: AddAssign + CoordFloat + Default + FloatConst,
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

    fn point_default(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        println!("Clip point_default()");
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

    #[inline]
    fn point_line(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        println!("Clip point_line()");
        self.base.line.point(p, m);
    }

    #[inline]
    fn point_ring(&mut self, p: &Coordinate<T>, m: Option<u8>) {
        println!("Clip point_ring()");
        self.base.ring.push(*p);
        self.base.ring_sink.point(p, m);
    }

    #[inline]
    fn line_start_default(&mut self) {
        println!("Clip line_start_default()");
        self.point_fn = Self::point_line;
        self.base.line.line_start();
    }

    #[inline]
    fn ring_start(&mut self) {
        println!("Clip ring_start()");
        self.base.ring_sink.line_start();
        self.base.ring.clear();
    }

    #[inline]
    fn line_end_default(&mut self) {
        println!("Clip line_end_default()");
        self.point_fn = Self::point_default;
        self.base.line.line_end();
    }

    fn ring_end(&mut self) {
        println!("Clip ring_end()");
        self.point_ring(&self.base.ring[0].clone(), None);
        self.base.ring_sink.line_end();

        let clean = self.base.ring_sink.clean();
        // deviation from javascript.
        // access to the javascript varible 'ring_buffer' is
        // through the ring_sink varible.
        let mut ring_segments = match self.base.ring_sink.get_stream().result() {
            Some(PathResultEnum::ClipBufferOutput(result)) => {
                // Can I find a way of doing this with the expense of dynamic conversion.
                result
            }
            Some(_) => {
                panic!("None buffer ");
            }
            // _ => {
            //     panic!("was expectcing a path result");
            // },
            None => panic!("was expecting something."),
        };

        let n = ring_segments.len();
        let m;
        let segment: Vec<Vec<Coordinate<T>>>;
        let point: Coordinate<T>;

        self.base.ring.pop();
        // self.base.polygon.push(self.base.ring);
        // in this javascript version this value is set to NULL
        // is my assumption that this is valid true?
        // self.ring = None;
        self.base.ring = Vec::new();

        if n != 0 {
            return;
        }

        // No intersections.
        match clean {
            CleanEnum::NoIntersections => {
                let segment = ring_segments.first().unwrap().clone();
                m = segment.len() - 1;
                if m > 0 {
                    if !self.base.polygon_started {
                        self.base.sink.polygon_start();
                        self.base.polygon_started = true;
                    }
                    self.base.sink.line_start();
                    for i in 0..m {
                        println!("layer below point()");
                        let le = segment[i];
                        self.base.sink.point(&le.p, le.m);
                    }
                    self.base.sink.line_end();
                }
                return;
            }
            CleanEnum::IntersectionsRejoin => {
                // Rejoin connected segments.
                // TODO reuse ringBuffer.rejoin()?
                if n > 1 {
                    // ringSegments.push(ringSegments.pop().concat(ringSegments.shift()));

                    let mut combined = ring_segments.first().unwrap().clone();
                    let mut last = ring_segments.last().unwrap().clone();
                    combined.append(&mut last);
                    ring_segments.push(combined);
                }
            }
            _ => {}
        }

        let mut filtered: Vec<Vec<LineElem<T>>> = ring_segments
            .iter()
            .filter(|segment| segment.len() > 1)
            .map(|s| s.clone())
            .collect();
        self.base.segments.append(&mut filtered);
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
                panic!("calling get_dst() on a blank");
            }
            ClipSinkEnum::Resample(r) => r.get_dst(),
            ClipSinkEnum::Src(s) => s.get_dst(),
        }
    }

    #[inline]
    fn point(&mut self, p: &Self::C, m: Option<u8>) {
        (self.point_fn)(self, p, m);
    }

    #[inline]
    fn line_start(&mut self) {
        println!("line_start()");
        (self.line_start_fn)(self);
    }

    #[inline]
    fn line_end(&mut self) {
        (self.line_end_fn)(self);
    }

    fn polygon_start(&mut self) {
        println!("Clip polygon start()");
        self.point_fn = Self::point_ring;
        self.line_start_fn = Self::ring_start;
        self.line_end_fn = Self::ring_end;
        self.base.segments.clear();
        self.base.polygon.clear();
    }

    fn polygon_end(&mut self) {
        println!("Clip polygon_end()");
        self.point_fn = Self::point_default;
        self.line_start_fn = Self::line_start_default;
        self.line_end_fn = Self::line_end_default;
        let segments_merged = merge(self.base.segments.clone());
        let start_inside = contains(&self.base.polygon, &self.base.start);

        if !segments_merged.is_empty() {
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
            // rejoin(
            //     &segments_merged,
            //     self.raw.compare_intersection,
            //     start_inside,
            //     self.interpolate,
            //     self.base.sink,
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
                ClipSinkEnum::Src(s) => {
                    s.line_start();
                    self.raw.interpolate(None, None, T::one(), s);
                    s.line_end();
                }
                ClipSinkEnum::Resample(s) => {
                    s.line_start();
                    self.raw.interpolate(None, None, T::one(), s);
                    s.line_end();
                }
            };
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
            ClipSinkEnum::Src(s) => {
                s.polygon_start();
                s.line_start();
                self.raw.interpolate(None, None, T::one(), s);
                s.line_end();
                s.polygon_end();
            }
            ClipSinkEnum::Resample(s) => {
                s.polygon_start();
                s.line_start();
                self.raw.interpolate(None, None, T::one(), s);
                s.line_end();
                s.polygon_end();
            }
            ClipSinkEnum::Blank => {
                panic!("ClickSinkEnum - actively using an unconnected blank");
            }
        };
    }
}
