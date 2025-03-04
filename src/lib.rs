//! Human-format-next

#![cfg_attr(not(feature = "std"), no_std)]

use core::fmt;

#[derive(Debug, Clone, Copy)]
/// Entry point to the lib. Use this to handle your formatting needs.
///
/// - `BASE`: the base
/// - `DECIMALS`: target decimal places (if not keeping the original number)
pub struct Formatter<const BASE: usize = 0, const DECIMALS: usize = 2> {
    /// Separator between numbers and units.
    ///
    /// Defaults to be " " (space)
    separator: &'static str,

    /// The abbreviated number's units.
    ///
    /// If the number is too large and no corresponding unit is found, the
    /// scientific notation like `3.0e99` will be used.
    units: &'static [&'static str],

    /// The custom unit attached after the abbreviated number's unit.
    custom_unit: Option<&'static str>,
}

impl Formatter {
    /// SI units (western format).
    pub const SI: Formatter<1000, 2> = Formatter::new(&["K", "M", "G", "T", "P", "E", "Z", "Y"]);

    /// Binary units (western format).
    pub const BINARY: Formatter<1024, 2> =
        Formatter::new(&["Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi", "Yi"]);

    /// Chinese units.
    pub const CHINESE: Formatter<10000, 2> =
        Formatter::new(&["万", "亿", "兆", "京", "垓", "秭", "穰", "沟"]);
}

