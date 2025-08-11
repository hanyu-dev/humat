//! Ranged unit

use core::num::NonZeroU128;

#[derive(Debug)]
/// Ranged unit.
pub struct RangedUnit {
    /// The maximum value of the range (excluded).
    ///
    /// This will be the minimum value of the next range.
    pub range_max: NonZeroU128,

    /// The abbr unit when the value is less than `range_max`.
    pub unit: Option<&'static str>,
}
