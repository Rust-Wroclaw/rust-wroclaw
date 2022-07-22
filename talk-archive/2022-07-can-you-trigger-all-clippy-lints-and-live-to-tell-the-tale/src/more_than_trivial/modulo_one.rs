pub fn calculate(a: i32, b: i32, o: i32) -> i32 {
    let x = a + b;
    let y = b + o;
    let xper = x % 1; // always 0
    let u = y % (0 - 1); // will either panic/overflow or be 0

    xper * u
}

#[cfg(does_not_compile)]
#[test]
fn modulo_minus_one_panic() {
    assert_eq!(i32::MIN % -1, 0);
}
