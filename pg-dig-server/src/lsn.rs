#![allow(dead_code)]

use std::fmt;
use std::str::FromStr;

/// Struct to represent a PostgreSQL LSN (Log Sequence Number)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lsn {
    high: u32,
    low: u32,
}

impl Lsn {
    /// Creates a new LSN from high and low 32-bit parts
    fn new(high: u32, low: u32) -> Self {
        Lsn { high, low }
    }

    /// Converts the LSN to a single 64-bit value
    fn to_u64(&self) -> u64 {
        ((self.high as u64) << 32) | (self.low as u64)
    }

    /// Creates an LSN from a 64-bit value
    pub(crate) fn from_u64(value: u64) -> Self {
        let high = (value >> 32) as u32;
        let low = (value & 0xFFFF_FFFF) as u32;
        Lsn::new(high, low)
    }
}

impl FromStr for Lsn {
    type Err = String;

    /// Parses an LSN string like "0/16B3740" into an Lsn struct
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid LSN format: {}", s));
        }

        let high = u32::from_str_radix(parts[0], 16)
            .map_err(|e| format!("Failed to parse high part: {}", e))?;
        let low = u32::from_str_radix(parts[1], 16)
            .map_err(|e| format!("Failed to parse low part: {}", e))?;
        Ok(Lsn::new(high, low))
    }
}

impl fmt::Display for Lsn {
    /// Formats the LSN as a string in "high/low" hexadecimal format
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:X}/{:X}", self.high, self.low)
    }
}