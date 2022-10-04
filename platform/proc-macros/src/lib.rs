mod component;
mod keys;
mod platform;

use proc_macro::TokenStream;
use quote::quote;

/// Creates a set of supported platforms/targets,
/// as well as a NativePlatform type which
/// corresponds to the current platform.
#[proc_macro]
pub fn targets(_: TokenStream) -> TokenStream {
    let types = platform::emit_types();
    let traits = platform::emit_traits();
    let chipsets = platform::impl_chipsets();
    let cpus = platform::impl_arches();
    let platforms = platform::impl_platforms();
    quote!(#types #traits #chipsets #cpus #platforms).into()
}

/// Creates a component used by the platform crate.
/// ```rs
/// proc_macro_platform::component!{
///     /// This is a doc comment!
///     Shell => hal::IShell impl {
///         amd64_ibmpc => {
///             fn init(){
///                 panic!("Init function for the AMD64-IBMPC platform")
///             }
///
///             fn on_request<'a, C: Interface<CPU, CS>>() -> Requested<'a, C>{
///                 panic!("Requesring component on the given platform!")
///             }
///         }
///     }
/// }
/// ```
#[proc_macro]
pub fn component(ts: TokenStream) -> TokenStream {
    component::component(ts)
}

/// Simplifies the process of creating keycodes.
#[proc_macro]
pub fn keys(ts: TokenStream) -> TokenStream {
    keys::keys(ts)
}
