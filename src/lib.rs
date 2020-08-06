use std::process;

use docopt::Docopt;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const USAGE: &str = "
Usage:
    nconv -h | --help
    nconv -v | --version
    nconv [ -b | -d | -o | -x ] [ -n ] <value>

Options:
    -h, --help         Print this message.
    -v, --version      Print version.
    -b, --binary       Set radix to binary.
    -d, --decimal      Set radix to decimal.
    -n, --negative     Use two's complement.
    -o, --octal        Set radix to octal.
    -x, --hexadecimal  Set radix to hexadecimal.

Example:
    nconv -d 0o27
    nconv -x 0b1010011
    nconv -b --negative 144
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
    pub fn new(argv: &[String]) -> Result<Config, &'static str> {
        let args = Docopt::new(USAGE)
            .and_then(|d| d.argv(argv.iter()).parse())
            .unwrap_or_else(|e| e.exit());

        if args.get_bool("-v") {
            println!("nconv {}", VERSION);
            process::exit(0);
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

        Ok(Config {
            value,
            radix,
            negative,
        })
    }
}

pub fn run(config: Config) -> Result<String, String> {
    println!("{:?}", config);

    if config.value.starts_with('0') && config.value.len() >= 2 {
        let n = match &config.value[0..2] {
            "0b" => from_string_radix(&config.value[2..], 2)?,
            "0o" => from_string_radix(&config.value[2..], 8)?,
            "0x" => from_string_radix(&config.value[2..], 16)?,
            _ => return Err(format!("Unknown prefix {}.", &config.value[0..2])),
        };

        match config.radix {
            Radix::Decimal => to_string_radix(n, 10),
            Radix::Binary => Ok(format!("0b{}", to_string_radix(n, 2)?)),
            Radix::Octal => Ok(format!("0o{}", to_string_radix(n, 8)?)),
            Radix::Hexadecimal => Ok(format!("0x{}", to_string_radix(n, 16)?)),
        }
    } else {
        let n = from_string_radix(&config.value, 10)?;
        match config.radix {
            Radix::Decimal => to_string_radix(n, 10),
            Radix::Binary => Ok(format!("0b{}", to_string_radix(n, 2)?)),
            Radix::Octal => Ok(format!("0o{}", to_string_radix(n, 8)?)),
            Radix::Hexadecimal => Ok(format!("0x{}", to_string_radix(n, 16)?)),
        }
    }
}

fn to_string_radix(mut n: u32, radix: u32) -> Result<String, String> {
    let mut s = vec![];

    while n > 0 {
        let d = n % radix;
        match std::char::from_digit(d, radix) {
            Some(c) => s.push(c),
            None => {
                return Err(format!(
                    "Invalid conversion value {} for radix {}.",
                    d, radix
                ))
            }
        }
        n /= radix;
    }

    if s.is_empty() {
        Ok(String::from("0"))
    } else {
        Ok(s.iter().rev().collect())
    }
}

fn from_string_radix(s: &str, radix: u32) -> Result<u32, String> {
    let mut result = 0;
    let mut power = 0;

    for c in s.chars().rev() {
        match char::to_digit(c, radix) {
            Some(n) => {
                if n >= radix {
                    return Err(format!("Invalid digit {} for radix {}.", n, radix));
                }
                result += n * radix.pow(power);
                power += 1;
            }
            None => return Err(format!("Cannot convert {} to integer radix {}.", s, radix)),
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config(mut args: Vec<&'static str>) -> Config {
        args.insert(0, "nconv");
        let args = args.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        Config::new(&args).unwrap()
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
    fn does_not_convert_invalid_argument() {
        let args = vec!["0h42"];
        let config = test_config(args);
        assert!(run(config).is_err());
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
    fn does_not_convert_invalid_radix() {
        let args = vec!["0b12"];
        let config = test_config(args);
        assert!(run(config).is_err());
    }

    // #[test]
    // fn converts_negative_decimal_to_binary() {
    //     let args = vec!["-b", "-n", "127"];
    //     let config = test_config(args);
    //     assert_eq!(run(config), Ok(String::from("0b10000001")))
    // }

    // #[test]
    // fn converts_negative_binary_to_decimal() {
    //     let args = vec!["-d", "-n", "0b10000001"];
    //     let config = test_config(args);
    //     assert_eq!(run(config), Ok(String::from("-127")))
    // }
}
