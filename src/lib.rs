/* Copyright 2016 Joshua Gentry
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

//! The purpose of this crate is to allow the the creation of objects that don't have the
//! synchronization overhead when used on one thread and the ability to run on multiple threads by
//! enabling the synchronization.
//!
//! Once synchronization is enabled the "cheapest" method is chosen to share the data between
//! multiple threads.  This means atomic objects when it can and mutexes when it can't.  The 64
//! bit data objects (f64, i64, u64) are shared via atomics when on a 64 bit architecture, and via
//! mutexes on a 32 bit architecture.
//!
//! # Examples
//!
//! ```
//! use shareable::SharedF32;
//!
//! // Only 1 instance of value1 is created, so no syncrhonization is used.
//! let mut value1 = SharedF32::new(63.23);
//!
//! println!("Value: {}", value1.get());
//!
//! value1.set(78.3);
//!
//! println!("Value: {}", value1.get());
//! ```
//!
//! ```
//! use std::sync::mpsc;
//! use std::thread;
//! use shareable::SharedObject;
//!
//! let mut value1 = SharedObject::new(String::from("abc"));
//!
//! // Syncronization is enabled at this point,
//! // all access to the object is now done via a mutex.
//! let mut value2 = value1.dup();
//!
//! let (tx, rx) = mpsc::channel();
//!
//! let thread = thread::spawn(move || {
//!     rx.recv();
//!     assert_eq!(*value2.get(), "xyz");
//! });
//!
//! value1.set(String::from("xyz"));
//!
//! tx.send(());
//! thread.join().unwrap();
//! ```
mod shared_f32;
#[cfg(target_pointer_width = "32")]
mod shared_f64_x32;
#[cfg(not(target_pointer_width = "32"))]
mod shared_f64_x64;
mod shared_i8;
mod shared_i16;
mod shared_i32;
#[cfg(target_pointer_width = "32")]
mod shared_i64_x32;
#[cfg(not(target_pointer_width = "32"))]
mod shared_i64_x64;
mod shared_isize;
mod shared_object;
mod shared_u8;
mod shared_u16;
mod shared_u32;
#[cfg(target_pointer_width = "32")]
mod shared_u64_x32;
#[cfg(not(target_pointer_width = "32"))]
mod shared_u64_x64;
mod shared_usize;

pub use shared_f32::SharedF32;
#[cfg(target_pointer_width = "32")]
pub use shared_f64_x32::SharedF64;
#[cfg(not(target_pointer_width = "32"))]
pub use shared_f64_x64::SharedF64;
pub use shared_i8::SharedI8;
pub use shared_i16::SharedI16;
pub use shared_i32::SharedI32;
#[cfg(target_pointer_width = "32")]
pub use shared_i64_x32::SharedI64;
#[cfg(not(target_pointer_width = "32"))]
pub use shared_i64_x64::SharedI64;
pub use shared_isize::SharedIsize;
pub use shared_object::SharedObject;
pub use shared_u8::SharedU8;
pub use shared_u16::SharedU16;
pub use shared_u32::SharedU32;
#[cfg(target_pointer_width = "32")]
pub use shared_u64_x32::SharedU64;
#[cfg(not(target_pointer_width = "32"))]
pub use shared_u64_x64::SharedU64;
pub use shared_usize::SharedUsize;
