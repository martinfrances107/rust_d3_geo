use super::Stream;

pub trait StreamIn {
    type SInput;
    fn stream_in(&mut self, stream: Self::SInput);
}

pub trait StreamCombo: Stream + StreamIn {}
