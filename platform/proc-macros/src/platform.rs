use quote::quote;

pub trait Arch {}

pub trait Chipset {}

macro_rules! targets {
    (
        chipsets => [$($chipset:ident;)*];
        arches => [
            $($arch:ident => {
                page_size: $arch_kernel_pg_size:literal,
            };)*
        ];
        platforms => [$($target:tt => $p_arch:ident, $p_cs:ident;)*] $(;)?;
    ) => {
        // Chipsets.
        $(
            #[allow(non_camel_case_types)]
            pub struct $chipset;
            impl Chipset for $chipset {}
        )*
        pub fn impl_chipsets() -> proc_macro2::TokenStream{
            quote::quote!{
                $(
                    #[allow(non_camel_case_types)]
                    pub struct $chipset;
                    impl ::platform::platforms::Chipset for $chipset{}
                )*
            }
        }
        // CPUs.
        $(
            #[allow(non_camel_case_types)]
            pub struct $arch;
            impl Arch for $arch{}
        )*
        pub fn impl_arches() -> proc_macro2::TokenStream{
            quote::quote!{
                $(
                    #[allow(non_camel_case_types)]
                    pub struct $arch;
                    impl ::platform::platforms::Arch for $arch{
                        const KERNEL_PG_SIZE:usize = $arch_kernel_pg_size;
                    }
                )*
            }
        }
        // Platforms.
        #[derive(Default)]
        pub struct PlatformsHandler{
            $(
                pub $target: Option<proc_macro2::TokenStream>,
            )*
        }
        impl PlatformsHandler{
            pub fn to_tokens(self, ident: &proc_macro2::Ident) -> proc_macro2::TokenStream{
                let mut impl_vec = vec![];
                $(
                    let name = stringify!($p_arch);
                    if let Some(target) = &self.$target{
                        impl_vec.push({
                            quote::quote!{
                                #[cfg(target_arch = #name)]
                                impl ::platform::Interface<
                                    ::platform::platforms::$p_arch,
                                    ::platform::platforms::$p_cs>
                                for #ident {
                                    #target
                                }
                            }
                        });
                    } else{
                        impl_vec.push({
                            quote::quote!{
                                impl ::platform::Interface<::platform::platforms::$p_arch, ::platform::platforms::$p_cs> for #ident {
                                    fn init(){}

                                    fn on_request() -> ::platform::Requested<Self> {
                                        ::platform::Requested::Unsupported
                                    }
                                }
                            }
                        });
                    }
                )*
                quote::quote!(#(#impl_vec)*)
            }
        }
        pub fn platforms_match(platforms: &mut PlatformsHandler, match_str: &impl AsRef<str>, impl_tokens: proc_macro2::TokenStream){
            let str = match_str.as_ref();
            match str{
                $(
                    stringify!($target) => platforms.$target = Some(impl_tokens),
                )*
                _ => panic!("invalid target {str}"),
            }
        }
        pub fn impl_platforms() -> proc_macro2::TokenStream{
            let mut vec_quotes = vec![];
            $(
                vec_quotes.push({
                    quote::quote!{
                        #[allow(non_camel_case_types)]
                        #[allow(unused)]
                        #[doc(hidden)]
                        pub type $target = ::platform::Platform<$p_arch,$p_cs>;

                        impl PlatformRestrict for $target{}

                        /// Native architecture.
                        pub type NativeArch = $p_arch;
                        /// Native chipset.
                        pub type NativeCS = $p_cs;
                        /// Native platform.
                        pub type NativePlatform = $target;
                    }
                });
            )*
            quote::quote!(#(#vec_quotes)*)
        }
    };
}

targets! {
    chipsets => [
        ibmpc;
    ];
    arches => [
        x86_64 => {
            page_size: 4096,
        };
    ];
    platforms => [
        x86_64_ibmpc => x86_64, ibmpc;
    ];
}

pub fn emit_types() -> proc_macro2::TokenStream {
    quote! {
        pub type ISRHandler = fn(usize);

        macro_rules! handlers{
            ($enum_name:ident => [ $($name:ident),* $(,)? ]) => {
                pub enum $enum_name {
                    $(
                        $name (ISRHandler)
                    ),*
                }
            }
        }

        #[doc(hidden)]
        handlers!{
            ExceptType => [
               PgFault,
                Arch
            ]
        }

        #[doc(hidden)]
        handlers!{
            IRQType => [
                Kbd,
                Arch
            ]
        }
    }
}

pub fn emit_traits() -> proc_macro2::TokenStream {
    quote! {
        pub trait Chipset {}

        pub trait Arch {
            const KERNEL_PG_SIZE: usize;
        }

        #[doc(hidden)]
        pub(crate) trait PlatformImpl{
            /// Register irq function.
            fn reg_irq_fn(ty: IRQType);

            /// Register exception function.
            fn reg_exc_fn(ty: ExceptType);

            /// Initializes the state.
            fn initialize();
        }

        #[doc(hidden)]
        pub(crate) trait PlatformRestrict: PlatformImpl{}
    }
}
