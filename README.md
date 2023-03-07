# const_for

This crate provides ergonomic for loops in const contexts using macros.

Regular for loops are not allowed in const contexts, because it relies on iterators, which are not available in const.\
This is rather annoying when writing const functions, as you need to write custom for loops using 'loop' or 'while'.

This crate provides a macro implementation of for loops that is usable in const contexts.\
An additional bonus is that these macros handle break and continue as expected.\
The only real difference to the regular for loop is that the iterator variable can be mutated within the loop. This is discouraged though.

Two macros are provided. A regular for loop and a reversed variant:

[cfor]\
[cfor_rev]

A reversed variant exists because ranges cannot be reversed in const contexts, as this produces an iterator.\
Sometimes this is needed though, and with the reversed variant this is possible.

The main restriction of this crate is that the macros only accept a standard(non-inclusive) Range. This means that it is only suitable where otherwise simple for loops would be used.

## Normal for loop

    cfor!(id; range => {
        // Body
    });

This is equivalent to this regular for loop:

    for id in range {
        // Body
    }

Example:

    let mut a = 0;
    cfor!(i; 0..5 => {
        a += i
    });
    assert!(a == 10)

If the body is a single statement, the curly braces are not needed, and the loop from the previous example can thus be written as

    cfor!(i; 0..10 => a += i);

## Reverse for loop

[cfor_rev] is very similar to [cfor], except backwards:\

    cfor_rev!(i; 0..5 => {
        // Body
    });

This is equivalent to this regular for loop:

    for i in (0..10).rev() {
        // Body 
    }
