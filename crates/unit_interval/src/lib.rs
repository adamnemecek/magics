#![warn(missing_docs)]
//! A module for working with unit intervals.

use serde::{Deserialize, Deserializer, Serialize};

/// A value in the closed interval [0.0, 1.0].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnitInterval(f64);

// impl sub and add for `UnitInterval`
impl std::ops::Add<Self> for UnitInterval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.0 + rhs.0 <= 1.0);
        #[allow(clippy::unwrap_used)]
        Self::new(self.0 + rhs.0).unwrap()
    }
}

impl std::ops::Sub<Self> for UnitInterval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self.0 - rhs.0 >= 0.0);
        #[allow(clippy::unwrap_used)]
        Self::new(self.0 - rhs.0).unwrap()
    }
}

/// An error that can occur when creating a `UnitInterval`.
#[derive(Debug)]
pub enum UnitIntervalError {
    /// The value is out of bounds, i.e. not in the interval [0.0, 1.0].
    OutOfBounds(f64),
}

impl std::fmt::Display for UnitIntervalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::OutOfBounds(value) => write!(
                f,
                "value {value} is out of bounds, a UnitInterval represents the closed interval \
                 [0.0, 1.0]",
            ),
        }
    }
}

impl std::error::Error for UnitIntervalError {}

/// A type alias for `Result<T, UnitIntervalError>`.
pub type Result<T> = std::result::Result<T, UnitIntervalError>;

impl UnitInterval {
    /// Creates a new `UnitInterval` from a value. Returns an error if the value
    /// is not in the interval [0.0, 1.0].
    ///
    /// # Errors
    ///
    /// Will return `Err` if `value` is not in the interval [0.0, 1.0]
    pub fn new(value: f64) -> Result<Self> {
        if (0.0..=1.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(UnitIntervalError::OutOfBounds(value))
        }
    }

    /// Creates a new `UnitInterval` from a value without checking if it is in
    /// the interval [0.0, 1.0]. # Safety
    /// The value must be in the interval [0.0, 1.0].
    ///
    /// # Safety
    /// You have to manually ensure the invariant that the given value is
    /// between [0.0, 1.0]
    #[must_use]
    pub const unsafe fn new_unchecked(value: f64) -> Self {
        Self(value)
    }

    /// Returns the inner value of the `UnitInterval`.
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }
}

impl TryFrom<f64> for UnitInterval {
    type Error = UnitIntervalError;

    fn try_from(value: f64) -> Result<Self> {
        Self::new(value)
    }
}

impl TryFrom<f32> for UnitInterval {
    type Error = UnitIntervalError;

    fn try_from(value: f32) -> Result<Self> {
        Self::new(f64::from(value))
    }
}

impl From<UnitInterval> for f64 {
    fn from(unit_interval: UnitInterval) -> Self {
        unit_interval.0
    }
}

impl From<UnitInterval> for f32 {
    #[allow(clippy::cast_possible_truncation)]
    fn from(unit_interval: UnitInterval) -> Self {
        unit_interval.0 as Self
    }
}

impl std::ops::Deref for UnitInterval {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for UnitInterval {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = f64::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

impl Serialize for UnitInterval {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use super::*;

    #[test]
    fn test_new() {
        assert!(matches!(UnitInterval::new(0.0), Ok(UnitInterval(0.0))));
        assert!(matches!(UnitInterval::new(0.5), Ok(UnitInterval(0.5))));
        assert!(matches!(UnitInterval::new(1.0), Ok(UnitInterval(1.0))));
        assert!(matches!(
            UnitInterval::new(-0.1),
            Err(UnitIntervalError::OutOfBounds(-0.1))
        ));

        assert!(matches!(
            UnitInterval::new(1.1),
            Err(UnitIntervalError::OutOfBounds(1.1))
        ));
    }

    #[test]
    #[allow(clippy::undocumented_unsafe_blocks)]
    fn test_new_unchecked() {
        assert_eq!(
            unsafe { UnitInterval::new_unchecked(0.0) },
            UnitInterval(0.0)
        );
        assert_eq!(
            unsafe { UnitInterval::new_unchecked(0.5) },
            UnitInterval(0.5)
        );
        assert_eq!(
            unsafe { UnitInterval::new_unchecked(1.0) },
            UnitInterval(1.0)
        );
    }

    #[test]
    fn test_get() {
        assert_relative_eq!(UnitInterval(0.0).get(), 0.0f64);
        assert_relative_eq!(UnitInterval(0.5).get(), 0.5f64);
        assert_relative_eq!(UnitInterval(1.0).get(), 1.0f64);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_try_from() {
        assert!(matches!(UnitInterval::try_from(0.0), Ok(UnitInterval(0.0))));
        assert!(matches!(UnitInterval::try_from(0.5), Ok(UnitInterval(0.5))));
        assert!(matches!(UnitInterval::try_from(1.0), Ok(UnitInterval(1.0))));
        assert!(matches!(
            UnitInterval::try_from(-0.1),
            Err(UnitIntervalError::OutOfBounds(-0.1))
        ));
        assert!(matches!(
            UnitInterval::try_from(1.1),
            Err(UnitIntervalError::OutOfBounds(1.1))
        ));

        // try_into
        let unit_interval: UnitInterval = 0.5.try_into().unwrap();
        assert_eq!(unit_interval, UnitInterval(0.5));
    }
}
