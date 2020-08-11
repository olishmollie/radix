use std::process::exit;

use docopt::Docopt;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const USAGE: &str = "
Usage:
    radix -h
    radix -v
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
";

#[derive(Debug)]
enum Radix {
    Binary,
    Decimal,
    Octal,
    Hexadecimal,
}

#[derive(Debug)]
/// Stores the options used for conversion.
pub struct Options {
    value: String,
    twos_complement: bool,
    radix: Option<Radix>,
}

impl Options {
    /// Parses command line arguments to create a new Options object.
    pub fn new(argv: Vec<&str>) -> Options {
        let args = Docopt::new(USAGE)
            .and_then(|d| d.argv(argv.iter()).parse())
            .unwrap_or_else(|e| e.exit());

        if args.get_bool("-v") {
            println!("radix {}", VERSION);
            exit(0);
        }

        let value = args.get_str("<value>").to_string();
        let twos_complement = args.get_bool("-n");

        let radix = if args.get_bool("-b") {
            Some(Radix::Binary)
        } else if args.get_bool("-o") {
            Some(Radix::Octal)
        } else if args.get_bool("-x") {
            Some(Radix::Hexadecimal)
        } else if args.get_bool("-d") {
            Some(Radix::Decimal)
        } else {
            None
        };

        Options {
            value,
            radix,
            twos_complement,
        }
    }
}

/// Given an Options object, returns a string representation of a value in the given radix or an error.
///
/// # Examples
/// ```
/// use radix::Options;
/// use radix::convert;
///
/// // The empty first argument is needed to mimic command line arguments.
/// assert_eq!(convert(Options::new(vec!["", "-d", "0o52"])), Ok("42".to_string()));
/// assert_eq!(convert(Options::new(vec!["", "-o", "0x2a"])), Ok("0o52".to_string()));
/// assert_eq!(convert(Options::new(vec!["", "-x", "0b101010"])), Ok("0x2a".to_string()));
/// ```
pub fn convert(options: Options) -> Result<String, String> {
    let mut n: u32;

    n = if options.value.starts_with('0') && options.value.len() >= 2 {
        match &options.value[0..2] {
            "0b" => from_string_radix(&options.value[2..], 2)?,
            "0o" => from_string_radix(&options.value[2..], 8)?,
            "0x" => from_string_radix(&options.value[2..], 16)?,
            _ => return Err(format!("unknown prefix {}", &options.value[0..2])),
        }
    } else {
        from_string_radix(&options.value, 10)?
    };

    if options.twos_complement {
        // Go through binary if we need two's complement representation.
        n = from_string_radix(
            &trim_leading_ones(&to_string_radix(!n + 1, 2)?, !is_negative(&options.value)),
            2,
        )?;
    }

    if options.radix.is_none() {
        Ok(format!(
            "Decimal: {}\nBinary: 0b{}\nOctal: 0o{}\nHexadecimal: 0x{}",
            to_string_radix(n, 10)?,
            to_string_radix(n, 2)?,
            to_string_radix(n, 8)?,
            to_string_radix(n, 16)?
        ))
    } else {
        Ok(match options.radix.unwrap() {
            Radix::Decimal => format!(
                "{}{}",
                if options.twos_complement { "-" } else { "" },
                to_string_radix(n, 10)?
            ),
            Radix::Binary => format!("0b{}", to_string_radix(n, 2)?),
            Radix::Octal => format!("0o{}", to_string_radix(n, 8)?),
            Radix::Hexadecimal => format!("0x{}", to_string_radix(n, 16)?),
        })
    }
}

fn from_string_radix(s: &str, radix: u32) -> Result<u32, String> {
    let mut result: u32 = 0;
    let mut power = 0;

    for c in s.chars().rev() {
        result = match char::to_digit(c, radix) {
            Some(digit) => match radix
                .checked_pow(power)
                .and_then(|p| digit.checked_mul(p))
                .and_then(|r| result.checked_add(r))
            {
                Some(m) => m,
                None => return Err(format!("{} will overflow a 32-bit integer", s)),
            },
            None => return Err(format!("invalid digit '{}' for radix {}", c, radix)),
        };
        power += 1;
    }

    Ok(result)
}

fn to_string_radix(mut n: u32, radix: u32) -> Result<String, String> {
    let mut s = vec![];

    while n > 0 {
        let d = n % radix;
        match std::char::from_digit(d, radix) {
            Some(c) => s.push(c),
            None => return Err(format!("invalid digit {} for radix {}", d, radix)),
        }
        n /= radix;
    }

    if s.is_empty() {
        Ok(String::from("0"))
    } else {
        Ok(s.iter().rev().collect())
    }
}

