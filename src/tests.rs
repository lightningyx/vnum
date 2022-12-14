use crate::value_enum;

#[test]
fn correct_values() {
    {
        value_enum! {
            enum Fruit: &'static str {
                Apple = "red",
                Banana = "yellow",
                Blueberry = "blue"
            }
        }
        let apple: Fruit = Fruit::Apple;
        assert_eq!(apple.value(), "red");

        let banana: Fruit = Fruit::Banana;
        assert_eq!(banana.value(), "yellow");

        let blueberry: Fruit = Fruit::Blueberry;
        assert_eq!(blueberry.value(), "blue");
    }
    // Check for each of the test enums with i16 type
    // They only differ in visibility and docs
    for_each!(
        {
            let apple: Fruit = Fruit::Apple;
            assert_eq!(apple.value(), -42_i16);

            let banana: Fruit = Fruit::Banana;
            assert_eq!(banana.value(), 7_i16);

            let blueberry: Fruit = Fruit::Blueberry;
            assert_eq!(blueberry.value(), 13_i16);
        };
        1 2 3 4 5 6 7
    );
}

// Utility macros:

// Execute code for every given test enum number
macro_rules! for_each {
    ( $code:block ; $n:tt $( $rest:tt )* ) => (
        {
            make_test_enum!($n);
            $code
        }
        for_each!( $code; $( $rest )* );
    );
    ( $_:block ; ) => ()
}

use for_each;

// Generated enums for testing.
macro_rules! make_test_enum {
    (1) => {
        // i16
        value_enum! {
            enum Fruit: i16 {
                Apple = -42,
                Banana = 7,
                Blueberry = 13
            }
        }
    };
    (2) => {
        // pub i16
        value_enum! {
            pub enum Fruit: i16 {
                Apple = -42,
                Banana = 7,
                Blueberry = 13
            }
        }
    };
    (3) => {
        // item docs i16
        value_enum! {
            /// Fruity docs
            enum Fruit: i16 {
                Apple = -42,
                Banana = 7,
                Blueberry = 13
            }
        }
    };
    (4) => {
        // item docs #2 i16
        value_enum! {
            /// Fruity docs
            /// second line
            enum Fruit: i16 {
                Apple = -42,
                Banana = 7,
                Blueberry = 13
            }
        }
    };
    (5) => {
        // variant docs i16
        value_enum! {
            enum Fruit: i16 {
                /// Docs for the apple
                Apple = -42,
                Banana = 7,
                Blueberry = 13
            }
        }
    };
    (6) => {
        // variant docs #2 i16
        value_enum! {
            enum Fruit: i16 {
                /// Docs for the apple
                /// second apple line
                Apple = -42,
                Banana = 7,
                Blueberry = 13
            }
        }
    };
    (7) => {
        // multiple variant docs i16
        value_enum! {
            enum Fruit: i16 {
                /// Docs for the apple
                /// second apple line
                Apple = -42,
                Banana = 7,
                /// Blue
                Blueberry = 13
            }
        }
    };
}

use make_test_enum;
