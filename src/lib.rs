#[macro_use]
extern crate proc_macro_hack;

#[macro_use]
extern crate binary_macros_impl;
pub use binary_macros_impl::*;

proc_macro_expr_decl!(base64! => base64_impl);

proc_macro_expr_decl!(base64url! => base64url_impl);
