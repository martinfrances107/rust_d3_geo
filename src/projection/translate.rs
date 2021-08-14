pub trait Translate {
    type C;
    type P;
    fn get_translate(&self) -> Self::C;

    // /**
    //  * Sets the projection’s translation offset to the specified two-element array [tx, ty] and returns the projection.
    //  * The translation offset determines the PIxel coordinates of the projection’s center. The default translation offset places ⟨0°,0°⟩ at the center of a 960×500 area.
    //  *
    //  * @param point A two-element array [tx, ty] specifying the translation offset. The default translation offset of defaults to [480, 250] places ⟨0°,0°⟩ at the center of a 960×500 area.
    //  */
    fn translate(self, t: &Self::C) -> Self::P;
}
