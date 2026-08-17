use ratatui::prelude::Color;
// use ratatui::style::Color;
use serde::{
    Deserialize, Deserializer, Serialize,
    de::{self, Visitor},
};
use std::fmt;

use super::err::ParseColorErr;

/// A color as written in a theme file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hex(Color);

impl Hex {
    /// Builds an opaque 24-bit color.
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(Color::Rgb(r, g, b))
    }

    /// Defers to whatever the terminal already uses.
    #[must_use]
    pub const fn terminal_default() -> Self {
        Self(Color::Reset)
    }

    /// The underlying ratatui color.
    #[must_use]
    pub const fn color(self) -> Color {
        self.0
    }

    /// Parses `#rgb`, `#rrggbb`, `default`/`reset`, or `indexed:N`.
    pub fn parse(input: &str) -> Result<Self, ParseColorErr> {
        let value = input.trim();

        match value.to_ascii_lowercase().as_str() {
            "default" | "reset" => return Ok(Self(Color::Reset)),
            _ => {}
        }

        if let Some(index) = value.strip_prefix("indexed:") {
            let index: u8 = index
                .trim()
                .parse()
                .map_err(|_| ParseColorErr::new(value))?;
            return Ok(Self(Color::Indexed(index)));
        }

        let digits = value
            .strip_prefix('#')
            .ok_or_else(|| ParseColorErr::new(value))?;

        match digits.len() {
            // #rgb is shorthand for #rrggbb.
            3 => {
                let (r, g, b) = (
                    Self::nibble(digits, 0)?,
                    Self::nibble(digits, 1)?,
                    Self::nibble(digits, 2)?,
                );
                Ok(Self::rgb(r * 17, g * 17, b * 17))
            }
            6 => Ok(Self::rgb(
                Self::byte(digits, 0)?,
                Self::byte(digits, 1)?,
                Self::byte(digits, 2)?,
            )),
            _ => Err(ParseColorErr::new(value)),
        }
    }
    fn nibble(digits: &str, index: usize) -> Result<u8, ParseColorErr> {
        let slice = digits
            .get(index..=index)
            .ok_or_else(|| ParseColorErr::new(digits))?;
        u8::from_str_radix(slice, 16).map_err(|_| ParseColorErr::new(digits))
    }
    fn byte(digits: &str, index: usize) -> Result<u8, ParseColorErr> {
        let start = index * 2;
        let slice = digits
            .get(start..start + 2)
            .ok_or_else(|| ParseColorErr::new(digits))?;
        u8::from_str_radix(slice, 16).map_err(|_| ParseColorErr::new(digits))
    }

    /// Reads a ramp of stops at `position`, running `0.0` to `1.0` first to last.
    #[must_use]
    pub fn blend(stops: &[Hex], position: f32) -> Color {
        let Some(first) = stops.first().copied() else {
            return Color::Reset;
        };

        let last = stops.len().saturating_sub(1);
        if last == 0 {
            return first.0;
        }

        let scaled = position.clamp(0.0, 1.0) * last as f32;
        let index = scaled.floor() as usize;
        let fraction = scaled - index as f32;

        let low = stops.get(index).copied().unwrap_or(first);
        let high = stops.get(index + 1).copied().unwrap_or(low);

        Self::mix(low, high, fraction)
    }

    /// Mixes two stops, or picks the nearer one when they cannot be mixed.
    fn mix(low: Hex, high: Hex, fraction: f32) -> Color {
        match (low.0, high.0) {
            (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => Color::Rgb(
                Self::lerp(r1, r2, fraction),
                Self::lerp(g1, g2, fraction),
                Self::lerp(b1, b2, fraction),
            ),
            _ if fraction < 0.5 => low.0,
            _ => high.0,
        }
    }

    /// One channel, `fraction` of the way from `from` to `to`.
    fn lerp(from: u8, to: u8, fraction: f32) -> u8 {
        let from = f32::from(from);
        let to = f32::from(to);

        (from + (to - from) * fraction).round().clamp(0.0, 255.0) as u8
    }
}
impl<'de> Deserialize<'de> for Hex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(HexVisitor)
    }
}
impl Serialize for Hex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let text = match self.0 {
            Color::Rgb(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
            Color::Indexed(i) => format!("indexed:{i}"),
            _ => "default".to_owned(),
        };
        serializer.serialize_str(&text)
    }
}

impl From<Hex> for Color {
    fn from(hex: Hex) -> Self {
        hex.0
    }
}

pub struct HexVisitor;
impl Visitor<'_> for HexVisitor {
    type Value = Hex;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a color such as \"#1ed760\", \"default\", or \"indexed:4\"")
    }

    fn visit_str<E>(self, value: &str) -> Result<Hex, E>
    where
        E: de::Error,
    {
        Hex::parse(value).map_err(|err| E::custom(err.to_string()))
    }
}
