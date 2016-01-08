#![feature(quote, plugin_registrar, rustc_private, custom_attribute)]

extern crate rustc;
extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::Registry;

use syntax::ext::base::{Annotatable, ExtCtxt, MultiDecorator};
use syntax::ext::base::{MacResult, DummyResult, MacEager};
use syntax::codemap;
use syntax::codemap::Span;
use syntax::ast::MetaItem;
use syntax::ast::{TokenTree};
use syntax::parse::token::intern;
use syntax::util::small_vector::SmallVector;

pub fn jbob_trace_se(ecx: &mut ExtCtxt, sp: Span, meta_item: &MetaItem, item: &Annotatable, push: &mut FnMut(Annotatable)) {
    ecx.span_warn(sp,&format!("jbob_trace_se:\n {:#?}\n{:#?}\n{:#?}", sp, meta_item, item));
}

fn jbob_trace_macro(ecx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
        -> Box<MacResult + 'static> {
    let mut parser = ecx.new_parser_from_tts(args);
    let item = parser.parse_item().unwrap().unwrap();
    let msg = &format!("jbob_trace_macro:\n sp={:#?}\nargs={:#?}\nitem={:#?}", sp, args,item);
    ecx.span_warn(sp,msg);
    MacEager::items(SmallVector::one(item))
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(intern("jbob_trace_se"), MultiDecorator(Box::new(jbob_trace_se)));
    reg.register_macro("jbob_trace_macro",jbob_trace_macro);
}
