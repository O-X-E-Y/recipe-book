use serde::{Deserialize, Serialize};
use std::{marker::PhantomData, str::FromStr};
use thiserror::Error;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Metric;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Imperial;

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum MeasurementError {
    #[error("String is empty")]
    EmptyString,
    #[error("Unknown unit")]
    UnknownUnit,
    #[error("Invalid format")]
    InvalidFormat,
    #[error("{0}")]
    CustomString(String), // #[error("{0}")]
                          // ParseFloatError(#[from] std::num::ParseFloatError),
}

/// Weight in mg
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Weight<T = Metric>(u64, PhantomData<T>);

impl<T> Weight<T> {
    pub const POUND: u64 = 453_592;
    pub const OUNCE: u64 = 28_349;

    const OUNCE_LIMIT: u64 = Self::OUNCE * 8;
    const POUND_LIMIT: u64 = Self::POUND * 4;
}

impl Weight {
    pub const fn new_metric(v: u64) -> Weight<Metric> {
        Weight(v, PhantomData)
    }

    pub const fn new_imperial(v: u64) -> Weight<Imperial> {
        Weight(v, PhantomData)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

impl<T> Weight<T> {
    pub const fn as_imperial(self) -> Weight<Imperial> {
        Weight(self.0, PhantomData)
    }

    pub const fn as_metric(self) -> Weight<Metric> {
        Weight(self.0, PhantomData)
    }
}

impl std::fmt::Display for Weight<Metric> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "0 g"),
            n @ 0..1000 => write!(f, "{n} mg"),
            n @ 1000..1_000_000 => write!(f, "{} g", n / 1000),
            n @ 1_000_000..10_000_000 => write!(f, "{:.1} kg", n as f64 / 1_000_000.),
            n @ 10_000_000.. => write!(f, "{} kg", n / 1_000_000),
        }
    }
}

impl std::fmt::Display for Weight<Imperial> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0..250 => write!(f, "0 oz"),
            250..500 => write!(f, "1/8 tsp"),
            500..1000 => write!(f, "1/4 tsp"),
            1000..2000 => write!(f, "1/2 tsp"),
            2000..4000 => write!(f, "1 tsp"),
            4000..8000 => write!(f, "1/2 tbsp"),
            8000..12000 => write!(f, "1 tbsp"),
            n @ 12000..Self::OUNCE => write!(f, "{:.1} oz", n as f64 / Self::OUNCE as f64),
            n @ Self::OUNCE..Self::OUNCE_LIMIT => {
                write!(f, "{:.1} g", n as f64 / Self::OUNCE as f64)
            }
            n @ Self::OUNCE_LIMIT..Self::POUND_LIMIT => {
                write!(f, "{:.1} g", n as f64 / Self::POUND as f64)
            }
            n @ Self::POUND_LIMIT.. => write!(f, "{} g", n / Self::POUND),
        }
    }
}

impl FromStr for Weight {
    type Err = MeasurementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MeasurementError::*;

        if s.is_empty() {
            return Err(EmptyString);
        }

        let (amount, last) = s.split_once(' ').ok_or(InvalidFormat)?;

        let unit = last
            .split_once(' ')
            .map(|(u, _)| u)
            .unwrap_or(last)
            .trim()
            .to_lowercase();
        let unit = unit.as_str().trim_end_matches('s');

        let amount = amount
            .trim()
            .parse::<f64>()
            .map_err(|e| CustomString(e.to_string()))?;

        let weight = match unit {
            "mg" | "milligram" => amount,
            "cg" | "centigram" => amount * 10.0,
            "dg" | "decigram" => amount * 100.0,
            "g" | "gram" => amount * 1_000.0,
            "kg" | "kilogram" => amount * 1_000_000.0,
            "oz" | "ounce" => amount * Self::OUNCE as f64,
            "pound" | "lb" => amount * Self::POUND as f64,
            _ => return Err(UnknownUnit),
        };

        Ok(Weight(weight as u64, PhantomData))
    }
}

/// Volume in 1/1000 mL
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Volume<T = Metric>(u64, PhantomData<T>);

impl<T> Volume<T> {
    pub const TSP: u64 = 4_928;
    pub const TBSP: u64 = 14_786;
    pub const OUNCE: u64 = 29_573;
    pub const RICE_CUP: u64 = 180_000;
    pub const CUP: u64 = 236_588;
    pub const QUART: u64 = 946_353;

    const LOWEST_LIMIT: u64 = Self::TSP / 15;
    const E_TSP_LIMIT: u64 = Self::TSP * 12 / 80;
    const Q_TSP_LIMIT: u64 = Self::TSP * 12 / 40;
    const H_TSP_LIMIT: u64 = Self::TSP * 12 / 20;
    const TQ_TSP_LIMIT: u64 = Self::TSP * 120000 / 133333;
    const TSP_LIMIT: u64 = Self::TSP * 12 / 10;
    const H_TBSP_LIMIT: u64 = Self::TBSP * 12 / 20;
    const TBSP_LIMIT: u64 = Self::TBSP * 12 / 10;
    const OUNCE_LIMIT: u64 = Self::OUNCE * 8;
    const CUP_LIMIT: u64 = Self::QUART * 190 / 200;
    const QUART_LIMIT: u64 = Self::QUART * 5;
}

