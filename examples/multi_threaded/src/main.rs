use std::sync::mpsc::{self, Receiver, Sender};

use d3_geo_rs::{
    last_point::LastPoint,
    path::Result,
    projection::{
        projector_common::Message,
        stream_transform_radians::StreamTransformRadians,
    },
    stream::{EndPointMT, StreamMT},
};
use geo_types::Coord;

fn main() {
    let (tx1, rx1): (Sender<Message<f64>>, Receiver<Message<f64>>) =
        mpsc::channel();
    let (tx2, rx2): (Sender<Message<f64>>, Receiver<Message<f64>>) =
        mpsc::channel();
    let (tx3, rx3): (Sender<Message<f64>>, Receiver<Message<f64>>) =
        mpsc::channel();

    let stage1 = StreamTransformRadians::default().gen_stage(tx2, rx1);
    // handles.push(stage1);

    let stage2 = LastPoint::default().gen_stage(tx3, rx2);

    let handles = [stage1, stage2];

    for i in 1..100_000 {
        let p = Coord {
            x: i as f64,
            y: i as f64,
        };

        if let Err(e) = tx1.send(Message::Point((p, None))) {
            panic!("Broken pipe sending point {i} {e}");
        }

        if let Err(e) = tx1.send(Message::EndPoint(EndPointMT::Dummy)) {
            panic!("Pipe broken sending request for Endpoint {e:#?}");
        }
        match rx3.recv() {
            Ok(Message::EndPoint(EndPointMT::LastPoint(mut lp))) => {
                if let Some(p) = lp.result() {
                    println!("{i} point {p:?}");
                } else {
                    panic!("Received unexpected message while waiting for endpoint");
                }
            }
            _ => {
                panic!("Broken pipe wait for endpoint message");
            }
        }
    }

    let _ = tx1.send(Message::ShutDown);

    // Error or not wait for all stages to complete.
    print!("Waiting for stages to return complete ... ");
    for h in handles {
        h.join().unwrap();
    }
    println!("complete");
}
