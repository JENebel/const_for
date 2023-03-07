#![feature(const_trait_impl)]
use std::ops::{Range, RangeInclusive};

#[macro_export]
/// ## Macro for a for loop usable in const context
/// 
///     const_for!(i; 0..10 => {
///        // Body here
///     });
/// 
/// is roughly equivalent to 
/// 
///     for i in 0..10 {
///         // Body here
///     }
/// 
/// But is usable in const context.
/// 
/// Break and continue work as expected too.
macro_rules! const_for {
    ($var: ident; $range: expr => $body: expr) => {
        let (start, end) = $range.bounds();
        if start < end {
            let mut $var = start - 1;
            loop {
                $var += 1;
                if $var >= end {
                    break
                }

                $body
            }
        } else {
            let mut $var = start;
            loop {
                $var -= 1;
                if $var < end {
                    break
                }

                $body
            }
        }
    };
}

#[const_trait]
pub trait ConstRangetrait<T> {
    fn rev_const(&self) -> Range<T>;
    fn bounds(&self) -> (T, T);
}

impl<T: Copy> const ConstRangetrait<T> for Range<T> {
    fn rev_const(&self) -> Range<T> {
        Range { start: self.end, end: self.start }
    }

    fn bounds(&self) -> (T, T) {
        (self.start, self.end)
    }
}

impl<T: Copy> const ConstRangetrait<T> for RangeInclusive<T> {
    fn rev_const(&self) -> Range<T> {
        Range { start: *self.end(), end: *self.start() }
    }

    fn bounds(&self) -> (T, T) {
        (*self.start(), *self.end())
    }
}