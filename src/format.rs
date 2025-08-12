//! Formatting implementation.

#![allow(clippy::cast_precision_loss)]

mod preset;

use core::fmt;

use const_for::const_for;

use crate::unit::RangedUnit;

#[derive(Debug, Clone, Copy)]
/// A collection of ranged units.
pub struct Formatter<const N: usize = 0> {
    /// Separator between numbers and units.
    ///
    /// Defaults to be " " (space)
    separator: &'static str,

    /// The abbreviated number's units.
    ///
    /// If the number is too large and no corresponding unit is found, the
    /// scientific notation like `3.0e99` will be used.
    ranged_units: &'static [RangedUnit; N],

    /// The custom unit attached after the abbreviated number's unit.
    custom_unit: Option<&'static str>,
}

impl Formatter {
    /// Binary units (`Ki`, `Mi`, `Gi`, `Ti`, `Pi`, `Ei`, `Zi`, `Yi`)
    pub const BINARY: Formatter<9> = Formatter {
        ranged_units: &preset::BINARY_UNITS,
        separator: " ",
        custom_unit: None,
    };
    /// Chinese units (`万`, `亿`, `兆`, `京`, `垓`, `秭`, `穰`, `沟`)
    pub const CHINESE: Formatter<9> = Formatter {
        ranged_units: &preset::CHINESE_UNITS,
        separator: " ",
        custom_unit: None,
    };
    /// Decimal units (`K`, `M`, `G`, `T`, `P`, `E`, `Z`, `Y`)
    pub const SI: Formatter<9> = Formatter {
        ranged_units: &preset::SI_UNITS,
        separator: " ",
        custom_unit: None,
    };
}

impl<const N: usize> Formatter<N> {
    #[inline]
    #[must_use]
    /// Creates a custom formatter with the given ranged units.
    ///
    /// ## Constrains
    ///
    /// - The first `ranged_unit.range_max` is the base, the nth
    ///   `ranged_unit.range_max` is the nth power of the first `ranged_unit.
    ///   range_max`.
    /// - `ranged_units` SHOULD NOT be empty.
    pub const fn custom(ranged_units: &'static [RangedUnit; N]) -> Option<Self> {
        if ranged_units.is_empty() {
            return None;
        }

        let base = ranged_units[0].range_max.get();

        const_for!(i in 1..N => {
            #[allow(clippy::cast_possible_truncation)]
            if ranged_units[i].range_max.get() != base.pow((i + 1) as u32) {
                return None;
            }
        });

        #[allow(unsafe_code, reason = "Has checked")]
        Some(unsafe { Self::custom_unchecked(ranged_units) })
    }

    #[allow(
        unsafe_code,
        reason = "The caller's responsibility to ensure the `ranged_units` is valid."
    )]
    #[inline]
    #[must_use]
    /// See [`Formatter::custom`].
    ///
    /// ## Safety
    ///
    /// See [`Formatter::custom`].
    pub const unsafe fn custom_unchecked(ranged_units: &'static [RangedUnit; N]) -> Self {
        Self {
            separator: " ",
            ranged_units,
            custom_unit: None,
        }
    }

    #[inline]
    #[must_use]
    /// Sets the separator between numbers and units.
    pub const fn with_separator(self, separator: &'static str) -> Self {
        Self { separator, ..self }
    }

    #[inline]
    #[must_use]
    /// Set custom unit attached after the abbreviated number's unit.
    pub const fn with_custom_unit(self, custom_unit: &'static str) -> Self {
        Self {
            custom_unit: Some(custom_unit),
            ..self
        }
    }

    #[inline]
    #[must_use]
    /// Formats a number, with default 2 decimal places.
    ///
    /// Any number type that implements the [`Humat`] trait is supported.
    pub fn format(&self, target: impl Humat) -> Formatted {
        target.humat(self)
    }

    #[inline]
    #[must_use]
    /// Formats a number with fixed decimal places.
    ///
    /// Any number type that implements the [`Humat`] trait is supported.
    pub fn format_fixed_dp<const DECIMAL_PLACES: usize>(&self, target: impl Humat) -> Formatted<DECIMAL_PLACES> {
        target.humat_fixed_dp(self)
    }
}

impl<const N: usize> Formatter<N> {
    #[inline]
    #[must_use]
    /// Formats an unsigned integer, with default 2 decimal places.
    pub const fn format_uint(&self, target: u128) -> Formatted {
        self.format_uint_fixed_dp(target)
    }

