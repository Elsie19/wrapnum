//! Ever wanted to just make a number and have it automatically wrap around an
//! arbitrary minimum/maximum without ever thinking about it after creating the number?
//! Here you go.
//!
//! The goal of this is to act as much like any other integer type available, so that you can just
//! think of this as a number and leave all the wrapping to us!

use std::{
    fmt::Display,
    ops::{Add, AddAssign, Index, IndexMut, Rem, Sub, SubAssign},
};

use num::{zero, ToPrimitive};

#[derive(PartialEq, Clone, Copy, Debug)]
/// Number with arbitrary wrapping.
pub struct WrapNum<T> {
    /// Current value.
    pub value: T,
    /// Minimum value.
    pub min: T,
    /// Maximimum value.
    pub max: T,
}

impl<T> Display for WrapNum<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T, U> Index<WrapNum<U>> for Vec<T>
where
    U: ToPrimitive + Copy,
{
    type Output = T;

    fn index(&self, index: WrapNum<U>) -> &Self::Output {
        let idx = index
            .value
            .to_usize()
            .expect("Failed to convert index to usize");
        &self[idx]
    }
}

impl<T, U> IndexMut<WrapNum<U>> for Vec<T>
where
    U: ToPrimitive + Copy,
{
    fn index_mut(&mut self, index: WrapNum<U>) -> &mut Self::Output {
        &mut self[index
            .value
            .to_usize()
            .expect("Failed to convert index to usize")]
    }
}

impl<T> Add for WrapNum<T>
where
    T: Add<Output = T> + Sub<Output = T> + Ord + num::Bounded + Rem<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let range = self.max - self.min;
        let wrapped_value = (self.value + rhs.value - self.min) % range + self.min;

        Self {
            value: wrapped_value,
            min: self.min,
            max: self.max,
        }
    }
}

impl<T> Sub for WrapNum<T>
where
    T: Sub<Output = T> + Add<Output = T> + Rem<Output = T> + Ord + num::Bounded + num::One + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let range = self.max - self.min;

        let result = if self.value < rhs.value {
            self.max - self.min + (self.value - rhs.value)
        } else {
            self.value - rhs.value
        };

        let wrapped_value = (result - self.min) % range + self.min;

        Self {
            value: wrapped_value,
            min: self.min,
            max: self.max,
        }
    }
}

impl<T> AddAssign<T> for WrapNum<T>
where
    T: Add<Output = T> + Sub<Output = T> + Rem<Output = T> + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        let range = self.max - self.min;

        let result = self.value + rhs;

        *self = Self {
            value: (result - self.min) % range + self.min,
            min: self.min,
            max: self.max,
        }
    }
}

impl<T> SubAssign<T> for WrapNum<T>
where
    T: Sub<Output = T> + Add<Output = T> + Rem<Output = T> + PartialOrd + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        let range = self.max - self.min;

        let result = if self.value < rhs {
            self.max - self.min + (self.value - rhs)
        } else {
            self.value - rhs
        };

        *self = Self {
            value: (result - self.min) % range + self.min,
            min: self.min,
            max: self.max,
        };
    }
}

impl<T> From<T> for WrapNum<T>
where
    T: Copy + num::Bounded + num::Zero,
{
    fn from(value: T) -> Self {
        Self {
            value,
            min: zero(),
            max: T::max_value(),
        }
    }
}

impl<T> Default for WrapNum<T>
where
    T: num::Bounded + num::Zero,
{
    /// The default behavior is to set [`WrapNum::value`] and [`WrapNum::min`] to [`zero()`] and
    /// [`WrapNum::max`] to [`num::Bounded::max_value()`].
    fn default() -> Self {
        WrapNum {
            value: zero(),
            min: zero(),
            max: T::max_value(),
        }
    }
}

impl<T> WrapNum<T>
where
    T: num::Bounded + num::Zero + PartialOrd,
{
    /// Create new wrapped number and automatic zeroed [`WrapNum::value`].
    pub fn new(max: T) -> Self {
        Self {
            max,
            ..Default::default()
        }
    }

    /// Create new wrapped number with given max.
    ///
    /// # Panics
    /// This will panic if `value > max`.
    pub fn new_max(value: T, max: T) -> Self {
        assert!(!(value > max), "`value` is greater than `max`.");
        Self {
            value,
            max,
            ..Default::default()
        }
    }

    /// Create new wrapped number with given min/max.
    ///
    /// # Panics
    /// This will panic if `value > max` or `value < min`.
    pub fn new_min_max(value: T, min: T, max: T) -> Self {
        if value > max {
            panic!("`value` is greater than `max`.");
        } else if value < min {
            panic!("`value` is less than `min`.");
        }
        Self { value, min, max }
    }
}

#[macro_export]
/// Create [`WrapNum`] with value, minimum and maximum.
///
/// # Running
/// 1. With one value passed, which will set that as the maximum and default everything else to
///    `0`.
/// 2. With one inclusive value passed (`=5`) and defaults everything else as above.
/// 3. With a value and a maximum.
/// 4. With a value, a minimum, and a maximum.
/// 5. With a range passed (`5..30`).
/// 6. With an inclusive range passed (`5..=30`).
macro_rules! wrap {
    ($max:expr) => {
        $crate::WrapNum::new($max)
    };
    (=$max:expr) => {
        $crate::WrapNum::new($max + 1)
    };
    ($v:expr, $max:expr) => {
        $crate::WrapNum::new_max($v, $max)
    };
    (($min:expr)..($max:expr)) => {
        $crate::WrapNum::new_min_max($min, $min, $max)
    };
    (($min:expr)..=($max:expr)) => {
        $crate::WrapNum::new_min_max($min, $min, $max + 1)
    };
    ($v:expr, $min:expr, $max:expr) => {
        $crate::WrapNum::new_min_max($v, $min, $max)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_usize() {
        let mut result = wrap!(50);
        for _ in 0..60 {
            result += 1;
            println!("{}", result);
        }
    }

    #[test]
    fn months() {
        let mut months = wrap!(11);
        for _ in 0..11 {
            months += 1;
        }
        assert_eq!(months.value, 0);
    }

    #[test]
    fn custom_min() {
        let mut mins = wrap!(5, 5, 7);
        mins += 3;
        // 5+1 = 6
        // 6+1 = 7 (wrapped) = 5
        // 5+1 = 6
        assert_eq!(mins.value, 6);
    }

    #[test]
    fn can_convert() {
        let mut mins = wrap!(=5);
        mins += 5 as u16;
    }

    #[test]
    fn has_indexing() {
        let here = wrap!(5);
        let oh = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(oh[here], 10);
    }
}
