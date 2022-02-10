use learn_utils::math::{add, subtract};

#[test]
fn math_adds_5_and_5_to_equal_10() {
    let a: i8 = 5;
    let b: i8 = 5;
    let c: i8 = add(a, b);
    assert_eq!(c, 10);
}

#[test]
fn math_subtracts_5_and_5_to_equal_0() {
    let a: i8 = 5;
    let b: i8 = 5;
    let c: i8 = subtract(a, b);
    assert_eq!(c, 0);
}
