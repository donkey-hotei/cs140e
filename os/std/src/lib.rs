// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![stable(feature= "rust1", since = "1.0.0")]

// Don't link to std. We are std.
#![no_std]

// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]

// Tell the compiler to link to either panic_abort or panic_unwind
#![needs_panic_runtime]

// std may use features in a platform-specific way
#![allow(unused_features)]

// std is implemented with unstable features, many of which are internal
// compiler details that will never be stable
#![cfg_attr(test, feature(test, update_panic_count))]
// #![feature(alloc)]
// #![feature(alloc_error_handler)]
// #![feature(alloc_system)]
#![feature(allocator_api)]
#![feature(allocator_internals)]
#![feature(allow_internal_unsafe)]
#![feature(allow_internal_unstable)]
#![feature(align_offset)]
#![feature(arbitrary_self_types)]
#![feature(array_error_internals)]
#![feature(asm)]
#![feature(box_syntax)]
#![feature(cfg_target_has_atomic)]
#![feature(cfg_target_thread_local)]
#![feature(cfg_target_vendor)]
#![feature(char_error_internals)]
#![feature(compiler_builtins_lib)]
#![cfg_attr(stage0, feature(min_const_fn))]
#![feature(const_int_ops)]
// #![feature(const_ip)]
#![feature(const_raw_ptr_deref)]
// #![feature(const_cstr_unchecked)]
#![feature(core_intrinsics)]
#![feature(dropck_eyepatch)]
#![feature(duration_as_u128)]
#![feature(exact_size_is_empty)]
#![feature(external_doc)]
#![feature(fixed_size_array)]
#![feature(fn_traits)]
// #![feature(fnbox)]
#![feature(futures_api)]
#![feature(generator_trait)]
#![feature(hashmap_internals)]
#![feature(int_error_internals)]
#![feature(integer_atomics)]
#![feature(lang_items)]
#![feature(libc)]
#![feature(link_args)]
#![feature(linkage)]
#![feature(needs_panic_runtime)]
#![feature(never_type)]
#![feature(nll)]
#![feature(exhaustive_patterns)]
#![feature(on_unimplemented)]
#![feature(optin_builtin_traits)]
#![feature(panic_internals)]
// #![feature(panic_unwind)]
#![feature(pin)]
#![feature(prelude_import)]
#![feature(ptr_internals)]
#![feature(raw)]
#![feature(rustc_attrs)]
#![feature(rustc_const_unstable)]
#![feature(std_internals)]
#![cfg_attr(not(stage0), feature(stdsimd))]
// #![feature(shrink_to)]
#![feature(slice_concat_ext)]
#![feature(slice_internals)]
#![feature(slice_patterns)]
#![feature(staged_api)]
#![feature(stmt_expr_attributes)]
#![feature(str_internals)]
#![feature(rustc_private)]
#![feature(thread_local)]
// #![feature(toowned_clone_into)]
#![feature(try_from)]
// #![feature(try_reserve)]
#![feature(unboxed_closures)]
#![feature(untagged_unions)]
#![feature(unwind_attributes)]
#![feature(doc_cfg)]
#![feature(doc_masked)]
#![feature(doc_spotlight)]
#![feature(doc_alias)]
#![feature(doc_keyword)]
#![feature(panic_info_message)]
#![feature(non_exhaustive)]
#![feature(pattern)]
#![feature(unicode_internals)]
#![feature(split_ascii_whitespace)]
#![feature(panic_implementation)]
#![feature(core_panic_info)]

#![default_lib_allocator]

// Always use alloc_system during stage0 since we don't know if the alloc_*
// crate the stage0 compiler will pick by default is enabled (e.g.
// if the user has disabled jemalloc in `./configure`).
// `force_alloc_system` is *only* intended as a workaround for local rebuilds
// with a rustc without jemalloc.
// FIXME(#44236) shouldn't need MSVC logic
// #[cfg(all(not(target_env = "msvc"),
//           any(all(stage0, not(test)), feature = "force_alloc_system")))]
// #[global_allocator]
// static ALLOC: alloc_system::System = alloc_system::System;

// Explicitly import the prelude. The compiler uses this same unstable attribute
// to import the prelude implicitly when building crates that depend on std.
#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

// // Access to Bencher, etc.
// #[cfg(test)] extern crate test;
// #[cfg(test)] extern crate rand;

// Re-export a few macros from core
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::{panic, assert_eq, assert_ne, debug_assert, debug_assert_eq, debug_assert_ne};
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::{unreachable, unimplemented, write, writeln, try};

extern crate core as __core;

// #[allow(unused_imports)] // macros from `alloc` are not used on all platforms
// #[macro_use]
// extern crate alloc as alloc_crate;
// extern crate alloc_system;
// #[doc(masked)]
// extern crate libc;

// // We always need an unwinder currently for backtraces
// #[doc(masked)]
// #[allow(unused_extern_crates)]
// extern crate unwind;

