use syn;
use syn::visit::Visit;

pub trait Node {
    fn visit<'a, T: Visit<'a>>(&'a self, visit: &mut T);
}

macro_rules! impl_node {
    ($t:ident, $f:ident) => {
        impl Node for syn::$t {
            fn visit<'a, T: Visit<'a>>(&'a self, visit: &mut T) {
                visit.$f(self);
            }
        }
    }
}

impl_node!(Expr, visit_expr);
impl_node!(Field, visit_field);
impl_node!(FnArg, visit_fn_arg);
impl_node!(GenericArgument, visit_generic_argument);
impl_node!(GenericParam, visit_generic_param);
impl_node!(NestedMeta, visit_nested_meta);
impl_node!(Pat, visit_pat);
impl_node!(PathSegment, visit_path_segment);
impl_node!(Type, visit_type);
impl_node!(TypeParamBound, visit_type_param_bound);
impl_node!(WherePredicate, visit_where_predicate);
