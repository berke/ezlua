//! Low level bindings to Lua 5.4/5.3/5.2/5.1 including LuaJIT.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::c_int;

pub use lua54::*;

pub const LUA_MAX_UPVALUES: c_int = 255;

// I believe `luaL_traceback` < 5.4 requires this much free stack to not error.
// 5.4 uses `luaL_Buffer`
pub const LUA_TRACEBACK_STACK: c_int = 11;

// The minimum alignment guaranteed by the architecture. This value is used to
// add fast paths for low alignment values.
// Copied from https://github.com/rust-lang/rust/blob/master/library/std/src/sys/common/alloc.rs
#[cfg(all(any(
    target_arch = "x86",
    target_arch = "arm",
    target_arch = "mips",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "sparc",
    target_arch = "asmjs",
    target_arch = "wasm32",
    target_arch = "hexagon",
    all(target_arch = "riscv32", not(target_os = "espidf")),
    all(target_arch = "xtensa", not(target_os = "espidf")),
)))]
pub const SYS_MIN_ALIGN: usize = 8;
#[cfg(all(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "mips64",
    target_arch = "s390x",
    target_arch = "sparc64",
    target_arch = "riscv64",
    target_arch = "wasm64",
)))]
pub const SYS_MIN_ALIGN: usize = 16;
// The allocator on the esp-idf platform guarentees 4 byte alignment.
#[cfg(all(any(
    all(target_arch = "riscv32", target_os = "espidf"),
    all(target_arch = "xtensa", target_os = "espidf"),
)))]
pub const SYS_MIN_ALIGN: usize = 4;

pub mod lua54;
