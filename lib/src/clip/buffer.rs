use std::collections::VecDeque;
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread::{self, JoinHandle};

use geo::Coord;
use geo::CoordFloat;

use crate::path::Result;
use crate::projection::projector_common::{ChannelStatus, Message};
use crate::stream::{Stream, StreamMT};

use super::line_elem::LineElem;

/// Buffer is a path endpoint.
///
/// Stored data can be extracted via [`Buffer::result()`]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Buffer<T>
where
    T: CoordFloat,
{
    /// Clip segments.
    pub lines: VecDeque<Vec<LineElem<T>>>,
}

impl<T> Default for Buffer<T>
where
    T: CoordFloat,
{
    #[inline]
    fn default() -> Self {
        Self {
            lines: VecDeque::default(),
        }
    }
}

impl<T> Result for Buffer<T>
where
    T: CoordFloat,
{
    type Out = VecDeque<Vec<LineElem<T>>>;

    fn result(&mut self) -> Self::Out {
        let mut result = VecDeque::new();
        core::mem::swap(&mut result, &mut self.lines);
        result
    }
}

impl<T> Buffer<T>
where
    T: CoordFloat,
{
    /// Stitch first and last elements together.
    pub(super) fn rejoin(&mut self) {
        if self.lines.len() > 1 {
            let line_last = self
                .lines
                .pop_back()
                .expect("rejoin: len() > 1 guarantees pop_back succeeds");
            let line_first = self
                .lines
                .pop_front()
                .expect("rejoin: len() > 1 guarantees pop_front succeeds");

            // Pre-allocate combined vector to avoid intermediate allocations
            let mut combined =
                Vec::with_capacity(line_last.len() + line_first.len());
            combined.extend(line_last);
            combined.extend(line_first);
            self.lines.push_back(combined);
        }
    }
}

impl<T> Stream for Buffer<T>
where
    T: CoordFloat,
{
    type T = T;
    type EP = Self;

    fn endpoint(&mut self) -> &mut Self::EP {
        self
    }

    #[inline]
    fn point(&mut self, p: &Coord<T>, m: Option<u8>) {
        self.lines.back_mut().map_or_else(
            || panic!("buffers: lines was not properly initialised."),
            |line| {
                line.push(LineElem { p: *p, m });
            },
        );
    }

    #[inline]
    fn line_start(&mut self) {
        // Create empty vector - capacity will grow as needed when points are added
        self.lines.push_back(Vec::new());
    }
}

impl<T> StreamMT<T> for Buffer<T>
where
    T: 'static + CoordFloat + Send,
{
    fn gen_stage(
        mut self,
        _tx: SyncSender<Message<T>>,
        rx: Receiver<Message<T>>,
    ) -> JoinHandle<ChannelStatus<T>> {
        // Stage pipelines.
        thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(message) => {
                        match message {
                            Message::Point((p, m)) => {
                                self.lines.back_mut().map_or_else(
                                    || panic!("buffers: lines was not properly initialised."),
                                    |line| {
                                        line.push(LineElem { p, m });
                                    },
                                );
                            }
                            Message::LineStart => {
                                self.lines.push_back(Vec::new());
                            }
                            // TODO is EndPoint a NoOP?
                            // Should I pass Sphere
                            Message::EndPoint(_)
                            | Message::LineEnd
                            | Message::PolygonStart
                            | Message::PolygonEnd
                            | Message::Sphere => {
                                // No-op for these message types
                            }
                            Message::ShutDown => {
                                // Gracefully exit on shutdown
                                return ChannelStatus::ShuntDownReceived;
                            }
                            Message::ShutDownWithReturn(_end_point_mt) => {
                                // TODO: Handle returning the endpoint
                                // For now, exit gracefully
                                return ChannelStatus::ShuntDownReceived;
                            }
                        }
                        // Continue processing more messages
                    }
                    Err(e) => {
                        // Channel closed or error receiving
                        return ChannelStatus::Rx(e);
                    }
                }
            }
        })
    }
}
