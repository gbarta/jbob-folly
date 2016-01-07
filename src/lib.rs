#![feature(plugin_registrar, rustc_private, custom_attribute)]

extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::Registry;

use syntax::ext::base::{Annotatable, ExtCtxt, MultiDecorator};
use syntax::codemap::Span;
use syntax::ast::MetaItem;
use syntax::parse::token::intern;

pub fn jbob_expander(ecx: &mut ExtCtxt, sp: Span, meta_item: &MetaItem, item: &Annotatable, push: &mut FnMut(Annotatable)) {
    ecx.span_warn(sp,&format!("COMPILE TIMEEEE, {:#?}\n{:#?}\n{:#?}", sp, meta_item, item));
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(intern("jbob_expander"), MultiDecorator(Box::new(jbob_expander)));
}
