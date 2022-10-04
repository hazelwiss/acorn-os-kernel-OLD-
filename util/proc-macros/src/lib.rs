mod concat_ident;
mod statemachine;

use proc_macro::TokenStream;

/// Creates a simple state machine.
#[proc_macro]
pub fn statemachine(ts: TokenStream) -> TokenStream {
    statemachine::statemachine(ts)
}

/// Concatinates identifiers.
///
/// ```rs
/// let concat_ident!(hello,_son) = 20;
/// assert_eq!(hello_son, 20);
/// ```
#[proc_macro]
pub fn concat_ident(ts: TokenStream) -> TokenStream {
    concat_ident::concat_ident(ts)
}
