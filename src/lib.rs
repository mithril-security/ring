// Copyright 2015-2016 Brian Smith.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHORS DISCLAIM ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

//! Safe, fast, small crypto using Rust with BoringSSL's cryptography
//! primitives.
//!
//! # Feature Flags
//!
//! <table>
//! <tr><th>Feature
//!     <th>Description
//! <tr><td><code>alloc (default)</code>
//!     <td>Enable features that require use of the heap, RSA in particular.
//! <tr><td><code>dev_urandom_fallback (default)</code>
//!     <td>This is only applicable to Linux. On Linux, by default,
//!         <code>ring::rand::SystemRandom</code> will fall back to reading
//!         from <code>/dev/urandom</code> if the <code>getrandom()</code>
//!         syscall isn't supported at runtime. When the
//!         <code>dev_urandom_fallback</code> feature is disabled, such
//!         fallbacks will not occur. See the documentation for
//!         <code>rand::SystemRandom</code> for more details.
//! <tr><td><code>std</code>
//!     <td>Enable features that use libstd, in particular
//!         <code>std::error::Error</code> integration. Implies `alloc`.
//! <tr><td><code>wasm32_c</code>
//!     <td>Enables features that require a C compiler on wasm32 targets, such as
//!        the <code>constant_time</code> module, HMAC verification, and PBKDF2
//!        verification. Without this feature, only a subset of functionality
//!        is provided to wasm32 targets so that a C compiler isn't needed. A
//!        typical invocation would be:
//!        <code>TARGET_CC=clang-10 TARGET_AR=llvm-ar-10 cargo test --target=wasm32-unknown-unknown --features=wasm32_c</code>
//!        with <code>llvm-ar-10</code> and <code>clang-10</code> in <code>$PATH</code>.
//!        (Going forward more functionality should be enabled by default, without
//!        requiring these hacks, and without requiring a C compiler.)
//! <tr><td><code>force_std_detection</code>
//!     <td>This is only applicable to x86. By default, <i>ring</i> will use
//!         custom logic with the CPUID instruction to figure out which CPU
//!         features are available. With this feature, the standard
//!         <code>std::is_x86_feature_detected</code> macro will be used
//!         instead.
//! </table>

#![doc(html_root_url = "https://briansmith.org/rustdoc/")]
#![allow(
    clippy::collapsible_if,
    clippy::identity_op,
    clippy::len_without_is_empty,
    clippy::len_zero,
    clippy::let_unit_value,
    clippy::many_single_char_names,
    clippy::needless_range_loop,
    clippy::new_without_default,
    clippy::neg_cmp_op_on_partial_ord,
    clippy::range_plus_one,
    clippy::too_many_arguments,
    clippy::trivially_copy_pass_by_ref,
    clippy::type_complexity,
    clippy::unreadable_literal,
    missing_copy_implementations,
    missing_debug_implementations,
    non_camel_case_types,
    non_snake_case,
    unsafe_code
)]
// `#[derive(...)]` uses `trivial_numeric_casts` and `unused_qualifications`
// internally.
#![deny(missing_docs, unused_qualifications, variant_size_differences)]
#![forbid(unused_results)]
#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

#[macro_use]
mod debug;

#[macro_use]
pub mod test;

#[macro_use]
mod arithmetic;

#[macro_use]
mod bssl;

#[macro_use]
mod polyfill;

pub mod aead;
pub mod agreement;

mod bits;

pub(crate) mod c;
pub mod constant_time;

pub mod io;

mod cpu;
pub mod digest;
mod ec;
mod endian;
pub mod error;
pub mod hkdf;
pub mod hmac;
mod limb;
pub mod pbkdf2;
pub mod pkcs8;
pub mod rand;

#[cfg(feature = "alloc")]
mod rsa;

pub mod signature;

mod sealed {
    /// Traits that are designed to only be implemented internally in *ring*.
    //
    // Usage:
    // ```
    // use crate::sealed;
    //
    // pub trait MyType: sealed::Sealed {
    //     // [...]
    // }
    //
    // impl sealed::Sealed for MyType {}
    // ```
    pub trait Sealed {}
}

/// # Information about using *ring* in SGX
///
/// ## CPU feature detection
/// On `x86_64-fortanix-unknown-sgx`, feature detection is done using the
/// `std::is_x86_feature_detected` macro, which currently only supports
/// features enabled at compile-time. You must enable at least the `aes` and
/// `pclmul` features, otherwise *ring* will panic at runtime. See the [GitHub
/// issue](https://github.com/fortanix/rust-sgx/issues/26) for more
/// information.
///
/// To set compile-time features, you can either specify them as an environment
/// variable:
///
/// ```text
/// RUSTFLAGS="-C target-feature=+aes,+pclmul"
/// ```
///
/// Or you may configure them per target in [`.cargo/config`].
///
/// [`.cargo/config`]: https://doc.rust-lang.org/cargo/reference/config.html#configuration-keys
///
/// ## Entropy source
/// The entropy source used in SGX is the hardware random number generator
/// provided by the RDRAND instruction.
///
/// ## Nightly only
/// The `x86_64-fortanix-unknown-sgx` target is only available on nightly, and
/// *ring* Continuous Builds only build it for nightly. See the [GitHub
/// issue](https://github.com/briansmith/ring/issues/779) for more information.
///
/// ## Continuous Testing
/// While the *ring* test suite works in SGX, and it is run manually from time
/// to time, it doesn't run automatically as part of a Continuous Testing
/// setup. See the [GitHub issue](https://github.com/briansmith/ring/issues/778)
/// for more information.
#[cfg(target_vendor = "teaclave")]
pub mod sgx {}
