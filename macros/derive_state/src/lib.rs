use itertools::Itertools;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    ConstParam, Data, DeriveInput, Fields, GenericParam, Index, LifetimeParam, Token, TypeParam,
    parse_macro_input, parse2, punctuated::Punctuated, spanned::Spanned,
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

    struct FieldInfo {
        ty: syn::Type,
        decl: proc_macro2::TokenStream,
        path: proc_macro2::TokenStream,
    }

    let fields_info: Vec<FieldInfo> = fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let ty = f.ty.clone();
            let decl = match &f.ident {
                Some(ident) => quote! {#ident:},
                None => quote! {},
            };

            let path = match &f.ident {
                Some(ident) => quote! {#ident},
                None => {
                    let index = Index::from(i);
                    quote! {#index}
                }
            };

            FieldInfo { ty, decl, path }
        })
        .collect();

    fn get_elementary_types(ty: &syn::Type) -> Vec<syn::Type> {
        match ty {
            syn::Type::Array(type_array) => get_elementary_types(type_array.elem.as_ref()),
            syn::Type::Path(_) => vec![ty.clone()],
            syn::Type::Tuple(type_tuple) => type_tuple
                .elems
                .iter()
                .flat_map(get_elementary_types)
                .collect(),
            _ => todo!(),
        }
    }

    let field_elementary_types: Vec<_> = fields
        .iter()
        .cloned()
        .flat_map(|f| get_elementary_types(&f.ty))
        .unique()
        .collect();

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
        let debug_fields: Vec<_> = fields_info
            .iter()
            .map(|FieldInfo { path, .. }| {
                let path_str = path.to_string();
                quote! {.field(#path_str, &self.#path)}
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
        fn add_elements(
            ty: &syn::Type,
            lhs: proc_macro2::TokenStream,
            rhs: proc_macro2::TokenStream,
        ) -> proc_macro2::TokenStream {
            match ty {
                syn::Type::Path(_) => quote! {#lhs + #rhs},
                syn::Type::Array(type_array) => {
                    let add =
                        add_elements(type_array.elem.as_ref(), quote! {#lhs[i]}, quote! {#rhs[i]});
                    quote! { std::array::from_fn(|i| #add) }
                }
                syn::Type::Tuple(type_tuple) => {
                    let add: Vec<_> = type_tuple
                        .elems
                        .iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            let index = syn::Index::from(i);
                            add_elements(ty, quote! {#lhs.#index}, quote! {#rhs.#index})
                        })
                        .collect();
                    quote! { ( #(#add),* ) }
                }
                _ => panic!(),
            }
        }
        let added_fields: Vec<_> = fields_info
            .iter()
            .map(|FieldInfo { ty, decl, path }| {
                let added = add_elements(&ty, quote! {self.#path}, quote! {rhs.#path});
                quote! {
                    #decl #added
                }
            })
            .collect();
        let add_constructor = constructor(added_fields);
        quote! {
            impl<#generic_params> std::ops::Add for #struct_name<#generic_idents> #generic_where {
                type Output = Self;
                fn add(self, rhs: Self) -> Self::Output {
                    #add_constructor
                }
            }

        }
    };
    let sub_impl = {
        fn substract_elements(
            ty: &syn::Type,
            lhs: proc_macro2::TokenStream,
            rhs: proc_macro2::TokenStream,
        ) -> proc_macro2::TokenStream {
            match ty {
                syn::Type::Path(_) => quote! {#lhs - #rhs},
                syn::Type::Array(type_array) => {
                    let sub = substract_elements(
                        type_array.elem.as_ref(),
                        quote! {#lhs[i]},
                        quote! {#rhs[i]},
                    );
                    quote! { std::array::from_fn(|i| #sub) }
                }
                syn::Type::Tuple(type_tuple) => {
                    let sub: Vec<_> = type_tuple
                        .elems
                        .iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            let index = syn::Index::from(i);
                            substract_elements(ty, quote! {#lhs.#index}, quote! {#rhs.#index})
                        })
                        .collect();
                    quote! { ( #(#sub),* ) }
                }
                _ => panic!(),
            }
        }
        let substracted_fields: Vec<_> = fields_info
            .iter()
            .map(|FieldInfo { ty, decl, path }| {
                let sub = substract_elements(&ty, quote! {self.#path}, quote! {rhs.#path});
                quote! {
                    #decl #sub
                }
            })
            .collect();
        let sub_constructor = constructor(substracted_fields);
        quote! {
            impl<#generic_params> std::ops::Sub for #struct_name<#generic_idents> #generic_where {
                type Output = Self;
                fn sub(self, rhs: Self) -> Self::Output {
                    #sub_constructor
                }
            }

        }
    };
    let add_assign_impl = {
        fn add_assign_elements(
            ty: &syn::Type,
            lhs: proc_macro2::TokenStream,
            rhs: proc_macro2::TokenStream,
        ) -> proc_macro2::TokenStream {
            match ty {
                syn::Type::Path(_) => quote! {#lhs += #rhs;},
                syn::Type::Array(type_array) => {
                    let add_assign =
                        add_assign_elements(type_array.elem.as_ref(), quote! {*__l}, quote! {__r});
                    quote! {
                        {
                            for (__l, __r) in #lhs.iter_mut().zip(#rhs.iter()) {
                                #add_assign
                            }
                        }
                    }
                }
                syn::Type::Tuple(type_tuple) => {
                    let add_assign: Vec<_> = type_tuple
                        .elems
                        .iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            let index = syn::Index::from(i);
                            add_assign_elements(ty, quote! {#lhs.#index}, quote! {#rhs.#index})
                        })
                        .collect();
                    quote! { #(#add_assign)* }
                }
                _ => panic!(),
            }
        }
        let add_assigned_fields: Vec<_> = fields_info
            .iter()
            .map(|FieldInfo { ty, decl: _, path }| {
                add_assign_elements(ty, quote! {self.#path}, quote! {rhs.#path})
            })
            .collect();
        quote! {
            impl<#generic_params> std::ops::AddAssign for #struct_name<#generic_idents> #generic_where {
                fn add_assign(&mut self, rhs: Self) {
                    #(#add_assigned_fields)*
                }
            }
        }
    };
    let mul_impl = {
        fn multiply_elements(
            ty: &syn::Type,
            lhs: proc_macro2::TokenStream,
            rhs: proc_macro2::TokenStream,
        ) -> proc_macro2::TokenStream {
            match ty {
                syn::Type::Path(_) => quote! {#lhs * #rhs},
                syn::Type::Array(type_array) => {
                    let mul = multiply_elements(type_array.elem.as_ref(), quote! {#lhs[i]}, rhs);
                    quote! { std::array::from_fn(|i| #mul) }
                }
                syn::Type::Tuple(type_tuple) => {
                    let mul: Vec<_> = type_tuple
                        .elems
                        .iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            let index = syn::Index::from(i);
                            multiply_elements(ty, quote! {#lhs.#index}, rhs.clone())
                        })
                        .collect();
                    quote! { ( #(#mul),* ) }
                }
                _ => panic!(),
            }
        }
        let multiplied_fields: Vec<_> = fields_info
            .iter()
            .map(|FieldInfo { ty, decl, path }| {
                let multiplied = multiply_elements(&ty, quote! {self.#path}, quote! {rhs});
                quote! {
                    #decl #multiplied
                }
            })
            .collect();
        let mul_constructor = constructor(multiplied_fields);
        let mut generic_where_extended = generic_where.clone().unwrap_or(syn::WhereClause {
            where_token: Token![where](generic_where.span()),
            predicates: Punctuated::new(),
        });
        for elementary_type in field_elementary_types.iter() {
            generic_where_extended.predicates.push(
                parse2(
                    quote! {#elementary_type: std::ops::Mul<__Scalar, Output = #elementary_type>},
                )
                .unwrap(),
            )
        }
        quote! {
            impl<__Scalar: Copy, #generic_params> std::ops::Mul<__Scalar> for #struct_name<#generic_idents> #generic_where_extended {
                type Output = Self;
                fn mul(self, rhs: __Scalar) -> Self::Output {
                    #mul_constructor
                }
            }
        }
    };
    let div_impl = {
        fn divide_elements(
            ty: &syn::Type,
            lhs: proc_macro2::TokenStream,
            rhs: proc_macro2::TokenStream,
        ) -> proc_macro2::TokenStream {
            match ty {
                syn::Type::Path(_) => quote! {#lhs / #rhs},
                syn::Type::Array(type_array) => {
                    let div = divide_elements(type_array.elem.as_ref(), quote! {#lhs[i]}, rhs);
                    quote! { std::array::from_fn(|i| #div) }
                }
                syn::Type::Tuple(type_tuple) => {
                    let div: Vec<_> = type_tuple
                        .elems
                        .iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            let index = syn::Index::from(i);
                            divide_elements(ty, quote! {#lhs.#index}, rhs.clone())
                        })
                        .collect();
                    quote! { ( #(#div),* ) }
                }
                _ => panic!(),
            }
        }
        let divided_fields: Vec<_> = fields_info
            .iter()
            .map(|FieldInfo { ty, decl, path }| {
                let divided = divide_elements(&ty, quote! {self.#path}, quote! {rhs});
                quote! {
                    #decl #divided
                }
            })
            .collect();
        let div_constructor = constructor(divided_fields);
        let mut generic_where_extended = generic_where.clone().unwrap_or(syn::WhereClause {
            where_token: Token![where](generic_where.span()),
            predicates: Punctuated::new(),
        });
        for elementary_type in field_elementary_types.iter() {
            generic_where_extended.predicates.push(
                parse2(
                    quote! {#elementary_type: std::ops::Div<__Scalar, Output = #elementary_type>},
                )
                .unwrap(),
            )
        }
        quote! {
            impl<__Scalar: Copy, #generic_params> std::ops::Div<__Scalar> for #struct_name<#generic_idents> #generic_where_extended {
                type Output = Self;
                fn div(self, rhs: __Scalar) -> Self::Output {
                    #div_constructor
                }
            }
        }
    };
    let neg_impl = {
        fn negate_elements(
            ty: &syn::Type,
            lhs: proc_macro2::TokenStream,
        ) -> proc_macro2::TokenStream {
            match ty {
                syn::Type::Path(_) => quote! {-#lhs},
                syn::Type::Array(type_array) => {
                    let neg = negate_elements(type_array.elem.as_ref(), quote! {#lhs[i]});
                    quote! { std::array::from_fn(|i| #neg) }
                }
                syn::Type::Tuple(type_tuple) => {
                    let neg: Vec<_> = type_tuple
                        .elems
                        .iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            let index = syn::Index::from(i);
                            negate_elements(ty, quote! {#lhs.#index})
                        })
                        .collect();
                    quote! { ( #(#neg),* ) }
                }
                _ => panic!(),
            }
        }
        let negated_fields: Vec<_> = fields_info
            .iter()
            .map(|FieldInfo { ty, decl, path }| {
                let divided = negate_elements(&ty, quote! {self.#path});
                quote! {
                    #decl #divided
                }
            })
            .collect();
        let neg_constructor = constructor(negated_fields);
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
        fn zero_elements(ty: &syn::Type) -> proc_macro2::TokenStream {
            match ty {
                syn::Type::Path(_) => quote! {num_traits::identities::zero()},
                syn::Type::Array(type_array) => {
                    let zero = zero_elements(type_array.elem.as_ref());
                    let len = &type_array.len;
                    quote! {[#zero; #len]}
                }
                syn::Type::Tuple(type_tuple) => {
                    let zero: Vec<_> = type_tuple
                        .elems
                        .iter()
                        .map(|ty| zero_elements(ty))
                        .collect();
                    quote! { ( #(#zero),* ) }
                }
                _ => panic!(),
            }
        }
        fn is_zero_elements(
            ty: &syn::Type,
            lhs: proc_macro2::TokenStream,
        ) -> proc_macro2::TokenStream {
            match ty {
                syn::Type::Path(_) => quote! {#lhs.is_zero()},
                syn::Type::Array(type_array) => {
                    let is_zero = is_zero_elements(type_array.elem.as_ref(), quote! {__elem});
                    quote! { #lhs.iter().all(|__elem| #is_zero) }
                    // quote!{false}
                }
                syn::Type::Tuple(type_tuple) => {
                    let neg: Vec<_> = type_tuple
                        .elems
                        .iter()
                        .enumerate()
                        .map(|(i, ty)| {
                            let index = syn::Index::from(i);
                            is_zero_elements(ty, quote! {#lhs.#index})
                        })
                        .collect();
                    quote! { ( #(#neg)&&* ) }
                }
                _ => panic!(),
            }
        }
        let zero_fields: Vec<_> = fields_info
            .iter()
            .map(|FieldInfo { ty, decl, .. }| {
                let zero = zero_elements(ty);
                quote! {#decl #zero}
            })
            .collect();
        let zero_constructor = constructor(zero_fields);
        let is_zero_fields: Vec<_> = fields_info
            .iter()
            .map(|FieldInfo { ty, path, .. }| is_zero_elements(ty, quote! {self.#path}))
            .collect();
        quote! {
            impl<#generic_params> num_traits::identities::Zero for #struct_name<#generic_idents> #generic_where {
                fn zero() -> Self {
                    #zero_constructor
                }
                fn is_zero(&self) -> bool {
                    #(#is_zero_fields)&&*
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

    TokenStream::from(all_impl)
}
