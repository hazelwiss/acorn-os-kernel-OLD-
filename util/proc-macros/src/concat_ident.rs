use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{parse::Parse, Token};

#[allow(unused)]
struct Concat {
    l: Ident,
    comma: Token![,],
    r: Ident,
}

impl Parse for Concat {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let l = input.parse()?;
        let comma = input.parse()?;
        let r = input.parse()?;
        Ok(Self { l, comma, r })
    }
}

impl ToTokens for Concat {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = format!("{}{}", self.l, self.r);
        let ident = Ident::new(&ident, Span::call_site());
        tokens.extend(quote!(#ident))
    }
}

pub fn concat_ident(ts: TokenStream) -> TokenStream {
    let concat: Concat = syn::parse(ts).expect("failure to concatinate idents.");
    quote!(#concat).into()
}
