#[macro_use]
extern crate criterion;
extern crate pretty_assertions;

use core::time::Duration;
use std::sync::mpsc::sync_channel;

use criterion::Criterion;
use d3_geo_rs::last_point::LastPoint;
use d3_geo_rs::path::Result;
use d3_geo_rs::projection::orthographic::Orthographic;
use d3_geo_rs::projection::projector_common::Message;
use d3_geo_rs::projection::stream_transform_radians::StreamTransformRadians;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::Projector;
use d3_geo_rs::projection::RawBase;
use d3_geo_rs::projection::ScaleSet;
use d3_geo_rs::projection::TranslateSet;
use d3_geo_rs::stream::EndPointMT;
use d3_geo_rs::stream::Stream;
use d3_geo_rs::stream::StreamMT;
use geo::Coord;

use d3_geo_rs::rot::rotate_radians::RotateRadians;
use d3_geo_rs::rot::rotation_identity::RotationIdentity;
use d3_geo_rs::rot::rotator_radians::RotatorRadians;

fn criterion_benchmark(c: &mut Criterion) {
    let mut g = c.benchmark_group("transforms");

    let mut o_proj = Orthographic::builder()
        .scale_set(240_f64)
        .translate_set(&Coord {
            x: 800_f64,
            y: 600_f64,
        })
        .build();

    // Increased the default run time by 3 seconds after gettings warnings that the task was taking too long.
    g.measurement_time(Duration::from_secs(10));

    let ep = LastPoint::default();

    let mut str = o_proj.stream(&ep);

    g.bench_function("single-threaded", |b| {
        b.iter(|| {
            for i in 1..100_000 {
                let p = Coord {
                    x: i as f64,
                    y: i as f64,
                };
                str.point(&p, None);
                assert_ne!(Some(p), str.endpoint().result());
            }
        })
    });

    static CHANNEL_CAPACITY: usize = 4096;
    let (tx, rx1) = sync_channel(CHANNEL_CAPACITY);
    let (tx2, rx2) = sync_channel(CHANNEL_CAPACITY);
    let (tx3, rx3) = sync_channel(CHANNEL_CAPACITY);
    let (tx4, rx4) = sync_channel(CHANNEL_CAPACITY);
    let (tx5, rx5) = sync_channel(CHANNEL_CAPACITY);
    let (tx6, rx6) = sync_channel(CHANNEL_CAPACITY);
    let (tx7, rx) = sync_channel(CHANNEL_CAPACITY);

    let stage1 = StreamTransformRadians::default().gen_stage(tx2, rx1);
    let stage2 = RotatorRadians::new(RotateRadians::<f64>::I(
        RotationIdentity::default(),
    ))
    .gen_stage(tx3, rx2);
    let stage3 = RotatorRadians::new(RotateRadians::<f64>::I(
        RotationIdentity::default(),
    ))
    .gen_stage(tx4, rx3);
    let stage4 = RotatorRadians::new(RotateRadians::<f64>::I(
        RotationIdentity::default(),
    ))
    .gen_stage(tx5, rx4);
    let stage5 = RotatorRadians::new(RotateRadians::<f64>::I(
        RotationIdentity::default(),
    ))
    .gen_stage(tx6, rx5);

    let stage6 = LastPoint::default().gen_stage(tx7, rx6);

    let handles = [stage1, stage2, stage3, stage5, stage6];

    g.bench_function("multi-threaded", |b| b.iter(|| {
      for i in 1..100_000 {
        let p = Coord {
            x: i as f64,
            y: i as f64,
        };

        if let Err(e) = tx.send(Message::Point((p, None))) {
            panic!("Broken pipe sending point {i} {e}");
        }

        if let Err(e) = tx.send(Message::EndPoint(EndPointMT::Dummy)) {
            panic!("Pipe broken sending request for Endpoint {e:#?}");
        }
        match rx.recv() {
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

    let _ = tx.send(Message::ShutDown);

    // Error or not wait for all stages to complete.
    for h in handles {
        h.join().unwrap();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
