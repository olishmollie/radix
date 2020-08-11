use std::process::exit;

use docopt::Docopt;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const USAGE: &str = "
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
    radix -d 0o27
    radix -x 0b1010011
    radix -b --negative 144
";

#[derive(Debug)]
enum Radix {
    Binary,
    Decimal,
    Octal,
    Hexadecimal,
}

#[derive(Debug)]
pub struct Config {
    value: String,
    radix: Radix,
    negative: bool,
}

impl Config {
    pub fn new(argv: &[String]) -> Config {
        let args = Docopt::new(USAGE)
            .and_then(|d| d.argv(argv.iter()).parse())
            .unwrap_or_else(|e| e.exit());

        if args.get_bool("-v") {
            println!("radix {}", VERSION);
            exit(0);
        }

        let value = args.get_str("<value>").to_string();
        let negative = args.get_bool("-n");

        let radix: Radix;
        if args.get_bool("-b") {
            radix = Radix::Binary;
        } else if args.get_bool("-o") {
            radix = Radix::Octal;
        } else if args.get_bool("-x") {
            radix = Radix::Hexadecimal;
        } else {
            radix = Radix::Decimal;
        }

        Config {
            value,
            radix,
            negative,
        }
    }
}

pub fn run(config: Config) -> Result<String, String> {
    let mut n: u32;
    let mut s: String;

    n = if config.value.starts_with('0') && config.value.len() >= 2 {
        match &config.value[0..2] {
            "0b" => from_string_radix(&config.value[2..], 2)?,
            "0o" => from_string_radix(&config.value[2..], 8)?,
            "0x" => from_string_radix(&config.value[2..], 16)?,
            _ => return Err(format!("unknown prefix {}", &config.value[0..2])),
        }
    } else {
        from_string_radix(&config.value, 10)?
    };

    if config.negative {
        println!("is_negative = {}", is_negative(&config.value));
        s = trim_leading_ones(&to_string_radix(!n + 1, 2)?, !is_negative(&config.value));
        n = from_string_radix(&s, 2)?;
        println!("s = {}, n = {}", s, n);
    }

    s = match config.radix {
        Radix::Decimal => format!(
            "{}{}",
            if config.negative { "-" } else { "" },
            to_string_radix(n, 10)?
        ),
        Radix::Binary => format!("0b{}", to_string_radix(n, 2)?),
        Radix::Octal => format!("0o{}", to_string_radix(n, 8)?),
        Radix::Hexadecimal => format!("0x{}", to_string_radix(n, 16)?),
    };

    Ok(s)
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

    fn test_config(mut args: Vec<&'static str>) -> Config {
        args.insert(0, "radix");
        let args = args.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        Config::new(&args)
    }

    #[test]
    fn converts_decimal_to_binary() {
        let args = vec!["42", "-b"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0b101010")));
    }

    #[test]
    fn converts_hexadecimal_to_binary() {
        let args = vec!["0x2a", "-b"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0b101010")));
    }

    #[test]
    fn converts_octal_to_binary() {
        let args = vec!["0o52", "-b"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0b101010")));
    }

    #[test]
    fn converts_decimal_to_octal() {
        let args = vec!["42", "-o"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0o52")));
    }

    #[test]
    fn converts_binary_to_octal() {
        let args = vec!["0b101010", "-o"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0o52")));
    }

    #[test]
    fn converts_hexadecimal_to_octal() {
        let args = vec!["0x2a", "-o"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0o52")));
    }

    #[test]
    fn converts_decimal_to_hexadecimal() {
        let args = vec!["42", "-x"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0x2a")))
    }

    #[test]
    fn converts_binary_to_hexadecimal() {
        let args = vec!["0b101010", "-x"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0x2a")))
    }

    #[test]
    fn converts_octal_to_hexadecimal() {
        let args = vec!["0o52", "-x"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0x2a")))
    }

    #[test]
    fn converts_binary_to_decimal() {
        let args = vec!["0b101010"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("42")));
    }

    #[test]
    fn converts_hexadecimal_to_decimal() {
        let args = vec!["0x2a"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("42")));
    }

    #[test]
    fn converts_octal_to_decimal() {
        let args = vec!["0o52"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("42")));
    }

    #[test]
    fn parses_long_flags() {
        let args = vec!["0o52", "--binary"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0b101010")));
    }

    #[test]
    fn converts_its_own_radix() {
        let args = vec!["42", "-d"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("42")))
    }

    #[test]
    fn converts_empty_argument() {
        let args = vec!["0b"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0")));
    }

    #[test]
    fn converts_one_char_arguments() {
        let args = vec!["5", "-b"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0b101")));
    }

    #[test]
    fn converts_negative_decimal_to_binary() {
        let args = vec!["5", "-b", "-n"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("0b1011")));
    }

    #[test]
    fn converts_negative_binary_to_decimal() {
        let args = vec!["0b1011", "-n"];
        let config = test_config(args);
        assert_eq!(run(config), Ok(String::from("-5")));
    }

    #[test]
    fn does_not_convert_invalid_radix() {
        let args = vec!["0b12"];
        let config = test_config(args);
        assert!(run(config).is_err());
    }

    #[test]
    fn does_not_convert_invalid_argument() {
        let args = vec!["0h42"];
        let config = test_config(args);
        assert!(run(config).is_err());
    }

    #[test]
    fn reports_error_on_overflow() {
        let args = vec!["0x23423349827349"];
        let config = test_config(args);
        assert!(run(config).is_err());
    }

    #[test]
    fn test_is_negative() {
        assert!(is_negative("0b1011"));
    }
}
