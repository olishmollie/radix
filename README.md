# radix

A command line number base converter.

`brew install olishmollie/tools/radix`

```
Usage:
    radix -h | --help
    radix -v | --version
    radix [ -b | -d | -o | -x ] [ -n ] <value>

Options:
    -h, --help         Print this message.
    -v, --version      Print version.
    -b, --binary       Set radix to binary.
    -d, --decimal      Set radix to decimal.
    -n, --negative     Use two's complement.
    -o, --octal        Set radix to octal.
    -x, --hexadecimal  Set radix to hexadecimal.

Example:
    radix 42
    radix -d 0o52
    radix -x 0b101010
    radix -no 0x2a
```