impl<const BASE: usize, const DECIMALS: usize> Formatter<BASE, DECIMALS> {
    #[inline]
    /// Create a new [`Formatter`] with given `BASE`, `DECIMALS` and units.
    pub const fn new(units: &'static [&'static str]) -> Self {
        Self {
            separator: " ",
            units,
            custom_unit: None,
        }
    }

    #[inline]
    /// Set the separator between numbers and units.
    pub const fn with_separator(self, separator: &'static str) -> Self {
        Self { separator, ..self }
    }

    #[inline]
    /// Set scales, including base and the abbreviated number's units.
    pub const fn with_scales<const N_BASE: usize>(
        self,
        units: &'static [&'static str],
    ) -> Formatter<N_BASE, DECIMALS> {
        // wait for feature `generic_const_exprs`
        debug_assert!(BASE > 0, "BASE CANNOT BE 0");

        Formatter {
            separator: self.separator,
            units,
            custom_unit: self.custom_unit,
        }
    }

    #[inline]
    /// Set custom unit attached after the abbreviated number's unit.
    pub const fn with_custom_unit(self, custom_unit: &'static str) -> Self {
        Self {
            custom_unit: Some(custom_unit),
            ..self
        }
    }

    #[inline]
    /// Set the decimal places to keep.
    pub const fn with_decimals<const N_DECIMALS: usize>(self) -> Formatter<BASE, N_DECIMALS> {
        // wait for feature `generic_const_exprs`
        debug_assert!(
            N_DECIMALS <= f64::DIGITS as usize,
            "DECIMALS too large, for RELEASE profile will make use of f64::DIGITS",
        );

        Formatter {
            separator: self.separator,
            units: self.units,
            custom_unit: self.custom_unit,
        }
    }

    #[inline]
    /// Formats the given `number` into a human-readable string using the
    /// specified units and separator.
    ///
    /// See [`NumberT`] for all types we accept as param.
    ///
    /// # Notice
    ///
    /// For better performance (may be so), you may need
    /// [`format_int`](Self::format_int) or [`format_uint`](Self::format_uint).
    ///
    /// # Limitation
    ///
    /// `f64` can only handle 15 decimal places at most. We may introduce
    /// `macro_toolset` for large number formatting.
    pub fn format(&self, number: impl NumberT) -> FormatResult<DECIMALS> {
        if let Some(integer) = number.integer() {
            self.format_general(integer, number.fraction())
                .set_result_is_negative(number.is_negative())
        } else {
            #[cfg(feature = "std")]
            {
                self.format_float(
                    number
                        .fraction()
                        .expect("must be floating number which is too large"),
                )
            }

            #[cfg(not(feature = "std"))]
            #[allow(unsafe_code)]
            unsafe {
                core::hint::unreachable_unchecked()
            }
        }
    }

    #[inline]
    /// Formats the given `number` into a human-readable string using the
    /// specified units and separator.
    ///
    /// We accept any number that fits into `isize`. For `i128`, see
    /// [`format_large_int`](Self::format_large_int).
    pub fn format_int(&self, number: impl Into<i128>) -> FormatResult<DECIMALS> {
        let number: i128 = number.into();

        self.format_general(number.unsigned_abs(), None)
            .set_result_is_negative(number.is_negative())
    }

    #[inline]
    /// Formats the given `number` into a human-readable string using the
    /// specified units and separator.
    pub fn format_uint(&self, number: impl Into<u128>) -> FormatResult<DECIMALS> {
        self.format_general(number.into(), None)
    }

    /// Formats the given `number` into a human-readable string using the
    /// specified units and separator.
    ///
    /// # Params
    ///
    /// - `integer`: the integer part of the number.
    ///   - For float, [`f32::trunc`] or [`f64::trunc`] may helps you.
    /// - `fraction`: the fractional part of the number.
    ///   - For float, [`f32::fract`] or [`f64::fract`] may helps you.
    ///   - For integer, leave it `None`.
    ///
    /// # Notice
    ///
    /// It's NOT recommended that you use this directly, use
    /// [`format`](Self::format) instead unless you know exactly what you do.
    pub fn format_general(&self, integer: u128, fraction: Option<f64>) -> FormatResult<DECIMALS> {
        let base = BASE as u128;

        if integer < base {
            return FormatType::General {
                integer,
                fraction,
                unit: None,
            }
            .formatter_result(self);
        }

        let mut index: usize = 0;
        let mut value = integer;

        while value >= base {
            value /= base;
            index += 1;
        }

        match self.units.get(index - 1) {
            Some(&unit) => {
                let leftover = {
                    let leftover_exp = (base).pow(index as u32);
                    (integer - value * leftover_exp) as f64 / leftover_exp as f64
                };

                #[cfg(feature = "std")]
                {
                    // fraction may be larger than 1.
                    let leftover_fraction = fraction.unwrap_or(0.0) + leftover.fract();

                    FormatType::General {
                        integer: value
                            + leftover.trunc() as u128
                            + leftover_fraction.trunc() as u128,
                        fraction: Some(leftover_fraction.fract()),
                        unit: Some(unit),
                    }
                }

                #[cfg(not(feature = "std"))]
                {
                    let mut leftover = leftover;

                    // fraction may be larger than 1.
                    let mut integer = value;
                    while leftover >= 1.0 {
                        leftover -= 1.0;
                        integer += 1;
                    }

                    let mut fraction = leftover + fraction.unwrap_or(0.0);
                    while fraction >= 1.0 {
                        fraction -= 1.0;
                        integer += 1;
                    }

                    FormatType::General {
                        integer,
                        fraction: Some(fraction),
                        unit: Some(unit),
                    }
                }
            }
            None => {
                let mut exponent: usize = 0;
                let mut value = integer;
                // Safe: have checked DECIMALS <= u32::MAX
                let target_len = 10usize.pow((DECIMALS as u32).min(f64::DIGITS) + 1);

                loop {
                    value /= 10;
                    exponent += 1;

                    if value < target_len as _ {
                        break;
                    }
                }

                // calc the leftover
                {
                    let mut value = value;
                    loop {
                        value /= 10;
                        exponent += 1;

                        if value < 10 {
                            break;
                        }
                    }
                }

                FormatType::Scientific {
                    coefficient: value as f64 / (target_len / 10) as f64,
                    exponent,
                }
            }
        }
        .formatter_result(self)
    }

    #[cfg(feature = "std")]
    /// Formats the given `number` into a human-readable string using the
    /// specified units and separator.
    ///
    /// # Notice
    ///
    /// It's NOT recommended that you use this directly, floating point
    /// calculation often slower than integer arithmetic. Use
    /// [`format`](Self::format) instead unless you know exactly what you do.
    pub fn format_float(&self, number: f64) -> FormatResult<DECIMALS> {
        let base = BASE as f64;
        if number < base {
            return FormatType::Float { number, unit: None }.formatter_result(self);
        }

        let mut index: usize = 0;
        let mut value = number;

        while value >= base {
            value /= base;
            index += 1;
        }

        match self.units.get(index - 1) {
            Some(&unit) => {
                let leftover = {
                    let leftover_exp = base.powi(index as i32);
                    (number - value * leftover_exp) / leftover_exp
                };

                FormatType::Float {
                    number: value + leftover,
                    unit: Some(unit),
                }
            }
            None => {
                let value = number.log10();

                FormatType::Scientific {
                    coefficient: 10.0f64.powf(value.fract()),
                    exponent: value.trunc() as _,
                }
            }
        }
        .formatter_result(self)
    }
}

#[allow(private_bounds)]
/// Sealed trait for number that can be formatted, including:
///
/// - `u8`
/// - `u16`
/// - `u32`
/// - `u64`
/// - `u128`
/// - `i8`
/// - `i16`
/// - `i32`
/// - `i64`
/// - `i128`
/// - `f32`
/// - `f64`
pub trait NumberT: number_sealed::NumberT {}

impl<T: number_sealed::NumberT> NumberT for T {}

mod number_sealed {
    pub(super) trait NumberT: Copy {
        fn is_negative(self) -> bool;

        fn integer(self) -> Option<u128>;

        #[inline]
        fn fraction(self) -> Option<f64> {
            None
        }
    }

    macro_rules! impl_number_trait {
        (UINT: $($ty:ident),+) => {
            $(
                impl NumberT for $ty {
                    #[inline]
                    fn is_negative(self) -> bool {
                        false
                    }

                    #[inline]
                    fn integer(self) -> Option<u128> {
                        Some(self as _)
                    }
                }
            )+
        };
        (INT: $($ty:ident),+) => {
            $(
                impl NumberT for $ty {
                    #[inline]
                    fn is_negative(self) -> bool {
                        self < 0
                    }

                    #[inline]
                    fn integer(self) -> Option<u128> {
                        Some(self.unsigned_abs() as _)
                    }
                }
            )+
        };
    }

