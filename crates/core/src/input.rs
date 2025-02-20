#![allow(clippy::redundant_pub_crate)]
#![allow(unused)]

use std::cell::RefCell;

#[derive(Copy, Clone)]
pub enum Part {
    One,
    Two,
}

pub struct Input<'a> {
    pub part: Part,
    pub text: &'a str,
    #[cfg(feature = "visualization")]
    pub rendered_svg: RefCell<String>,
}

impl<'a> Input<'a> {
    pub const fn is_part_one(&self) -> bool {
        matches!(self.part, Part::One)
    }

    pub const fn is_part_two(&self) -> bool {
        matches!(self.part, Part::Two)
    }

    pub fn part_values<T>(&self, if_part_one: T, if_part_two: T) -> T {
        // See https://github.com/rust-lang/rust/issues/66753 for missing_const_for_fn.
        #![allow(clippy::missing_const_for_fn)]
        match self.part {
            Part::One => if_part_one,
            Part::Two => if_part_two,
        }
    }

    #[cfg(test)]
    #[allow(clippy::missing_const_for_fn)]
    pub fn part_one(text: &'a str) -> Self {
        Self {
            part: Part::One,
            text,
            #[cfg(feature = "visualization")]
            rendered_svg: RefCell::new("".to_string()),
        }
    }

    #[cfg(test)]
    #[allow(clippy::missing_const_for_fn)]
    pub fn part_two(text: &'a str) -> Self {
        Self {
            part: Part::Two,
            text,
            #[cfg(feature = "visualization")]
            rendered_svg: RefCell::new("".to_string()),
        }
    }
}

#[cfg(test)]
macro_rules! test_part_one {
    ($input:tt => $expected:expr) => {
        assert_eq!(solve(&Input::part_one($input)), Ok($expected));
    };
}
#[cfg(test)]
pub(crate) use test_part_one;

#[cfg(test)]
macro_rules! test_part_two {
    ($input:tt => $expected:expr) => {
        assert_eq!(solve(&Input::part_two($input)), Ok($expected));
    };
}
#[cfg(test)]
pub(crate) use test_part_two;

#[cfg(test)]
macro_rules! test_part_one_error {
    ($input:tt => $expected:expr) => {
        assert_eq!(Err($expected.into()), solve(&Input::part_one($input)));
    };
}
#[cfg(test)]
pub(crate) use test_part_one_error;

#[cfg(test)]
macro_rules! test_part_two_error {
    ($input:tt => $expected:expr) => {
        assert_eq!(Err($expected.into()), solve(&Input::part_two($input)));
    };
}
#[cfg(test)]
pub(crate) use test_part_two_error;
