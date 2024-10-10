use core::fmt::Debug;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::SyncSender;
use std::thread;
use std::thread::JoinHandle;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::projection::projector_common::ChannelStatus;
use crate::projection::projector_common::Message;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::StreamMT;
use crate::stream::Unconnected;
use crate::Transform;

use super::rotate_radians::RotateRadians;

/// A Stream node, that applied a rotator transform.
#[derive(Debug, Clone)]
pub struct RotatorRadians<STATE, T>
where
    T: CoordFloat,
{
    state: STATE,
    rotate: RotateRadians<T>,
}

impl<T> RotatorRadians<Unconnected, T>
where
    T: CoordFloat,
{
    /// Constructor.
    #[inline]
    pub(crate) const fn new(rotate: RotateRadians<T>) -> Self {
        Self {
            state: Unconnected,
            rotate,
        }
    }
}

impl<T> Connectable for RotatorRadians<Unconnected, T>
where
    T: CoordFloat,
{
    type Output<SINK> = RotatorRadians<Connected<SINK>, T>;
    /// Connects the next stage in the stream pipeline.
    #[inline]
    fn connect<SINK>(&self, sink: SINK) -> Self::Output<SINK> {
        RotatorRadians {
            state: Connected { sink },
            rotate: self.rotate.clone(),
        }
    }
}

impl<EP, SINK, T> Stream for RotatorRadians<Connected<SINK>, T>
where
    SINK: Stream<EP = EP, T = T>,
    T: CoordFloat + FloatConst,
{
    type EP = EP;
    type T = T;

    #[inline]
    fn endpoint(&mut self) -> &mut Self::EP {
        self.state.sink.endpoint()
    }

    #[inline]
    fn line_end(&mut self) {
        self.state.sink.line_end();
    }

    #[inline]
    fn line_start(&mut self) {
        self.state.sink.line_start();
    }

    #[inline]
    fn point(&mut self, p: &Coord<T>, m: Option<u8>) {
        self.state.sink.point(&self.rotate.transform(p), m);
    }

    #[inline]
    fn polygon_end(&mut self) {
        self.state.sink.polygon_end();
    }

    #[inline]
    fn polygon_start(&mut self) {
        self.state.sink.polygon_start();
    }

    #[inline]
    fn sphere(&mut self) {
        self.state.sink.sphere();
    }
}

impl<T> StreamMT<T> for RotatorRadians<Unconnected, T>
where
    T: 'static + CoordFloat + FloatConst + Send,
{
    /// Generate a thread which stage on the responsibility of the
    /// `StreamTransformRadians` pipeline stage.
    ///
    /// Consumes a Self
    fn gen_stage(
        self,
        tx: SyncSender<Message<T>>,
        rx: Receiver<Message<T>>,
    ) -> JoinHandle<ChannelStatus<T>> {
        // Stage pipelines.
        thread::spawn(move || {
            // The thread takes ownership over `thread_tx`
            // Each thread queues a message in the channel
            let a;
            loop {
                a = match rx.recv() {
                    Ok(message) => {
                        let res_tx = match message {
                            Message::Point((p, m)) => {
                                let p_trans = self.rotate.transform(&p);
                                let message = Message::Point((p_trans, m));
                                tx.send(message)
                            }
                            Message::ShutDownWithReturn(_)
                            | Message::EndPoint(_)
                            | Message::LineEnd
                            | Message::LineStart
                            | Message::PolygonStart
                            | Message::PolygonEnd
                            | Message::Sphere
                            | Message::ShutDown => tx.send(message),
                        };
                        match res_tx {
                            Ok(()) => {
                                continue;
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
