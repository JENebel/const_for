use const_for::*;

#[test]
const fn simple_for() {
    let mut a = 0;
    const_for!(i; 0..5 => {
        a += i
    });
    assert!(a == 10);
}

#[test]
const fn unbracketed_for() {
    let mut a = 0;
    const_for!(i; 0..5 => a += i);
    assert!(a == 10)
}

#[test]
const fn works_for_unsigned() {
    let start: u8 = 0;
    let mut a = 0;
    const_for!(i; start..5 => {
        a += i
    });
    assert!(a == 10);
}

#[test]
const fn passing_0_should_work() {
    let mut a = 0;
    const_for!(i; -4..5 => {
        a += i
    });
    assert!(a == 0);
}

#[test]
fn should_give_same_as_normal_for() {
    let mut a: [usize; 10] = [0; 10];
    let mut index = 0;
    const_for!(i; 0..a.len() => {
        a[index] = i;
        index += 1;
    });

    let mut b: [usize; 10] = [0; 10];
    index = 0;
    for i in 0..b.len() {
        b[index] = i;
        index += 1;
    }

    assert!(a == b);
}

#[test]
fn should_give_same_as_normal_for_passing_0() {
    let mut c: [i32; 10] = [0; 10];
    let mut index = 0;
    const_for!(i; -5..(c.len()-5) as i32 => {
        c[index] = i;
        index += 1;
    });

    let mut d: [i32; 10] = [0; 10];
    index = 0;
    for i in -5..(d.len()-5) as i32 {
        d[index] = i;
        index += 1;
    }

    assert!(c == d);
}

#[test]
const fn simple_for_rev() {
    let mut a = 0;
    const_for_rev!(i; 0..5 => {
        a += i;
    });
    assert!(a == 10);
}

#[test]
const fn unbracketed_for_rev() {
    let mut a = 0;
    const_for_rev!(i; 0..5 => a += i);
    assert!(a == 10)
}

#[test]
const fn rev_works_for_unsigned() {
    let start: u8 = 0;
    let end: u8 = 5;
    let mut a = 0;
    const_for!(i; start..end => {
        a += i
    });
    assert!(a == 10);
}

#[test]
const fn rev_passing_0_should_work() {
    let mut a = 0;
    const_for!(i; -4..5 => {
        a += i
    });
    assert!(a == 0);
}

#[test]
fn rev_should_give_same_as_normal_for() {
    let mut a: [usize; 10] = [0; 10];
    let mut index = 0;
    const_for_rev!(i; 0..a.len() => {
        a[index] = i;
        index += 1;
    });

    let mut b: [usize; 10] = [0; 10];
    index = 0;
    for i in (0..b.len()).rev() {
        b[index] = i;
        index += 1;
    }

    assert!(a == b);
}

#[test]
fn rev_should_give_same_as_normal_for_passing_0() {
    let mut c: [i32; 10] = [0; 10];
    let mut index = 0;
    const_for_rev!(i; -5..(c.len()-5) as i32 => {
        c[index] = i;
        index += 1;
    });

    let mut d: [i32; 10] = [0; 10];
    index = 0;
    for i in (-5..(d.len()-5) as i32).rev() {
        d[index] = i;
        index += 1;
    }

    assert!(c == d);
}

#[test]
fn real_life_example() {
    assert!(gen_white_pawn_attacks_while() == gen_white_pawn_attacks_for());
}

const fn gen_white_pawn_attacks_while() -> [u64; 64] {
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

const fn gen_white_pawn_attacks_for() -> [u64; 64] {
    let mut masks = [0; 64];
    
    const_for!(rank; 0..8 => {
        const_for!(file; 0..8 => {
            let index = (rank*8+file) as usize;
            if file != 7 { masks[index] |= (1 << index) >> 7 as u64 }
            if file != 0 { masks[index] |= (1 << index) >> 9 as u64 }
        })
    });

    masks
}