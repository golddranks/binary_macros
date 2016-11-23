#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;
extern crate data_encoding;

use syntax::symbol::Symbol;
use syntax::ast::LitKind::ByteStr;
use syntax::ext::base;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;
use syntax::codemap::Span;
use syntax::tokenstream::TokenTree;

fn expand(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree])
        -> Result<Symbol, Box<MacResult + 'static>> {

    // Expand the insides of the macro first
    let mut exprs = match base::get_exprs_from_tts(cx, sp, tts) {
        Some(exprs) => exprs.into_iter(),
        None => return Err(DummyResult::any(sp)),
    };

    if exprs.len() != 1 {
        cx.span_err(sp, &format!("argument should be a single identifier, but got {} arguments", exprs.len()));
        return Err(DummyResult::any(sp));
    }

    match base::expr_to_string(cx, exprs.next().unwrap(), "expected string literal") {
        Some((interned_str, _)) => Ok(interned_str),
        None => Err(DummyResult::any(sp)),
    }
}


fn build_ast(cx: &mut ExtCtxt, sp: Span, result : Result<Vec<u8>, String>) -> Box<MacResult + 'static> {

    let binary_literal = match result {
        Ok(b) => b,
        Err(e) => {
            cx.span_err(sp, &*e);
            return DummyResult::any(sp)
        },
    };

    MacEager::expr(cx.expr_lit(sp, ByteStr(std::rc::Rc::new(binary_literal))))
}


fn base2(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'static> {
     match expand(cx, sp, tts) {
        Ok(input_string) => {
            let decoded_result = data_encoding::base2::decode(input_string.as_str().trim().as_bytes())
                .map_err(|e| format!("argument should be well-formed Base2 string literal (i.e. binary, with numbers 0 and 1), but it wasn't: {:?}", e) );
            build_ast(cx, sp, decoded_result)
        },
        Err(e) => e,
    }
}

fn base4(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'static> {
     match expand(cx, sp, tts) {
        Ok(input_string) => {
            let decoded_result = data_encoding::base4::decode(input_string.as_str().trim().as_bytes())
                .map_err(|e| format!("argument should be well-formed Base4 string literal (i.e. quaternary, with numbers 0-3), but it wasn't: {:?}", e) );
            build_ast(cx, sp, decoded_result)
        },
        Err(e) => e,
    }
}

fn base8(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'static> {
     match expand(cx, sp, tts) {
        Ok(input_string) => {
            let decoded_result = data_encoding::base8::decode(input_string.as_str().trim().as_bytes())
                .map_err(|e| format!("argument should be well-formed Base8 string literal (i.e. octal, with numbers 0-7, using = as end padding), but it wasn't: {:?}", e) );
            build_ast(cx, sp, decoded_result)
        },
        Err(e) => e,
    }
}

fn base16(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'static> {
     match expand(cx, sp, tts) {
        Ok(input_string) => {
            let decoded_result = data_encoding::base16::decode(input_string.as_str().trim().as_bytes())
                .map_err(|e| format!("argument should be well-formed Base16 string literal (i.e. hexadecimal, with numbers 0-9, A-F), but it wasn't: {:?}", e) );
            build_ast(cx, sp, decoded_result)
        },
        Err(e) => e,
    }
}

fn base32(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'static> {
     match expand(cx, sp, tts) {
        Ok(input_string) => {
            let decoded_result = data_encoding::base32::decode(input_string.as_str().trim().as_bytes())
                .map_err(|e| format!("argument should be well-formed Base32 string literal (with numbers A-Z, 2-7, using = as end padding), but it wasn't: {:?}", e) );
            build_ast(cx, sp, decoded_result)
        },
        Err(e) => e,
    }
}

fn base32hex(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'static> {
     match expand(cx, sp, tts) {
        Ok(input_string) => {
            let decoded_result = data_encoding::base32hex::decode(input_string.as_str().trim().as_bytes())
                .map_err(|e| format!("argument should be well-formed Base32-hex string literal (with numbers 0-9, A-V, using = as end padding), but it wasn't: {:?}", e) );
            build_ast(cx, sp, decoded_result)
        },
        Err(e) => e,
    }
}

fn base64(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'static> {
     match expand(cx, sp, tts) {
        Ok(input_string) => {
            let decoded_result = data_encoding::base64::decode(input_string.as_str().trim().as_bytes())
                .map_err(|e|format!("argument should be well-formed Base64 string literal (with numbers A-Z, a-z, 0-9, + and /, using = as end padding), but it wasn't: {:?}", e) );
            build_ast(cx, sp, decoded_result)
        },
        Err(e) => e,
    }
}

fn base64url(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'static> {
     match expand(cx, sp, tts) {
        Ok(input_string) => {
            let decoded_result = data_encoding::base64url::decode(input_string.as_str().trim().as_bytes())
                .map_err(|e|format!("argument should be well-formed url-compatible Base64 string literal (with numbers A-Z, a-z, 0-9, - and _, using = as end padding), but it wasn't: {:?}", e) );
            build_ast(cx, sp, decoded_result)
        },
        Err(e) => e,
    }
}


#[plugin_registrar]
pub fn plugin_registrar(reg: &mut rustc_plugin::Registry) {
    reg.register_macro("base2",  base2);
    reg.register_macro("base4",  base4);
    reg.register_macro("base8",  base8);
    reg.register_macro("base16", base16);
    reg.register_macro("base32", base32);
    reg.register_macro("base32hex", base32hex);
    reg.register_macro("base64", base64);
    reg.register_macro("base64url", base64url);
}
