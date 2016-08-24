# binary_macros
Rust macros for decoding binary-to-text encodings in string literals to [u8] literals.

This library decodes string literals that may contain various binary-to-text encodings, such as Base64, to binary literals at compile-time.

Why is this useful? Let's say you want to include a binary blob inside your crate. You can do that with `std::include_bytes!()`.
However, if you want your data to be easily editable, copy-pasteable et cetera (for including a Base64-encoded public key, for example),
a binary blob is going to be is a bit icky, and if you include text with `std::include_str!()`, you'll have to decode it runtime.

The macros from this crate support expanding macros inside them, and then decoding the result so can easily do the job compile-time:

```
#![feature(plugin)]
#![plugin(binary_macros)]

let public_key = base64!(include_str!());
``` 

You might also want to pull data from environmental variables:

```
#![feature(plugin)]
#![plugin(binary_macros, dotenv_macros)]

let public_key = binary_macros::base64!(dotenv!("MYCRATE_PUBLIC_KEY"));
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
}
```


Huge kudos to the [data-encoding](https://github.com/ia0/data-encoding) crate for providing a wide variety of encodings!