    #[inline]
    #[must_use]
    /// Formats an unsigned integer, with fixed decimal places.
    pub const fn format_uint_fixed_dp<const DECIMAL_PLACES: usize>(&self, target: u128) -> Formatted<DECIMAL_PLACES> {
        if target < self.ranged_units[0].range_max.get() {
            return Formatted {
                number: FormattedImpl::Int {
                    positive: true,
                    integer: target,
                    unit: self.ranged_units[0].unit,
                },
                separator: self.separator,
                custom_unit: self.custom_unit,
            };
        }

        let mut idx = 1;
        // Precision loss for very large numbers
        let number_precision_max = self.ranged_units[0].range_max.get() as f64
            - self.ranged_units[0].range_max.get() as f64 * 0.000_000_000_000_01;

        while idx < N {
            if target < self.ranged_units[idx].range_max.get() {
                let base = self.ranged_units[idx - 1].range_max.get();

                let integer_part = target / base;
                let leftover = target % base;

                let fractional_part = leftover as f64 / base as f64;

                let number = integer_part as f64 + fractional_part;
                let number = if number >= number_precision_max {
                    number_precision_max
                } else {
                    number
                };

                return Formatted {
                    number: FormattedImpl::F64 {
                        number,
                        unit: self.ranged_units[idx].unit,
                    },
                    separator: self.separator,
                    custom_unit: self.custom_unit,
                };
            }

            idx += 1;
        }

        Formatted {
            number: FormattedImpl::F64 {
                number: target as f64,
                unit: None,
            },
            separator: self.separator,
            custom_unit: self.custom_unit,
        }
    }

    #[inline]
    #[must_use]
    /// Formats a signed integer, with default 2 decimal places.
    pub const fn format_int(&self, target: i128) -> Formatted {
        self.format_uint_fixed_dp(target.unsigned_abs()).with_sign(target >= 0)
    }

    #[inline]
    #[must_use]
    /// Formats a signed integer with fixed decimal places.
    pub const fn format_int_fixed_dp<const DECIMAL_PLACES: usize>(&self, target: i128) -> Formatted<DECIMAL_PLACES> {
        self.format_uint_fixed_dp(target.unsigned_abs()).with_sign(target >= 0)
    }

    #[inline]
    #[must_use]
    /// Formats a `f64`, with default 2 decimal places.
    pub const fn format_double(&self, target: f64) -> Formatted {
        self.format_double_fixed_dp(target)
    }

    #[inline]
    #[must_use]
    /// Formats a `f64`, with fixed `DECIMAL_PLACES`.
    pub const fn format_double_fixed_dp<const DECIMAL_PLACES: usize>(&self, target: f64) -> Formatted<DECIMAL_PLACES> {
        if !target.is_finite() {
            return Formatted {
                number: FormattedImpl::F64 {
                    number: target,
                    unit: None,
                },
                separator: self.separator,
                custom_unit: self.custom_unit,
            };
        }

        if target < self.ranged_units[0].range_max.get() as f64 {
            return Formatted {
                number: FormattedImpl::F64 {
                    number: target,
                    unit: self.ranged_units[0].unit,
                },
                separator: self.separator,
                custom_unit: self.custom_unit,
            };
        }

        let mut idx = 1;

        while idx < N {
            if target < self.ranged_units[idx].range_max.get() as f64 {
                return Formatted {
                    number: FormattedImpl::F64 {
                        number: target / self.ranged_units[idx - 1].range_max.get() as f64,
                        unit: self.ranged_units[idx].unit,
                    },
                    separator: self.separator,
                    custom_unit: self.custom_unit,
                };
            }

            idx += 1;
        }

        Formatted {
            number: FormattedImpl::F64 {
                number: target,
                unit: None,
            },
            separator: self.separator,
            custom_unit: self.custom_unit,
        }
    }
}

// === Humat ===

/// Helper trait for formatting numbers in a human-readable way.
pub trait Humat {
    #[must_use]
    /// Formats the number with default 2 decimal places.
    fn humat<const N: usize>(self, formatter: &Formatter<N>) -> Formatted;

    #[must_use]
    /// Formats the number with fixed decimal places.
    fn humat_fixed_dp<const DECIMAL_PLACES: usize, const N: usize>(
        self,
        formatter: &Formatter<N>,
    ) -> Formatted<DECIMAL_PLACES>;
}

