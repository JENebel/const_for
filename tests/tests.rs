use const_for::*;

macro_rules! validate_loop {
    ($($loop:tt)*) => {
        let mut c_index = 0;
        let mut c_values_hit = [0; 1000];
        const_for!(i in $($loop)* => {
            c_values_hit[c_index] = i;
            c_index += 1;
        });

        let mut r_index = 0;
        let mut r_values_hit = [0; 1000];
        for i in $($loop)* {
            r_values_hit[r_index] = i;
            r_index += 1;
        };

        assert!(c_index == r_index);
        assert!(c_values_hit == r_values_hit);
    };
}

#[allow(unused_parens)]
#[test]
fn normal_for() {
    validate_loop!(0..10);
    validate_loop!(-10..10);
    validate_loop!((0..10));
    validate_loop!(100..10);
}

#[allow(unused_parens)]
#[test]
fn normal_rev() {
    validate_loop!((0..10).rev());
    validate_loop!((-10..10).rev());
    validate_loop!((0..10).rev());
    validate_loop!((100..10).rev());
}

#[allow(unused_parens)]
#[test]
fn custom_step_by() {
    validate_loop!((0..10).step_by(1));
    validate_loop!((-10..10).step_by(2));
    validate_loop!((0..10).step_by(3));
    validate_loop!((0..100).step_by(32));
    validate_loop!((100..10).step_by(4));
}

#[allow(unused_parens)]
#[test]
fn rev_and_custom_step_by() {
    /*validate_loop!((0..10).rev().step_by(1));
    validate_loop!((-10..10).rev().step_by(2));
    validate_loop!((0..10).rev().step_by(3));
    validate_loop!((0..100).rev().step_by(32));
    validate_loop!((100..10).rev().step_by(4));
    validate_loop!((0..10).step_by(1).rev());
    validate_loop!((-10..10).step_by(2).rev());*/

    /*println!("Const");
    const_for!(i in (0..100).rev().step_by(32) => {
        println!("{i}")
    });*/
    const_for!(i in (0..10).step_by(3).rev() => {

    })

    /*validate_loop!((0..10).step_by(3).rev());
    validate_loop!((0..100).step_by(32).rev());
    validate_loop!((100..10).step_by(4).rev());*/
}

#[test]
fn brian() {
    println!("{}", 1.max(3 % 2))
}