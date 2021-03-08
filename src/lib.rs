#![cfg_attr(not(feature = "std"), no_std)]
#![feature(const_if_match, const_loop, track_caller, proc_macro_hygiene)]
#![feature(asm)]
#![feature(unwind_attributes)]
#![feature(specialization)]

pub mod crc32;

pub mod params;

#[cfg(feature = "std")]
pub mod resource;

#[doc(hidden)]
pub mod cpp;

#[doc(hidden)]
pub mod common;

#[doc(inline)]
pub use common::root::*;

#[doc(inline)]
pub use cpp::root::*;

// Find the hash40 of a given string
pub const fn hash40(string: &str) -> u64 {
    let bytes = string.as_bytes();

    ((bytes.len() as u64) << 32) + crc32::crc32(bytes) as u64
}

impl phx::Hash40 {
    pub fn new(string: &str) -> Self {
        Self {
            hash: hash40(string),
        }
    }

    pub fn new_raw(raw: u64) -> Self {
        Self { hash: raw }
    }
}

mod lua_const;
