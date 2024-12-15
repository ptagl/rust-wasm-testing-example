#![allow(unreachable_code)]

/// This function returns a string containing the classic "Hello, world!" message.
/// However, it panics if called in a WebAssembly (WASM) environment. There is no
/// real reason for this panic other than to provide some useful use cases for the
/// test examples.
pub fn standard_hello() -> String {
    #[cfg(target_arch = "wasm32")]
    panic!("This function should not be called in a WASM environment");

    "Hello, world!".to_string()
}

/// This function returns a string containing the "Hello, WASM!" message.
/// However, it panics if called in a non-WASM environment.
pub fn wasm_hello() -> String {
    #[cfg(not(target_arch = "wasm32"))]
    panic!("This function should only be called in a WASM environment");

    "Hello, WASM!".to_string()
}

/// This function returns a generic greeting message that can be used
/// in any target.
pub fn all_targets_hello() -> String {
    "Hello, all targets!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    /// This test runs on all targets (both standard and WASM).
    fn test_all_targets_hello() {
        assert_eq!(all_targets_hello(), "Hello, all targets!");
    }

    /// This module contains unit tests meant to be run only in a non-WASM environment.
    mod standard_tests {
        use super::*;

        #[test]
        fn test_standard_hello() {
            assert_eq!(standard_hello(), "Hello, world!");
        }

        #[test]
        #[should_panic]
        fn test_wasm_hello_panic() {
            wasm_hello();
        }
    }

    /// This module contains unit tests meant to be run only in a WASM environment.
    #[cfg(target_arch = "wasm32")]
    mod wasm_tests {
        use wasm_bindgen_test::wasm_bindgen_test;

        use super::*;

        #[wasm_bindgen_test]
        #[should_panic]
        fn test_standard_hello_panic() {
            standard_hello();
        }

        #[wasm_bindgen_test]
        fn test_wasm_hello() {
            assert_eq!(wasm_hello(), "Hello, WASM!");
        }
    }
}
