#[macro_use]
extern crate proc_macro_hack;

#[allow(unused_imports)]
#[macro_use]
extern crate binary_macros_impl;

pub use binary_macros_impl::*;

proc_macro_expr_decl!(base2! => base2_impl);
proc_macro_expr_decl!(base4! => base4_impl);
proc_macro_expr_decl!(base8! => base8_impl);
proc_macro_expr_decl!(base16! => base16_impl);
proc_macro_expr_decl!(base32hex! => base32hex_impl);
proc_macro_expr_decl!(base32! => base32_impl);
proc_macro_expr_decl!(base64! => base64_impl);
proc_macro_expr_decl!(base64url! => base64url_impl);

proc_macro_expr_decl!(base2_nopad! => base2_nopad_impl);
proc_macro_expr_decl!(base4_nopad! => base4_nopad_impl);
proc_macro_expr_decl!(base8_nopad! => base8_nopad_impl);
proc_macro_expr_decl!(base16_nopad! => base16_nopad_impl);
proc_macro_expr_decl!(base32hex_nopad! => base32hex_nopad_impl);
proc_macro_expr_decl!(base32_nopad! => base32_nopad_impl);
proc_macro_expr_decl!(base64_nopad! => base64_nopad_impl);
proc_macro_expr_decl!(base64url_nopad! => base64url_nopad_impl);
