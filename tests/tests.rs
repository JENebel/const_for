use const_for::*;

#[test]
const fn simple_for() {
    let mut a = 0;
    cfor_rev!(i; 0..5 => a += i);
    assert!(a == 10)
}

#[test]
const fn bracketed_for() {
    let mut a = 0;
    cfor!(i; 0..5 => {
        a += i
    });
    assert!(a == 10);
}

#[test]
const fn simple_for_rev() {
    let mut a = 0;
    cfor_rev!(i; 0..5 => {
        a += i;
    });
    assert!(a == 10);
}