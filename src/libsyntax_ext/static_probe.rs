// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use syntax::ast;
use syntax::ext::base;
use syntax::ext::base::{ExtCtxt, MacEager};
use syntax::ext::build::AstBuilder;
use syntax::ext::hygiene::SyntaxContext;
//use syntax::feature_gate;
use syntax::symbol::Symbol;
use syntax_pos::Span;
use syntax::ptr::P;
use syntax::tokenstream::TokenTree;

pub fn expand_static_probe(cx: &mut ExtCtxt,
                           sp: Span,
                           _tt: &[TokenTree])
                           -> Box<dyn base::MacResult + 'static> {
    /*if !cx.ecfg.enable_trace_macros() {
        feature_gate::emit_feature_err(&cx.parse_sess,
                                       "static_probe",
                                       sp,
                                       feature_gate::GateIssue::Language,
                                       feature_gate::EXPLAIN_TRACE_MACROS);
        return base::DummyResult::any(sp);
    }*/
    let asm_text = "#probeasm";
    let stmt = ast::Stmt{
        id: ast::DUMMY_NODE_ID,
        node: ast::StmtKind::Expr(P(ast::Expr{
            id: ast::DUMMY_NODE_ID,
            span: sp,
            attrs: vec![].into(),
            node: ast::ExprKind::InlineAsm(P(ast::InlineAsm{
                asm: Symbol::intern(&asm_text),
                asm_str_style: ast::StrStyle::Cooked,
                outputs: vec![],
                inputs: vec![], //asm_expressions,
                clobbers: vec![],
                volatile: true,
                alignstack: false,
                dialect: ast::AsmDialect::Att,
                ctxt: SyntaxContext::empty(),
            }))
        })),
        span: sp
    };
    let block = P(ast::Block {
       stmts: vec![stmt],
       id: ast::DUMMY_NODE_ID,
       rules: ast::BlockCheckMode::Unsafe(ast::UnsafeSource::UserProvided),
       span: sp,
       recovered: false,
    });
    MacEager::expr(cx.expr_block(block))


    //base::DummyResult::any(sp)
}
