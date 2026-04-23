use proc_macro::TokenStream;
use quote::quote;
use syn::{
    ConstParam, Data, DeriveInput, Fields, GenericParam, Ident, LifetimeParam, Token, TypeParam,
    parse, parse_macro_input, parse2, punctuated::Punctuated, spanned::Spanned,
};

#[proc_macro_derive(State)]
pub fn my_derive_state(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let struct_name = derive_input.ident;

    let fields = match derive_input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => fields.named,
            Fields::Unnamed(fields) => fields.unnamed,
            Fields::Unit => panic!("Unit structs are not supported."),
        },
        Data::Enum(_) | Data::Union(_) => panic!("Enum's and Union's are not supported."),
    };

    let is_tupple_struct = fields.iter().all(|f| f.ident.is_none());

    let constructor = |fields: Vec<proc_macro2::TokenStream>| {
        if is_tupple_struct {
            quote! { Self (#(#fields),*) }
        } else {
            quote! { Self {#(#fields),*} }
        }
    };

    let field_types: Vec<_> = fields.iter().cloned().map(|f| f.ty).collect();

    let generic_params = derive_input.generics.params;
    let generic_where = derive_input.generics.where_clause;
    let generic_idents =
        Punctuated::<_, Token![,]>::from_iter(generic_params.iter().map(|param| match param {
            GenericParam::Lifetime(LifetimeParam { lifetime, .. }) => quote! {#lifetime},
            GenericParam::Type(TypeParam { ident, .. }) => quote! {#ident},
            GenericParam::Const(ConstParam { ident, .. }) => quote! {#ident},
        }));

    let clone_impl = quote! {
        impl<#generic_params> Clone for #struct_name<#generic_idents> #generic_where {
            fn clone(&self) -> Self {
                *self
            }
        }
    };
    let copy_impl = quote! {
        impl<#generic_params> Copy for #struct_name<#generic_idents> #generic_where {}
    };
    let debug_impl = {
        let debug_fields: Vec<_> = fields
            .iter()
            .enumerate()
            .map(|(i, f)| match &f.ident {
                Some(ident) => {
                    let ident_str = ident.to_string();
                    quote! { .field(#ident_str, &self.#ident) }
                }
                None => {
                    let index = syn::Index::from(i);
                    let index_str = i.to_string();
                    quote! { .field(#index_str, &self.#index) }
                }
            })
            .collect();
        quote! {
            impl<#generic_params> std::fmt::Debug for #struct_name<#generic_idents> #generic_where {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.debug_struct(stringify!(#struct_name))
                        #(#debug_fields)*
                        .finish()
                }
            }
        }
    };
    let add_impl = {
        let add_fields: Vec<_> = fields
            .iter()
            .enumerate()
            .map(|(i, f)| match &f.ident {
                Some(ident) => quote! { #ident: self.#ident + rhs.#ident },
                None => {
                    let index = syn::Index::from(i);
                    quote! { self.#index + rhs.#index }
                }
            })
            .collect();
        let add_constructor = constructor(add_fields);
        quote! {
            impl<#generic_params> std::ops::Add for #struct_name<#generic_idents> #generic_where {
                type Output = Self;
                fn add(self, rhs: Self) -> Self::Output {
                    #add_constructor
                }
            }

        }
    };
    let add_assign_impl = {
        let add_assign_fields: Vec<_> = fields
            .iter()
            .enumerate()
            .map(|(i, f)| match &f.ident {
                Some(ident) => quote! { self.#ident += rhs.#ident },
                None => {
                    let index = syn::Index::from(i);
                    quote! { self.#index += rhs.#index }
                }
            })
            .collect();
        quote! {
            impl<#generic_params> std::ops::AddAssign for #struct_name<#generic_idents> #generic_where {
                fn add_assign(&mut self, rhs: Self) {
                    #(#add_assign_fields;)*
                }
            }
        }
    };
    let sub_impl = {
        let sub_fields: Vec<_> = fields
            .iter()
            .enumerate()
            .map(|(i, f)| match &f.ident {
                Some(ident) => quote! { #ident: self.#ident - rhs.#ident },
                None => {
                    let index = syn::Index::from(i);
                    quote! { self.#index - rhs.#index }
                }
            })
            .collect();
        let sub_constructor = constructor(sub_fields);
        quote! {
            impl<#generic_params> std::ops::Sub for #struct_name<#generic_idents> #generic_where {
                type Output = Self;
                fn sub(self, rhs: Self) -> Self::Output {
                    #sub_constructor
                }
            }

        }
    };
    let mul_impl = {
        let mul_fields: Vec<_> = fields
            .iter()
            .enumerate()
            .map(|(i, f)| match &f.ident {
                Some(ident) => quote! { #ident: self.#ident * rhs },
                None => {
                    let index = syn::Index::from(i);
                    quote! { self.#index * rhs }
                }
            })
            .collect();
        let mul_constructor = constructor(mul_fields);
        let mut extended_where = generic_where.clone().unwrap_or(syn::WhereClause {
            where_token: Token![where](generic_where.span()),
            predicates: Punctuated::new(),
        });
        for ty in field_types.iter() {
            extended_where
                .predicates
                .push(parse2(quote! {#ty: std::ops::Mul<__Scalar, Output = #ty>}).unwrap())
        }
        quote! {
            impl<__Scalar: Copy, #generic_params> std::ops::Mul<__Scalar> for #struct_name<#generic_idents> #extended_where {
                type Output = Self;
                fn mul(self, rhs: __Scalar) -> Self::Output {
                    #mul_constructor
                }
            }
        }
    };
    let div_impl = {
        let div_fields: Vec<_> = fields
            .iter()
            .enumerate()
            .map(|(i, f)| match &f.ident {
                Some(ident) => quote! { #ident: self.#ident / rhs },
                None => {
                    let index = syn::Index::from(i);
                    quote! { self.#index / rhs }
                }
            })
            .collect();
        let div_constructor = constructor(div_fields);
        let mut extended_where = generic_where.clone().unwrap_or(syn::WhereClause {
            where_token: Token![where](generic_where.span()),
            predicates: Punctuated::new(),
        });
        for ty in field_types.iter() {
            extended_where
                .predicates
                .push(parse2(quote! {#ty: std::ops::Div<__Scalar, Output = #ty>}).unwrap())
        }
        quote! {
            impl<__Scalar: Copy, #generic_params> std::ops::Div<__Scalar> for #struct_name<#generic_idents> #extended_where {
                type Output = Self;
                fn div(self, rhs: __Scalar) -> Self::Output {
                    #div_constructor
                }
            }
        }
    };
    let neg_impl = {
        let neg_fields: Vec<_> = fields
            .iter()
            .enumerate()
            .map(|(i, f)| match &f.ident {
                Some(ident) => quote! { #ident: - self.#ident },
                None => {
                    let index = syn::Index::from(i);
                    quote! { - self.#index }
                }
            })
            .collect();
        let neg_constructor = constructor(neg_fields);
        quote! {
            impl<#generic_params> std::ops::Neg for #struct_name<#generic_idents> #generic_where {
                type Output = Self;
                fn neg(self) -> Self::Output {
                    #neg_constructor
                }
            }
        }
    };
    let zero_impl = {
        let field_values_self: Vec<_> = fields
            .iter()
            .enumerate()
            .map(|(i, f)| match &f.ident {
                Some(ident) => {
                    quote! { self.#ident }
                }
                None => {
                    let index = syn::Index::from(i);
                    quote! { self.#index }
                }
            })
            .collect();
        let zero_fields: Vec<_> = fields
            .iter()
            .map(|f| match &f.ident {
                Some(ident) => quote! { #ident: num_traits::identities::zero() },
                None => quote! { num_traits::identities::zero() },
            })
            .collect();
        let zero_constructor = constructor(zero_fields);
        quote! {
            impl<#generic_params> num_traits::identities::Zero for #struct_name<#generic_idents> #generic_where {
                fn zero() -> Self {
                    #zero_constructor
                }
                fn is_zero(&self) -> bool {
                    #(#field_values_self.is_zero())&&*
                }
            }
        }
    };

    let all_impl = quote! {
        #clone_impl
        #copy_impl
        #debug_impl
        #add_impl
        #add_assign_impl
        #sub_impl
        #mul_impl
        #div_impl
        #neg_impl
        #zero_impl
    };

    // let zero_impl = quote! {"
    //         impl num_traits::identities::Zero for #struct_name {
    //             fn zero() -> Self {
    //                 Self {
    //                     ...
    //                 }
    //             }
    //         }
    //     "};

    // TokenStream::from(zero_impl)
    TokenStream::from(all_impl)
}
