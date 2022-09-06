//
// SPDX-License-Identifier: Apache-2.0 OR MIT
//
// Copyright (C) 2022 Shun Sakai
//

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub enum FromHexError {
    ParseIntError(ParseIntError),
    HexFormatError,
}

impl fmt::Display for FromHexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseIntError(err) => {
                write!(f, "{}", err)
            }
            Self::HexFormatError => {
                write!(f, "Invalid hexadecimal notation")
            }
        }
    }
}

impl Error for FromHexError {}

impl From<ParseIntError> for FromHexError {
    fn from(error: ParseIntError) -> Self {
        Self::ParseIntError(error)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: Option<u8>,
}

impl Color {
    /// Returns the components as an array.
    pub fn channels(&self) -> [u8; 4] {
        [
            self.red,
            self.green,
            self.blue,
            self.alpha.unwrap_or(u8::MAX),
        ]
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{self:x}")
    }
}

impl FromStr for Color {
    type Err = FromHexError;

    fn from_str(hex: &str) -> Result<Self, Self::Err> {
        let hex = hex.strip_prefix('#').unwrap_or(hex);
        match hex.len() {
            3 | 4 => {
                let red = u8::from_str_radix(&hex[..1].repeat(2), 16)?;
                let green = u8::from_str_radix(&hex[1..2].repeat(2), 16)?;
                let blue = u8::from_str_radix(&hex[2..3].repeat(2), 16)?;
                let alpha = hex
                    .get(3..4)
                    .map(|a| u8::from_str_radix(&a.repeat(2), 16))
                    .transpose()?;
                Ok(Self {
                    red,
                    green,
                    blue,
                    alpha,
                })
            }
            6 | 8 => {
                let red = u8::from_str_radix(&hex[..2], 16)?;
                let green = u8::from_str_radix(&hex[2..4], 16)?;
                let blue = u8::from_str_radix(&hex[4..6], 16)?;
                let alpha = hex
                    .get(6..8)
                    .map(|a| u8::from_str_radix(a, 16))
                    .transpose()?;
                Ok(Self {
                    red,
                    green,
                    blue,
                    alpha,
                })
            }
            _ => Err(Self::Err::HexFormatError),
        }
    }
}

impl fmt::LowerHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(alpha) = self.alpha {
            write!(
                f,
                "{:02x}{:02x}{:02x}{:02x}",
                self.red, self.green, self.blue, alpha
            )
        } else {
            write!(f, "{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        assert_eq!(
            format!(
                "{}",
                Color {
                    red: 18,
                    green: 58,
                    blue: 188,
                    alpha: None
                }
            ),
            "#123abc"
        );
        assert_eq!(
            format!(
                "{}",
                Color {
                    red: 18,
                    green: 52,
                    blue: 171,
                    alpha: Some(205)
                }
            ),
            "#1234abcd"
        );
    }

    #[test]
    fn from_str_of_rgb() {
        assert_eq!(
            Color::from_str("#123abc").unwrap(),
            Color {
                red: 18,
                green: 58,
                blue: 188,
                alpha: None
            }
        );
        assert_eq!(
            Color::from_str("123abc").unwrap(),
            Color {
                red: 18,
                green: 58,
                blue: 188,
                alpha: None
            }
        );
        assert_eq!(
            Color::from_str("#000000").unwrap(),
            Color {
                red: u8::MIN,
                green: u8::MIN,
                blue: u8::MIN,
                alpha: None
            }
        );
        assert_eq!(
            Color::from_str("#ffffff").unwrap(),
            Color {
                red: u8::MAX,
                green: u8::MAX,
                blue: u8::MAX,
                alpha: None
            }
        );
        assert!(Color::from_str("#gggggg").is_err());
        assert_eq!(
            Color::from_str("#123").unwrap(),
            Color {
                red: 17,
                green: 34,
                blue: 51,
                alpha: None
            }
        );
        assert_eq!(
            Color::from_str("abc").unwrap(),
            Color {
                red: 170,
                green: 187,
                blue: 204,
                alpha: None
            }
        );
        assert_eq!(
            Color::from_str("#000").unwrap(),
            Color {
                red: u8::MIN,
                green: u8::MIN,
                blue: u8::MIN,
                alpha: None
            }
        );
        assert_eq!(
            Color::from_str("#fff").unwrap(),
            Color {
                red: u8::MAX,
                green: u8::MAX,
                blue: u8::MAX,
                alpha: None
            }
        );
        assert!(Color::from_str("#ggg").is_err());
    }

    #[test]
    fn from_str_of_rgba() {
        assert_eq!(
            Color::from_str("#1234abcd").unwrap(),
            Color {
                red: 18,
                green: 52,
                blue: 171,
                alpha: Some(205)
            }
        );
        assert_eq!(
            Color::from_str("1234abcd").unwrap(),
            Color {
                red: 18,
                green: 52,
                blue: 171,
                alpha: Some(205)
            }
        );
        assert_eq!(
            Color::from_str("#00000000").unwrap(),
            Color {
                red: u8::MIN,
                green: u8::MIN,
                blue: u8::MIN,
                alpha: Some(u8::MIN)
            }
        );
        assert_eq!(
            Color::from_str("#ffffffff").unwrap(),
            Color {
                red: u8::MAX,
                green: u8::MAX,
                blue: u8::MAX,
                alpha: Some(u8::MAX)
            }
        );
        assert!(Color::from_str("#gggggggg").is_err());
        assert_eq!(
            Color::from_str("#1234").unwrap(),
            Color {
                red: 17,
                green: 34,
                blue: 51,
                alpha: Some(68)
            }
        );
        assert_eq!(
            Color::from_str("abcd").unwrap(),
            Color {
                red: 170,
                green: 187,
                blue: 204,
                alpha: Some(221)
            }
        );
        assert_eq!(
            Color::from_str("#0000").unwrap(),
            Color {
                red: u8::MIN,
                green: u8::MIN,
                blue: u8::MIN,
                alpha: Some(u8::MIN)
            }
        );
        assert_eq!(
            Color::from_str("#ffff").unwrap(),
            Color {
                red: u8::MAX,
                green: u8::MAX,
                blue: u8::MAX,
                alpha: Some(u8::MAX)
            }
        );
        assert!(Color::from_str("#gggg").is_err());
    }

    #[test]
    fn from_str_of_invalid_hexadecimal_notation() {
        assert!(Color::from_str("#1").is_err());
        assert!(Color::from_str("1").is_err());
        assert!(Color::from_str("#12").is_err());
        assert!(Color::from_str("12").is_err());
        assert!(Color::from_str("#1234a").is_err());
        assert!(Color::from_str("1234a").is_err());
        assert!(Color::from_str("#1234abc").is_err());
        assert!(Color::from_str("1234abc").is_err());

        assert!(Color::from_str("#").is_err());
        assert!(Color::from_str("").is_err());
    }

    #[test]
    fn lower_hex() {
        assert_eq!(
            format!(
                "{:x}",
                Color {
                    red: 18,
                    green: 58,
                    blue: 188,
                    alpha: None
                }
            ),
            "123abc"
        );
        assert_eq!(
            format!(
                "{:x}",
                Color {
                    red: 18,
                    green: 52,
                    blue: 171,
                    alpha: Some(205)
                }
            ),
            "1234abcd"
        );
    }
}