    impl_number_trait!(UINT: u8, u16, u32, u64, usize, u128);
    impl_number_trait!(INT: i8, i16, i32, i64, isize, i128);

    #[cfg(feature = "std")]
    impl NumberT for f32 {
        #[inline]
        fn is_negative(self) -> bool {
            self < 0.0
        }

        #[inline]
        fn integer(self) -> Option<u128> {
            Some(self.trunc() as _)
        }

        #[inline]
        fn fraction(self) -> Option<f64> {
            Some(self.fract() as _)
        }
    }

    #[cfg(feature = "std")]
    impl NumberT for f64 {
        #[inline]
        fn is_negative(self) -> bool {
            self < 0.0
        }

        #[inline]
        fn integer(self) -> Option<u128> {
            if self < 3.40282366920938e+38 {
                Some(self.trunc() as _)
            } else {
                None
            }
        }

        #[inline]
        fn fraction(self) -> Option<f64> {
            if self < 3.40282366920938e+38 {
                Some(self.fract() as _)
            } else {
                Some(self as _)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
/// Format result
///
/// This implements [`Display`](fmt::Display) and `to_string` is supported.
pub struct FormatResult<const DECIMALS: usize> {
    result: FormatType<DECIMALS>,
    result_is_negative: bool,
    separator: &'static str,
    custom_unit: Option<&'static str>,
}

impl<const DECIMALS: usize> fmt::Display for FormatResult<DECIMALS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.result_is_negative {
            write!(f, "-")?;
        }

        match self.result {
            FormatType::General {
                integer,
                fraction: _fraction,
                unit,
            } => {
                write!(f, "{integer}")?;

                // Keep 15, f64::DIGITS
                #[cfg(feature = "std")]
                {
                    let full_fraction = _fraction.map(|fraction| format!("{fraction:.15}"));
                    let fraction = full_fraction
                        .as_ref()
                        .map(|full_fraction| {
                            let digits = (f64::DIGITS as usize).min(DECIMALS);
                            &full_fraction[1..digits + 2]
                        })
                        .unwrap_or_default();
                    write!(f, "{fraction}")?;
                };

                if unit.is_some() {
                    write!(f, "{}{}", self.separator, unit.unwrap())?;
                }

                if self.custom_unit.is_some() {
                    if unit.is_none() {
                        write!(f, "{}", self.separator)?;
                    }

                    write!(f, "{}", self.custom_unit.unwrap())?;
                };
            }
            #[cfg(feature = "std")]
            FormatType::Float { number, unit } => {
                // Keep 15, f64::DIGITS
                let number = format!("{number:.15}");
                let digits = (f64::DIGITS as usize).min(DECIMALS);
                let number = &number[1..digits + 2];
                write!(f, "{number}")?;

                if unit.is_some() {
                    write!(f, "{}{}", self.separator, unit.unwrap())?;
                }

                if self.custom_unit.is_some() {
                    if unit.is_none() {
                        write!(f, "{}", self.separator)?;
                    }

                    write!(f, "{}", self.custom_unit.unwrap())?;
                };
            }
            FormatType::Scientific {
                coefficient,
                exponent,
            } => {
                #[cfg(not(feature = "std"))]
                write!(f, "{coefficient}")?;

                #[cfg(feature = "std")]
                {
                    // Keep 15, f64::DIGITS
                    let coefficient = format!("{coefficient:.15}");
                    let digits = (f64::DIGITS as usize).min(DECIMALS);
                    let coefficient = &coefficient[..digits + 2];
                    write!(f, "{coefficient}")?;
                }

                write!(f, "e{exponent}")?;

                if self.custom_unit.is_some() {
                    write!(f, "{}{}", self.separator, self.custom_unit.unwrap())?;
                };
            }
        };

        Ok(())
    }
}

impl<const DECIMALS: usize> FormatResult<DECIMALS> {
    #[inline]
    const fn set_result_is_negative(self, result_is_negative: bool) -> Self {
        Self {
            result_is_negative,
            ..self
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum FormatType<const DECIMALS: usize> {
    /// General
    General {
        /// The integer part.
        integer: u128,

        /// The fractional part.
        fraction: Option<f64>,

        /// The abbreviated number's unit.
        unit: Option<&'static str>,
    },

    #[cfg(feature = "std")]
    /// General
    Float {
        /// The integer part.
        number: f64,

        /// The abbreviated number's unit.
        unit: Option<&'static str>,
    },

    /// Scientific notation
    Scientific {
        /// The coefficient part, must be within `1.0` ~ `9.99...`
        coefficient: f64,
        /// The exponent part, must be a positive integer
        exponent: usize,
    },
}

impl<const DECIMALS: usize> FormatType<DECIMALS> {
    #[inline]
    const fn formatter_result<const BASE: usize>(
        self,
        formatter: &Formatter<BASE, DECIMALS>,
    ) -> FormatResult<DECIMALS> {
        FormatResult {
            result: self,
            result_is_negative: false,
            separator: formatter.separator,
            custom_unit: formatter.custom_unit,
        }
    }
}
