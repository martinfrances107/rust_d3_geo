use crate::clip::Clean;
use crate::stream::stream_in_trait::StreamIn;
use crate::stream::Stream;

pub trait LineTrait: Clean + Stream + StreamIn {}
