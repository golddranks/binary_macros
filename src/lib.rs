#[macro_use]
extern crate proc_macro_hack;
extern crate binary_macros_impl;

#[proc_macro_hack]
pub use binary_macros_impl::base16;

#[proc_macro_hack]
pub use binary_macros_impl::base32hex;

#[proc_macro_hack]
pub use binary_macros_impl::base32;

#[proc_macro_hack]
pub use binary_macros_impl::base64;

#[proc_macro_hack]
pub use binary_macros_impl::base64url;

#[proc_macro_hack]
pub use binary_macros_impl::base32hex_nopad;

#[proc_macro_hack]
pub use binary_macros_impl::base32_nopad;

#[proc_macro_hack]
pub use binary_macros_impl::base64_nopad;

#[proc_macro_hack]
pub use binary_macros_impl::base64url_nopad;
