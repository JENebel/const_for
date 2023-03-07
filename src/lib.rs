#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

/// A for loop that is usable in const contexts
/// 
/// Provides a for loop over a range that can be used in a const contexts.\
/// Break and continue both work just like in a regular for loop.
/// 
/// The only functional difference between this and a regular for loop,
/// is that this one allows direct mutation of the iteration variable. This is however discouraged.
/// 
/// # Examples
/// ```
/// # use const_for::*;
/// let mut a = 0;
/// cfor!(i; 0..5 => {
///     a += i
/// });
/// assert!(a == 10)
/// ```
/// 
/// This is equivalent to the following regular for loop, but is usable in const context.
/// ```
/// let mut a = 0;
/// for i in 0..5 {
///     a += i
/// }
/// assert!(a == 10)
/// ```
/// 
/// If the body is just a single statement, the curly braces are not needed.
/// ```
/// # use const_for::*;
/// let mut a = 0;
/// cfor!(i; 0..5 => a += i);
/// assert!(a == 10)
/// ```
#[macro_export]
macro_rules! cfor {
    ($var: ident; $range: expr => $body: expr) => {
        {
            let mut $var = $range.start;
            let mut first_ite = true;

            loop {
                if !first_ite {
                    $var += 1
                }
                first_ite = false;

                if $var >= $range.end {
                    break
                }

                $body
            }
        }
    };
}

/// A reversed for loop that is usable in const contexts
/// 
/// Similar to [cfor], but iterates in reversed order.
/// 
/// # Examples
/// ```
/// # use const_for::*;
/// cfor_rev!(i; 0..3 => {
///     // ite. 1: i = 2
///     // ite. 2: i = 1
///     // ite. 3: i = 0
/// });
/// ```
/// 
/// This is equivalent to the following regular for loop, but is usable in const context.
/// ```
/// for i in (0..3).rev() {
///     // body
/// }
/// ```
#[macro_export]
macro_rules! cfor_rev {
    ($var: ident; $range: expr => $body: expr) => {
        {
            let mut $var = $range.end - 1;
            let mut first_ite = true;

            loop {
                if !first_ite {
                    $var -= 1;
                }
                first_ite = false;

                $body;

                if $var <= $range.start {
                    break
                }
            }
        }
    };
}