use num_traits::Float;
use num_traits::FloatConst;

#[derive(Clone, Copy)]
struct LineTuple<F>
where F: Float {
  x: F,
  y: F,
  m: u16,
}

pub struct ClipBuffer<F>
where F: Float {
  lines: Vec<Vec<LineTuple<F>>>,
  line: Vec<LineTuple<F>>,
}

impl<F> ClipBuffer<F>
where F: Float {
  pub fn new() -> Self {
    return Self {
      lines:Vec::new(),
      line: Vec::new(),
    };
  }

  fn point(&mut self, x: F, y: F, m :u16) {
    self.line.push(LineTuple{x,y,m});
  }

  fn lineStart(&mut self) {
      self.line.clear();
      self.lines.push(self.line);
  }

  fn lineEnd(&self) {
    // no-op.
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
    let result = self.lines;
    self.lines.clear();
    self.line.clear();
    return result;
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
