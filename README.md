# Rust WASM Testing Example

This repository demonstrates how to effectively test Rust code across different targets, specifically focusing on native (standard) environments and WebAssembly (WASM).

## Why Cross-Target Testing Matters

When developing Rust code that targets multiple targets, it's common to use conditional compilation (`#[cfg]` attributes) to handle target-specific behavior. However, this can lead to subtle bugs and issues that only manifest on certain targets.

For instance, we may expect a function to return the same value on all targets, but due to the conditional compilation, it may behave differently in native and WASM environments. We would never know about this discrepancy if we only run tests on the native target.

By implementing cross-target testing, we can identify and address these issues early in the development process, ensuring the reliability and portability of our Rust code.

## Project Description

This project provides three example functions demonstrating different target-specific behaviors:

*   `all_targets_hello()`: This function returns "Hello, all targets!" and is designed to work correctly in both native and WASM environments.
*   `standard_hello()`: This function returns "Hello, world!" in native environments. It *panics* if called in a WASM environment.
*   `wasm_hello()`: This function returns "Hello, WASM!" in WASM environments. It *panics* if called in a native environment.

The tests are organized as follows:

*   A top-level test function `test_all_targets_hello` tests the `all_targets_hello` function. This test is meant to be **run on both targets**.
*   The `standard_tests` module contains tests specifically for native environments. It includes a test for `standard_hello` and a test that verifies the expected panic when `wasm_hello` is called in a native context.
*   The `wasm_tests` module contains tests specifically for WASM environments. It includes a test for `wasm_hello` and a test that verifies the expected panic when `standard_hello` is called in a WASM context.

This structure demonstrates how to write tests that are specific to certain targets and also how to write tests that are shared between targets.

## Requirements

To run these tests, you will need the following:

*   **Rust:** Make sure you have a recent version of Rust installed. You can install it from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install). [Tested with Rust stable `1.83.0`]
*   **wasm-pack:** This tool is used for building and testing WASM modules. Install it using Cargo:

    ```bash
    cargo install wasm-pack
    ```

    [Tested with version `0.13.1`]

*   **Node.js:** `wasm-pack test` can run tests in a Node.js environment. You can install it from [https://nodejs.org/](https://nodejs.org/) or through a version manager like [NVM](https://github.com/nvm-sh/nvm). [Tested with version `v23.1.0`]

## Running Tests

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/ptagl/rust-wasm-testing-example.git
    cd rust-wasm-testing-example
    ```

2.  **Run native tests:**

    ```bash
    cargo test
    ```

    This will compile and run the tests for your native target.

3.  **Run WASM tests (using Node.js):**

    ```bash
    wasm-pack test --node
    ```

Both commands should run a total of 3 tests each (one shared test and two target-specific tests).

## License
[MIT License](LICENSE)
