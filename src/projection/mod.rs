mod scale_translate;
mod scale_translate_rotate;
mod azimuthal;
mod geo_projection;
mod scale_translate_rotate;
mod scale_translate;

pub mod geo_projection_mutator;
pub mod geo_stream;
pub mod stereographic;

/// GeoProjections has a stream cache.
enum CACHE {
  STREAM,
  NONE,
}
