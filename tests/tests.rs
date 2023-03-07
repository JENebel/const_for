#![feature(const_trait_impl)]
use const_for::*;

#[test]
const fn simple_for() {
    let mut a = 0;

    const_for!(i; 0..5 => {
        a += i;
    });
    
    assert!(a == 10);
}

#[test]
const fn simple_break() {
    let mut a = 0;

    const_for!(i; 0..=10 => {
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

#[test]
const fn simple_rev_for() {
    let mut a = 0;

    const_for!(i; (0..10).rev_const() => {
        a += 1;
    });
    
    assert!(a == 10);
}

#[test]
const fn simple_rev_for_bad() {
    let mut a = 0;

    const_for!(i; 10..0 => {
        a += 1;
    });
    
    assert!(a == 11);
}


#[test]
fn simdaple_rev_for_bad() {
    let b = 0..=10;

    b.start();
}


