#![warn(missing_docs)]
//! A simple crate for a vector with a minimum length.

use serde::{Deserialize, Deserializer, Serialize};

/// A vector with a minimum length.
///
/// It is a wrapper around `Vec<T>` that ensures that the vector has at least
/// `N` elements. It is useful when you want to ensure that a vector has at
/// least a certain number of elements. but don't want to check it every time
/// you access the vector.
#[derive(Debug)]
pub struct MinLenVec<T, const N: usize>(Vec<T>);

impl<T: std::clone::Clone, const N: usize> std::clone::Clone for MinLenVec<T, N> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
        // self.0.clone()
        // todo!()
    }
}

/// Error type for `MinLenVec`.
#[derive(Debug, PartialEq, Eq)]
pub enum MinLenVecError {
    /// Not enough elements in the vector.
    /// Happens when you call [`MinLenVec::new`] with a vector that has less
    /// than N elements. Or when you call [`MinLenVec::pop`] and the vector
    /// has exactly N elements.
    // NotEnoughElements(usize),
    NotEnoughElements {
        /// Minimum number of elements
        min: usize,
        /// Actual number of elements
        actual: usize,
    },
}

impl std::fmt::Display for MinLenVecError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NotEnoughElements { min, actual } => write!(
                f,
                "Not enough elements, expected at least {min}, got {actual}"
            ),
        }
    }
}

impl std::error::Error for MinLenVecError {}

/// Result type for `MinLenVec`.
pub type Result<T> = std::result::Result<T, MinLenVecError>;

#[allow(clippy::len_without_is_empty)]
impl<T, const N: usize> MinLenVec<T, N> {
    /// Create a new `MinLenVec` from a vector.
    /// Returns an error if the vector has less than `N` elements.
    ///
    /// # Errors
    ///
    /// Will return `Err` if then length of `data` is less than `N`
    pub fn new(data: Vec<T>) -> Result<Self> {
        if data.len() < N {
            return Err(MinLenVecError::NotEnoughElements {
                min: N,
                actual: data.len(),
            });
        }
        Ok(Self(data))
    }

    /// Consume the `MinLenVec` and return the inner vector.
    #[inline(always)]
    #[must_use]
    pub fn into_inner(self) -> Vec<T> {
        self.0
    }

    /// Get the length of the vector.
    /// This is the length of the inner vector, not the minimum length.
    #[inline(always)]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Get a slice of the elements of the vector.
    #[inline(always)]
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        self.0.as_slice()
    }

    /// Get a mutable slice of the elements of the vector.
    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.0.as_mut_slice()
    }

    /// Returns an iterator over references of self
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }

    /// Push an element to the vector.
    #[inline(always)]
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    /// Pop an element from the vector.
    ///
    /// # Errors
    ///
    /// Will return `Err` if the vector has exactly `N` elements.
    /// This is to ensure the invariant that the vector always has at least `N`
    /// elements.
    #[inline(always)]
    #[allow(clippy::missing_panics_doc)] // if it panics, it means there is a bug in our implementation
    pub fn pop(&mut self) -> Result<T> {
        if self.0.len() <= N {
            return Err(MinLenVecError::NotEnoughElements {
                min: N,
                actual: self.0.len(),
            });
        }
        Ok(self.0.pop().expect("there is always at least N elements"))
    }

    /// Get a reference to the first element of the vector.
    /// Since the vector has at least `N` elements, this will always return a
    /// first element.
    #[inline(always)]
    #[must_use]
    pub fn first(&self) -> &T {
        &self.0[0]
    }

    /// Get a reference to the last element of the vector.
    /// Since the vector has at least `N` elements, this will always return a
    /// last element.
    #[inline(always)]
    #[must_use]
    pub fn last(&self) -> &T {
        &self.0[self.0.len() - 1]
    }
}

impl<T, const N: usize> std::iter::IntoIterator for MinLenVec<T, N> {
    type IntoIter = std::vec::IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T, const N: usize> std::ops::Index<usize> for MinLenVec<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, const N: usize> std::ops::IndexMut<usize> for MinLenVec<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T, const N: usize> TryFrom<Vec<T>> for MinLenVec<T, N> {
    type Error = MinLenVecError;

    fn try_from(value: Vec<T>) -> std::result::Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<T, const N: usize> From<MinLenVec<T, N>> for Vec<T> {
    fn from(value: MinLenVec<T, N>) -> Self {
        value.into_inner()
    }
}

impl<T: Clone, const N: usize> From<[T; N]> for MinLenVec<T, N> {
    fn from(value: [T; N]) -> Self {
        Self::new(value.to_vec()).expect("there are always N elements")
    }
}

impl<T, const N: usize> Serialize for MinLenVec<T, N>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T, const N: usize> Deserialize<'de> for MinLenVec<T, N>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Vec::<T>::deserialize(deserializer)?;
        Self::new(v).map_err(serde::de::Error::custom)
    }
}

