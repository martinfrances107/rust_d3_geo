use num_traits::Float;

// use crate::stream::GeoStream;
use crate::transform_stream::TransformStream;

#[derive(Clone, Copy, Debug)]
struct LineTuple<F>
where
  F: Float,
{
  x: F,
  y: F,
  m: Option<u8>,
}
#[derive(Debug)]
pub struct ClipBuffer<F>
where
  F: Float,
{
  lines: Vec<Vec<LineTuple<F>>>,
  line: Vec<LineTuple<F>>,
}

impl<F> ClipBuffer<F>
where
  F: Float + 'static,
{
  pub fn new() -> Box<dyn TransformStream<F>> {
    return Box::new(Self {
      lines: Vec::new(),
      line: Vec::new(),
    });
  }

  fn rejoin(&mut self) {
    if self.lines.len() > 1 {
      // Shift from the top end.
      let lines_shift = self.lines.remove(0);
      // Pop from the bottom end.
      let lines_pop = self.lines.pop().unwrap_or(Vec::new());
      let join = [lines_pop, lines_shift].concat();
      self.lines.push(join);
    }
  }

  fn result(&mut self) -> Vec<Vec<LineTuple<F>>> {
    self.lines.clear();
    self.line.clear();
    let result = &self.lines;
    return result.to_vec();
  }
}

impl<'a, F> TransformStream<F> for ClipBuffer<F>
where
  F: Float,
{
  fn point(&mut self, x: F, y: F, m: Option<u8>) {
    self.line.push(LineTuple { x, y, m });
  }

  fn line_start(&mut self) {
    self.line.clear();
    // self.lines.push(self.line);
  }

  fn line_end(&mut self) {
    // no-op.
  }
}
// import noop from "../noop.js";

// export default function() {
//   var lines = [],
//       line;
//   return {
//     point: function(x, y, m) {
//       line.push([x, y, m]);
//     },
//     lineStart: function() {
//       lines.push(line = []);
//     },
//     lineEnd: noop,
//     rejoin: function() {
//       if (lines.length > 1) lines.push(lines.pop().concat(lines.shift()));
//     },
//     result: function() {
//       var result = lines;
//       lines = [];
//       line = null;
//       return result;
//     }
//   };
// }
