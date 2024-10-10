use std::sync::mpsc::{Receiver, SyncSender};
use std::thread::{self, JoinHandle};

use geo::CoordFloat;
use geo_types::Coord;

use crate::path::Result;
use crate::projection::projector_common::{ChannelStatus, Message};
use crate::stream::{EndPointMT, Stream, StreamMT};

/// Stream endpoint: Retain the last point.
///
/// This endpoint is used in the `AlbersUSA` projection.
/// If serves as a point mask. The albers has clipping bounds
/// and if a point flows through the pipe line and is retained
/// by `LastPoint` it is in alaska, the lower 48 or in hawaii.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct LastPoint<T: CoordFloat>(Option<Coord<T>>);

impl<T> Stream for LastPoint<T>
where
    T: CoordFloat,
{
    type EP = Self;
    type T = T;

    #[inline]
    fn endpoint<'a>(&mut self) -> &mut Self {
        self
    }

    #[inline]
    fn point(&mut self, p: &Coord<T>, _m: Option<u8>) {
        self.0 = Some(*p);
    }
}

impl<T> StreamMT<T> for LastPoint<T>
where
    T: 'static + CoordFloat + Send,
{
    /// Generate a thread which stage on the responsibility of the
    /// `StreamTransformRadians` pipeline stage.
    ///
    /// Consumes a Self
    fn gen_stage(
        mut self,
        tx: SyncSender<Message<T>>,
        rx: Receiver<Message<T>>,
    ) -> JoinHandle<ChannelStatus<T>> {
        // Stage pipelines.
        thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            let a;
            'message_loop: loop {
                a = match rx.recv() {
                    Ok(message) => {
                        let res_tx = match message {
                            Message::Point((p, _m)) => {
                                self.0 = Some(p);
                                Ok(())
                            }
                            Message::EndPoint(_) => {
                                if let Err(e) = tx.send(Message::EndPoint(
                                    EndPointMT::LastPoint(self.clone()),
                                )) {
                                    return ChannelStatus::Tx(e);
                                }
                                Ok(())
                            }
                            Message::LineEnd
                            | Message::LineStart
                            | Message::PolygonStart
                            | Message::PolygonEnd
                            | Message::Sphere => {
                                // NoOp
                                Ok(())
                            }
                            Message::ShutDown => {
                                if let Err(e) = tx.send(message) {
                                    return ChannelStatus::Tx(e);
                                }
                                return ChannelStatus::ShuntDownReceived;
                            }
                            Message::ShutDownWithReturn(_dummy) => {
                                if let Err(e) =
                                    tx.send(Message::ShutDownWithReturn(
                                        EndPointMT::LastPoint(self.clone()),
                                    ))
                                {
                                    return ChannelStatus::Tx(e);
                                }
                                return ChannelStatus::ShuntDownReceived;
                            }
                        };
                        match res_tx {
                            Ok(()) => {
                                continue 'message_loop;
                            }
                            Err(e) => ChannelStatus::Tx(e),
                        }
                    }
                    Err(e) => ChannelStatus::Rx(e),
                };
                break;
            }
            a
        })
    }
}

impl<T> Result for LastPoint<T>
where
    T: CoordFloat,
{
    type Out = Option<Coord<T>>;

    fn result(&mut self) -> Self::Out {
        let out = self.0;
        self.0 = None;
        out
    }
}
