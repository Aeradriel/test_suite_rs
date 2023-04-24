# test_suite_rs
 Provides a macro to create a test suite with a setup and teardown function.
 Each test block generates a separate test function that will run
 setup and teardown functions if provided.

# Example
```rust
use test_suite_rs::test_suite;

fn setup() -> (i32, String) {
    (43, "my_string".to_owned())
}

fn teardown() {}

test_suite! {
    - name: test_mod
    - setup: setup(i32, String)
    - teardown: teardown

    test should_return_true(nbr, my_string) {
        assert_eq!(nbr, 43);
        assert_eq!(&my_string, "my_string");
    }

    test should_return_false {
        assert!(true);
    }
}
```

 Generates the following code (simplified):

```rust
mod test_mod {
    use super::*;

    #[test]
    fn should_return_true() {
        let (nbr, my_string) = setup();

        assert_eq!(nbr, 43);
        assert_eq!(&my_string, "my_string");
        teardown();
    }
}
```