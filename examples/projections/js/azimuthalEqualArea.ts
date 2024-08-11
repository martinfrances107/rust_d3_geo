import {
  geoAzimuthalEqualArea,
  geoGraticule10,
  geoPath,
  GeoPermissibleObjects,
} from "d3-geo";

export default async function azimuthalEqualArea(world: GeoPermissibleObjects) {
  const canvas: HTMLCanvasElement = document.querySelector(
    "#azimuthal-equal-area-js"
  );

  const context = canvas.getContext("2d");

  const width = canvas.width;
  const height = canvas.height;

  const projection = geoAzimuthalEqualArea()
    .scale(width / 3)
    .translate([width / 2, height / 2])
    .precision(0.1);

  const path = geoPath(projection, context);

  context.strokeStyle = "#69b2a2";
  path(world);
  context.stroke();
  path(geoGraticule10());
  context.stroke();
}
