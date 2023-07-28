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
        - name: $suite_name:ident
        $(- setup: $setup:ident $(($($arg_type:ty),+))?)?
        $(- teardown: $teardown:ident)?
        $(use $top_level_imports:ident::*;)?
        $(mod $mod_name:ident {
            $(use $mod_imports:ident::*;)?
            $(test $test_name:ident$(($($($arg_name:ident)*),+))? $test:block)*
        })*
    ) => {
        mod $suite_name {
            $(use super::$setup;)?
            $(use super::$teardown;)?
            $(use $top_level_imports::*;)?

            fn __internal_test_suite_setup() $($(-> ($($arg_type),*))?)? {
                $($setup())?
            }

            fn __internal_test_suite_teardown() {
                $($teardown();)?
            }

            $(
                mod $mod_name {
                    use super::__internal_test_suite_setup;
                    use super::__internal_test_suite_teardown;
                    $(use $mod_imports::*;)?

                    $(
                        #[test]
                        fn $test_name() {
                            // Assign the return value of the setup function to the given names (if specified)
                            $(let ($($($arg_name)*),*) =)? __internal_test_suite_setup();
                            // Running test code
                            let test_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| { $test }));
                            // Running teardown function
                            let teardown_result = std::panic::catch_unwind(move || { __internal_test_suite_teardown(); });
                            // Process test results
                            test_result.unwrap();
                            teardown_result.unwrap();
                        }
                    )*
                }
            )*
        }
    };
    (
        - name: $suite_name:ident
        $(- setup: $setup:ident $(($($arg_type:ty),+))?)?
        $(- teardown: $teardown:ident)?
        $(use $top_level_imports:ident::*;)?
        $(test $test_name:ident$(($($($arg_name:ident)*),+))? $test:block)*
    ) => {
        mod $suite_name {
            $(use super::$setup;)?
            $(use super::$teardown;)?
            $(use $top_level_imports::*;)?

            fn __internal_test_suite_setup() $($(-> ($($arg_type),*))?)? {
                $($setup())?
            }

            fn __internal_test_suite_teardown() {
                $($teardown();)?
            }

            $(
                #[test]
                fn $test_name() {
                    // Assign the return value of the setup function to the given names (if specified)
                    $(let ($($($arg_name)*),*) =)? __internal_test_suite_setup();
                    // Running test code
                    let test_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| { $test }));
                    // Running teardown function
                    let teardown_result = std::panic::catch_unwind(move || { __internal_test_suite_teardown(); });
                    // Process test results
                    test_result.unwrap();
                    teardown_result.unwrap();
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

    fn test_func_in_super() -> bool {
        true
    }

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

        use super::*;

        test creates_the_test {
            assert!(test_func_in_super());
        }
    }

    test_suite! {
        - name: test_suite_mutability
        - setup: setup(i32, &'static str)

        test creates_the_test(mut nbr, _string) {
            assert_eq!(nbr, 43);
            nbr = 100;
            assert_eq!(nbr, 100);
        }
    }

    test_suite! {
        - name: test_suite_with_mods

        use super::*;

        mod test_mod {
            use super::*;

            test test1 {
                assert!(test_func_in_super())
            }

            test test2 {
                assert_eq!(1, 1);
            }
        }
    }

    test_suite! {
        - name: test_suite_with_mods_and_setup
        - setup: setup(i32, &'static str)

        use super::*;

        mod test_mod {
            use super::*;

            test test1 {
                assert!(test_func_in_super())
            }

            test test2(nbr, _string) {
                assert_eq!(nbr, 43);
            }
        }
    }
}
