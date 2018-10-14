extern crate proc_macro2;
extern crate syn;

mod error;

pub use error::Error;

use proc_macro2::TokenStream;
use syn::visit::{Visit, visit_file};

use std::fmt::Write;

/// Format a `TokenStream` representing a Rust file
pub fn fmt_file(tts: TokenStream) -> Result<String, Error> {
    let file: syn::File = syn::parse2(tts)?;

    let mut visitor = FormatFile::new();
    visit_file(&mut visitor, &file);

    Ok(visitor.out)
}

struct FormatFile {
    out: String,
}

impl FormatFile {
    fn new() -> FormatFile {
        FormatFile { out: "".to_string() }
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
            write!(self.out, "//!{}\n", value);
        } else {
            write!(self.out, "///{}\n", value);
        }
    }

    fn visit_attribute_body(&mut self, i: &syn::Attribute) {
        unimplemented!();
        /*
    pub path: Path,
    pub tts: TokenStream,
         */
    }

    fn visit_items(&mut self, i: &[syn::Item]) {
        for item in i {
            self.visit_item(item);
        }
    }
}

impl<'a> syn::visit::Visit<'a> for FormatFile {

    fn visit_abi(&mut self, i: &'a syn::Abi) {
        unimplemented!();
    }

    fn visit_angle_bracketed_generic_arguments(&mut self, i: &'a syn::AngleBracketedGenericArguments) {
        unimplemented!();
    }

    fn visit_arg_captured(&mut self, i: &'a syn::ArgCaptured) {
        unimplemented!();
    }

    fn visit_arg_self(&mut self, i: &'a syn::ArgSelf) {
        unimplemented!();
    }

    fn visit_arg_self_ref(&mut self, i: &'a syn::ArgSelfRef) {
        unimplemented!();
    }

    fn visit_arm(&mut self, i: &'a syn::Arm) {
        unimplemented!();
    }

    fn visit_attr_style(&mut self, i: &'a syn::AttrStyle) {
        unimplemented!();
    }

    fn visit_attribute(&mut self, i: &'a syn::Attribute) {
        if is_doc_attr(&i) {
            self.visit_doc_attribute(i);
        } else if is_inner_attr(i) {
            write!(self.out, "#![");
            self.visit_attribute_body(i);
            write!(self.out, "]\n");
        } else {
            write!(self.out, "#[");
            self.visit_attribute_body(i);
            write!(self.out, "]\n");
        }
    }

    fn visit_bare_fn_arg(&mut self, i: &'a syn::BareFnArg) {
        unimplemented!();
    }

    fn visit_bare_fn_arg_name(&mut self, i: &'a syn::BareFnArgName) {
        unimplemented!();
    }

    fn visit_bin_op(&mut self, i: &'a syn::BinOp) {
        unimplemented!();
    }

    fn visit_binding(&mut self, i: &'a syn::Binding) {
        unimplemented!();
    }