/// A type alias for a `MinLenVec` with a minimum length of 1.
pub type OneOrMore<T> = MinLenVec<T, 1>;
/// A type alias for a `MinLenVec` with a minimum length of 2.
pub type TwoOrMore<T> = MinLenVec<T, 2>;

/// Creates a `OneOrMore`<T> containing the arguments.
#[macro_export]
macro_rules! one_or_more {
    ($first:expr $(, $rest:expr)* $(,)?) => {
        OneOrMore::new(vec![$first $(, $rest)*]).expect("a vector of length 1 or more provided")
    };
}

/// Creates a `TwoOrMore`<T> containing the arguments.
#[macro_export]
macro_rules! two_or_more {
    ($first:expr, $second:expr $(, $rest:expr)* $(,)?) => {
        TwoOrMore::new(vec![$first, $second $(, $rest)*]).expect("a vector of length 2 or more provided")
    };
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_min_len_vec() {
        assert!(matches!(
            MinLenVec::<_, 3>::new(vec![1, 2]),
            Err(MinLenVecError::NotEnoughElements { min: 3, actual: 2 })
        ));

        assert!(matches!(
            MinLenVec::<_, 3>::new(vec![1, 2, 3]),
            Ok(MinLenVec(_))
        ));

        assert!(matches!(
            MinLenVec::<_, 1>::new(vec![1.0]),
            Ok(MinLenVec(_))
        ));
        assert!(matches!(
            MinLenVec::<i32, 1>::new(vec![]),
            Err(MinLenVecError::NotEnoughElements { min: 1, actual: 0 })
        ));
    }

    #[test]
    fn test_min_len_vec_push() {
        let mut v = MinLenVec::<_, 3>::new(vec![1, 2, 3]).unwrap();
        assert_eq!(v.len(), 3);
        v.push(4);
        assert_eq!(v.len(), 4);
        v.push(5);
        assert_eq!(v.len(), 5);
    }

    #[test]
    fn test_min_len_vec_pop() {
        let mut v = MinLenVec::<_, 3>::new(vec![1, 2, 3, 4, 5]).unwrap();
        assert_eq!(v.len(), 5);
        assert_eq!(v.pop(), Ok(5));
        assert_eq!(v.len(), 4);
        assert_eq!(v.pop(), Ok(4));
        assert_eq!(v.len(), 3);
        assert_eq!(
            v.pop(),
            Err(MinLenVecError::NotEnoughElements { min: 3, actual: 3 })
        );
        assert_eq!(v.len(), 3);
        assert_eq!(
            v.pop(),
            Err(MinLenVecError::NotEnoughElements { min: 3, actual: 3 })
        );
        assert_eq!(v.len(), 3);
    }

    #[test]
    fn test_min_len_vec_index() {
        let v = MinLenVec::<_, 3>::new(vec![1, 2, 3]).unwrap();
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);
    }

    #[test]
    fn test_min_len_vec_into_inner() {
        let v = MinLenVec::<_, 3>::new(vec![1, 2, 3]).unwrap();
        let inner = v.into_inner();
        assert_eq!(inner, vec![1, 2, 3]);
    }

    #[test]
    fn test_min_len_vec_from() {
        let v = MinLenVec::<_, 3>::from([1, 2, 3]);
        assert_eq!(v.len(), 3);
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);
    }

    #[test]
    fn test_first() {
        let v = MinLenVec::<_, 3>::new(vec![1, 2, 3]).unwrap();
        assert_eq!(v.first(), &1);
    }

    #[test]
    fn test_last() {
        let v = MinLenVec::<_, 4>::new(vec![1, 2, 3, 4]).unwrap();
        assert_eq!(v.last(), &4);
    }

    #[test]
    fn test_one_or_more_macro() {
        let v = one_or_more!["one"];
        assert_eq!(v.into_inner(), vec!["one"]);

        let v = one_or_more![1, 2, 3];
        assert_eq!(v.into_inner(), vec![1, 2, 3]);
    }

    #[test]
    fn test_two_or_more_macro() {
        let v = two_or_more!["one", "two"];
        assert_eq!(v.into_inner(), vec!["one", "two"]);

        let v = two_or_more![1, 2, 3];
        assert_eq!(v.into_inner(), vec![1, 2, 3]);
    }

    #[test]
    fn test_try_into() {
        let v = vec![1, 2];
        let mlv: MinLenVec<_, 2> = TryFrom::try_from(v).unwrap();

        assert_eq!(mlv.len(), 2);
        assert_eq!(mlv[0], 1);
        assert_eq!(mlv[1], 2);

        let v = vec![1];
        assert!(matches!(
            MinLenVec::<_, 2>::try_from(v),
            Err(MinLenVecError::NotEnoughElements { min: 2, actual: 1 })
        ));
    }
}
