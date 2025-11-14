pub struct AngleBracketedGenericParams {
    #[allow(dead_code)]
    pub lt_token: syn::Token![<],
    pub params: syn::punctuated::Punctuated<syn::GenericParam, syn::Token![,]>,
    #[allow(dead_code)]
    pub gt_token: syn::Token![>],
}

impl syn::parse::Parse for AngleBracketedGenericParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {

        let lt_token = input.parse::<syn::Token![<]>()?;
       
        let mut params : syn::punctuated::Punctuated<
            syn::GenericParam, syn::Token![,]
            > = syn::punctuated::Punctuated::new();

        loop {
            if input.peek(syn::Token![>]) {
                break
            } else {
                params.push_value(input.parse()?);
                if input.peek(syn::Token![,]) {
                    params.push_punct(input.parse::<syn::Token![,]>()?);
                } else {
                    break
                }
            }
        }

        let gt_token = input.parse::<syn::Token![>]>()?;

        Ok(Self {
            lt_token,
            params,
            gt_token,
        })
    }
}
