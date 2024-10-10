use core::fmt::Debug;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::thread;
use std::thread::JoinHandle;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::StreamMT;
use crate::stream::Unconnected;

use super::projector_common::ChannelStatus;
use super::projector_common::Message;

// A path node.
//
/// Type-Driven API, STATE prevent calls to `Self::connect()`
/// on a perviously connected object
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamTransformRadians<STATE, T> {
    state: STATE,
    frac_pi_180: T,
}

impl<T> Connectable for StreamTransformRadians<Unconnected, T>
where
    T: CoordFloat,
{
    type Output<SC> = StreamTransformRadians<Connected<SC>, T>;
    #[inline]
    /// Connect this node to the next node on the path.
    fn connect<SC>(&self, sink: SC) -> Self::Output<SC> {
        StreamTransformRadians {
            state: Connected { sink },
            frac_pi_180: self.frac_pi_180,
        }
    }
}
/// Not auto deriving here - it does not makes sense to provide
/// a default for the connected state.
impl<T> Default for StreamTransformRadians<Unconnected, T>
where
    T: CoordFloat + FloatConst,
{
    #[inline]
    fn default() -> Self {
        Self {
            state: Unconnected,
            frac_pi_180: T::PI() / T::from(180).unwrap(),
        }
    }
}

impl<EP, T, SINK> Stream for StreamTransformRadians<Connected<SINK>, T>
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
    fn point(&mut self, p: &Coord<T>, m: Option<u8>) {
        self.state.sink.point(
            &Coord {
                x: p.x * self.frac_pi_180,
                y: p.y * self.frac_pi_180,
            },
            m,
        );
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

impl<T> StreamMT<T> for StreamTransformRadians<Unconnected, T>
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
                            Message::Point((p, m)) => {
                                let p_trans = Coord {
                                    x: p.x * self.frac_pi_180,
                                    y: p.y * self.frac_pi_180,
                                };
                                let message = Message::Point((p_trans, m));
                                tx.send(message)
                            }
                            Message::EndPoint(_)
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
