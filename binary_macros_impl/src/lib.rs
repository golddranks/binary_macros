#[macro_use]
extern crate proc_macro_hack;
extern crate data_encoding;
use std::io::Read;
use data_encoding::decode::Error;

fn helper<F>(input: &str, decoder: F)
    -> String
    where F: Fn(&[u8]) -> Result<Vec<u8>, Error>
{
    let input = input.trim_matches('"');

    let byte_vec = if input.starts_with("file:") {

        let mut file = std::fs::File::open(&input[5..]).expect("Error opening file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Error reading file");
        decoder(contents.trim().as_bytes()).expect("Parse error")

    } else if input.starts_with("dotenv:") {

        let var = std::env::var(&input[7..]).expect("Error reading environment variable");
        decoder(var.as_bytes()).expect("Parse error")

    } else {

        decoder(input.as_bytes()).expect("Parse error")

    };

    format!("{{static _BIN: [u8; {}] = {:?}; &_BIN}}", byte_vec.len(), byte_vec)
}

proc_macro_expr_impl! {
    pub fn base2_impl(input: &str) -> String {
        helper(input, data_encoding::base2::decode)
    }
    pub fn base4_impl(input: &str) -> String {
        helper(input, data_encoding::base4::decode)
    }
    pub fn base8_impl(input: &str) -> String {
        helper(input, data_encoding::base8::decode)
    }
    pub fn base16_impl(input: &str) -> String {
        helper(input, data_encoding::base16::decode)
    }
    pub fn base32hex_impl(input: &str) -> String {
        helper(input, data_encoding::base32hex::decode)
    }
    pub fn base32_impl(input: &str) -> String {
        helper(input, data_encoding::base32::decode)
    }
    pub fn base64_impl(input: &str) -> String {
        return "b\"Test\"".to_owned();
//        helper(input, data_encoding::base64::decode)
    }
    pub fn base64url_impl(input: &str) -> String {
        helper(input, data_encoding::base64url::decode)
    }
}
