#[macro_use]
extern crate criterion;
extern crate pretty_assertions;

use core::time::Duration;

use criterion::Criterion;
use d3_geo_rs::projection::stream_transform_radians::StreamTransformRadians;
use d3_geo_rs::stream::Connectable;
use d3_geo_rs::stream::DrainStub;
use d3_geo_rs::stream::Stream;
use geo::Coord;

fn transform_loop() {
  let ep = DrainStub::default();
  let mut str = StreamTransformRadians::default().connect(ep);

  for i in 1..100_000{
    let p = Coord{x: i as f32, y: i as f32};
    str.point(&p, None);
    assert_ne!(p, str.endpoint().last_point);
  }
}

fn criterion_benchmark(c: &mut Criterion) {
  let mut g = c.benchmark_group("transforms");

  // Increased the default run time by 3 seconds after gettings warnings that the task was taking too long.
  g.measurement_time(Duration::from_secs(10));

  g.bench_function("transforms", |b| b.iter(transform_loop));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
