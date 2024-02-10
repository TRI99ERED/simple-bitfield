//! Module containing [`ConvError`] and [`ConvTarget`].

use std::{
    error::Error,
    fmt::{Debug, Display},
};

pub type ConvResult<T> = Result<T, ConvError>;

/// Target or instigator for conversions.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ConvTarget {
    Set(usize),
    Index(usize),
    Enum(usize),
    Raw(usize),
}

/// Conversion error.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConvError {
    from: ConvTarget,
    to: ConvTarget,
}

impl Debug for ConvTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Set(size) => write!(f, "Bitset{size}"),
            Self::Index(size) => write!(f, "Index<Bitset{size}>"),
            Self::Enum(size) => write!(f, "Enum({size} variants)"),
            Self::Raw(n) => write!(f, "{n}usize"),
        }
    }
}

impl Display for ConvTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Set(size) => write!(f, "Bitset (size {size})"),
            Self::Index(max) => write!(f, "Index (max = {max})"),
            Self::Enum(size) => write!(f, "Enum ({size} variants)"),
            Self::Raw(n) => write!(f, "{n}usize"),
        }
    }
}

impl ConvError {
    /// Constructs new value of ConvError.
    ///
    /// # Examples
    /// ```rust
    /// # use std::error::Error;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use bitworks::error::{ConvError, ConvTarget};
    ///
    /// //Oh no! I couldn't convert from a bitset to my enum!
    /// let error = ConvError::new(ConvTarget::Set(8), ConvTarget::Enum(8));
    ///
    /// assert_eq!(error.to_string(), "failed to convert from Bitset (size 8) to Enum (8 variants)");
    /// #   Ok(())
    /// # }
    /// ```
    pub const fn new(from: ConvTarget, to: ConvTarget) -> Self {
        Self { from, to }
    }
}

impl Error for ConvError {}

impl Display for ConvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to convert from {} to {}", self.from, self.to)
    }
}
