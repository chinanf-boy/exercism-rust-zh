#[test]
fn empty() {
    assert_eq!(abbreviate(""), "");
}

#[test]
//#[ignore]
fn basic() {
    assert_eq!(abbreviate("Portable Network Graphics"), "PNG");
}

#[test]
//#[ignore]
fn lowercase_words() {
    assert_eq!(abbreviate("Ruby on Rails"), "ROR");
}

#[test]
//#[ignore]
fn camelcase() {
    assert_eq!(abbreviate("HyperText Markup Language"), "HTML");
}

#[test]
//#[ignore]
fn punctuation() {
    assert_eq!(abbreviate("First In, First Out"), "FIFO");
}

#[test]
//#[ignore]
fn all_caps_words() {
    assert_eq!(abbreviate("PHP: Hypertext Preprocessor"), "PHP");
}

#[test]
//#[ignore]
fn non_acronym_all_caps_word() {
    assert_eq!(abbreviate("GNU Image Manipulation Program"), "GIMP");
}

#[test]
//#[ignore]
fn hyphenated() {
    assert_eq!(
        abbreviate("Complementary metal-oxide semiconductor"),
        "CMOS"
    );
}
