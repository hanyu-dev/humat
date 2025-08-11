//! Some preset units for formatting numbers.

use core::num::NonZeroU128;

use crate::unit::RangedUnit;

pub(super) static BINARY_UNITS: [RangedUnit; 9] = [
    RangedUnit {
        range_max: NonZeroU128::new(1_024_u128.pow(1)).unwrap(),
        unit: None,
    },
    RangedUnit {
        range_max: NonZeroU128::new(1_024_u128.pow(2)).unwrap(),
        unit: Some("Ki"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(1_024_u128.pow(3)).unwrap(),
        unit: Some("Mi"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(1_024_u128.pow(4)).unwrap(),
        unit: Some("Gi"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(1_024_u128.pow(5)).unwrap(),
        unit: Some("Ti"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(1_024_u128.pow(6)).unwrap(),
        unit: Some("Pi"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(1_024_u128.pow(7)).unwrap(),
        unit: Some("Ei"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(1_024_u128.pow(8)).unwrap(),
        unit: Some("Zi"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(1_024_u128.pow(9)).unwrap(),
        unit: Some("Yi"),
    },
];

pub(super) static CHINESE_UNITS: [RangedUnit; 9] = [
    RangedUnit {
        range_max: NonZeroU128::new(10_000_u128.pow(1)).unwrap(),
        unit: None,
    },
    RangedUnit {
        range_max: NonZeroU128::new(10_000_u128.pow(2)).unwrap(),
        unit: Some("万"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(10_000_u128.pow(3)).unwrap(),
        unit: Some("亿"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(10_000_u128.pow(4)).unwrap(),
        unit: Some("兆"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(10_000_u128.pow(5)).unwrap(),
        unit: Some("京"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(10_000_u128.pow(6)).unwrap(),
        unit: Some("垓"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(10_000_u128.pow(7)).unwrap(),
        unit: Some("秭"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(10_000_u128.pow(8)).unwrap(),
        unit: Some("穰"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(10_000_u128.pow(9)).unwrap(),
        unit: Some("沟"),
    },
];

pub(super) static SI_UNITS: [RangedUnit; 9] = [
    RangedUnit {
        range_max: NonZeroU128::new(1_000_u128.pow(1)).unwrap(),
        unit: None,
    },
    RangedUnit {
        range_max: NonZeroU128::new(1_000_u128.pow(2)).unwrap(),
        unit: Some("K"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(1_000_u128.pow(3)).unwrap(),
        unit: Some("M"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(1_000_u128.pow(4)).unwrap(),
        unit: Some("G"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(1_000_u128.pow(5)).unwrap(),
        unit: Some("T"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(1_000_u128.pow(6)).unwrap(),
        unit: Some("P"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(1_000_u128.pow(7)).unwrap(),
        unit: Some("E"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(1_000_u128.pow(8)).unwrap(),
        unit: Some("Z"),
    },
    RangedUnit {
        range_max: NonZeroU128::new(1_000_u128.pow(9)).unwrap(),
        unit: Some("Y"),
    },
];
