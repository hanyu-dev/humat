//! Human-format-next

use std::fmt;

#[derive(Debug, Clone, Copy)]
/// Entry point to the lib. Use this to handle your formatting needs.
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
        Formatter {
            separator: self.separator,
            units: self.units,
            custom_unit: self.custom_unit,
        }
    }

    #[inline]
    /// Formats the given `number` into a human-readable string using the
    /// specified units and separator.
    pub fn format_int(&self, number: impl Into<isize>) -> FormatResult<DECIMALS> {
        let number: isize = number.into();

        self.format(number.unsigned_abs(), None)
            .set_result_is_negative(number.is_negative())
    }

    #[inline]
    /// Formats the given `number` into a human-readable string using the
    /// specified units and separator.
    pub fn format_uint(&self, number: impl Into<usize>) -> FormatResult<DECIMALS> {
        self.format(number.into(), None)
    }

    #[inline]
    /// Formats the given `number` into a human-readable string using the
    /// specified units and separator.
    pub fn format_float(&self, number: f64) -> FormatResult<DECIMALS> {
        self.format(number.trunc() as _, Some(number.fract()))
    }

    /// Formats the given `number` into a human-readable string using the
    /// specified units and separator.
    ///
    /// We recommend that you use [`format_float`](Self::format_float) or
    /// [`format_int`](Self::format_int).
    ///
    /// # Params
    ///
    /// - `integer`: the integer part of the number.
    ///   - For float, [`f32::trunc`] or [`f64::trunc`] may helps you.
    /// - `fraction`: the fractional part of the number.
    ///   - For float, [`f32::fract`] or [`f64::fract`] may helps you.
    ///   - For integer, leave it `None`.
    pub fn format(&self, integer: usize, fraction: Option<f64>) -> FormatResult<DECIMALS> {
        if integer < BASE || { BASE == 0 } || { DECIMALS > f64::DIGITS as usize } {
            // Wait for feature `generic_const_exprs`
            debug_assert!(BASE > 0, "BASE CANNOT BE 0");
            debug_assert!(
                DECIMALS <= f64::DIGITS as usize,
                "DECIMALS too large, for RELEASE profile will make use of {}",
                f64::DIGITS
            );
            return FormatType::General {
                integer,
                fraction,
                unit: None,
            }
            .formatter_result(self);
        }

        let mut index: usize = 0;
        let mut value = integer;

        loop {
            value /= BASE;
            index += 1;

            if value < BASE {
                break;
            }
        }

        match self.units.get(index - 1) {
            Some(&unit) => {
                let leftover = {
                    let leftover_exp = BASE.pow(index as u32);
                    (integer - value * leftover_exp) as f64 / leftover_exp as f64
                };

                // fraction may be larger than 1.
                let leftover_fraction = fraction.unwrap_or(0.0) + leftover.fract();

                FormatType::General {
                    integer: value + leftover.trunc() as usize + leftover_fraction.trunc() as usize,
                    fraction: Some(leftover_fraction.fract()),
                    unit: Some(unit),
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

                    if value < target_len {
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

        let custom_unit = self.custom_unit.unwrap_or_default();
        match &self.result {
            FormatType::General {
                integer,
                fraction,
                unit,
            } => {
                // Keep 15, f64::DIGITS
                let full_fraction = fraction.map(|fraction| format!("{fraction:.15}"));
                let fraction = full_fraction
                    .as_ref()
                    .map(|full_fraction| {
                        let digits = (f64::DIGITS as usize).min(DECIMALS);
                        &full_fraction[1..digits + 2]
                    })
                    .unwrap_or_default();

                let separator_before_unit = if (*unit).is_some() {
                    self.separator
                } else {
                    ""
                };
                let unit = (*unit).unwrap_or_default();

                write!(
                    f,
                    "{integer}{fraction}{separator_before_unit}{unit}{custom_unit}",
                )
            }
            FormatType::Scientific {
                coefficient: value,
                exponent,
            } => {
                let separator_before_custom_unit = if self.custom_unit.is_some() {
                    self.separator
                } else {
                    ""
                };

                write!(
                    f,
                    "{value}e{exponent}{separator_before_custom_unit}{custom_unit}",
                )
            }
        }
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
        integer: usize,

        /// The fractional part.
        fraction: Option<f64>,

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
