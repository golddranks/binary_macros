# binary_macros
Rust macros for decoding base64 and hexadecimal -like encodings from string literals to [u8] literals at compile-time.

Version 0.6 is intended to have stable interface. Releasing 1.0 later. Tested and working on stable Rust. The first supported version is 1.15.0. Bug reports, pull requests etc. welcome!

Why are these macros useful? Let's say you want to include a binary blob inside your crate, a public key, for example. You can do that with the `include_bytes!()` macro from the Rust `std`. However, editing, viewing and copy-pasting raw binary blobs is hard! There is a reason public keys are often distributed as base64. On the other hand, if you include text with the `include_str!()` macro, you'll have to decode it runtime. Why defer it to runtime if you can do it compile-time?

To get started, include this in your Cargo.toml dependencies:

```
[dependencies]
binary_macros = "1.0"
```
Your source code:
```
#[macro_use]
extern crate binary_macros;
```
...and then you are ready to use the macros!
```
let public_key = base64!("aeSwwNywhbrmSuk32vuZmQRWHOKXbU1LziU18GAxVOE=");
```
This crate also supports prefixing the input with `file:` or `env:` to load input from file (path relative to current working directory) or environment variable. The `env:` prefix supports also `.env` files. (Check out [rust-dotenv](https://github.com/slapresta/rust-dotenv))
```
let public_key_a = base64!("file:id_rsa.pub");
let public_key_b = base64!("env:MYCRATE_PUBLIC_KEY");
``` 

## Included macros:
Number 97 (ASCII 'a') included with different encodings for example:
```
base2!("01100001") // Binary. Uses numbers 0-1. Group of 8 digits = 1 byte.
base4!("1201") // Base4. Uses numbers 0-3. Group of 4 digits = 1 byte.
base8!("302=====") // Octal. Uses numbers 0-7. Group of 8 digits = 3 bytes, uses = as end padding.
base16!("61") // Hexadecimal. Uses numbers 0-7. Group of 2 digits = 1 byte.
base32!("C4======") // Base32. Uses numbers A-Z and 2-7. Group of 8 digits = 5 bytes, uses = as end padding.
base32hex!("ME======") // Base32 that uses extended hexadecimal: 0-9 and A-V. Group of 8 digits = 5 bytes, uses = as end padding.
base64!("YQ==") // Base64. Uses numbers A-Z, a-z, 0-9, + and /. Group of 4 digits = 3 bytes, uses = as end padding.
base64url!("_A==") // URL-compatible Base64. Uses numbers A-Z, a-z, 0-9, - and _. Group of 4 digits = 3 bytes, uses = as end padding.
base2_nopad!("01100001") // No padding version of base2.
base4_nopad!("1201") // No padding version of base4.
base8_nopad!("302") // No padding version of base8.
base16_nopad!("61") // No padding version of base16.
base32_nopad!("C4") // No padding version of base32.
base32hex_nopad!("ME") // No padding version of base32hex.
base64_nopad!("YQ==") // No padding version of base64.
base64url_nopad!("_A") // No padding version of base64url.
```


Huge kudos to the [data-encoding](https://github.com/ia0/data-encoding) crate for providing a wide variety of encodings!
