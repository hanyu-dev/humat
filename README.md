# human-format-next

![Crates.io Version](https://img.shields.io/crates/v/human-format-next)
![Documentation](https://img.shields.io/docsrs/human-format-next)
![GitHub Tag](https://img.shields.io/github/v/tag/hanyu-dev/human-format-next)

Formatting numbers for us, while the machines are still at bay.

The primary purpose for this crate is to format numbers in a customizable fashion based around magnitudes.

## MSRV

1.59.0

## License

MIT License.

## Credits

This crate is inspired by [BobGneu/human-format-rs](https://github.com/BobGneu/human-format-rs), about 20x faster.
If you just want a simple and fast solution, this crate is for you.

While [BobGneu/human-format-rs](https://github.com/BobGneu/human-format-rs) has `no_std` support, this crate does
not provide such functionality since it depends heavily on std `f64` methods.
