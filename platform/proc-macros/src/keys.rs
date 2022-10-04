use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Token};

pub struct Key {
    ident: Ident,
    val: char,
}

struct Keys {
    keys: Vec<Key>,
}

impl Parse for Keys {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut keys = Vec::new();
        loop {
            if input.is_empty() {
                break;
            }
            let ident: Ident = input.parse()?;
            let arrow: Option<Token![=>]> = input.parse()?;
            if arrow.is_some() {
                let val: syn::LitChar = input.parse()?;
                keys.push(Key {
                    ident,
                    val: val.value(),
                })
            } else {
                let ident_cmp = ident.to_string();
                if ident_cmp.len() == 1 {
                    let c = unsafe { ident_cmp.chars().nth(0).unwrap_unchecked() };
                    if !c.is_ascii() {
                        return Err(input.error(format!(
                            "identifier {ident} has to be a valid ascii character."
                        )));
                    }
                    let mut upper = c;
                    let mut lower = c;
                    upper.make_ascii_uppercase();
                    lower.make_ascii_lowercase();
                    keys.push(Key {
                        ident: Ident::new(&upper.to_string(), ident.span()),
                        val: upper,
                    });
                    keys.push(Key {
                        ident: Ident::new(&lower.to_string(), ident.span()),
                        val: lower,
                    });
                } else {
                    return Err(input.error(&format!(
                    "identifier '{ident}' has to be a single valid ascii character unless '=>' is used.",
                )));
                }
            }
            if !input.peek(Token![,]) {
                if !input.is_empty() {
                    return Err(input.error("missing coma."));
                }
                break;
            } else {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Self { keys })
    }
}

impl ToTokens for Keys {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut macro_vec = vec![];
        let mut variant_vec = vec![];
        for key in &self.keys {
            let ident = &key.ident;
            let val = &key.val;
            macro_vec.push(quote!([#val] => { { ::platform::types::KeyCode::#ident } }));
            variant_vec.push(quote!(#ident = #val as u8));
        }
        tokens.extend(quote! {
            /// Describes different keycodes.
            #[allow(non_camel_case_types)]
            #[allow(missing_docs)]
            #[repr(u8)]
            pub enum KeyCode{
                #(#variant_vec),*
            }
            /// Translates the given token to the corresponding key.
            ///
            /// ```rs
            /// assert_eq!(key!['='], KeyCode::Eq);
            /// assert_eq!(key!['a'], KeyCode::a);
            /// ```
            #[macro_export]
            macro_rules! key{
                #(#macro_vec);*
            }
        });
    }
}

pub fn keys(ts: TokenStream) -> TokenStream {
    let keys: Keys = syn::parse(ts).expect("Failed to parse keys");
    quote!(#keys).into()
}
