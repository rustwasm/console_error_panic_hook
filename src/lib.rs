//! # `console_error_panic_hook`
//!
//! [![](https://docs.rs/console_error_panic_hook/badge.svg)](https://docs.rs/console_error_panic_hook/)
//! [![](https://img.shields.io/crates/v/console_error_panic_hook.svg)](https://crates.io/crates/console_error_panic_hook)
//! [![](https://img.shields.io/crates/d/console_error_panic_hook.png)](https://crates.io/crates/console_error_panic_hook)
//! [![Build Status](https://travis-ci.org/rustwasm/console_error_panic_hook.svg?branch=master)](https://travis-ci.org/rustwasm/console_error_panic_hook)
//!
//! This crate lets you debug panics on `wasm32-unknown-unknown` by providing a
//! panic hook that forwards panic messages to
//! [`console.error`](https://developer.mozilla.org/en-US/docs/Web/API/Console/error).
//!
//! When an error is reported with `console.error`, browser devtools and node.js
//! will typically capture a stack trace and display it with the logged error
//! message.
//!
//! ## Usage
//!
//! There are two ways to install this panic hook.
//!
//! First, you can set the hook yourself by calling `std::panic::set_hook` in
//! some initialization function:
//!
//! ```
//! extern crate console_error_panic_hook;
//! use std::panic;
//!
//! fn my_init_function() {
//!     panic::set_hook(Box::new(console_error_panic_hook::hook));
//!
//!     // ...
//! }
//! ```
//!
//! Alternatively, use `set_once` on some common code path to ensure that
//! `set_hook` is called, but only the one time. Under the hood, this uses
//! `std::sync::Once`.
//!
//! ```
//! extern crate console_error_panic_hook;
//!
//! struct MyBigThing;
//!
//! impl MyBigThing {
//!     pub fn new() -> MyBigThing {
//!         console_error_panic_hook::set_once();
//!
//!         MyBigThing
//!     }
//! }
//! ```

#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

#[macro_use]
extern crate cfg_if;

use std::panic;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        extern crate wasm_bindgen;
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen]
        extern {
            #[wasm_bindgen(js_namespace = console)]
            fn error(msg: String);
        }

        fn hook_impl(info: &panic::PanicInfo) {
            error(info.to_string());
        }
    } else {
        use std::io::{self, Write};

        fn hook_impl(info: &panic::PanicInfo) {
            let _ = writeln!(io::stderr(), "{}", info);
        }
    }
}

/// A panic hook for use with
/// [`std::panic::set_hook`](https://doc.rust-lang.org/nightly/std/panic/fn.set_hook.html)
/// that logs panics into
/// [`console.error`](https://developer.mozilla.org/en-US/docs/Web/API/Console/error).
///
/// On non-wasm targets, prints the panic to `stderr`.
pub fn hook(info: &panic::PanicInfo) {
    hook_impl(info);
}

/// Set the `console.error` panic hook the first time this is called. Subsequent
/// invocations do nothing.
#[inline]
pub fn set_once() {
    use std::sync::{ONCE_INIT, Once};
    static SET_HOOK: Once = ONCE_INIT;
    SET_HOOK.call_once(|| {
        panic::set_hook(Box::new(hook));
    });
}
