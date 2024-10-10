#[macro_use]
extern crate criterion;
extern crate pretty_assertions;

use core::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

use criterion::Criterion;
use d3_geo_rs::last_point::LastPoint;
use d3_geo_rs::path::Result;
use d3_geo_rs::projection::projector_common::Message;
use d3_geo_rs::projection::stream_transform_radians::StreamTransformRadians;
use d3_geo_rs::stream::Connectable;
use d3_geo_rs::stream::EndPointMT;
use d3_geo_rs::stream::Stream;
use d3_geo_rs::stream::StreamMT;
use geo::Coord;

fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("transforms");

    // Increased the default run time by 3 seconds after gettings warnings that the task was taking too long.
    g.measurement_time(Duration::from_secs(10));

    let ep = LastPoint::default();
    let mut str = StreamTransformRadians::default().connect(ep);

    g.bench_function("transforms", |b| {
        b.iter(|| {
            for i in 1..100_000 {
                let p = Coord {
                    x: i as f32,
                    y: i as f32,
                };
                str.point(&p, None);
                assert_ne!(Some(p), str.endpoint().result());
            }
        })
    });

    // Moved pipeline setup code here.
    let (tx1, rx1): (Sender<Message<f64>>, Receiver<Message<f64>>) =
        mpsc::channel();
    let (tx2, rx2): (Sender<Message<f64>>, Receiver<Message<f64>>) =
        mpsc::channel();
    let (tx3, rx3): (Sender<Message<f64>>, Receiver<Message<f64>>) =
        mpsc::channel();

    let stage1 = StreamTransformRadians::default().gen_stage(tx2, rx1);
    let stage2 = LastPoint::default().gen_stage(tx3, rx2);
    let handles = [stage1, stage2];

    g.bench_function("multi_threaded", |b| b.iter(|| {
      for i in 1..100 {
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
                if let Some(p_out) = lp.result() {
                    assert_ne!(p, p_out);
                } else {
                    panic!("Received unexpected message while waiting for endpoint");
                }
            }
            _ => {
                panic!("Broken pipe wait for endpoint message");
            }
        }
    }
    }));

    let _ = tx1.send(Message::ShutDown);

    // Error or not wait for all stages to complete.
    for h in handles {
        h.join().unwrap();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
