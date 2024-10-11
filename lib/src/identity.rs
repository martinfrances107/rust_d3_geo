use core::fmt::Debug;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::thread;
use std::thread::JoinHandle;

use geo::CoordFloat;
use geo_types::Coord;

use crate::projection::projector_common::ChannelStatus;
use crate::projection::projector_common::Message;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::StreamMT;
use crate::stream::Unconnected;

/// Identity is a stream pipe line stage.
/// that acts as a pass through node.
#[derive(Clone, Debug)]
pub struct Identity<STATE> {
    state: STATE,
}

/// Not auto deriving here - it does not makes sense to provide
/// a default for the connected state.
impl Default for Identity<Unconnected> {
    #[inline]
    fn default() -> Self {
        Self { state: Unconnected }
    }
}

impl Connectable for Identity<Unconnected> {
    /// The resultant builder type.
    type Output<SC> = Identity<Connected<SC>>;

    #[inline]
    fn connect<SC>(&self, sink: SC) -> Self::Output<SC> {
        Identity {
            state: Connected { sink },
        }
    }
}

impl<EP, SINK, T> Stream for Identity<Connected<SINK>>
where
    SINK: Stream<EP = EP, T = T>,
    T: CoordFloat,
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
    fn point(&mut self, p: &Coord<Self::T>, m: Option<u8>) {
        self.state.sink.point(p, m);
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

impl<T> StreamMT<T> for Identity<Unconnected>
where
    T: 'static + CoordFloat + Send,
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
            'message_loop: loop {
                a = match rx.recv() {
                    Ok(message) => {
                        let res_tx = match message {
                            Message::Point(_)
                            | Message::EndPoint(_)
                            | Message::LineEnd
                            | Message::LineStart
                            | Message::PolygonStart
                            | Message::PolygonEnd
                            | Message::Sphere => tx.send(message),
                            Message::ShutDown
                            | Message::ShutDownWithReturn(_) => {
                                if let Err(e) = tx.send(Message::ShutDown) {
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
