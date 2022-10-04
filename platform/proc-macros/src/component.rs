use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{braced, parse::Parse, punctuated::Punctuated, Attribute, Path, Token};

use crate::platform;

#[allow(dead_code)]
struct TargetImpl {
    target_name: Ident,
    arrow_token: Token![=>],
    brace: syn::token::Brace,
    target_impl: proc_macro2::TokenStream,
}

#[allow(dead_code)]
struct Component {
    attrs: Vec<Attribute>,
    ident: Ident,
    arrow_token: Token![=>],
    hal: Path,
    impl_token: Token![impl],
    brace: syn::token::Brace,
    impl_contents: Punctuated<TargetImpl, Token![,]>,
}

impl Parse for TargetImpl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let target_name = input.parse()?;
        let arrow_token = input.parse()?;
        let target_impl;
        let brace = braced!(target_impl in input);
        let target_impl = target_impl.parse()?;
        Ok(Self {
            target_name,
            arrow_token,
            brace,
            target_impl,
        })
    }
}

impl Parse for Component {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let ident = input.parse()?;
        let arrow_token = input.parse()?;
        let hal = input.parse()?;
        let impl_token = input.parse()?;
        let impl_contents;
        let brace = braced!(impl_contents in input);
        let impl_contents = impl_contents.call(Punctuated::parse_terminated)?;
        Ok(Self {
            attrs,
            ident,
            arrow_token,
            hal,
            impl_token,
            brace,
            impl_contents,
        })
    }
}

impl ToTokens for Component {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            attrs,
            ident,
            hal,
            impl_contents,
            ..
        } = self;
        let mut platforms = platform::PlatformsHandler::default();
        for entry in impl_contents {
            platform::platforms_match(
                &mut platforms,
                &entry.target_name.to_string(),
                entry.target_impl.clone(),
            )
        }
        let impl_contents = platforms.to_tokens(ident);
        tokens.extend(quote! {
            #(#attrs)*
            pub struct #ident(#hal);

            impl ::core::ops::Deref for #ident{
                type Target = #hal;

                fn deref(&self) -> &Self::Target{
                    &self.0
                }
            }

            impl ::core::ops::DerefMut for #ident{
                fn deref_mut(&mut self) -> &mut Self::Target{
                    &mut self.0
                }
            }

            #impl_contents
        });
    }
}

pub(crate) fn component(ts: TokenStream) -> TokenStream {
    let component: Component = syn::parse(ts).expect("unable to parse component.");
    quote! {#component}.into()
}
