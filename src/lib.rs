//! The `OrdVec` trait provides an extension to `Vec` to allow for inserting items in order, both
//! ascending and descending.
//!
//! # Examples
//!
//! ```
//! use ordered_vec::OrdVec;
//!
//! let mut values: Vec<i32> = Vec::new();
//! values.push_ord_ascending(5);
//! values.push_ord_ascending(3);
//! values.push_ord_ascending(7);
//! values.push_ord_ascending(1);
//!
//! assert_eq!(values, [1, 3, 5, 7]);
//! ```
//! ```
//! use ordered_vec::OrdVec;
//!
//! let mut values: Vec<i32> = Vec::new();
//! values.push_ord_descending(5);
//! values.push_ord_descending(3);
//! values.push_ord_descending(7);
//! values.push_ord_descending(1);
//!
//! assert_eq!(values, [7, 5, 3, 1]);
//! ```

use std::cmp::Ordering;

/// A trait for adding elements to a vector in sorted order, both ascending and descending.
pub trait OrdVec<T: PartialOrd> {
    /// Inserts `item` into `self` in sorted ascending order. Returns the index at which `item` was inserted.
    /// # Examples
    ///
    /// ```
    ///use ordered_vec::OrdVec;
    ///let mut values: Vec<f64> = Vec::new();
    ///assert_eq!(values.push_ord_ascending(5.5), Ok(0));
    ///assert_eq!(values, [5.5]);
    ///
    ///assert_eq!(values.push_ord_ascending(3.14), Ok(0));
    ///assert_eq!(values, [3.14, 5.5]);
    ///
    ///assert_eq!(values.push_ord_ascending(7.77), Ok(2));
    ///assert_eq!(values, [3.14, 5.5, 7.77]);
    ///
    /// ```
    fn push_ord_ascending(&mut self, item: T) -> Result<usize, OrdVecError>;
    /// Inserts `item` into `self` in sorted descending order. Returns the index at which `item` was inserted.
    /// # Examples
    ///
    /// ```
    ///use ordered_vec::OrdVec;
    ///let mut values: Vec<f64> = Vec::new();
    ///assert_eq!(values.push_ord_descending(5.5), Ok(0));
    ///assert_eq!(values, [5.5]);
    ///
    ///assert_eq!(values.push_ord_descending(3.14), Ok(1));
    ///assert_eq!(values, [5.5, 3.14]);
    ///
    ///assert_eq!(values.push_ord_descending(7.77), Ok(0));
    ///assert_eq!(values, [7.77, 5.5, 3.14]);
    ///
    /// ```
    fn push_ord_descending(&mut self, item: T) -> Result<usize, OrdVecError>;
}

impl<T: PartialOrd> OrdVec<T> for Vec<T> {
    fn push_ord_ascending(&mut self, item: T) -> Result<usize, OrdVecError> {
        let idx = binary_search_index_ascending(&item, self);
        match idx {
            Some(idx) => {
                self.insert(idx, item);
                Ok(idx)
            }
            None => Err(OrdVecError),
        }
    }

