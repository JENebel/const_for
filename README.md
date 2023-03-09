# Const for loops

[![GitHub](https://img.shields.io/badge/GitHub-black?logo=github)](https://github.com/JENebel/const_for)
[![crates.io](https://img.shields.io/crates/v/const_for?logo=rust&logoColor=b7410e)](http://crates.io/crates/const_for)
[![Docs](https://img.shields.io/docsrs/const_for/latest?logo=Docs.rs)](https://docs.rs/const_for/latest)

- [Const for loops](#const-for-loops)
  - [Introduction](#introduction)
  - [Normal for loop](#normal-for-loop)
  - [Reverse for loop](#reverse-for-loop)
  - [Example](#example)

## Introduction

This crate provides a macro for making ergonomic for loops in const.

Regular for loops are not allowed in const contexts, because it relies on iterators, which are not available in const.\
This is rather annoying when writing const functions, as you need to write custom for loops using 'loop' or 'while'.

This crate provides a macro implementation of for loops that is usable in const contexts.\
An additional bonus is that these macros handle break and continue as expected.\
The only real difference to the regular for loop is that the iterator variable can be mutated within the loop. This is discouraged though.

Two macros are provided. A regular for loop and a reversed variant.\
A reversed variant exists because ranges cannot be reversed in const contexts, as this produces an iterator.\
Sometimes this is needed though, and with the reversed variant this is possible.

The main restriction of this crate is that the macros only support a standard(non-inclusive) Range.

## Normal for loop

    const_for!(id; range => {
        // Body
    });

This is equivalent to this regular for loop:

    for id in range {
        // Body
    }

Example:

    let mut a = 0;
    const_for!(i in 0..5 => {
        a += i
    });
    assert!(a == 10)

If the body is a single statement, the curly braces are not needed, and the loop from the previous example can thus be written as

    const_for!(i in 0..10 => a += i);

## Reverse for loop

The reversed for loop is very similar to the normal one, except backwards:\

    ctfor_rev!(i in 0..5 => {
        // Body
    });

This is equivalent to:

    for i in (0..5).rev() {
        // Body 
    }

## Example

Here is an example of how this crate helped make some actual code much nicer and readable.

The code was taken (and edited a bit for clarity) from the [Cadabra](https://github.com/JENebel/Cadabra/blob/master/prepare_constants.rs) chess engine.

Before:

    const fn gen_white_pawn_attacks() -> [u64; 64] {
        let mut masks = [0; 64];
        
        let mut rank: u8 = 0;
        while rank < 8 {
            let mut file: u8 = 0;
            while file < 8 {
                let index = (rank*8+file) as usize;
                if file != 7 { masks[index] |= (1 << index) >> 7 as u64 }
                if file != 0 { masks[index] |= (1 << index) >> 9 as u64 }

                file += 1;
            }
            rank += 1;
        }

        masks
    }

After:

    const fn gen_white_pawn_attacks() -> [u64; 64] {
        let mut masks = [0; 64];
        
        ctfor!(rank in 0..8 => {
            ctfor!(file in 0..8 => {
                let index = (rank*8+file) as usize;
                if file != 7 { masks[index] |= (1 << index) >> 7 as u64 }
                if file != 0 { masks[index] |= (1 << index) >> 9 as u64 }
            })
        });

        masks
    }
