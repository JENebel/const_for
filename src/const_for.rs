/// A for loop that is usable in const contexts.
/// 
/// See the [crate-level documentation](crate) for more information. 
#[macro_export]
macro_rules! const_for {
    ($var:pat_param in ($range:expr).step_by($step:expr) => $body:stmt) => {
        {
            let _: usize = $step;
            let mut __ite = $range.start;
            let __end = $range.end;
            let mut __is_first = true;
            let __step = $step;

            loop {
                if !__is_first {
                    __ite += __step
                }
                __is_first = false;

                let $var = __ite;

                if __ite >= __end {
                    break
                }

                $body
            }
        }
    };

    ($var:pat_param in ($range:expr).rev().step_by($step:expr) => $body:stmt) => {
        {
            let _: usize = $step;
            let mut __ite = $range.end;
            let __start = $range.start;
            let mut __is_first = true;
            let __step = $step;

            loop {
                if !__is_first {
                    if __step + __start >= __ite {
                        break
                    }
                    __ite -= __step
                }
                __is_first = false;

                if __ite <= __start {
                    break
                }

                // cannot underflow as __ite > __start
                let $var = __ite - 1;

                $body
            }
        }
    };

    ($var:pat_param in ($range:expr).rev() => $body:stmt) => {
        const_for!($var in ($range).rev().step_by(1) => $body)
    };

    ($var:pat_param in ($range:expr).step_by($step:expr).rev() => $body:stmt) => {
        const_for!($var in ($range.start..$range.end - ($range.end - $range.start - 1) % $step).rev().step_by($step) => $body)
    };

    ($var:pat_param in $range:expr => $body:stmt) => {
        const_for!($var in ($range).step_by(1) => $body)
    };
}