macro_rules! impl_number {
    ($fty:ident $cty:ident => $($ty:ident)*) => {
        impl<const N: usize> Formatter<N> {
            $(
                pastey::paste! {
                    #[inline]
                    #[must_use]
                    #[doc = concat!("Formats ", stringify!($ty), ", with default 2 decimal places.")]
                    pub const fn [<format_ $ty>](&self, target: $ty) -> Formatted {
                        self.[<format_ $fty _fixed_dp>](target as $cty)
                    }

                    #[inline]
                    #[must_use]
                    #[doc = concat!("Formats ", stringify!($ty), ", with fixed `DECIMAL_PLACES`.")]
                    pub const fn [<format_ $ty _fixed_dp>]<const DECIMAL_PLACES: usize>(&self, target: $ty) -> Formatted<DECIMAL_PLACES> {
                        self.[<format_ $fty _fixed_dp>](target as $cty)
                    }
                }
            )*
        }

        $(
            impl Humat for $ty {
                pastey::paste! {
                    #[inline]
                    fn humat<const N: usize>(self, formatter: &Formatter<N>) -> Formatted {
                        formatter.[<format_ $ty>](self)
                    }

                    #[inline]
                    fn humat_fixed_dp<const DECIMAL_PLACES: usize, const N: usize>(
                        self,
                        formatter: &Formatter<N>,
                    ) -> Formatted<DECIMAL_PLACES> {
                        formatter.[<format_ $ty _fixed_dp>](self)
                    }
                }
            }
        )*
    };
}

impl_number!(uint u128 => usize u128 u64 u32 u16 u8);
impl_number!(int i128 => isize i128 i64 i32 i16 i8);
impl_number!(double f64 => f64 f32);

// === Formatted ===

#[derive(Debug)]
/// The number to be formatted.
enum FormattedImpl {
    /// An integer with an optional fractional part.
    Int {
        /// Whether the number is positive.
        positive: bool,

        /// The integer part.
        integer: u128,

        /// The abbreviated number's unit.
        unit: Option<&'static str>,
    },

    /// An `f64`
    F64 {
        /// The integer part.
        number: f64,

        /// The abbreviated number's unit.
        unit: Option<&'static str>,
    },
}

#[derive(Debug)]
/// The formatted number, with default 2 decimal places.
pub struct Formatted<const DECIMAL_PLACES: usize = 2> {
    /// The formatted number.
    number: FormattedImpl,

    /// Separator between numbers and units.
    ///
    /// Defaults to be " " (space)
    separator: &'static str,

    /// The custom unit attached after the abbreviated number's unit.
    custom_unit: Option<&'static str>,
}

impl<const DECIMAL_PLACES: usize> Formatted<DECIMAL_PLACES> {
    #[inline]
    const fn with_sign(mut self, positive: bool) -> Self {
        match &mut self.number {
            FormattedImpl::Int { positive: p, .. } => *p = positive,
            FormattedImpl::F64 { number, .. } => {
                if !positive {
                    *number = -(*number);
                }
            }
        }

        self
    }

    #[inline]
    /// Set the decimal places for the formatted number.
    #[must_use]
    pub fn with_decimal_places<const NEW_DECIMAL_PLACES: usize>(self) -> Formatted<NEW_DECIMAL_PLACES> {
        #[allow(unsafe_code, reason = "compile time const value")]
        unsafe {
            core::mem::transmute(self)
        }
    }

    #[inline]
    /// Returns the raw number as a `f64`.
    #[must_use]
    pub const fn number(&self) -> f64 {
        match self.number {
            FormattedImpl::Int { positive, integer, .. } => integer as f64 * if positive { 1.0 } else { -1.0 },
            FormattedImpl::F64 { number, .. } => number,
        }
    }

    #[inline]
    /// Returns the separator between numbers and units.
    #[must_use]
    pub const fn separator(&self) -> &'static str {
        self.separator
    }

    #[inline]
    /// Returns the custom unit attached after the abbreviated number's unit.
    #[must_use]
    pub const fn custom_unit(&self) -> Option<&'static str> {
        self.custom_unit
    }

}

impl<const DECIMAL_PLACES: usize> fmt::Display for Formatted<DECIMAL_PLACES> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let separator = self.separator;

        match self.number {
            FormattedImpl::Int {
                positive,
                integer,
                unit,
            } => {
                let sign = if positive { "" } else { "-" };

                match (unit, self.custom_unit) {
                    (Some(unit), Some(custom_unit)) => write!(f, "{sign}{integer}{separator}{unit}{custom_unit}"),
                    (Some(unit), None) => write!(f, "{sign}{integer}{separator}{unit}"),
                    (None, Some(custom_unit)) => write!(f, "{sign}{integer}{separator}{custom_unit}"),
                    (None, None) => write!(f, "{sign}{integer}"),
                }
            }
            FormattedImpl::F64 { number, unit } => {
                let mut formatted = ryuu::Formatter::format_f64(number);

                let formatted = formatted.as_str_adjusting_dp::<DECIMAL_PLACES>();

                match (unit, self.custom_unit) {
                    (Some(unit), Some(custom_unit)) => write!(f, "{formatted}{separator}{unit}{custom_unit}"),
                    (Some(unit), None) => write!(f, "{formatted}{separator}{unit}"),
                    (None, Some(custom_unit)) => write!(f, "{formatted}{separator}{custom_unit}"),
                    (None, None) => write!(f, "{formatted}"),
                }
            }
        }
    }
}
