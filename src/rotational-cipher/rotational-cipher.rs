#[test]
fn rotate_a_1() {
    assert_eq!("b", rotate("a", 1));
}

#[test]
//#[ignore]
fn rotate_a_26() {
    assert_eq!("a", rotate("a", 26));
}

#[test]
//#[ignore]
fn rotate_a_0() {
    assert_eq!("a", rotate("a", 0));
}

#[test]
//#[ignore]
fn rotate_m_13() {
    assert_eq!("z", rotate("m", 13));
}

#[test]
//#[ignore]
fn rotate_n_13_with_wrap() {
    assert_eq!("a", rotate("n", 13));
}

#[test]
//#[ignore]
fn rotate_caps() {
    assert_eq!("TRL", rotate("OMG", 5));
}

#[test]
//#[ignore]
fn rotate_spaces() {
    assert_eq!("T R L", rotate("O M G", 5));
}

#[test]
//#[ignore]
fn rotate_numbers() {
    assert_eq!("Xiwxmrk 1 2 3 xiwxmrk", rotate("Testing 1 2 3 testing", 4));
}

#[test]
//#[ignore]
fn rotate_punctuation() {
    assert_eq!("Gzo\'n zvo, Bmviyhv!", rotate("Let\'s eat, Grandma!", 21));
}

#[test]
//#[ignore]
fn rotate_all_the_letters() {
    assert_eq!(
        "Gur dhvpx oebja sbk whzcf bire gur ynml qbt.",
        rotate("The quick brown fox jumps over the lazy dog.", 13)
    );
}
