//! Provides a macro to create a test suite with a setup and teardown function.
//! Each test block generates a separate test function that will run
//! setup and teardown functions if provided.
//!
//! # Example
//! ```
//!
//! # mod test {
//! use test_suite_rs::test_suite;
//!
//! fn setup() -> (i32, String) {
//!     (43, "my_string".to_owned())
//! }
//!
//! fn teardown() {}
//!
//! test_suite! {
//!     - name: test_mod
//!     - setup: setup(i32, String)
//!     - teardown: teardown
//!
//!     test should_return_true(nbr, my_string) {
//!         assert_eq!(nbr, 43);
//!         assert_eq!(&my_string, "my_string");
//!     }
//!
//!     test should_return_false {
//!         assert!(true);
//!     }
//! }
//! # }
//!```
//!
//! Generates the following code (simplified):
//!
//!```
//! # fn setup() -> (i32, String) {
//! #    (43, "my_string".to_owned())
//! # }
//!
//! # fn teardown() {}
//!
//! mod test_mod {
//! #   use std::assert_eq;
//!     use super::*;
//!
//!     #[test]
//!     fn should_return_true() {
//!         let (nbr, my_string) = setup();
//!
//!         assert_eq!(nbr, 43);
//!         assert_eq!(&my_string, "my_string");
//!         teardown();
//!     }
//! }


/// Creates a test suite with a setup and teardown function.
/// Each test block generates a separate test function that will run
/// setup and teardown functions if provided.
///
/// # Example
/// ```
///
/// # mod test {
/// use test_suite_rs::test_suite;
///
/// fn setup() -> (i32, String) {
///     (43, "my_string".to_owned())
/// }
///
/// fn teardown() {}
///
/// test_suite! {
///     - name: test_mod
///     - setup: setup(i32, String)
///     - teardown: teardown
///
///     test should_return_true(nbr, my_string) {
///         assert_eq!(nbr, 43);
///         assert_eq!(&my_string, "my_string");
///     }
///
///     test should_return_false {
///         assert!(true);
///     }
/// }
/// # }
///```
///
/// Generates the following code (simplified):
///
///```
/// # fn setup() -> (i32, String) {
/// #    (43, "my_string".to_owned())
/// # }
///
/// # fn teardown() {}
///
/// mod test_mod {
/// #   use std::assert_eq;
///     use super::*;
///
///     #[test]
///     fn should_return_true() {
///         let (nbr, my_string) = setup();
///
///         assert_eq!(nbr, 43);
///         assert_eq!(&my_string, "my_string");
///         teardown();
///     }
/// }
#[macro_export]
macro_rules! test_suite {
    (
        $(- setup: $setup:ident $(($($arg_type:ty),+))?)?
        $(- teardown: $teardown:ident)?
        $(test $test_name:ident$($($arg_name:ident),+)? $test:block)*
    ) => {
        test_suite! {
            - name: test
            - setup: $setup
            - teardown: $teardown

            $($test_name $test)*
        };
    };
    (
        - name: $mod_name:ident
        $(- setup: $setup:ident $(($($arg_type:ty),+))?)?
        $(- teardown: $teardown:ident)?
        $(test $test_name:ident$(($($arg_name:ident),+))? $test:block)*
    ) => {
        mod $mod_name {
            use super::*;

            fn __internal_test_suite_setup() $($(-> ($($arg_type),*))?)? {
                $($setup())?
            }

            fn __internal_test_suite_teardown() {
                $(
                    $teardown();
                )?
            }

            $(
                #[test]
                fn $test_name() {
                    // Assign the return value of the setup function to the given names (if specified)
                    $(let ($($arg_name),*) =)? __internal_test_suite_setup();
                    // Running test code
                    $test
                    // Running teardown function
                    __internal_test_suite_teardown();
                }
            )*
        }
    };
}

#[cfg(test)]
mod test {
    fn setup() -> (i32, &'static str) {
        (43, "my_string")
    }

    fn teardown() {}

    test_suite! {
        - name: test_suite_full
        - setup: setup(i32, &'static str)
        - teardown: teardown

        test creates_the_test(nbr, string) {
            assert_eq!(string, "my_string");
            assert_eq!(nbr, 43);
        }
    }

    test_suite! {
        - name: test_suite_no_teardown
        - setup: setup(i32, &'static str)

        test creates_the_test(nbr, string) {
            assert_eq!(string, "my_string");
            assert_eq!(nbr, 43);
        }
    }

    test_suite! {
        - name: test_suite_no_setup
        - teardown: teardown

        test creates_the_test {
            assert!(true);
        }
    }
}