    fn push_ord_descending(&mut self, item: T) -> Result<usize, OrdVecError> {
        let idx = binary_search_index_descending(&item, self);
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
fn binary_search_index_ascending<T: PartialOrd>(item: &T, values: &[T]) -> Option<usize> {
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

#[inline]
fn binary_search_index_descending<T: PartialOrd>(item: &T, values: &[T]) -> Option<usize> {
    if values.is_empty() {
        return Some(0);
    }
    let mut start: usize = 0;
    let mut end = values.len() - 1;
    if item >= &values[start] {
        return Some(start);
    }

    if item <= &values[end] {
        return Some(end + 1);
    }
    let mut idx: usize = mid_point(start, end);
    loop {
        match item.partial_cmp(&values[idx]) {
            Some(Ordering::Less) => {
                start = idx;
                idx = mid_point(start, end);
                if end - start <= 1 {
                    idx = end;
                    break;
                }
            }
            Some(Ordering::Greater) => {
                end = idx;
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
    fn test_binary_search_index_ascending() {
        let values_1 = [1, 2, 3, 4, 5];
        assert_eq!(binary_search_index_ascending(&4, &values_1), Some(3));
        assert_eq!(binary_search_index_ascending(&6, &values_1), Some(5));
        assert_eq!(binary_search_index_ascending(&0, &values_1), Some(0));

        let values_2 = ["apple", "banana", "cherry", "date", "elderberry"];
        assert_eq!(binary_search_index_ascending(&"cherry", &values_2), Some(2));
        assert_eq!(binary_search_index_ascending(&"fig", &values_2), Some(5));
        assert_eq!(
            binary_search_index_ascending(&"apricot", &values_2),
            Some(1)
        );
        assert_eq!(
            binary_search_index_ascending(&"aardvark", &values_2),
            Some(0)
        );

        let values_3 = [1.5, 2.7, 3.1, 4.9, 5.2];
        assert_eq!(binary_search_index_ascending(&2.7, &values_3), Some(1));
        assert_eq!(binary_search_index_ascending(&3.15, &values_3), Some(3));
        assert_eq!(binary_search_index_ascending(&1.0, &values_3), Some(0));

        let values_4 = [1, 2, 3, 4];
        assert_eq!(binary_search_index_ascending(&5, &values_4), Some(4));
        assert_eq!(binary_search_index_ascending(&0, &values_4), Some(0));
    }

    #[test]
    fn test_binary_search_index_descending() {
        let values = [4, 3, 2, 1];
        assert_eq!(binary_search_index_descending(&1, &values), Some(4));
        assert_eq!(binary_search_index_descending(&2, &values), Some(2));
        assert_eq!(binary_search_index_descending(&3, &values), Some(1));
        assert_eq!(binary_search_index_descending(&4, &values), Some(0));
        assert_eq!(binary_search_index_descending(&5, &values), Some(0));
        assert_eq!(binary_search_index_descending(&0, &values), Some(4));
        let values = [4., 3., 2., 1.];
        assert_eq!(binary_search_index_descending(&2.5, &values), Some(2));

        let values = [1];
        assert_eq!(binary_search_index_descending(&1, &values), Some(0));
        assert_eq!(binary_search_index_descending(&0, &values), Some(1));
        assert_eq!(binary_search_index_descending(&2, &values), Some(0));

        let values = [];
        assert_eq!(binary_search_index_descending(&1, &values), Some(0));
        assert_eq!(binary_search_index_descending(&0, &values), Some(0));
        assert_eq!(binary_search_index_descending(&2, &values), Some(0));

        let values = ["elderberry", "date", "cherry", "banana", "apple"];
        assert_eq!(binary_search_index_descending(&"cherry", &values), Some(2));
        assert_eq!(binary_search_index_descending(&"fig", &values), Some(0));
        assert_eq!(binary_search_index_descending(&"apricot", &values), Some(4));
        assert_eq!(
            binary_search_index_descending(&"aardvark", &values),
            Some(5)
        );
        assert_eq!(
            binary_search_index_descending(&"elderberry", &values),
            Some(0)
        );
        assert_eq!(binary_search_index_descending(&"apple", &values), Some(5));

        let values_2 = [5.2, 4.9, 3.1, 2.7, 1.5];
        assert_eq!(binary_search_index_descending(&2.7, &values_2), Some(3));
        assert_eq!(binary_search_index_descending(&3.15, &values_2), Some(2));
        assert_eq!(binary_search_index_descending(&1.0, &values_2), Some(5));
    }

    #[test]
    fn test_ord_vec_ascending() {
        let mut values: Vec<i32> = Vec::new();

        assert_eq!(values.push_ord_ascending(5), Ok(0));
        assert_eq!(values, [5]);

        assert_eq!(values.push_ord_ascending(3), Ok(0));
        assert_eq!(values, [3, 5]);

        assert_eq!(values.push_ord_ascending(7), Ok(2));
        assert_eq!(values, [3, 5, 7]);

        assert_eq!(values.push_ord_ascending(1), Ok(0));
        assert_eq!(values, [1, 3, 5, 7]);

        assert_eq!(values.push_ord_ascending(9), Ok(4));
        assert_eq!(values, [1, 3, 5, 7, 9]);

        assert_eq!(values.push_ord_ascending(8), Ok(4));
        assert_eq!(values, [1, 3, 5, 7, 8, 9]);

        assert_eq!(values.push_ord_ascending(100), Ok(6));
        assert_eq!(values, [1, 3, 5, 7, 8, 9, 100]);

        assert_eq!(values.push_ord_ascending(3), Ok(1));
        assert_eq!(values, [1, 3, 3, 5, 7, 8, 9, 100]);

        assert_eq!(values.push_ord_ascending(2), Ok(1));
        assert_eq!(values, [1, 2, 3, 3, 5, 7, 8, 9, 100]);

        assert_eq!(values.push_ord_ascending(0), Ok(0));
        assert_eq!(values, [0, 1, 2, 3, 3, 5, 7, 8, 9, 100]);
    }

    #[test]
    fn test_ord_vec_descending() {
        let mut values: Vec<i32> = Vec::new();

        assert_eq!(values.push_ord_descending(5), Ok(0));
        assert_eq!(values, [5]);

        assert_eq!(values.push_ord_descending(3), Ok(1));
        assert_eq!(values, [5, 3]);

        assert_eq!(values.push_ord_descending(7), Ok(0));
        assert_eq!(values, [7, 5, 3]);

        assert_eq!(values.push_ord_descending(1), Ok(3));
        assert_eq!(values, [7, 5, 3, 1]);

        assert_eq!(values.push_ord_descending(9), Ok(0));
        assert_eq!(values, [9, 7, 5, 3, 1]);

        assert_eq!(values.push_ord_descending(8), Ok(1));
        assert_eq!(values, [9, 8, 7, 5, 3, 1]);

        assert_eq!(values.push_ord_descending(100), Ok(0));
        assert_eq!(values, [100, 9, 8, 7, 5, 3, 1]);

        assert_eq!(values.push_ord_descending(3), Ok(5));
        assert_eq!(values, [100, 9, 8, 7, 5, 3, 3, 1]);

        assert_eq!(values.push_ord_descending(2), Ok(7));
        assert_eq!(values, [100, 9, 8, 7, 5, 3, 3, 2, 1]);

        assert_eq!(values.push_ord_descending(0), Ok(9));
        assert_eq!(values, [100, 9, 8, 7, 5, 3, 3, 2, 1, 0]);
    }
}
