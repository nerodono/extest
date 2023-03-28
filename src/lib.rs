#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;

/// Group macro. See module-level documentation
#[cfg(feature = "group")]
#[proc_macro_attribute]
pub fn group(args: TokenStream, stream: TokenStream) -> TokenStream {
    group::group_impl(args, stream)
}

#[cfg(feature = "group")]
mod group;