fn is_negative(s: &str) -> bool {
    if s.starts_with('0') && s.len() >= 2 {
        match &s[0..2] {
            "0b" => s[2..].starts_with('1'),
            "0o" => {
                char::to_digit(s[2..].chars().next().unwrap_or('0'), 8).map_or(false, |d| d > 4)
            }
            "0x" => {
                char::to_digit(s[2..].chars().next().unwrap_or('0'), 16).map_or(false, |d| d > 7)
            }
            _ => false,
        }
    } else {
        false
    }
}

fn trim_leading_ones(bin_str: &str, leave_one: bool) -> String {
    bin_str
        .chars()
        .position(|c| c == '0')
        .map_or(bin_str.to_string(), |i| {
            format!("{}{}", if leave_one { "1" } else { "" }, &bin_str[i..])
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_decimal_to_binary() {
        assert_eq!(
            convert(Options::new(vec!["", "-b", "42"])),
            Ok(String::from("0b101010"))
        );
    }

    #[test]
    fn converts_hexadecimal_to_binary() {
        assert_eq!(
            convert(Options::new(vec!["", "-b", "0x2a"])),
            Ok(String::from("0b101010"))
        );
    }

    #[test]
    fn converts_octal_to_binary() {
        assert_eq!(
            convert(Options::new(vec!["", "-b", "0o52"])),
            Ok(String::from("0b101010"))
        );
    }

    #[test]
    fn converts_decimal_to_octal() {
        assert_eq!(
            convert(Options::new(vec!["", "-o", "42"])),
            Ok(String::from("0o52"))
        );
    }

    #[test]
    fn converts_binary_to_octal() {
        assert_eq!(
            convert(Options::new(vec!["", "-o", "0b101010"])),
            Ok(String::from("0o52"))
        );
    }

    #[test]
    fn converts_hexadecimal_to_octal() {
        assert_eq!(
            convert(Options::new(vec!["", "-o", "0x2a"])),
            Ok(String::from("0o52"))
        );
    }

    #[test]
    fn converts_decimal_to_hexadecimal() {
        assert_eq!(
            convert(Options::new(vec!["", "-x", "42"])),
            Ok(String::from("0x2a"))
        )
    }

    #[test]
    fn converts_binary_to_hexadecimal() {
        assert_eq!(
            convert(Options::new(vec!["", "-x", "0b101010"])),
            Ok(String::from("0x2a"))
        )
    }

    #[test]
    fn converts_octal_to_hexadecimal() {
        assert_eq!(
            convert(Options::new(vec!["", "-x", "0o52"])),
            Ok(String::from("0x2a"))
        )
    }

    #[test]
    fn converts_binary_to_decimal() {
        assert_eq!(
            convert(Options::new(vec!["", "-d", "0b101010"])),
            Ok(String::from("42"))
        );
    }

    #[test]
    fn converts_hexadecimal_to_decimal() {
        assert_eq!(
            convert(Options::new(vec!["", "-d", "0x2a"])),
            Ok(String::from("42"))
        );
    }

    #[test]
    fn converts_octal_to_decimal() {
        assert_eq!(
            convert(Options::new(vec!["", "-d", "0o52"])),
            Ok(String::from("42"))
        );
    }

    #[test]
    fn converts_its_own_radix() {
        assert_eq!(
            convert(Options::new(vec!["", "-d", "42"])),
            Ok(String::from("42"))
        )
    }

    #[test]
    fn converts_empty_argument() {
        assert_eq!(
            convert(Options::new(vec!["", "-d", "0b"])),
            Ok(String::from("0"))
        );
    }

    #[test]
    fn converts_one_char_arguments() {
        assert_eq!(
            convert(Options::new(vec!["", "-b", "5"])),
            Ok(String::from("0b101"))
        );
    }

    #[test]
    fn converts_negative_decimal_to_binary() {
        assert_eq!(
            convert(Options::new(vec!["", "-b", "-n", "5"])),
            Ok(String::from("0b1011"))
        );
    }

    #[test]
    fn converts_negative_binary_to_decimal() {
        assert_eq!(
            convert(Options::new(vec!["", "-d", "-n", "0b1011"])),
            Ok(String::from("-5"))
        );
    }

    #[test]
    fn does_not_convert_invalid_radix() {
        assert!(convert(Options::new(vec!["", "-d", "0b12"])).is_err());
    }

    #[test]
    fn does_not_convert_invalid_argument() {
        assert!(convert(Options::new(vec!["", "-d", "0h42"])).is_err());
    }

    #[test]
    fn reports_error_on_overflow() {
        assert!(convert(Options::new(vec!["", "-d", "0x123456789"])).is_err());
    }
}
