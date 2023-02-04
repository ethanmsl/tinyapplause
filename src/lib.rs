//! # Tiny Applause
//! This crate is just to trial using the CLAP crate (for creating CLI Apps)
//!
//! ## Other goals:
//! - test docstring formatting
//! - test doctests
//! - test proper structuring
//! - test multi-target binary builds
//! - test graphical output options
//!

/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`.
/// Note: this requires referencing our cratename (`tinyapplause`)
///
/// ```
/// let result = tinyapplause::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Usually doc comments may include sections "Examples", "Panics" and "Failures".
///
/// The next function divides two numbers.
///
/// # Examples
///
/// ```
/// let result = tinyapplause::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// tinyapplause::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
