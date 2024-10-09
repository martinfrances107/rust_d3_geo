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

fn transform_loop() {
    let ep = LastPoint::default();
    let mut str = StreamTransformRadians::default().connect(ep);

    for i in 1..100_000 {
        let p = Coord {
            x: i as f32,
            y: i as f32,
        };
        str.point(&p, None);
        assert_ne!(Some(p), str.endpoint().result());
    }
}

// Looks for changes in
fn mt_loop() {
    let (tx1, rx1): (Sender<Message<f64>>, Receiver<Message<f64>>) =
        mpsc::channel();

    let (tx2, rx2): (Sender<Message<f64>>, Receiver<Message<f64>>) =
        mpsc::channel();

    let (tx3, rx3): (Sender<Message<f64>>, Receiver<Message<f64>>) =
        mpsc::channel();

    let mut str = StreamTransformRadians::default();

    let stage1 = str.gen_stage(tx2, rx1);

    let ep = LastPoint::default();

    let stage2 = ep.gen_stage(tx3, rx2);

    for i in 1..100_000 {
        let p = Coord {
            x: i as f64,
            y: i as f64,
        };
        println!("About to send");
        if let Err(e) = tx1.send(Message::Point((p, None))) {
            panic!("broken pipe {e}");
        }
        println!("send complete about to receive ....");
        match rx3.recv() {
            Ok(Message::EndPoint(EndPointMT::LastPoint(mut lp))) => {
                if let Some(p) = lp.result() {
                    println!("End of the line p {p:#?}");
                } else {
                    panic!("failed1");
                }
            }
            _ => {
                panic!("failed2");
            }
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("transforms");

    // Increased the default run time by 3 seconds after gettings warnings that the task was taking too long.
    g.measurement_time(Duration::from_secs(10));

    g.bench_function("transforms", |b| b.iter(transform_loop));

    // Moved pipeline setup code here.

    g.bench_function("multi_threaded", |b| b.iter(mt_loop));

    // Move pipeline close code here.
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
