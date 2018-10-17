#![allow(unused_variables)]

extern crate proc_macro2;
extern crate syn;

mod error;
mod node;
mod punct;

pub use error::Error;
use node::Node;
use punct::{Space, Punctuation};
use punct::Space::*;

use proc_macro2::TokenStream;
use syn::punctuated::Punctuated;
use syn::visit::Visit;

use std::fmt::{self, Write};

/// Format a `TokenStream`
pub fn fmt(tts: TokenStream) -> Result<String, Error> {
    let file: syn::File = syn::parse2(tts)?;

    let mut visitor = FormatFile::new();
    visitor.visit_file(&file);

    // Ensure there is a trailing newline character
    if !visitor.out.is_empty() {
        let idx = visitor.out.len() - 1;

        if &visitor.out[idx..] != "\n" {
            visitor.out.push_str("\n");
        }
    }

    Ok(visitor.out)
}

struct FormatFile {
    /// Formatted code is appended to this buffer.
    out: String,

    /// Indentation level
    indent: usize,

    /// Set to true when visiting a statement item
    is_stmt_item: bool,
}

/// Number of spaces per indentation level
const DEFAULT_INDENT: usize = 4;

impl FormatFile {
    fn new() -> FormatFile {
        FormatFile {
            out: "".to_string(),
            indent: 0,
            is_stmt_item: false,
        }
    }

    fn visit_attributes(&mut self, i: &[syn::Attribute]) {
        for attr in i {
            self.visit_attribute(attr);
        }
    }

    fn visit_inner_attributes(&mut self, i: &[syn::Attribute]) {
        for attr in i {
            if is_inner_attr(attr) {
                self.visit_attribute(attr);
            }
        }
    }

    fn visit_outer_attributes(&mut self, i: &[syn::Attribute]) {
        for attr in i {
            if !is_inner_attr(attr) {
                self.visit_attribute(attr);
            }
        }
    }

