use proc_macro::TokenStream;

mod isr;

#[proc_macro]
pub fn isr_x86_64_def(ts: TokenStream) -> TokenStream {
    isr::isr_x86_64(ts)
}