    fn visit_block(&mut self, i: &'a syn::Block) {
        unimplemented!();
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

    fn visit_expr(&mut self, i: &'a syn::Expr) {
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
        unimplemented!();
    }

    fn visit_expr_block(&mut self, i: &'a syn::ExprBlock) {
        unimplemented!();
    }

    fn visit_expr_box(&mut self, i: &'a syn::ExprBox) {
        unimplemented!();
    }

    fn visit_expr_break(&mut self, i: &'a syn::ExprBreak) {
        unimplemented!();
    }

    fn visit_expr_call(&mut self, i: &'a syn::ExprCall) {
        unimplemented!();
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
        unimplemented!();
    }


    fn visit_expr_for_loop(&mut self, i: &'a syn::ExprForLoop) {
        unimplemented!();
    }


    fn visit_expr_group(&mut self, i: &'a syn::ExprGroup) {
        unimplemented!();
    }


    fn visit_expr_if(&mut self, i: &'a syn::ExprIf) {
        unimplemented!();
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

    fn visit_expr_lit(&mut self, i: &'a syn::ExprLit) {
        unimplemented!();
    }


    fn visit_expr_loop(&mut self, i: &'a syn::ExprLoop) {
        unimplemented!();
    }


    fn visit_expr_macro(&mut self, i: &'a syn::ExprMacro) {
        unimplemented!();
    }


    fn visit_expr_match(&mut self, i: &'a syn::ExprMatch) {
        unimplemented!();
    }


    fn visit_expr_method_call(&mut self, i: &'a syn::ExprMethodCall) {
        unimplemented!();
    }

    fn visit_expr_paren(&mut self, i: &'a syn::ExprParen) {
        unimplemented!();
    }

    fn visit_expr_path(&mut self, i: &'a syn::ExprPath) {
        unimplemented!();
    }


    fn visit_expr_range(&mut self, i: &'a syn::ExprRange) {
        unimplemented!();
    }


    fn visit_expr_reference(&mut self, i: &'a syn::ExprReference) {
        unimplemented!();
    }


    fn visit_expr_repeat(&mut self, i: &'a syn::ExprRepeat) {
        unimplemented!();
    }


    fn visit_expr_return(&mut self, i: &'a syn::ExprReturn) {
        unimplemented!();
    }


    fn visit_expr_struct(&mut self, i: &'a syn::ExprStruct) {
        unimplemented!();
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
        unimplemented!();
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
        unimplemented!();
    }


    fn visit_field_pat(&mut self, i: &'a syn::FieldPat) {
        unimplemented!();
    }


    fn visit_field_value(&mut self, i: &'a syn::FieldValue) {
        unimplemented!();
    }

    fn visit_fields(&mut self, i: &'a syn::Fields) {
        unimplemented!();
    }

    fn visit_fields_named(&mut self, i: &'a syn::FieldsNamed) {
        unimplemented!();
    }

    fn visit_fields_unnamed(&mut self, i: &'a syn::FieldsUnnamed) {
        unimplemented!();
    }

    fn visit_file(&mut self, i: &'a syn::File) {
        unimplemented!();
    }

    fn visit_fn_arg(&mut self, i: &'a syn::FnArg) {
        unimplemented!();
    }

    fn visit_fn_decl(&mut self, i: &'a syn::FnDecl) {
        unimplemented!();
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

    fn visit_generic_argument(&mut self, i: &'a syn::GenericArgument) {
        unimplemented!();
    }


    fn visit_generic_method_argument(&mut self, i: &'a syn::GenericMethodArgument) {
        unimplemented!();
    }

    fn visit_generic_param(&mut self, i: &'a syn::GenericParam) {
        unimplemented!();
    }

    fn visit_generics(&mut self, i: &'a syn::Generics) {
        unimplemented!();
    }
    fn visit_ident(&mut self, i: &'a syn::Ident) {
        unimplemented!();
    }

    fn visit_impl_item(&mut self, i: &'a syn::ImplItem) {
        unimplemented!();
    }

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
        unimplemented!();
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
        unimplemented!();
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

        write!(self.out, "mod {}", i.ident);

        // TODO abstract?
        match i.content {
            Some((_, ref items)) => {
                write!(self.out, " {{\n");
                self.visit_items(items);
                write!(self.out, "}}\n");
            }
            _ => {
                write!(self.out, ";");
            }
        }
    }

    fn visit_item_static(&mut self, i: &'a syn::ItemStatic) {
        unimplemented!();
    }

    fn visit_item_struct(&mut self, i: &'a syn::ItemStruct) {
        unimplemented!();
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
        unimplemented!();
    }

    fn visit_item_verbatim(&mut self, i: &'a syn::ItemVerbatim) {
        unimplemented!();
    }


    fn visit_label(&mut self, i: &'a syn::Label) {
        unimplemented!();
    }
    fn visit_lifetime(&mut self, i: &'a syn::Lifetime) {
        unimplemented!();
    }

    fn visit_lifetime_def(&mut self, i: &'a syn::LifetimeDef) {
        unimplemented!();
    }

    fn visit_lit(&mut self, i: &'a syn::Lit) {
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
        unimplemented!();
    }

    fn visit_lit_str(&mut self, i: &'a syn::LitStr) {
        unimplemented!();
    }

    fn visit_lit_verbatim(&mut self, i: &'a syn::LitVerbatim) {
        unimplemented!();
    }


    fn visit_local(&mut self, i: &'a syn::Local) {
        unimplemented!();
    }

    fn visit_macro(&mut self, i: &'a syn::Macro) {
        unimplemented!();
    }

    fn visit_macro_delimiter(&mut self, i: &'a syn::MacroDelimiter) {
        unimplemented!();
    }

    fn visit_member(&mut self, i: &'a syn::Member) {
        unimplemented!();
    }

    fn visit_meta(&mut self, i: &'a syn::Meta) {
        unimplemented!();
    }

    fn visit_meta_list(&mut self, i: &'a syn::MetaList) {
        unimplemented!();
    }

    fn visit_meta_name_value(&mut self, i: &'a syn::MetaNameValue) {
        unimplemented!();
    }

    fn visit_method_sig(&mut self, i: &'a syn::MethodSig) {
        unimplemented!();
    }


    fn visit_method_turbofish(&mut self, i: &'a syn::MethodTurbofish) {
        unimplemented!();
    }

    fn visit_nested_meta(&mut self, i: &'a syn::NestedMeta) {
        unimplemented!();
    }

    fn visit_parenthesized_generic_arguments(&mut self, i: &'a syn::ParenthesizedGenericArguments) {
        unimplemented!();
    }


    fn visit_pat(&mut self, i: &'a syn::Pat) {
        unimplemented!();
    }


    fn visit_pat_box(&mut self, i: &'a syn::PatBox) {
        unimplemented!();
    }


    fn visit_pat_ident(&mut self, i: &'a syn::PatIdent) {
        unimplemented!();
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
        unimplemented!();
    }


    fn visit_pat_tuple_struct(&mut self, i: &'a syn::PatTupleStruct) {
        unimplemented!();
    }


    fn visit_pat_verbatim(&mut self, i: &'a syn::PatVerbatim) {
        unimplemented!();
    }


    fn visit_pat_wild(&mut self, i: &'a syn::PatWild) {
        unimplemented!();
    }

    fn visit_path(&mut self, i: &'a syn::Path) {
        unimplemented!();
    }

    fn visit_path_arguments(&mut self, i: &'a syn::PathArguments) {
        unimplemented!();
    }

    fn visit_path_segment(&mut self, i: &'a syn::PathSegment) {
        unimplemented!();
    }

    fn visit_predicate_eq(&mut self, i: &'a syn::PredicateEq) {
        unimplemented!();
    }

    fn visit_predicate_lifetime(&mut self, i: &'a syn::PredicateLifetime) {
        unimplemented!();
    }

    fn visit_predicate_type(&mut self, i: &'a syn::PredicateType) {
        unimplemented!();
    }

    fn visit_qself(&mut self, i: &'a syn::QSelf) {
        unimplemented!();
    }


    fn visit_range_limits(&mut self, i: &'a syn::RangeLimits) {
        unimplemented!();
    }

    fn visit_return_type(&mut self, i: &'a syn::ReturnType) {
        unimplemented!();
    }

    fn visit_span(&mut self, i: &'a proc_macro2::Span) {
        unimplemented!();
    }


    fn visit_stmt(&mut self, i: &'a syn::Stmt) {
        unimplemented!();
    }

    fn visit_trait_bound(&mut self, i: &'a syn::TraitBound) {
        unimplemented!();
    }

    fn visit_trait_bound_modifier(&mut self, i: &'a syn::TraitBoundModifier) {
        unimplemented!();
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

    fn visit_type(&mut self, i: &'a syn::Type) {
        unimplemented!();
    }

    fn visit_type_array(&mut self, i: &'a syn::TypeArray) {
        unimplemented!();
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
        unimplemented!();
    }

    fn visit_type_param_bound(&mut self, i: &'a syn::TypeParamBound) {
        unimplemented!();
    }

    fn visit_type_paren(&mut self, i: &'a syn::TypeParen) {
        unimplemented!();
    }

    fn visit_type_path(&mut self, i: &'a syn::TypePath) {
        unimplemented!();
    }

    fn visit_type_ptr(&mut self, i: &'a syn::TypePtr) {
        unimplemented!();
    }

    fn visit_type_reference(&mut self, i: &'a syn::TypeReference) {
        unimplemented!();
    }

    fn visit_type_slice(&mut self, i: &'a syn::TypeSlice) {
        unimplemented!();
    }

    fn visit_type_trait_object(&mut self, i: &'a syn::TypeTraitObject) {
        unimplemented!();
    }

    fn visit_type_tuple(&mut self, i: &'a syn::TypeTuple) {
        unimplemented!();
    }

    fn visit_type_verbatim(&mut self, i: &'a syn::TypeVerbatim) {
        unimplemented!();
    }

    fn visit_un_op(&mut self, i: &'a syn::UnOp) {
        unimplemented!();
    }

    fn visit_use_glob(&mut self, i: &'a syn::UseGlob) {
        unimplemented!();
    }

    fn visit_use_group(&mut self, i: &'a syn::UseGroup) {
        unimplemented!();
    }

    fn visit_use_name(&mut self, i: &'a syn::UseName) {
        unimplemented!();
    }

    fn visit_use_path(&mut self, i: &'a syn::UsePath) {
        unimplemented!();
    }

    fn visit_use_rename(&mut self, i: &'a syn::UseRename) {
        unimplemented!();
    }

    fn visit_use_tree(&mut self, i: &'a syn::UseTree) {
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
                write!(self.out, "pub ");
            }
            actual => {
                unimplemented!("{:?}", actual);
            }
        }
    }

    fn visit_where_clause(&mut self, i: &'a syn::WhereClause) {
        unimplemented!();
    }

    fn visit_where_predicate(&mut self, i: &'a syn::WherePredicate) {
        unimplemented!();
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
