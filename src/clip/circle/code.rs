  // Generates a 4-bit vector representing the location of a point relative to
  // the small circle's bounding box.
  fn code(lambda: f64, phi: f64) -> u8{
    let  r = smallRadius ? radius : pi - radius,
        code = 0;
    if (lambda < -r) code |= 1; // left
    else if (lambda > r) code |= 2; // right
    if (phi < -r) code |= 4; // below
    else if (phi > r) code |= 8; // above
    return code;
  }

  // function code(lambda, phi) {
  //   var r = smallRadius ? radius : pi - radius,
  //       code = 0;
  //   if (lambda < -r) code |= 1; // left
  //   else if (lambda > r) code |= 2; // right
  //   if (phi < -r) code |= 4; // below
  //   else if (phi > r) code |= 8; // above
  //   return code;
  // }