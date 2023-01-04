//! The `OrdVec` trait provides an extension to `Vec` to allow for inserting items in order.
//!
//! # Examples
//!
//! ```
//! use ordered_vec::OrdVec;
//!
//! let mut values: Vec<i32> = Vec::new();
//! values.push_ord(5);
//! values.push_ord(3);
//! values.push_ord(7);
//! values.push_ord(1);
//!
//! assert_eq!(values, [1, 3, 5, 7]);
//! ```

use std::cmp::Ordering;

/// A trait for adding elements to a vector in sorted order
pub trait OrdVec<T: PartialOrd> {
    /// Inserts `item` into `self` in sorted order. Returns the index at which `item` was inserted.
    /// # Examples
    ///
    /// ```
    ///use ordered_vec::OrdVec;
    ///let mut values: Vec<f64> = Vec::new();
    ///assert_eq!(values.push_ord(5.5), Ok(0));
    ///assert_eq!(values, [5.5]);
    ///
    ///assert_eq!(values.push_ord(3.14), Ok(0));
    ///assert_eq!(values, [3.14, 5.5]);
    ///
    ///assert_eq!(values.push_ord(7.77), Ok(2));
    ///assert_eq!(values, [3.14, 5.5, 7.77]);
    ///
    /// ```
    fn push_ord(&mut self, item: T) -> Result<usize, OrdVecError>;
}

impl<T: PartialOrd> OrdVec<T> for Vec<T> {
    fn push_ord(&mut self, item: T) -> Result<usize, OrdVecError> {
        let idx = binary_search_index(&item, self);
        match idx {
            Some(idx) => {
                self.insert(idx, item);
                Ok(idx)
            }
            None => Err(OrdVecError),
        }
    }
}

// Create an OrdVecError struct that implements the error and Display trait
#[derive(Debug, PartialEq, Eq)]
pub struct OrdVecError;

impl std::fmt::Display for OrdVecError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Failed to add item to vector")
    }
}

impl std::error::Error for OrdVecError {}

/// Finds the mid_point between two integers
#[inline]
fn mid_point(a: usize, b: usize) -> usize {
    (a + b) / 2
}

/// Returns the index at which `item` should be inserted into `values` to preserve sorted order.
#[inline]
fn binary_search_index<T: PartialOrd>(item: &T, values: &[T]) -> Option<usize> {
    if values.is_empty() {
        return Some(0);
    }
    let mut start: usize = 0;
    let mut end = values.len() - 1;
    if item <= &values[start] {
        return Some(start);
    }

    if item >= &values[end] {
        return Some(end + 1);
    }
    let mut idx: usize = mid_point(start, end);
    loop {
        match item.partial_cmp(&values[idx]) {
            Some(Ordering::Less) => {
                end = idx;
                idx = mid_point(start, end);
                if end - start <= 1 {
                    idx = end;
                    break;
                }
            }
            Some(Ordering::Greater) => {
                start = idx;
                idx = mid_point(start, end);
                if end - start <= 1 {
                    idx = end;
                    break;
                }
            }
            Some(Ordering::Equal) => {
                break;
            }
            None => {
                return None;
            }
        }
    }
    Some(idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mid_point() {
        for i in 0..100 {
            assert_eq!(mid_point(0, i), i / 2);
        }
    }

    #[test]
    fn test_binary_search_index() {
        let values_1 = [1, 2, 3, 4, 5];
        assert_eq!(binary_search_index(&4, &values_1), Some(3));
        assert_eq!(binary_search_index(&6, &values_1), Some(5));
        assert_eq!(binary_search_index(&0, &values_1), Some(0));

        let values_2 = ["apple", "banana", "cherry", "date", "elderberry"];
        assert_eq!(binary_search_index(&"cherry", &values_2), Some(2));
        assert_eq!(binary_search_index(&"fig", &values_2), Some(5));
        assert_eq!(binary_search_index(&"apricot", &values_2), Some(1));
        assert_eq!(binary_search_index(&"aardvark", &values_2), Some(0));

        let values_3 = [1.5, 2.7, 3.1, 4.9, 5.2];
        assert_eq!(binary_search_index(&2.7, &values_3), Some(1));
        assert_eq!(binary_search_index(&3.15, &values_3), Some(3));
        assert_eq!(binary_search_index(&1.0, &values_3), Some(0));

        let values_4 = [1, 2, 3, 4];
        assert_eq!(binary_search_index(&5, &values_4), Some(4));
        assert_eq!(binary_search_index(&0, &values_4), Some(0));
    }

    #[test]
    fn test_ord_vec() {
        let mut values: Vec<i32> = Vec::new();

        assert_eq!(values.push_ord(5), Ok(0));
        assert_eq!(values, [5]);

        assert_eq!(values.push_ord(3), Ok(0));
        assert_eq!(values, [3, 5]);

        assert_eq!(values.push_ord(7), Ok(2));
        assert_eq!(values, [3, 5, 7]);

        assert_eq!(values.push_ord(1), Ok(0));
        assert_eq!(values, [1, 3, 5, 7]);

        assert_eq!(values.push_ord(9), Ok(4));
        assert_eq!(values, [1, 3, 5, 7, 9]);

        assert_eq!(values.push_ord(8), Ok(4));
        assert_eq!(values, [1, 3, 5, 7, 8, 9]);

        assert_eq!(values.push_ord(100), Ok(6));
        assert_eq!(values, [1, 3, 5, 7, 8, 9, 100]);

        assert_eq!(values.push_ord(3), Ok(1));
        assert_eq!(values, [1, 3, 3, 5, 7, 8, 9, 100]);

        assert_eq!(values.push_ord(2), Ok(1));
        assert_eq!(values, [1, 2, 3, 3, 5, 7, 8, 9, 100]);

        assert_eq!(values.push_ord(0), Ok(0));
        assert_eq!(values, [0, 1, 2, 3, 3, 5, 7, 8, 9, 100]);
    }
}