// // During testing, this crate is not actually the "real" std library, but rather
// // it links to the real std library, which was compiled from this same source
// // code. So any lang items std defines are conditionally excluded (or else they
// // would generate duplicate lang item errors), and any globals it defines are
// // _not_ the globals used by "real" std. So this import, defined only during
// // testing gives test-std access to real-std lang items and globals. See #2912
// #[cfg(test)] extern crate std as realstd;

// The standard macros that are not built-in to the compiler.
#[macro_use]
mod macros;

// The Rust prelude
pub mod prelude;

// Public module declarations and re-exports
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::any;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::cell;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::clone;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::cmp;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::convert;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::default;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::hash;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::intrinsics;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::iter;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::marker;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::mem;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::ops;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::ptr;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::raw;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::result;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::option;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::isize;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i8;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i16;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i32;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i64;
#[stable(feature = "i128", since = "1.26.0")]
pub use core::i128;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::usize;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u8;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u16;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u32;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u64;
// #[stable(feature = "rust1", since = "1.0.0")]
// pub use alloc_crate::boxed;
// #[stable(feature = "rust1", since = "1.0.0")]
// pub use alloc_crate::rc;
// #[stable(feature = "rust1", since = "1.0.0")]
// pub use alloc_crate::borrow;
// #[stable(feature = "rust1", since = "1.0.0")]
// pub use alloc_crate::fmt;
// #[stable(feature = "rust1", since = "1.0.0")]
// pub use alloc_crate::format;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::fmt;
// #[unstable(feature = "pin", issue = "49150")]
// pub use core::pin;
// #[stable(feature = "rust1", since = "1.0.0")]
// pub use alloc_crate::slice;
// #[stable(feature = "rust1", since = "1.0.0")]
// pub use alloc_crate::str;
// #[stable(feature = "rust1", since = "1.0.0")]
// pub use alloc_crate::string;
// #[stable(feature = "rust1", since = "1.0.0")]
// pub use alloc_crate::vec;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::char;
#[stable(feature = "i128", since = "1.26.0")]
pub use core::u128;
#[stable(feature = "core_hint", since = "1.27.0")]
pub use core::hint;

// NOTE: This is an addition. This should actually come from `alloc`.
pub mod str;
pub mod slice;

// pub mod f32;
// pub mod f64;

// #[macro_use]
// pub mod thread;
// pub mod ascii;
// pub mod collections;
// pub mod env;
// pub mod error;
// pub mod ffi;
// pub mod fs;
pub mod io;
// pub mod net;
// pub mod num;
// pub mod os;
// pub mod panic;
// pub mod path;
// pub mod process;
pub mod sync;
// pub mod time;

// #[unstable(feature = "futures_api",
//            reason = "futures in libcore are unstable",
//            issue = "50547")]
// pub mod task {
//     //! Types and Traits for working with asynchronous tasks.
//     #[doc(inline)]
//     pub use core::task::*;
//     #[doc(inline)]
//     pub use alloc_crate::task::*;
// }
// 
// #[unstable(feature = "futures_api",
//            reason = "futures in libcore are unstable",
//            issue = "50547")]
// pub mod future;
// 
// Platform-abstraction modules
// #[macro_use]
// mod sys_common;
// mod sys;
// 
// pub mod alloc;
// 
// // Private support modules
// mod panicking;
// mod memchr;
// 
// // The runtime entry point and a few unstable public functions used by the
// // compiler
// pub mod rt;
// 
// // Pull in the the `stdsimd` crate directly into libstd. This is the same as
// // libcore's arch/simd modules where the source of truth here is in a different
// // repository, but we pull things in here manually to get it into libstd.
// //
// // Note that the #[cfg] here is intended to do two things. First it allows us to
// // change the rustc implementation of intrinsics in stage0 by not compiling simd
// // intrinsics in stage0. Next it doesn't compile anything in test mode as
// // stdsimd has tons of its own tests which we don't want to run.
// #[path = "../stdsimd/stdsimd/mod.rs"]
// #[allow(missing_debug_implementations, missing_docs, dead_code)]
// #[unstable(feature = "stdsimd", issue = "48556")]
// #[cfg(all(not(stage0), not(test)))]
// mod stdsimd;
// 
// // A "fake" module needed by the `stdsimd` module to compile, not actually
// // exported though.
// #[cfg(not(stage0))]
// mod coresimd {
//     pub use core::arch;
// }
// 
// #[stable(feature = "simd_arch", since = "1.27.0")]
// #[cfg(all(not(stage0), not(test)))]
// pub use stdsimd::arch;
// 
// // Include a number of private modules that exist solely to provide
// // the rustdoc documentation for primitive types. Using `include!`
// // because rustdoc only looks for these modules at the crate level.
// include!("primitive_docs.rs");
// 
// // Include a number of private modules that exist solely to provide
// // the rustdoc documentation for the existing keywords. Using `include!`
// // because rustdoc only looks for these modules at the crate level.
// include!("keyword_docs.rs");
