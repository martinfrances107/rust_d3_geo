use core::fmt::Debug;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

use geo::CoordFloat;
use geo_types::Coord;
use num_traits::FloatConst;

use crate::compose::Compose;
use crate::projection::projector_common::ChannelStatus;
use crate::projection::projector_common::Message;
use crate::projection::transform::scale_translate_rotate::ScaleTranslateRotate;
use crate::stream::Connectable;
use crate::stream::Connected;
use crate::stream::Stream;
use crate::stream::StreamMT;
use crate::stream::Unconnected;
use crate::Transform;

/// Resample None.
///
/// A pass-through module, when no resampling is required.
#[derive(Clone, Debug)]
pub struct None<PR, STATE, T> {
    state: STATE,
    projection_transform: Compose<PR, ScaleTranslateRotate<T>>,
}

impl<PR, T> None<PR, Unconnected, T> {
    #[inline]
    /// Constructor: Resample None.
    pub const fn new(
        projection_transform: Compose<PR, ScaleTranslateRotate<T>>,
    ) -> Self {
        Self {
            state: Unconnected,
            projection_transform,
        }
    }
}

impl<PR, T> Connectable for None<PR, Unconnected, T>
where
    PR: Clone,
    T: CoordFloat,
{
    type Output<SC> = None<PR, Connected<SC>, T>;

    #[inline]
    fn connect<SC>(&self, sink: SC) -> Self::Output<SC> {
        None::<PR, Connected<SC>, T> {
            state: Connected { sink },
            projection_transform: self.projection_transform.clone(),
        }
    }
}

impl<EP, PR, SC, T> Stream for None<PR, Connected<SC>, T>
where
    SC: Stream<EP = EP, T = T>,
    PR: Transform<T = T>,
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

    fn point(&mut self, p: &Coord<T>, m: Option<u8>) {
        let t = &self.projection_transform.transform(p);
        self.state.sink.point(t, m);
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

impl<PR, T> StreamMT<T> for None<PR, Unconnected, T>
where
    PR: 'static + Transform<T = T> + Send,
    T: 'static + CoordFloat + FloatConst + Send,
{
    /// Generate a thread which stage on the responsibility of the
    /// `StreamTransformRadians` pipeline stage.
    ///
    /// Consumes a Self
    fn gen_stage(
        self,
        tx: Sender<Message<T>>,
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
                                let p_trans =
                                    self.projection_transform.transform(&p);
                                let message = Message::Point((p_trans, m));
                                tx.send(message)
                            }
                            Message::EndPoint(_)
                            | Message::LineEnd
                            | Message::LineStart
                            | Message::PolygonStart
                            | Message::PolygonEnd
                            | Message::Sphere
                            | Message::ShutDown
                            | Message::ShutDownWithReturn(_) => {
                                tx.send(message)
                            }
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
