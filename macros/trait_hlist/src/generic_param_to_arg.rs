pub fn generic_param_to_arg(param: syn::GenericParam) -> syn::GenericArgument {
    match param {
        syn::GenericParam::Lifetime(syn::LifetimeParam { lifetime, .. }) => {
            syn::GenericArgument::Lifetime(lifetime)
        }
        syn::GenericParam::Type(syn::TypeParam { ident, .. }) => {
            syn::GenericArgument::Type(syn::parse_quote!(#ident))
        }
        syn::GenericParam::Const(syn::ConstParam { ident, .. }) => {
            syn::GenericArgument::Const(syn::parse_quote!(#ident))
        }
    }
}
