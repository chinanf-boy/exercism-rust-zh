#[test]
fn square_one() {
    assert_eq!(square(1), 1);
}

#[test]
//#[ignore]
fn square_two() {
    assert_eq!(square(2), 2);
}

#[test]
//#[ignore]
fn square_three() {
    assert_eq!(square(3), 4);
}

#[test]
//#[ignore]
fn square_four() {
    assert_eq!(square(4), 8);
}

#[test]
//#[ignore]
fn square_sixteen() {
    assert_eq!(square(16), 32_768);
}

#[test]
//#[ignore]
fn square_thirty_two() {
    assert_eq!(square(32), 2_147_483_648);
}

#[test]
//#[ignore]
fn square_sixty_four() {
    assert_eq!(square(64), 9_223_372_036_854_775_808);
}

#[test]
//#[ignore]
#[should_panic(expected = "Square must be between 1 and 64")]
fn square_zero_panics() {
    square(0);
}

#[test]
//#[ignore]
#[should_panic(expected = "Square must be between 1 and 64")]
fn square_sixty_five_panics() {
    square(65);
}

#[test]
//#[ignore]
fn total_sums_all_squares() {
    assert_eq!(total(), 18_446_744_073_709_551_615);
}
