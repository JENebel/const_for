# const_for

This crate provides a macro for a for loop that is usable in const context.

Usage is pretty simple:

    const_for!(var; range => {
        // Body
    });

Here is a simple example:

    const fn simple_for() {
        let mut a = 0;

        const_for!(i; 0..5 => {
            a += i;
        });
        
        assert!(a == 10);
    }

## Reverse for loop

As reversing of a range is not possible in const as it uses an iterator, this crate provides a rev_const() function for ranges, that allows a similar syntax to reversing, but is usable in const contexts. To use this const range reversing, the const_trait_impl feature is needed.

    #![feature(const_trait_impl)]

    const_for!(i; (0..5).rev_const() => {
        // Body
    });

This is equivalent to this regular for loop:

    for i in (0..10).rev() {
        // Body 
    }

This is however not necessarily needed, as the following gives the same result. But this syntax is far away from the regular rust syntax, and is discouraged.

    const_for!(i; 10..0 => {
        // Body
    });
