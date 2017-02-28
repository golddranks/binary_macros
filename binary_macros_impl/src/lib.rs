#[macro_use]
extern crate proc_macro_hack;

extern crate data_encoding;

proc_macro_expr_impl! {
    pub fn base64_impl(input: &str) -> String {
        let byte_vec = data_encoding::base64::decode(input.as_bytes()).expect("Parse error!");
        format!("b[{:?}]", byte_vec)
    }
    pub fn base64url_impl(input: &str) -> String {
        let byte_vec = data_encoding::base64url::decode(input.as_bytes()).expect("Parse error!");
        format!("b[{:?}]", byte_vec)
    }
}
