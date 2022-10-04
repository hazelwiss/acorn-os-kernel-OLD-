use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{
    parse::Parse, punctuated::Punctuated, token::Colon, Attribute, Generics, Token, Type,
    Visibility,
};

/// # Attributes
/// custom attributes have to be wrapped into
/// interface(...)
/// where '...' describes the attribute.
///
/// all possible attributes:
///
/// - ignore:
///     ignores the field as part of the interface and
///     treats it like a normal struct field.
///
/// ```
/// interface!{
///     /// Doc comment to explain the interface.
///     struct Interface{
///         #[interface(ignore)]
///         ignored: u8,
///         /// Doc comment to explain the function.
///         not_ignored: fn(arg: usize),
///     }
/// }
/// ```
///
#[proc_macro]
pub fn interface(ts: TokenStream) -> TokenStream {
    let parsed = syn::parse::<Struct>(ts).expect("error parsing interface");
    interface_macro(parsed)
}

struct ReturnType {
    arrow: Token![->],
    ty: Type,
}

impl ToTokens for ReturnType {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { arrow, ty } = self;
        tokens.extend(quote! {
            #arrow #ty
        })
    }
}

struct Struct {
    attrs: Vec<Attribute>,
    vis: Visibility,
    struct_token: Token![struct],
    ident: Ident,
    generics: Generics,
    fields: Fields,
}

struct FnArg {
    ident: Ident,
    colon: Token![:],
    ty: Type,
}

enum Field {
    Interface {
        attrs: Vec<Attribute>,
        ident: Ident,
        colon: Token![:],
        fn_token: Token![fn],
        #[allow(unused)]
        parant: syn::token::Paren,
        args: Punctuated<FnArg, Token![,]>,
        return_type: Option<ReturnType>,
    },
    Normal(syn::Field),
}

struct Fields {
    #[allow(unused)]
    brace: syn::token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for Struct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse::<Visibility>()?;
        let struct_token = input.parse::<Token![struct]>()?;
        let ident = input.parse::<Ident>()?;
        let generics = input.parse::<Generics>()?;
        if !generics.params.is_empty() {
            return Err(input.error("Currently does not support generics."));
        }
        let fields = input.parse::<Fields>()?;
        Ok(Self {
            attrs,
            vis,
            struct_token,
            ident,
            generics,
            fields,
        })
    }
}

impl Parse for FnArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.parse::<Ident>()?;
        let colon = input.parse::<Colon>()?;
        let ty = input.parse::<Type>()?;
        Ok(Self { ident, colon, ty })
    }
}

impl Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let mut ignore = false;
        attrs
            .iter()
            .filter(|a| {
                if let Some(ident) = a.path.get_ident() {
                    ident.to_string() == "interface"
                } else {
                    false
                }
            })
            .for_each(|a| {
                let str = a.tokens.to_string();
                if str.len() > 2 {
                    let arg = &str[1..str.len() - 1];
                    match arg {
                        "ignore" => ignore = true,
                        _ => {
                            println!("unused attribute")
                        }
                    }
                }
            });
        Ok(if ignore {
            let mut field = syn::Field::parse_named(&input)?;
            field.attrs = attrs;
            Self::Normal(field)
        } else {
            let ident = input.parse::<Ident>()?;
            let colon = input.parse::<Token![:]>()?;
            let fn_token = input.parse::<Token![fn]>()?;
            let content;
            let parant = syn::parenthesized!(content in input);
            let args = Punctuated::<FnArg, Token![,]>::parse_terminated(&content)?;
            if args.trailing_punct() {
                return Err(input.error("No trailing punctuation allowed."));
            }
            let return_type = if input.peek(Token![->]) {
                let arrow = input.parse::<Token![->]>()?;
                let ty = input.parse::<Type>()?;
                Some(ReturnType { arrow, ty })
            } else {
                None
            };
            Self::Interface {
                attrs,
                ident,
                colon,
                fn_token,
                parant,
                args,
                return_type,
            }
        })
    }
}

impl Parse for Fields {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let brace = syn::braced!(content in input);
        let fields = Punctuated::<Field, Token![,]>::parse_terminated(&content)?;
        Ok(Self { brace, fields })
    }
}

struct DefFnArg<'a>(&'a FnArg);

struct ImplFnArg<'a>(&'a FnArg);

struct DefField<'a>(&'a Field);

struct ImplField<'a>(&'a Field);

struct DefFields<'a>(&'a Fields);

struct ImplFields<'a>(&'a Fields);

struct DefStruct<'a>(&'a Struct);

struct ImplStruct<'a>(&'a Struct);

impl<'a> ToTokens for DefFnArg<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ty = &self.0.ty;
        tokens.extend(quote!(#ty));
    }
}

impl<'a> ToTokens for ImplFnArg<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self(a) = self;
        let FnArg { ident, colon, ty } = a;
        tokens.extend(quote!(#ident #colon #ty));
    }
}

impl<'a> ToTokens for DefField<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self(f) = self;
        tokens.extend(match f {
            Field::Interface {
                attrs,
                ident,
                colon,
                fn_token,
                args,
                return_type,
                ..
            } => {
                let args = args.iter().map(|a| DefFnArg(a));
                quote!(#(#attrs)* pub #ident #colon #fn_token ( #(#args)* ) #return_type)
            }
            Field::Normal(n) => quote!(#n),
        })
    }
}

impl<'a> ToTokens for ImplField<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self(f) = self;
        tokens.extend(match f {
            Field::Interface {
                attrs,
                ident,
                args,
                return_type,
                ..
            } => {
                let idents: Vec<Ident> = args.iter().map(|a| a.ident.clone()).collect();
                let args = args.iter().map(|a| ImplFnArg(a));
                quote! {
                    #(#attrs)*
                    pub fn #ident(&self, #(#args)*) #return_type {
                        (self.#ident)(#(#idents),*)
                    }
                }
            }
            Field::Normal(n) => quote!(#n),
        })
    }
}

impl<'a> ToTokens for DefFields<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self(s) = self;
        let fields = s.fields.iter().map(|f| DefField(f));
        tokens.extend(quote! {
            #(#fields),*
        });
    }
}

impl<'a> ToTokens for ImplFields<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self(s) = self;
        let fields = s.fields.iter().map(|f| ImplField(f));
        tokens.extend(quote! {
            #(#fields)*
        });
    }
}

impl<'a> ToTokens for DefStruct<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self(s) = self;
        let Struct {
            attrs,
            vis,
            struct_token,
            ident,
            generics,
            fields,
        } = s;
        let def_field = DefFields(fields);
        tokens.extend(quote! {
            #(#attrs)*
            #vis #struct_token #ident #generics {
                #def_field
            }
        });
    }
}

impl<'a> ToTokens for ImplStruct<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self(s) = self;
        let Struct { ident, fields, .. } = s;
        let impl_fields = ImplFields(fields);
        tokens.extend(quote! {
            impl #ident{
                #impl_fields
            }
            impl ::hal::IHAL for #ident{}
        });
    }
}

impl ToTokens for Struct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let def = {
            let def_struct = DefStruct(self);
            quote!(#def_struct)
        };
        let imp = {
            let impl_struct = ImplStruct(self);
            quote!(#impl_struct)
        };
        tokens.extend(quote!(#def #imp));
    }
}

fn interface_macro(parsed: Struct) -> TokenStream {
    quote!(
        #parsed
    )
    .into()
}
