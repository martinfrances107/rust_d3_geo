pub trait StreamIn {
    type SInput;
    fn stream_in(&mut self, stream: Self::SInput);
}
