#![no_std]

/*!
   This is an internal crate used by the `cgp-component-macro` crate. We implement the
   proc macros for `cgp-component` as a library, so that it can be more easily tested.
   The constructs are then re-exported as proc macros in the `cgp-component-macro` crate,
   which is defined as a proc macro crate.
*/

extern crate alloc;

pub mod delegate_components;
pub mod derive_component;
pub mod for_each_replace;
pub mod preset;
pub mod type_component;

#[cfg(test)]
mod tests;

pub use crate::delegate_components::delegate_components;
pub use crate::derive_component::derive_component;
pub use crate::for_each_replace::{handle_for_each_replace, handle_replace};
pub use crate::preset::define_preset;
pub use crate::type_component::derive::derive_type_component;
