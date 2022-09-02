use phf::phf_ordered_map;
use std::fmt;
use std::fmt::{Display, Formatter};

const PREFIXES: phf::OrderedMap<&'static str, f64> = phf_ordered_map! {
    "Y" => 1e24_f64,
    "Z" => 1e21_f64,
    "E" => 1e18_f64,
    "T" => 1e15_f64,
    "P" => 1e12_f64,
    "G" => 1e9_f64,
    "M" => 1e6_f64,
    "k" => 1e3_f64,
    ""  => 1e0_f64,
    "m" => 1e-3_f64,
    "u" => 1e-6_f64,
    "n" => 1e-9_f64,
    "p" => 1e-12_f64,
    "f" => 1e-15_f64,
    "a" => 1e-18_f64,
    "z" => 1e-21_f64,
    "y" => 1e-24_f64,
};

#[derive(Debug, Clone)]
pub struct Number {
    /// Base value of this Number
    value: f64,

    /// SI Prefix symbol for this Number
    symbol: String,

    /// Magnitude factor of the SI Prefix
    factor: f64,
}

impl Number {
    /// Computes the normalized value of this Number
    pub fn to_f64(&self) -> f64 {
        self.value * self.factor
    }

    /// Converts the given value into a Number
    /// Performs the necessary calculations to determine the correct prefix
    pub fn from_f64(n: f64) -> Number {
        for (symbol, factor) in PREFIXES.entries() {
            let x = n / factor;
            if (x > 1_f64) & (x < 1000_f64) {
                return Number {
                    value: x,
                    symbol: symbol.to_string(),
                    factor: *factor,
                };
            }
        }
        Number {
            value: n,
            symbol: "".to_string(),
            factor: 1e0_f64,
        }
    }

    pub fn f64_to_string(n: f64) -> String {
        for (symbol, factor) in PREFIXES.entries() {
            let x = n / factor;
            if (1_f64..1000_f64).contains(&x) {
                return format!("{}{}", x, symbol);
            }
        }
        format!("{}", n)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value, self.symbol)
    }
}

#[cfg(test)]
mod tests {
    use crate::number::Number;
    #[test]
    fn format() {
        let n = Number::from_f64(1_024_f64);
        assert_eq!(format!("{}", n), "1.024k");
    }
    #[test]
    fn basic() {
        let n = Number::from_f64(102.4_f64);
        assert_eq!(n.value, 102.4_f64);
        assert_eq!(n.symbol, "");
        assert_eq!(n.factor, 1e0_f64);
        assert_eq!(n.to_f64(), 102.4_f64);
    }
    #[test]
    fn basic_kilo() {
        let n = Number::from_f64(1_024_f64);
        assert_eq!(n.value, 1.024_f64);
        assert_eq!(n.symbol, "k");
        assert_eq!(n.factor, 1e3_f64);
        assert_eq!(n.to_f64(), 1_024_f64);
    }
    #[test]
    fn basic_mega() {
        let n = Number::from_f64(1_024_000_f64);
        assert_eq!(n.value, 1.024_f64);
        assert_eq!(n.symbol, "M");
        assert_eq!(n.factor, 1e6_f64);
        assert_eq!(n.to_f64(), 1_024_000_f64);
    }
    #[test]
    fn basic_milli() {
        let n = Number::from_f64(0.010_24_f64);
        assert_eq!(n.value, 10.24_f64);
        assert_eq!(n.symbol, "m");
        assert_eq!(n.factor, 1e-3_f64);
        assert_eq!(n.to_f64(), 0.010_24_f64);
    }
    #[test]
    fn basic_micro() {
        let n = Number::from_f64(0.000_010_24_f64);
        assert_eq!(n.value, 10.24_f64);
        assert_eq!(n.symbol, "u");
        assert_eq!(n.factor, 1e-6_f64);
        assert_eq!(n.to_f64(), 0.000_010_24_f64);
    }
}
