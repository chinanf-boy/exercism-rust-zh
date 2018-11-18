macro_rules! tests {
    ($property_test_func:ident {
        $( $(#[$attr:meta])* $test_name:ident( $( $param:expr ),* ); )+
    }) => {
        $(
            $(#[$attr])*
            #[test]
            fn $test_name() {
                $property_test_func($( $param ),* )
            }
        )+
    }
}

fn test_classification(num: u64, result: Classification) {
    assert_eq!(classify(num), Some(result));
}

#[test]
fn basic() {
    assert_eq!(classify(0), None);
}

tests! {
    test_classification {
        test_1(1, Classification::Deficient);
        test_2(2, Classification::Deficient);
        test_4(4, Classification::Deficient);
        test_6(6, Classification::Perfect);
        test_12(12, Classification::Abundant);
        test_28(28, Classification::Perfect);
        test_30(30, Classification::Abundant);
        test_32(32, Classification::Deficient);
        test_33550335(33550335, Classification::Abundant);
        test_33550336(33550336, Classification::Perfect);
        test_33550337(33550337, Classification::Deficient);
    }
}
