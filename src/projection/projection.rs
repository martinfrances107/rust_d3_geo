struct Projection {};

impl Projection {

  // projection.stream = function(stream) {
  //   return cache && cacheStream === stream ? cache : cache = transformRadians(transformRotate(rotate)(preclip(projectResample(postclip(cacheStream = stream)))));
  // };

  // projection.preclip = function(_) {
  //   return arguments.length ? (preclip = _, theta = undefined, reset()) : preclip;
  // };

  // projection.postclip = function(_) {
  //   return arguments.length ? (postclip = _, x0 = y0 = x1 = y1 = null, reset()) : postclip;
  // };

  // projection.clipAngle = function(_) {
  //   return arguments.length ? (preclip = +_ ? clipCircle(theta = _ * radians) : (theta = null, clipAntimeridian), reset()) : theta * degrees;
  // };

  // projection.clipExtent = function(_) {
  //   return arguments.length ? (postclip = _ == null ? (x0 = y0 = x1 = y1 = null, identity) : clipRectangle(x0 = +_[0][0], y0 = +_[0][1], x1 = +_[1][0], y1 = +_[1][1]), reset()) : x0 == null ? null : [[x0, y0], [x1, y1]];
  // };

  // projection.scale = function(_) {
  //   return arguments.length ? (k = +_, recenter()) : k;
  // };

  // projection.translate = function(_) {
  //   return arguments.length ? (x = +_[0], y = +_[1], recenter()) : [x, y];
  // };

  // projection.center = function(_) {
  //   return arguments.length ? (lambda = _[0] % 360 * radians, phi = _[1] % 360 * radians, recenter()) : [lambda * degrees, phi * degrees];
  // };

  // projection.rotate = function(_) {
  //   return arguments.length ? (deltaLambda = _[0] % 360 * radians, deltaPhi = _[1] % 360 * radians, deltaGamma = _.length > 2 ? _[2] % 360 * radians : 0, recenter()) : [deltaLambda * degrees, deltaPhi * degrees, deltaGamma * degrees];
  // };

  // projection.angle = function(_) {
  //   return arguments.length ? (alpha = _ % 360 * radians, recenter()) : alpha * degrees;
  // };

  // projection.reflectX = function(_) {
  //   return arguments.length ? (sx = _ ? -1 : 1, recenter()) : sx < 0;
  // };

  // projection.reflectY = function(_) {
  //   return arguments.length ? (sy = _ ? -1 : 1, recenter()) : sy < 0;
  // };

  // projection.precision = function(_) {
  //   return arguments.length ? (projectResample = resample(projectTransform, delta2 = _ * _), reset()) : sqrt(delta2);
  // };

  // projection.fitExtent = function(extent, object) {
  //   return fitExtent(projection, extent, object);
  // };

  // projection.fitSize = function(size, object) {
  //   return fitSize(projection, size, object);
  // };

  // projection.fitWidth = function(width, object) {
  //   return fitWidth(projection, width, object);
  // };

  // projection.fitHeight = function(height, object) {
  //   return fitHeight(projection, height, object);
  // };



}
