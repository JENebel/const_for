#![feature(const_trait_impl)]

use std::ops::Range;

#[macro_export]
macro_rules! const_for {
    ($var: ident; $range: expr => $body: expr) => {
        if $range.start < $range.end {
            let mut $var = $range.start - 1;
            loop {
                $var += 1;
                if $var >= $range.end {
                    break
                }

                $body
            }
        } else {
            let mut $var = $range.start;
            loop {
                $var -= 1;
                if $var < $range.end {
                    break
                }

                $body
            }
        }
    };
}

struct ConstRange<T> {
    pub start: T,
    pub end: T,
}

#[const_trait]
trait ConstRangetrait<T> {
    fn rev_const(&self) -> ConstRange<T>;
}

impl<T: Copy> const ConstRangetrait<T> for Range<T> {
    fn rev_const(&self) -> ConstRange<T> {
        ConstRange { start: self.end, end: self.start }
    }
}

impl<T: Copy> const ConstRangetrait<T> for ConstRange<T> {
    fn rev_const(&self) -> ConstRange<T> {
        ConstRange { start: self.end, end: self.start }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    const fn simple_for() {
        let mut a = 0;

        const_for!(i; (0..10) => {
            a += 1;
        });
        
        assert!(a == 10);
    }

    #[test]
    const fn simple_break() {
        let mut a = 0;

        const_for!(i; 0..10 => {
            if i == 5 {
                break
            }

            a += 1;
        });

        assert!(a == 5)
    }

    #[test]
    const fn simple_continue() {
        let mut a = 0;

        const_for!(i; 0..10 => {
            if i % 2 == 0 {
                continue
            }

            a += 1;
        });

        assert!(a == 5)
    }
}
