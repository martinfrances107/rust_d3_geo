// Takes a line and cuts into visible segments. Return values used for polygon
// clipPing: 0 - there were intersections or the line was empty; 1 - no
// intersections 2 - there were intersections, and the first and last segments
// should be rejoined.
#[derive(Debug, Clone, Copy)]
pub enum CleanEnum {
    Undefined,
    IntersectionsOrEmpty,
    NoIntersections,
    IntersectionsRejoin,
}

impl Default for CleanEnum {
    fn default() -> CleanEnum {
        CleanEnum::Undefined
    }
}

pub trait Clean {
    /// A clip trait.
    /// Rejoin first and last segments if there were intersections and the first
    /// and last points were visible.
    fn clean(&self) -> CleanEnum;
}
