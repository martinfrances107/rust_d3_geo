pub trait ClipExtent {
    type C;
    fn get_clip_extent(&self) -> Option<[Self::C; 2]>;
    fn clip_extent(self, extent: Option<[Self::C; 2]>) -> Self;
}
