//! # Bindings for RIOT system calls
//!
//! This crate contains dynamically generated Rust FFI bindings to the [RIOT
//! Operating System](https://riot-os.org/).
//!
//! Those bindings are inherently unsafe; it is recommended that their safe
//! abstractions in the [riot-wrappers] crate are used in most applications.
//!
//! [riot-wrappers]: https://crates.io/crates/riot-wrappers
//!
//! ## RIOT integration
//!
//! Which functions and structs are present in this crate, and sometimes their
//! details, inherently depends on the RIOT configuration this will be used with.
//! For example, RIOT's `struct _thread` only has a member `name` if `DEVHELP` is
//! set for a build, and its `flags` member is only present if the `thread_flags`
//! module is in use.
//!
//! All the relevant information -- including the location of the actually used
//! RIOT header files -- is contained in the RIOT environment variables
//! `CFLAGS_WITH_MACROS` and `INCLUDES`; both need to be passed in to the Rust
//! build system as a `RIOT_CFLAGS` environment variable.
//!
//! In addition, riot-sys also needs to know the C compiler to properly expand the
//! header files before transpilation; that information is passed in `RIOT_CC`.
//!
//! When using riot-sys, it is usually easiest to run from a target within the Make
//! system like this:
//!
//! ~~~~
//! target/thumbv7m-none-eabi/debug/libmy_app.a: always
//! 	CC= CFLAGS= CPPFLAGS= RIOT_CC="${CC}" RIOT_CFLAGS="$(CFLAGS_WITH_MACROS) $(INCLUDES)" cargo build --target thumbv7m-none-eabi
//!
//! .PHONY: always
//! ~~~~
//!
//! (CFLAGS etc. need to be cleared, for otherwise Cargo would assume those are
//! host flags.)
//!
//!
//! The `RIOT_CC` and `RIOT_CFLAGS` are made available to dependent modules through
//! Cargo; see [riot-wrappers]'s build.sh for an example.
//!
//!
//! As an alternative to passing `RIOT_CFLAGS` and `RIOT_CC`, the path to a
//! compile-commands.json file can be passed in `RIOT_COMPILE_COMMANDS_JSON`, with
//! a `RIOT_USEMODULES` to go with it containing the list of used modules. The advantage of this
//! approach is that on the RIOT side, LLVM-compativble CFLAGS are produced immaterial of which C
//! compiler is used. Even when this alternative is used, the extracted CC and CFLAGS are still
//! passed down to dependent crates as they were before. (The passed down CC will just always be
//! clang).
//!
//! ## Extension
//!
//! Currently, only a subset of all the RIOT headers is processed; all the relevant
//! header files are included in this crate's `riot-headers.h` header file. If you
//! need access to more RIOT APIs, more includes can be added there.
//!
//! ## External build dependencies
//!
//! This crate's operation depends on [C2Rust] being installed.
//! As right now some of the required fixes to C2Rust are not merged upstream yet,
//! (and as it requires a particular nightly version),
//! it should be installed like this:
//!
//!     $ git clone https://github.com/chrysn-pull-requests/c2rust/ -b for-riot
//!     $ cd c2rust
//!     $ rustup install nightly-2019-12-05
//!     $ rustup component add --toolchain nightly-2019-12-05 rustfmt rustc-dev
//!     $ cargo +nightly-2019-12-05 install --locked --debug --path c2rust
//!
//! [C2Rust]: https://c2rust.com/
//!
//! ---
//!
//! The main contents of this crate (ie. everything not in a module) is generated by bindgen.
//!
//! Unlike the inline module (which contains the C2Rust transpilate), it is not moved into a
//! dedicated linked module and reexported (in analogy to the inline), for that'd need explicit
//! `pub use linked::mutex_t` etc for every type that's present in both and thus not imported for
//! either. As long as this is inlined here, linked types (which are predominantly used so far)
//! take precedence automatically.
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(const_raw_ptr_deref)]
#![feature(const_mut_refs)]
// when experimenting with C2Rust generated extern functions, C library fn are pulled in and they
// have stuff like `pub type iovec`
#![feature(extern_types)]
// eg. for irq_enable on arm
#![feature(llvm_asm)]
#![feature(const_impl_trait)]

pub mod libc;

pub mod inline;
pub use inline::*;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[macro_use]
pub use cstr_core::cstr;
