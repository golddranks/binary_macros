extern crate data_encoding;
extern crate dotenv;
extern crate proc_macro;
extern crate proc_macro_hack;

use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use std::io::Read;
use std::str::FromStr;

fn helper(input: TokenStream, decoder: data_encoding::Encoding) -> TokenStream {
    let input = input.to_string();
    let input = input.trim_matches('"');

    let byte_vec = if input.starts_with("file:") {
        let mut file = std::fs::File::open(&input[5..]).expect("Error opening file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Error reading file");
        decoder
            .decode(contents.trim().as_bytes())
            .expect("Parse error")
    } else if input.starts_with("env:") {
        dotenv::dotenv().ok();
        let var = std::env::var(&input[4..]).expect("Error reading environment variable");
        decoder.decode(var.as_bytes()).expect("Parse error")
    } else {
        decoder.decode(input.as_bytes()).expect("Parse error")
    };

    TokenStream::from_str(&format!("{{static _BIN: [u8; {}] = {:?}; &_BIN}}", byte_vec.len(), byte_vec)).expect("Parse error")
}

#[proc_macro_hack]
pub fn base16(input: TokenStream) -> TokenStream {
    helper(input, data_encoding::HEXLOWER)
}

#[proc_macro_hack]
pub fn base32hex(input: TokenStream) -> TokenStream {
    helper(input, data_encoding::BASE32HEX)
}

#[proc_macro_hack]
pub fn base32(input: TokenStream) -> TokenStream {
    helper(input, data_encoding::BASE32)
}

#[proc_macro_hack]
pub fn base64(input: TokenStream) -> TokenStream {
    helper(input, data_encoding::BASE64)
}

#[proc_macro_hack]
pub fn base64url(input: TokenStream) -> TokenStream {
    helper(input, data_encoding::BASE64URL)
}

#[proc_macro_hack]
pub fn base32hex_nopad(input: TokenStream) -> TokenStream {
    helper(input, data_encoding::BASE32HEX_NOPAD)
}

#[proc_macro_hack]
pub fn base32_nopad(input: TokenStream) -> TokenStream {
    helper(input, data_encoding::BASE32_NOPAD)
}

#[proc_macro_hack]
pub fn base64_nopad(input: TokenStream) -> TokenStream {
    helper(input, data_encoding::BASE64_NOPAD)
}

#[proc_macro_hack]
pub fn base64url_nopad(input: TokenStream) -> TokenStream {
    helper(input, data_encoding::BASE64URL_NOPAD)
}
