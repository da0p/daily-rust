//! # art
//!
//! `art` is a collection of utilities to make performing certain
//! calculations more convenient.
//!
/// Adds one to the number given
///
/// # Examples
/// ```
/// let arg = 5;
/// let answer = art::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        println!(
            "Mixing colors {:#?} + {:#?} = {:#?}!",
            PrimaryColor::Red,
            PrimaryColor::Yellow,
            SecondaryColor::Green
        );
        SecondaryColor::Green
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_one_to_two() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