impl Volume {
    pub const fn new_metric(v: u64) -> Volume<Metric> {
        Volume(v, PhantomData)
    }

    pub const fn new_imperial(v: u64) -> Volume<Imperial> {
        Volume(v, PhantomData)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

impl<T> Volume<T> {
    pub const fn as_imperial(self) -> Volume<Imperial> {
        Volume(self.0, PhantomData)
    }

    pub const fn as_metric(self) -> Volume<Metric> {
        Volume(self.0, PhantomData)
    }
}

impl std::fmt::Display for Volume<Metric> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0..500 => write!(f, "0 ml"),
            n @ 500..500_000 => write!(f, "{} ml", n / 1000),
            n @ 500_000..5_000_000 => write!(f, "{:.1} l", n as f64 / 1_000_000 as f64),
            n @ 5_000_000.. => write!(f, "{} l", n / 1_000_000),
        }
    }
}

impl std::fmt::Display for Volume<Imperial> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0..Self::LOWEST_LIMIT => write!(f, "0 tsp"),
            Self::LOWEST_LIMIT..Self::E_TSP_LIMIT => write!(f, "1/8 tsp"),
            Self::E_TSP_LIMIT..Self::Q_TSP_LIMIT => write!(f, "1/4 tsp"),
            Self::Q_TSP_LIMIT..Self::H_TSP_LIMIT => write!(f, "1/2 tsp"),
            Self::H_TSP_LIMIT..Self::TQ_TSP_LIMIT => write!(f, "3/4 tsp"),
            Self::TQ_TSP_LIMIT..Self::TSP_LIMIT => write!(f, "1 tsp"),
            Self::TSP_LIMIT..Self::H_TBSP_LIMIT => write!(f, "1/2 tbsp"),
            Self::H_TBSP_LIMIT..Self::TBSP_LIMIT => write!(f, "1 tbsp"),
            n @ Self::TBSP_LIMIT..Self::OUNCE_LIMIT => {
                write!(f, "{:.1} floz", n as f64 / Self::OUNCE as f64)
            }
            n @ Self::OUNCE_LIMIT..Self::CUP_LIMIT => {
                write!(f, "{:.1} cups", n as f64 / Self::CUP as f64)
            }
            n @ Self::CUP_LIMIT..Self::QUART_LIMIT => {
                write!(f, "{:.1} quarts", (n as f64 / Self::QUART as f64))
            }
            n @ Self::QUART_LIMIT.. => write!(f, "{} quarts", n / Self::QUART),
        }
    }
}

impl FromStr for Volume {
    type Err = MeasurementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MeasurementError::*;

        if s.is_empty() {
            return Err(EmptyString);
        }

        let (amount, last) = s.split_once(' ').ok_or(InvalidFormat)?;

        let unit = last
            .split_once(' ')
            .map(|(u, _)| u)
            .unwrap_or(last)
            .trim()
            .to_lowercase();
        let unit = unit.as_str().trim_end_matches('s');

        let amount = amount
            .trim()
            .parse::<f64>()
            .map_err(|e| CustomString(e.to_string()))?;

        let volume = match unit {
            "ml" | "milliliter" | "millilitre" => amount * 1_000.0,
            "cl" | "centiliter" | "centilitre" => amount * 10_000.0,
            "dl" | "deciliter" | "decilitre" => amount * 100_000.0,
            "l" | "liter" | "litre" => amount * 1_000_000.0,
            "tsp" => amount * Self::TSP as f64,
            "tbsp" => amount * Self::TBSP as f64,
            "floz" => amount * Self::OUNCE as f64,
            "rice" if s.contains("cup") => amount * Self::RICE_CUP as f64,
            "cup" => amount * Self::CUP as f64,
            "quart" => amount * Self::QUART as f64,
            _ => return Err(UnknownUnit),
        };

        Ok(Volume(volume as u64, PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_metric() {
        let w = Volume::new_metric(900000);
        let i = w.as_imperial();

        println!("{i}");
    }

    #[test]
    fn parse_weight() {
        let a = "10 g";
        let b = "10 pounds of eggs";
        let c = "10000 KGs of cheese";

        assert_eq!(a.parse::<Weight>().unwrap().get(), 10000);
        assert_eq!(
            b.parse::<Weight>().unwrap().get(),
            10 * Weight::<Metric>::POUND
        );
        assert_eq!(c.parse::<Weight>().unwrap().get(), 10_000_000_000);
    }
}
