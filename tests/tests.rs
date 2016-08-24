#![feature(plugin)]

#![plugin(binary_macros)]

#[test]
fn binary_works() {
    assert_eq!(base2!("01100001"), b"a");
}

#[test]
fn base4_works() {
    assert_eq!(base4!("1201"), b"a");
}

#[test]
fn octal_works() {
    assert_eq!(base8!("302====="), b"a");
}

#[test]
fn hex_works() {
    assert_eq!(base16!("61"), b"a");
}

#[test]
fn base32hex_works() {
    assert_eq!(base32hex!("C4======"), b"a");
}

#[test]
fn base32_works() {
    assert_eq!(base32!("ME======"), b"a");
}

#[test]
fn base64_works() {
    assert_eq!(base64!("YQ=="), b"a");
}

#[test]
fn base64url_works() {
    assert_eq!(base64url!("_A=="), b"\xfc");
}
