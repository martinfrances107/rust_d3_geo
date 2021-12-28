// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const rust = import('../pkg');

d3.json("./world-atlas/world/50m.json", function (data) {
  rust
    .then(m => {
      console.log("wasm is imported");

      const world = topojson.feature(data, data.objects.countries);
      const n_geometry = world.features.length;
      p_vec = [];
      mp_vec = [];
      for (i = 0; i < 241; i++) {
        const f = world.features[i];
        if (f.geometry.type == "Polygon") {
          f.geometry.exterior = f.geometry.coordinates[0];
          f.geometry.interiors = [];
          // console.log(f.geometry);
          p_vec.push(f.geometry);
        } else if (f.geometry.type == "MultiPolygon") {
          // console.log(f.geometry);
          // console.log(f.geometry.coordinates[0][0]);
          // Convert Mulipolygon with a single entry into a
          let mp = {
            type: "Polygon",
            exterior: f.geometry.coordinates[0][0],
            interiors: [],
          };
          mp_vec.push(mp);
        } else {
          console.log("unknown geo");
          // console.log(f.geometry.type);
          console.log(f.geometry);
        }

      }
      m.start(p_vec, mp_vec);

    })

});


