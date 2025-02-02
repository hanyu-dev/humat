//! Tests

use human_format_next::Formatter;

#[test]
fn test_si() {
    assert_eq!(
        "0",
        Formatter::SI
            .with_decimals::<2>()
            .format_uint(0usize)
            .to_string()
    );
    assert_eq!(
        "1",
        Formatter::SI
            .with_decimals::<2>()
            .format_uint(1usize)
            .to_string()
    );
    assert_eq!(
        "2",
        Formatter::SI
            .with_decimals::<2>()
            .format_uint(2usize)
            .to_string()
    );
    assert_eq!(
        "999",
        Formatter::SI
            .with_decimals::<2>()
            .format_uint(999usize)
            .to_string()
    );
    assert_eq!(
        "1.00 K",
        Formatter::SI
            .with_decimals::<2>()
            .format_uint(1_000usize)
            .to_string()
    );
    // No rounding
    assert_eq!(
        "1.00 K",
        Formatter::SI
            .with_decimals::<2>()
            .format_uint(1_009usize)
            .to_string()
    );
    assert_eq!(
        "9.00 K",
        Formatter::SI
            .with_decimals::<2>()
            .format_uint(9_009usize)
            .to_string()
    );
    assert_eq!(
        "9.99 K",
        Formatter::SI
            .with_decimals::<2>()
            .format_uint(9_999usize)
            .to_string()
    );
    assert_eq!(
        "99.00 K",
        Formatter::SI
            .with_decimals::<2>()
            .format_uint(99_009usize)
            .to_string()
    );
    assert_eq!(
        "99.99 K",
        Formatter::SI
            .with_decimals::<2>()
            .format_uint(99_999usize)
            .to_string()
    );
    assert_eq!(
        "999.00 K",
        Formatter::SI
            .with_decimals::<2>()
            .format_uint(999_009usize)
            .to_string()
    );
    assert_eq!(
        "999.99 K",
        Formatter::SI
            .with_decimals::<2>()
            .format_uint(999_999usize)
            .to_string()
    );
    assert_eq!(
        "1.00 M",
        Formatter::SI
            .with_decimals::<2>()
            .format_uint(1_000_000usize)
            .to_string()
    );
    assert_eq!(
        "1.00 M",
        Formatter::SI
            .with_decimals::<2>()
            .format_uint(1_009_999usize)
            .to_string()
    );
    // Test usize::MAX
    assert_eq!(
        "18.44 E",
        Formatter::SI
            .with_decimals::<2>()
            .format_uint(usize::MAX)
            .to_string()
    );
}

const SI_TEST: Formatter<1000, 2> = Formatter::new(&["K", "M", "G"]);

#[test]
fn test_si_test() {
    assert_eq!("0", SI_TEST.format_uint(0usize).to_string());
    assert_eq!(
        "1",
        SI_TEST.with_decimals::<2>().format_uint(1usize).to_string()
    );
    assert_eq!(
        "2",
        SI_TEST.with_decimals::<2>().format_uint(2usize).to_string()
    );
    assert_eq!(
        "999",
        SI_TEST
            .with_decimals::<2>()
            .format_uint(999usize)
            .to_string()
    );
    assert_eq!(
        "1.00 K",
        SI_TEST
            .with_decimals::<2>()
            .format_uint(1_000usize)
            .to_string()
    );
    // No rounding
    assert_eq!(
        "1.00 K",
        SI_TEST
            .with_decimals::<2>()
            .format_uint(1_009usize)
            .to_string()
    );
    assert_eq!(
        "9.00 K",
        SI_TEST
            .with_decimals::<2>()
            .format_uint(9_009usize)
            .to_string()
    );
    assert_eq!(
        "9.99 K",
        SI_TEST
            .with_decimals::<2>()
            .format_uint(9_999usize)
            .to_string()
    );
    assert_eq!(
        "99.00 K",
        SI_TEST
            .with_decimals::<2>()
            .format_uint(99_009usize)
            .to_string()
    );
    assert_eq!(
        "99.99 K",
        SI_TEST
            .with_decimals::<2>()
            .format_uint(99_999usize)
            .to_string()
    );
    assert_eq!(
        "999.00 K",
        SI_TEST
            .with_decimals::<2>()
            .format_uint(999_009usize)
            .to_string()
    );
    assert_eq!(
        "999.99 K",
        SI_TEST
            .with_decimals::<2>()
            .format_uint(999_999usize)
            .to_string()
    );
    assert_eq!(
        "1.00 M",
        SI_TEST
            .with_decimals::<2>()
            .format_uint(1_000_000usize)
            .to_string()
    );
    assert_eq!(
        "1.00 M",
        SI_TEST
            .with_decimals::<2>()
            .format_uint(1_009_999usize)
            .to_string()
    );
    // Test usize::MAX, too large and no corresponding unit, use scientific notation
    assert_eq!(
        "1.84e19",
        SI_TEST
            .with_decimals::<2>()
            .format_uint(usize::MAX)
            .to_string()
    );
}

#[test]
fn test_chinese() {
    assert_eq!(
        "0",
        Formatter::CHINESE
            .with_decimals::<2>()
            .format_uint(0usize)
            .to_string()
    );
    assert_eq!(
        "1",
        Formatter::CHINESE
            .with_decimals::<2>()
            .format_uint(1usize)
            .to_string()
    );
    assert_eq!(
        "2",
        Formatter::CHINESE
            .with_decimals::<2>()
            .format_uint(2usize)
            .to_string()
    );
    assert_eq!(
        "9999",
        Formatter::CHINESE
            .with_decimals::<2>()
            .format_uint(9999usize)
            .to_string()
    );
    assert_eq!(
        "1.00 万",
        Formatter::CHINESE
            .with_decimals::<2>()
            .format_uint(1_0000usize)
            .to_string()
    );
    // No rounding
    assert_eq!(
        "1.00 万",
        Formatter::CHINESE
            .with_decimals::<2>()
            .format_uint(1_0099usize)
            .to_string()
    );
    assert_eq!(
        "99.00 万",
        Formatter::CHINESE
            .with_decimals::<2>()
            .format_uint(99_0099usize)
            .to_string()
    );
    assert_eq!(
        "99.99 万",
        Formatter::CHINESE
            .with_decimals::<2>()
            .format_uint(99_9999usize)
            .to_string()
    );
    assert_eq!(
        "999.00 万",
        Formatter::CHINESE
            .with_decimals::<2>()
            .format_uint(999_0099usize)
            .to_string()
    );
    assert_eq!(
        "999.99 万",
        Formatter::CHINESE
            .with_decimals::<2>()
            .format_uint(999_9999usize)
            .to_string()
    );
    assert_eq!(
        "9999.00 万",
        Formatter::CHINESE
            .with_decimals::<2>()
            .format_uint(9999_0099usize)
            .to_string()
    );
    assert_eq!(
        "9999.99 万",
        Formatter::CHINESE
            .with_decimals::<2>()
            .format_uint(9999_9999usize)
            .to_string()
    );
    assert_eq!(
        "1.00 亿",
        Formatter::CHINESE
            .with_decimals::<2>()
            .format_uint(1_0000_0000usize)
            .to_string()
    );
    assert_eq!(
        "1.00 亿",
        Formatter::CHINESE
            .with_decimals::<2>()
            .format_uint(1_0099_9999usize)
            .to_string()
    );
    // Test usize::MAX
    assert_eq!(
        "1844.67 京",
        Formatter::CHINESE
            .with_decimals::<2>()
            .format_uint(usize::MAX)
            .to_string()
    );
}