    fn visit_doc_attribute(&mut self, i: &syn::Attribute) {
        use syn::Meta::*;
        use syn::Lit::*;

        let meta = i.parse_meta().unwrap();

        match meta {
            NameValue(name_value) => {
                match name_value.lit {
                    Str(s) => {
                        self.visit_doc_comment(is_inner_attr(i), &s.value());
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    fn visit_doc_comment(&mut self, is_inner: bool, value: &str) {
        if is_inner {
            write!(self, "//!{}\n", value);
        } else {
            write!(self, "///{}\n", value);
        }
    }

    fn visit_attribute_body(&mut self, i: &syn::Attribute) {
        let meta = i.parse_meta().unwrap();
        self.visit_meta(&meta);
    }

    fn visit_items(&mut self, i: &[syn::Item]) {
        for item in i {
            self.visit_item(item);
        }
    }

    fn visit_impl_items(&mut self, i: &[syn::ImplItem]) {
        for (pos, item) in i.iter().enumerate() {
            self.visit_impl_item(item);

            if pos + 1 < i.len() {
                write!(self, "\n");
            }
            write!(self, "\n");
        }
    }

    fn visit_macro_body(&mut self, tts: &TokenStream) {
        // TODO: How to do this?
        // Should it be parsed as `syn::Stmt`?
        write!(self, "{}", tts.to_string());
    }

    /// Returns `true` if where clause was present
    fn visit_where_clause_if_present(&mut self, generics: &syn::Generics) -> bool {
        if let Some(ref where_clause) = generics.where_clause {
            write!(self, "\n");
            self.visit_where_clause(where_clause);
            true
        } else {
            false
        }
    }

    // ===== Formatting helpers =====

    fn visit_punctuated<T, U>(&mut self, punctuated: &Punctuated<T, U>, space: Space)
    where
        T: Node,
        U: Punctuation,
    {
        for pair in punctuated.pairs() {
            pair.value().visit(self);

            if let Some(punct) = pair.punct() {
                let (l, r) = match space {
                    NewLine => {
                        ("", "\n")
                    }
                    SpaceBoth => {
                        (" ", " ")
                    }
                    SpaceRight => {
                        ("", " ")
                    }
                    NoSpace => {
                        ("", "")
                    }
                };

                write!(self, "{}{}{}", l, punct.as_str(), r);
            }
        }
    }

    fn block<F, R>(&mut self, f: F) -> R
    where F: FnOnce(&mut Self) -> R,
    {
        write!(self, "{{\n");
        let res = self.indent(f);
        write!(self, "}}\n");
        res
    }

    fn block_no_nl<F, R>(&mut self, f: F) -> R
    where F: FnOnce(&mut Self) -> R,
    {
        write!(self, "{{\n");
        let res = self.indent(f);
        write!(self, "}}");
        res
    }

    fn indent<F, R>(&mut self, f: F) -> R
    where F: FnOnce(&mut Self) -> R
    {
        self.indent += 1;
        let ret = f(self);
        self.indent -= 1;
        ret
    }

    fn is_start_of_line(&self) -> bool {
        self.out.is_empty() ||
            self.out.as_bytes().last() == Some(&b'\n')
    }

    fn push_spaces(&mut self) {
        for _ in 0..(self.indent * DEFAULT_INDENT) {
            self.out.push_str(" ");
        }
}
}

impl<'a> syn::visit::Visit<'a> for FormatFile {

    fn visit_abi(&mut self, i: &'a syn::Abi) {
        unimplemented!();
    }

    fn visit_angle_bracketed_generic_arguments(&mut self, i: &'a syn::AngleBracketedGenericArguments) {
        if i.colon2_token.is_some() {
            write!(self, "::");
        }

        write!(self, "<");
        self.visit_punctuated(&i.args, SpaceRight);
        write!(self, ">");
    }

    fn visit_arg_captured(&mut self, i: &'a syn::ArgCaptured) {
        self.visit_pat(&i.pat);
        write!(self, ": ");
        self.visit_type(&i.ty);
    }

    fn visit_arg_self(&mut self, i: &'a syn::ArgSelf) {
        if i.mutability.is_some() {
            write!(self, "mut ");
        }

        write!(self, "self");
    }

    fn visit_arg_self_ref(&mut self, i: &'a syn::ArgSelfRef) {
        write!(self, "&");

        if let Some(ref lt) = i.lifetime {
            self.visit_lifetime(lt);
        }

        if i.mutability.is_some() {
            if i.lifetime.is_some() {
                write!(self, " ");
            }

            write!(self, "mut ");
        }

        write!(self, "self");
    }

    fn visit_arm(&mut self, i: &'a syn::Arm) {
        self.visit_attributes(&i.attrs);

        if i.leading_vert.is_some() {
            write!(self, "| ");
        }

        self.visit_punctuated(&i.pats, SpaceBoth);

        write!(self, " => ");

        self.visit_expr(&i.body);

        if i.comma.is_some() {
            write!(self, ",");
        }

        write!(self, "\n");
    }

    fn visit_attr_style(&mut self, i: &'a syn::AttrStyle) {
        unimplemented!();
    }

    fn visit_attribute(&mut self, i: &'a syn::Attribute) {
        if is_doc_attr(&i) {
            self.visit_doc_attribute(i);
        } else if is_inner_attr(i) {
            write!(self, "#![");
            self.visit_attribute_body(i);
            write!(self, "]\n");
        } else {
            write!(self, "#[");
            self.visit_attribute_body(i);
            write!(self, "]\n");
        }
    }

    fn visit_bare_fn_arg(&mut self, i: &'a syn::BareFnArg) {
        unimplemented!();
    }

    fn visit_bare_fn_arg_name(&mut self, i: &'a syn::BareFnArgName) {
        unimplemented!();
    }

    fn visit_bin_op(&mut self, i: &'a syn::BinOp) {
        use syn::BinOp::*;

        write!(self, "{}", match *i {
            Add(_) => "+",
            Sub(_) => "-",
            Mul(_) => "*",
            Div(_) => "/",
            Rem(_) => "%",
            And(_) => "&&",
            Or(_) => "||",
            BitXor(_) => "^",
            BitAnd(_) => "&",
            BitOr(_) => "|",
            Shl(_) => "<<",
            Shr(_) => ">>",
            Eq(_) => "==",
            Lt(_) => "<",
            Le(_) => "<=",
            Ne(_) => "!=",
            Ge(_) => ">=",
            Gt(_) => ">",
            AddEq(_) => "+=",
            SubEq(_) => "-=",
            MulEq(_) => "*=",
            DivEq(_) => "/=",
            RemEq(_) => "%=",
            BitXorEq(_) => "^=",
            BitAndEq(_) => "&=",
            BitOrEq(_) => "|=",
            ShlEq(_) => "<<=",
            ShrEq(_) => ">>=",
        });
    }

    fn visit_binding(&mut self, i: &'a syn::Binding) {
        unimplemented!();
    }

    fn visit_block(&mut self, i: &'a syn::Block) {
        self.block_no_nl(|v| {
            v.is_stmt_item = false;

            for stmt in &i.stmts {
                v.visit_stmt(stmt);
            }
        });
    }

    fn visit_bound_lifetimes(&mut self, i: &'a syn::BoundLifetimes) {
        unimplemented!();
    }

    fn visit_const_param(&mut self, i: &'a syn::ConstParam) {
        unimplemented!();
    }

    fn visit_constraint(&mut self, i: &'a syn::Constraint) {
        unimplemented!();
    }

    fn visit_expr_array(&mut self, i: &'a syn::ExprArray) {
        unimplemented!();
    }

    fn visit_expr_assign(&mut self, i: &'a syn::ExprAssign) {
        unimplemented!();
    }

    fn visit_expr_assign_op(&mut self, i: &'a syn::ExprAssignOp) {
        unimplemented!();
    }

    fn visit_expr_async(&mut self, i: &'a syn::ExprAsync) {
        unimplemented!();
    }

    fn visit_expr_binary(&mut self, i: &'a syn::ExprBinary) {
        self.visit_attributes(&i.attrs);
        self.visit_expr(&i.left);
        write!(self, " ");
        self.visit_bin_op(&i.op);
        write!(self, " ");
        self.visit_expr(&i.right);
    }

    fn visit_expr_block(&mut self, i: &'a syn::ExprBlock) {
        self.visit_attributes(&i.attrs);
        assert!(i.label.is_none()); // unimplemented!();
        self.visit_block(&i.block);
    }

    fn visit_expr_box(&mut self, i: &'a syn::ExprBox) {
        unimplemented!();
    }

    fn visit_expr_break(&mut self, i: &'a syn::ExprBreak) {
        unimplemented!();
    }

    fn visit_expr_call(&mut self, i: &'a syn::ExprCall) {
        self.visit_attributes(&i.attrs);
        self.visit_expr(&i.func);
        write!(self, "(");
        self.visit_punctuated(&i.args, SpaceRight);
        write!(self, ")");
    }

    fn visit_expr_cast(&mut self, i: &'a syn::ExprCast) {
        unimplemented!();
    }

    fn visit_expr_closure(&mut self, i: &'a syn::ExprClosure) {
        unimplemented!();
    }


    fn visit_expr_continue(&mut self, i: &'a syn::ExprContinue) {
        unimplemented!();
    }

    fn visit_expr_field(&mut self, i: &'a syn::ExprField) {
        self.visit_attributes(&i.attrs);
        self.visit_expr(&i.base);
        write!(self, ".");
        self.visit_member(&i.member);
    }

    fn visit_expr_for_loop(&mut self, i: &'a syn::ExprForLoop) {
        self.visit_attributes(&i.attrs);
        if let Some(ref _label) = i.label {
            unimplemented!();
            // TODO: Where should the new line go?
            // self.visit_label(label);
        }
        write!(self, "for ");
        self.visit_pat(&i.pat);
        write!(self, " in ");
        self.visit_expr(&i.expr);
        write!(self, " ");
        self.visit_block(&i.body);
    }

    fn visit_expr_group(&mut self, i: &'a syn::ExprGroup) {
        unimplemented!();
    }

    fn visit_expr_if(&mut self, i: &'a syn::ExprIf) {
        self.visit_attributes(&i.attrs);
        write!(self, "if ");
        self.visit_expr(&i.cond);
        write!(self, " ");
        self.visit_block(&i.then_branch);

        if let Some((_, ref else_branch)) = i.else_branch {
            write!(self, " else ");
            self.visit_expr(else_branch);
        }
    }

    fn visit_expr_in_place(&mut self, i: &'a syn::ExprInPlace) {
        unimplemented!();
    }

    fn visit_expr_index(&mut self, i: &'a syn::ExprIndex) {
        unimplemented!();
    }

    fn visit_expr_let(&mut self, i: &'a syn::ExprLet) {
        unimplemented!();
    }

    fn visit_expr_loop(&mut self, i: &'a syn::ExprLoop) {
        unimplemented!();
    }

    fn visit_expr_macro(&mut self, i: &'a syn::ExprMacro) {
        self.visit_attributes(&i.attrs);
        self.visit_macro(&i.mac);
    }

    fn visit_expr_match(&mut self, i: &'a syn::ExprMatch) {
        self.visit_attributes(&i.attrs);
        write!(self, "match ");
        self.visit_expr(&i.expr);
        write!(self, " ");
        self.block_no_nl(|v| {
            for arm in &i.arms {
                v.visit_arm(arm);
            }
        });
    }

    fn visit_expr_method_call(&mut self, i: &'a syn::ExprMethodCall) {
        self.visit_attributes(&i.attrs);
        self.visit_expr(&i.receiver);
        write!(self, ".");
        self.visit_ident(&i.method);
        assert!(i.turbofish.is_none());
        write!(self, "(");
        self.visit_punctuated(&i.args, SpaceRight);
        write!(self, ")");
    }

    fn visit_expr_paren(&mut self, i: &'a syn::ExprParen) {
        unimplemented!();
    }

    fn visit_expr_path(&mut self, i: &'a syn::ExprPath) {
        self.visit_attributes(&i.attrs);
        assert!(i.qself.is_none()); // unimplemented
        self.visit_path(&i.path);
    }

    fn visit_expr_range(&mut self, i: &'a syn::ExprRange) {
        unimplemented!();
    }

    fn visit_expr_reference(&mut self, i: &'a syn::ExprReference) {
        self.visit_attributes(&i.attrs);
        write!(self, "&");

        if i.mutability.is_some() {
            write!(self, "mut ");
        }

        self.visit_expr(&i.expr);
    }

    fn visit_expr_repeat(&mut self, i: &'a syn::ExprRepeat) {
        unimplemented!();
    }

    fn visit_expr_return(&mut self, i: &'a syn::ExprReturn) {
        unimplemented!();
    }

    fn visit_expr_struct(&mut self, i: &'a syn::ExprStruct) {
        self.visit_attributes(&i.attrs);
        self.visit_path(&i.path);
        write!(self, " ");
        self.block_no_nl(|v| {
            v.visit_punctuated(&i.fields, NewLine);

            if let Some(_) = i.dot2_token {
                write!(v, ".. ");
                let rest = i.rest.as_ref().unwrap();
                v.visit_expr(&rest);
            }
        })
    }

    fn visit_expr_try(&mut self, i: &'a syn::ExprTry) {
        unimplemented!();
    }

    fn visit_expr_try_block(&mut self, i: &'a syn::ExprTryBlock) {
        unimplemented!();
    }

    fn visit_expr_tuple(&mut self, i: &'a syn::ExprTuple) {
        unimplemented!();
    }

    fn visit_expr_type(&mut self, i: &'a syn::ExprType) {
        unimplemented!();
    }

    fn visit_expr_unary(&mut self, i: &'a syn::ExprUnary) {
        self.visit_attributes(&i.attrs);
        self.visit_un_op(&i.op);
        self.visit_expr(&i.expr);
    }

    fn visit_expr_unsafe(&mut self, i: &'a syn::ExprUnsafe) {
        unimplemented!();
    }

    fn visit_expr_verbatim(&mut self, i: &'a syn::ExprVerbatim) {
        unimplemented!();
    }


    fn visit_expr_while(&mut self, i: &'a syn::ExprWhile) {
        unimplemented!();
    }


    fn visit_expr_yield(&mut self, i: &'a syn::ExprYield) {
        unimplemented!();
    }

    fn visit_field(&mut self, i: &'a syn::Field) {
        self.visit_attributes(&i.attrs);
        self.visit_visibility(&i.vis);

        // TODO: Handle;
        assert!(i.ident.is_some());
        assert!(i.colon_token.is_some());

        self.visit_ident(i.ident.as_ref().unwrap());

        write!(self, ": ");
        self.visit_type(&i.ty);
    }


    fn visit_field_pat(&mut self, i: &'a syn::FieldPat) {
        unimplemented!();
    }


    fn visit_field_value(&mut self, i: &'a syn::FieldValue) {
        use syn::Member::*;

        self.visit_attributes(&i.attrs);

        match i.member {
            Named(ref ident) => {
                self.visit_ident(ident);
                write!(self, ": ");
                self.visit_expr(&i.expr);

            }
            Unnamed(_) => {
                unimplemented!();
            }
        }
    }

    fn visit_fields_named(&mut self, i: &'a syn::FieldsNamed) {
        self.block(|v| {
            v.visit_punctuated(&i.named, NewLine);
        });
    }

    fn visit_fields_unnamed(&mut self, i: &'a syn::FieldsUnnamed) {
        unimplemented!();
    }

    fn visit_file(&mut self, i: &'a syn::File) {
        assert!(i.shebang.is_none()); // unimplemented
        self.visit_attributes(&i.attrs);
        self.visit_items(&i.items);
    }

    fn visit_fn_decl(&mut self, i: &'a syn::FnDecl) {
        self.visit_generics(&i.generics);

        write!(self, "(");

        self.visit_punctuated(&i.inputs, SpaceRight);
        assert!(i.variadic.is_none()); // unimplemented

        write!(self, ")");

        self.visit_return_type(&i.output);
        self.visit_where_clause_if_present(&i.generics);
    }

    fn visit_foreign_item(&mut self, i: &'a syn::ForeignItem) {
        unimplemented!();
    }

    fn visit_foreign_item_fn(&mut self, i: &'a syn::ForeignItemFn) {
        unimplemented!();
    }

    fn visit_foreign_item_macro(&mut self, i: &'a syn::ForeignItemMacro) {
        unimplemented!();
    }

    fn visit_foreign_item_static(&mut self, i: &'a syn::ForeignItemStatic) {
        unimplemented!();
    }

    fn visit_foreign_item_type(&mut self, i: &'a syn::ForeignItemType) {
        unimplemented!();
    }

    fn visit_foreign_item_verbatim(&mut self, i: &'a syn::ForeignItemVerbatim) {
        unimplemented!();
    }

    fn visit_generic_method_argument(&mut self, i: &'a syn::GenericMethodArgument) {
        unimplemented!();
    }

    fn visit_generics(&mut self, i: &'a syn::Generics) {
        if i.params.is_empty() {
            return;
        }

        write!(self, "<");
        self.visit_punctuated(&i.params, SpaceRight);
        write!(self, ">");

        // Where clauses are not visited here
    }

    fn visit_ident(&mut self, i: &'a syn::Ident) {
        write!(self, "{}", i);
    }

    /*
     * TODO: Reorganize order?
    fn visit_impl_item(&mut self, i: &'a syn::ImplItem) {
        unimplemented!();
    }
    */

    fn visit_impl_item_const(&mut self, i: &'a syn::ImplItemConst) {
        unimplemented!();
    }

    fn visit_impl_item_existential(&mut self, i: &'a syn::ImplItemExistential) {
        unimplemented!();
    }

    fn visit_impl_item_macro(&mut self, i: &'a syn::ImplItemMacro) {
        unimplemented!();
    }

    fn visit_impl_item_method(&mut self, i: &'a syn::ImplItemMethod) {
        assert!(i.defaultness.is_none()); // unimplemented

        self.visit_attributes(&i.attrs);
        self.visit_visibility(&i.vis);
        self.visit_method_sig(&i.sig);

        if i.sig.decl.generics.where_clause.is_some() {
        } else {
            write!(self, " ");
        }

        self.visit_block(&i.block);
    }

    fn visit_impl_item_type(&mut self, i: &'a syn::ImplItemType) {
        unimplemented!();
    }

    fn visit_impl_item_verbatim(&mut self, i: &'a syn::ImplItemVerbatim) {
        unimplemented!();
    }

    fn visit_index(&mut self, i: &'a syn::Index) {
        unimplemented!();
    }

    /*
    fn visit_item(&mut self, i: &'a syn::Item) {
        unimplemented!();
    }
    */

    fn visit_item_const(&mut self, i: &'a syn::ItemConst) {
        unimplemented!();
    }

    fn visit_item_enum(&mut self, i: &'a syn::ItemEnum) {
        unimplemented!();
    }

    fn visit_item_existential(&mut self, i: &'a syn::ItemExistential) {
        unimplemented!();
    }

    fn visit_item_extern_crate(&mut self, i: &'a syn::ItemExternCrate) {
        unimplemented!();
    }

    fn visit_item_fn(&mut self, i: &'a syn::ItemFn) {
        unimplemented!();
    }

    fn visit_item_foreign_mod(&mut self, i: &'a syn::ItemForeignMod) {
        unimplemented!();
    }

    fn visit_item_impl(&mut self, i: &'a syn::ItemImpl) {
        self.visit_outer_attributes(&i.attrs);

        assert!(i.defaultness.is_none()); // unimplemented

        if i.unsafety.is_some() {
            write!(self, "unsafe ");
        }

        write!(self, "impl");

        self.visit_generics(&i.generics);

        assert!(i.trait_.is_none()); // unimplemented

        write!(self, " ");

        self.visit_type(&i.self_ty);

        if !self.visit_where_clause_if_present(&i.generics) {
            write!(self, " ");
        }

        self.block(|v| {
            v.visit_inner_attributes(&i.attrs);
            v.visit_impl_items(&i.items);
        });
    }

    fn visit_item_macro(&mut self, i: &'a syn::ItemMacro) {
        unimplemented!();
    }

    fn visit_item_macro2(&mut self, i: &'a syn::ItemMacro2) {
        unimplemented!();
    }

    fn visit_item_mod(&mut self, i: &'a syn::ItemMod) {
        self.visit_outer_attributes(&i.attrs);
        self.visit_visibility(&i.vis);

        write!(self, "mod {}", i.ident);

        // TODO abstract?
        match i.content {
            Some((_, ref items)) => {
                write!(self, " ");
                self.block(|me| {
                    me.visit_items(items);
                });
            }
            _ => {
                write!(self, ";");
            }
        }
    }

    fn visit_item_static(&mut self, i: &'a syn::ItemStatic) {
        unimplemented!();
    }

    fn visit_item_struct(&mut self, i: &'a syn::ItemStruct) {
        use syn::Fields::*;

        self.visit_attributes(&i.attrs);
        self.visit_visibility(&i.vis);

        write!(self, "struct {}", i.ident);

        self.visit_generics(&i.generics);

        match i.fields {
            Named(ref fields_named) => {
                if !self.visit_where_clause_if_present(&i.generics) {
                    write!(self, " ");
                }

                self.visit_fields_named(fields_named);
            }
            Unnamed(ref fields_unnaamed) => {
                self.visit_fields_unnamed(fields_unnaamed);
                self.visit_where_clause_if_present(&i.generics);
            }
            Unit => {}
        }

        if i.semi_token.is_some() {
            write!(self, ";");
        }
    }

    fn visit_item_trait(&mut self, i: &'a syn::ItemTrait) {
        unimplemented!();
    }

    fn visit_item_trait_alias(&mut self, i: &'a syn::ItemTraitAlias) {
        unimplemented!();
    }

    fn visit_item_type(&mut self, i: &'a syn::ItemType) {
        unimplemented!();
    }

    fn visit_item_union(&mut self, i: &'a syn::ItemUnion) {
        unimplemented!();
    }

    fn visit_item_use(&mut self, i: &'a syn::ItemUse) {
        self.visit_attributes(&i.attrs);
        self.visit_visibility(&i.vis);

        write!(self, "use ");

        if i.leading_colon.is_some() {
            write!(self, "::");
        }

        self.visit_use_tree(&i.tree);

        write!(self, ";\n");
    }

    fn visit_item_verbatim(&mut self, i: &'a syn::ItemVerbatim) {
        unimplemented!();
    }

    fn visit_label(&mut self, i: &'a syn::Label) {
        unimplemented!();
    }

    fn visit_lifetime(&mut self, i: &'a syn::Lifetime) {
        write!(self, "'");
        self.visit_ident(&i.ident);
    }

    fn visit_lifetime_def(&mut self, i: &'a syn::LifetimeDef) {
        unimplemented!();
    }

    fn visit_lit_bool(&mut self, i: &'a syn::LitBool) {
        unimplemented!();
    }

    fn visit_lit_byte(&mut self, i: &'a syn::LitByte) {
        unimplemented!();
    }

    fn visit_lit_byte_str(&mut self, i: &'a syn::LitByteStr) {
        unimplemented!();
    }

    fn visit_lit_char(&mut self, i: &'a syn::LitChar) {
        unimplemented!();
    }

    fn visit_lit_float(&mut self, i: &'a syn::LitFloat) {
        unimplemented!();
    }

    fn visit_lit_int(&mut self, i: &'a syn::LitInt) {
        use syn::IntSuffix::*;

        write!(self, "{}{}", i.value(), match i.suffix() {
            I8 => "i8",
            I16 => "i16",
            I32 => "i32",
            I64 => "i64",
            I128 => "i128",
            Isize => "isize",
            U8 => "u8",
            U16 => "u16",
            U32 => "u32",
            U64 => "u64",
            U128 => "u128",
            Usize => "usize",
            None => "",
        });
    }

    fn visit_lit_str(&mut self, i: &'a syn::LitStr) {
        write!(self, "{:?}", i.value());
    }

    fn visit_lit_verbatim(&mut self, i: &'a syn::LitVerbatim) {
        unimplemented!();
    }


    fn visit_local(&mut self, i: &'a syn::Local) {
        self.visit_attributes(&i.attrs);
        write!(self, "let ");
        self.visit_punctuated(&i.pats, SpaceBoth);

        if let Some((_, ref ty)) = i.ty {
            write!(self, ": ");
            self.visit_type(ty);
        }

        if let Some((_, ref init)) = i.init {
            write!(self, " = ");
            self.visit_expr(init);
        }
    }

    fn visit_macro(&mut self, i: &'a syn::Macro) {
        use syn::MacroDelimiter::*;

        self.visit_path(&i.path);
        write!(self, "!");

        match i.delimiter {
            Paren(_) => {
                write!(self, "(");
                self.visit_macro_body(&i.tts);
                write!(self, ")");
            }
            Brace(_) => {
                write!(self, "[");
                self.visit_macro_body(&i.tts);
                write!(self, "]");
            }
            Bracket(_) => {
                self.block_no_nl(|v| {
                    v.visit_macro_body(&i.tts);
                });
            }
        }
    }

    fn visit_macro_delimiter(&mut self, i: &'a syn::MacroDelimiter) {
        unimplemented!();
    }

    fn visit_meta_list(&mut self, i: &'a syn::MetaList) {
        self.visit_ident(&i.ident);
        write!(self, "(");
        self.visit_punctuated(&i.nested, SpaceRight);
        write!(self, ")");
    }

    fn visit_meta_name_value(&mut self, i: &'a syn::MetaNameValue) {
        self.visit_ident(&i.ident);
        write!(self, " = ");
        self.visit_lit(&i.lit);
    }

    fn visit_method_sig(&mut self, i: &'a syn::MethodSig) {
        assert!(i.constness.is_none()); // unimplemented
        assert!(i.abi.is_none()); // unimplemented

        if i.unsafety.is_some() {
            write!(self, "unsafe ");
        }

        if i.asyncness.is_some() {
            write!(self, "async ");
        }

        write!(self, "fn ");

        self.visit_ident(&i.ident);
        self.visit_fn_decl(&i.decl);
    }

    fn visit_method_turbofish(&mut self, i: &'a syn::MethodTurbofish) {
        unimplemented!();
    }

    fn visit_parenthesized_generic_arguments(&mut self, i: &'a syn::ParenthesizedGenericArguments) {
        unimplemented!();
    }

    fn visit_pat_box(&mut self, i: &'a syn::PatBox) {
        unimplemented!();
    }

    fn visit_pat_ident(&mut self, i: &'a syn::PatIdent) {
        if i.by_ref.is_some() {
            write!(self, "ref ");
        }
        if i.mutability.is_some() {
            write!(self, "mut ");
        }
        self.visit_ident(&i.ident);
        assert!(i.subpat.is_none()); // unimplemented
    }

    fn visit_pat_lit(&mut self, i: &'a syn::PatLit) {
        unimplemented!();
    }

    fn visit_pat_macro(&mut self, i: &'a syn::PatMacro) {
        unimplemented!();
    }

    fn visit_pat_path(&mut self, i: &'a syn::PatPath) {
        unimplemented!();
    }

    fn visit_pat_range(&mut self, i: &'a syn::PatRange) {
        unimplemented!();
    }

    fn visit_pat_ref(&mut self, i: &'a syn::PatRef) {
        unimplemented!();
    }

    fn visit_pat_slice(&mut self, i: &'a syn::PatSlice) {
        unimplemented!();
    }

    fn visit_pat_struct(&mut self, i: &'a syn::PatStruct) {
        unimplemented!();
    }

    fn visit_pat_tuple(&mut self, i: &'a syn::PatTuple) {
        write!(self, "(");

        self.visit_punctuated(&i.front, SpaceRight);

        assert!(i.dot2_token.is_none()); // unimplemented
        assert!(i.comma_token.is_none()); // unimplemented
        assert!(i.back.is_empty()); // unimplemented
        write!(self, ")");
    }

    fn visit_pat_tuple_struct(&mut self, i: &'a syn::PatTupleStruct) {
        self.visit_path(&i.path);
        self.visit_pat_tuple(&i.pat);
    }

    fn visit_pat_verbatim(&mut self, i: &'a syn::PatVerbatim) {
        unimplemented!();
    }

    fn visit_pat_wild(&mut self, i: &'a syn::PatWild) {
        write!(self, "_");
    }

    fn visit_path(&mut self, i: &'a syn::Path) {
        if i.leading_colon.is_some() {
            write!(self, "::");
        }

        self.visit_punctuated(&i.segments, NoSpace);
    }

    fn visit_predicate_eq(&mut self, i: &'a syn::PredicateEq) {
        unimplemented!();
    }

    fn visit_predicate_lifetime(&mut self, i: &'a syn::PredicateLifetime) {
        unimplemented!();
    }

    fn visit_predicate_type(&mut self, i: &'a syn::PredicateType) {
        assert!(i.lifetimes.is_none(), "{:?}", i.lifetimes); // unimplemented!();
        self.visit_type(&i.bounded_ty);
        write!(self, ": ");
        self.visit_punctuated(&i.bounds, SpaceBoth);
    }

    fn visit_qself(&mut self, i: &'a syn::QSelf) {
        unimplemented!();
    }

    fn visit_range_limits(&mut self, i: &'a syn::RangeLimits) {
        unimplemented!();
    }

    fn visit_return_type(&mut self, i: &'a syn::ReturnType) {
        use syn::ReturnType::*;

        match *i {
            Type(_, ref ty) => {
                write!(self, " -> ");
                self.visit_type(ty);
            }
            _ => {}
        }
    }

    fn visit_span(&mut self, i: &'a proc_macro2::Span) {
        unimplemented!();
    }

    fn visit_stmt(&mut self, i: &'a syn::Stmt) {
        use syn::Stmt::*;

        // Some custom spacing
        match *i {
            Item(_) => {
                self.is_stmt_item = true;
            }
            _ => {
                if self.is_stmt_item {
                    write!(self, "\n");
                }

                self.is_stmt_item = false;
            }
        }

        match *i {
            Local(ref v) => {
                self.visit_local(v);
                write!(self, ";\n");
            }
            Item(ref v) => {
                self.visit_item(v);
            }
            Expr(ref v) => {
                self.visit_expr(v);
                write!(self, "\n");
            }
            Semi(ref v, _) => {
                self.visit_expr(v);
                write!(self, ";\n");
            }
        }
    }

    fn visit_trait_bound(&mut self, i: &'a syn::TraitBound) {
        assert!(i.paren_token.is_none()); // unimplemented
        assert!(i.lifetimes.is_none());

        self.visit_trait_bound_modifier(&i.modifier);
        self.visit_path(&i.path);
    }

    fn visit_trait_item(&mut self, i: &'a syn::TraitItem) {
        unimplemented!();
    }

    fn visit_trait_item_const(&mut self, i: &'a syn::TraitItemConst) {
        unimplemented!();
    }

    fn visit_trait_item_macro(&mut self, i: &'a syn::TraitItemMacro) {
        unimplemented!();
    }

    fn visit_trait_item_method(&mut self, i: &'a syn::TraitItemMethod) {
        unimplemented!();
    }

    fn visit_trait_item_type(&mut self, i: &'a syn::TraitItemType) {
        unimplemented!();
    }

    fn visit_trait_item_verbatim(&mut self, i: &'a syn::TraitItemVerbatim) {
        unimplemented!();
    }

    fn visit_type_array(&mut self, i: &'a syn::TypeArray) {
        write!(self, "[");
        self.visit_type(&i.elem);
        write!(self, "; ");
        self.visit_expr(&i.len);
        write!(self, "]");
    }

    fn visit_type_bare_fn(&mut self, i: &'a syn::TypeBareFn) {
        unimplemented!();
    }

    fn visit_type_group(&mut self, i: &'a syn::TypeGroup) {
        unimplemented!();
    }

    fn visit_type_impl_trait(&mut self, i: &'a syn::TypeImplTrait) {
        unimplemented!();
    }

    fn visit_type_infer(&mut self, i: &'a syn::TypeInfer) {
        unimplemented!();
    }

    fn visit_type_macro(&mut self, i: &'a syn::TypeMacro) {
        unimplemented!();
    }

    fn visit_type_never(&mut self, i: &'a syn::TypeNever) {
        unimplemented!();
    }

    fn visit_type_param(&mut self, i: &'a syn::TypeParam) {
        // This will need special handling
        assert!(i.attrs.is_empty());

        self.visit_ident(&i.ident);

        if i.colon_token.is_none() {
            assert!(i.bounds.is_empty());
        } else {
            write!(self, ": ");
            self.visit_punctuated(&i.bounds, SpaceBoth);
        }

        if let Some(ref ty) = i.default {
            assert!(i.eq_token.is_some());
            write!(self, " = ");
            self.visit_type(ty);
        }
    }

    fn visit_type_paren(&mut self, i: &'a syn::TypeParen) {
        unimplemented!();
    }

    fn visit_type_path(&mut self, i: &'a syn::TypePath) {
        assert!(i.qself.is_none()); // TODO: handle
        self.visit_path(&i.path);
    }

    fn visit_type_ptr(&mut self, i: &'a syn::TypePtr) {
        unimplemented!();
    }

    fn visit_type_reference(&mut self, i: &'a syn::TypeReference) {
        write!(self, "&");

        if let Some(ref lt) = i.lifetime {
            self.visit_lifetime(lt);
        }

        if i.mutability.is_some() {
            if i.lifetime.is_some() {
                write!(self, " ");
            }

            write!(self, "mut ");
        }

        self.visit_type(&i.elem);
    }

    fn visit_type_slice(&mut self, i: &'a syn::TypeSlice) {
        write!(self, "[");
        self.visit_type(&i.elem);
        write!(self, "]");
    }

    fn visit_type_trait_object(&mut self, i: &'a syn::TypeTraitObject) {
        unimplemented!();
    }

    fn visit_type_tuple(&mut self, i: &'a syn::TypeTuple) {
        write!(self, "(");
        self.visit_punctuated(&i.elems, SpaceRight);
        write!(self, ")");
    }

    fn visit_type_verbatim(&mut self, i: &'a syn::TypeVerbatim) {
        unimplemented!();
    }

    fn visit_un_op(&mut self, i: &'a syn::UnOp) {
        use syn::UnOp::*;

        match *i {
            Deref(_) => {
                write!(self, "*");
            }
            Not(_) => {
                write!(self, "!");
            }
            Neg(_) => {
                write!(self, "-");
            }
        }
    }

    fn visit_use_glob(&mut self, i: &'a syn::UseGlob) {
        write!(self, "*");
    }

    fn visit_use_group(&mut self, i: &'a syn::UseGroup) {
        unimplemented!();
    }

    fn visit_use_path(&mut self, i: &'a syn::UsePath) {
        self.visit_ident(&i.ident);
        write!(self, "::");
        self.visit_use_tree(&i.tree);
    }

    fn visit_use_rename(&mut self, i: &'a syn::UseRename) {
        unimplemented!();
    }

    fn visit_variant(&mut self, i: &'a syn::Variant) {
        unimplemented!();
    }

    fn visit_vis_crate(&mut self, i: &'a syn::VisCrate) {
        unimplemented!();
    }

    fn visit_vis_public(&mut self, i: &'a syn::VisPublic) {
        unimplemented!();
    }

    fn visit_vis_restricted(&mut self, i: &'a syn::VisRestricted) {
        unimplemented!();
    }

    fn visit_visibility(&mut self, i: &'a syn::Visibility) {
        use syn::Visibility::*;

        match i {
            Public(_) => {
                write!(self, "pub ");
            }
            Inherited => {}
            actual => {
                unimplemented!("{:?}", actual);
            }
        }
    }

    fn visit_where_clause(&mut self, i: &'a syn::WhereClause) {
        write!(self, "where\n");

        self.indent(|v| {
            v.visit_punctuated(&i.predicates, NewLine);
        });
    }
}

fn is_inner_attr(attr: &syn::Attribute) -> bool {
    use syn::AttrStyle::*;

    match attr.style {
        Inner(_) => true,
        _ => false,
    }
}

fn is_doc_attr(attr: &syn::Attribute) -> bool {
    attr.path.segments.len() == 1 &&
        attr.path.segments[0].ident == "doc"
}

impl Write for FormatFile {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut first = true;
        let mut should_indent = self.is_start_of_line();

        for line in s.lines() {
            if !first {
                self.out.push_str("\n");
            }

            first = false;

            let do_indent = should_indent &&
                !line.is_empty() &&
                line.as_bytes()[0] != b'\n';

            if do_indent {
                self.push_spaces();
            }

            // If this loops again, then we just wrote a new line
            should_indent = true;

            self.out.push_str(line);
        }

        if s.as_bytes().last() == Some(&b'\n') {
            self.out.push_str("\n");
        }

        Ok(())
    }
}
