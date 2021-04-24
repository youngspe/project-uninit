//! Macros for safe (and unsafe) access to and initialization of fields of structs wrapped in `MaybeUninit<_>`
//! This crate uses the
//! [`ptr::addr_of!`](core::ptr::addr_of)
//! and [`ptr::addr_of_mut!`](core::ptr::addr_of_mut)
//! macros introduced in Rust 1.51 to avoid undefined behavior.
//!
//!
//! ## Examples
//! ### Initialize a struct one field at a time
//! ```
//! use core::mem::MaybeUninit;
//! use project_uninit::partial_init;
//! #[derive(PartialEq, Eq, Debug)]
//! struct Inner { value1: u8, value2: (i32, bool) }
//! #[derive(PartialEq, Eq, Debug)]
//! struct MyStruct { name: &'static str, inner: Inner }
//!
//! let mut target = MaybeUninit::<MyStruct>::uninit();
//!
//! let name: &mut &str = partial_init!(target => name = "Foo");
//! assert_eq!(*name, "Foo");
//! *name = "Bar";
//!
//! let (value1, value2_0): (&mut u8, &mut i32) = partial_init!(target => {
//!     inner => value1: 0xff,
//!     inner => value2 => 0: 1000,
//! });
//! assert_eq!(*value1, 0xff);
//! assert_eq!(*value2_0, 1000);
//! *value2_0 *= 2;
//!
//! let value2_1: &mut bool = partial_init!(target => inner => value2 => 1 = true);
//! assert_eq!(*value2_1, true);
//!
//! assert_eq!(unsafe { target.assume_init() }, MyStruct {
//!     name: "Bar",
//!     inner: Inner { value1: 0xff, value2: (2000, true) },
//! });
//! ```
//!
//! ### Obtain references to fields of a `MaybeUninit<_>` struct
//! ```
//! # use core::mem::MaybeUninit;
//! use project_uninit::project_uninit;
//!
//! #[derive(PartialEq, Eq, Debug)]
//! struct Person { name: &'static str, age: u32 }
//! let person = MaybeUninit::new(Person {
//!     name: "Alice",
//!     age: 22,
//! });
//! let (name, age): (&MaybeUninit<&str>, &MaybeUninit<u32>) = project_uninit!(
//!     person => { name, age }
//! );
//!
//! assert_eq!(unsafe { name.assume_init() }, "Alice");
//! assert_eq!(unsafe { age.assume_init() }, 22);
//!
//! ```
//!
//! ### Obtain mutable references to fields of a `MaybeUninit<_>` struct
//! ```
//! # use core::mem::MaybeUninit;
//! # #[derive(PartialEq, Eq, Debug)]
//! # struct Person { name: &'static str, age: u32 }
//! use project_uninit::project_uninit_mut;
//!
//! let mut person = MaybeUninit::new(Person {
//!     name: "Alice",
//!     age: 22,
//! });
//!
//! let (name, age): (&mut MaybeUninit<&str>, &mut MaybeUninit<u32>) = project_uninit_mut!(
//!     person => { name, age }
//! );
//!
//! *name = MaybeUninit::new("Alicia");
//! *age = MaybeUninit::new(24);
//!
//! assert_eq!(unsafe { person.assume_init() }, Person {
//!     name: "Alicia",
//!     age: 24,
//! });
//! ```
//! ## Safety
//!
//! It's safe to mutably project multiple fields as long as they are distinct.
//! For example, the following snippets do not compile:
//!
//! ```compile_fail
//! # use core::mem::MaybeUninit;
//! # #[derive(PartialEq, Eq, Debug)]
//! # struct Person { name: &'static str, age: u32 }
//! # use project_uninit::partial_init;
//! let mut person = MaybeUninit::<Person>::uninit();
//! let (name1, name2) = partial_init!(person => { name: "Bob", name: "Robert" });
//! ```
//!
//! ```compile_fail
//! # use core::mem::MaybeUninit;
//! # #[derive(PartialEq, Eq, Debug)]
//! # struct Person { name: &'static str, age: u32 }
//! # use project_uninit::project_uninit_mut;
//! let mut person = MaybeUninit::<Person>::uninit();
//! let (name1, name2) = project_uninit_mut!(person => { name, name });
//! ```
//!
//! ```compile_fail,E0499
//! # use core::mem::MaybeUninit;
//! # #[derive(PartialEq, Eq, Debug)]
//! # struct Person { name: &'static str, age: u32 }
//! # use project_uninit::project_uninit_mut;
//! let mut person = MaybeUninit::<Person>::uninit();
//! let name1 = project_uninit_mut!(person => name);
//! let name2 = project_uninit_mut!(person => name);
//! drop(name1);
//! ```
//!
//! Additionally, lifetime rules are enforced just like any other borrow.
//! The following will not compile:
//! ```compile_fail,E0597
//! # use core::mem::MaybeUninit;
//! # #[derive(PartialEq, Eq, Debug)]
//! # struct Person { name: &'static str, age: u32 }
//! # use project_uninit::project_uninit;
//! let age: &MaybeUninit<u32>;
//! {
//!     let person = MaybeUninit::<Person>::uninit();
//!     age = project_uninit!(person => age);
//! } // person is dropped while still borrowed by age
//! drop(age);
//! ```
//!
//! However, this will:
//! ```
//! # use core::mem::MaybeUninit;
//! # #[derive(PartialEq, Eq, Debug)]
//! # struct Person { name: &'static str, age: u32 }
//! # use project_uninit::project_uninit;
//! fn get_uninit_age<'a>(person: &'a MaybeUninit<Person>) -> &'a MaybeUninit<u32> {
//!     project_uninit!(person => age)
//! }
//! ```
#![no_std]

mod assert_unique;
mod partial_init;
mod project;
#[doc(hidden)]
pub mod utils;
