use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse, parse_macro_input, ItemFn, ReturnType, Type, Visibility};

/// ROM runtime function entry.
#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "#[entry] attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    let f = parse_macro_input!(input as ItemFn);

    #[cfg(not(feature = "rom-peripherals"))]
    if f.sig.inputs.len() != 0 {
        return parse::Error::new(
            f.sig.inputs.span(),
            "`#[entry]` function without rom peripherals should not include any parameter",
        )
        .to_compile_error()
        .into();
    }

    #[cfg(feature = "rom-peripherals")]
    if f.sig.inputs.len() != 2 {
        return parse::Error::new(
            f.sig.inputs.span(),
            "`#[entry]` function with rom peripherals should include exactly two parameters",
        )
        .to_compile_error()
        .into();
    }

    let valid_signature = f.sig.constness.is_none()
        && f.sig.asyncness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && matches!(f.sig.output, ReturnType::Type(_, ref t) if matches!(t.as_ref(), &Type::Never(_)));

    if !valid_signature {
        #[cfg(not(feature = "rom-peripherals"))]
        return parse::Error::new(
            f.sig.span(),
            "`#[entry]` function must have signature `[unsafe] fn() -> !`",
        )
        .to_compile_error()
        .into();
        #[cfg(feature = "rom-peripherals")]
        return parse::Error::new(
            f.sig.span(),
            "`#[entry]` function must have signature `[unsafe] fn(p: Peripherals, c: Clocks) -> !`",
        )
        .to_compile_error()
        .into();
    }

    let attrs = f.attrs;
    let unsafety = f.sig.unsafety;
    let stmts = f.block.stmts;
    #[cfg(feature = "rom-peripherals")]
    let inputs = f.sig.inputs;

    match () {
        #[cfg(not(feature = "rom-peripherals"))]
        () => quote!(
            #[allow(non_snake_case)]
            #[export_name = "main"]
            #(#attrs)*
            pub #unsafety fn main() -> ! {
                #(#stmts)*
            }
        ),
        #[cfg(feature = "rom-peripherals")]
        () => quote!(
            #[export_name = "main"]
            pub extern "C" fn main() -> ! {
                let p = unsafe { core::mem::transmute(()) };
                let c = bl_rom_rt::__new_clocks(40_000_000);
                unsafe { __bl_rom_rt_macros__main(p, c) }
            }
            #[allow(non_snake_case)]
            #[inline(always)]
            #(#attrs)*
            #unsafety fn __bl_rom_rt_macros__main(#inputs) -> ! {
                #(#stmts)*
            }
        ),
    }
    .into()
}

/// Interrupt handler function.
#[proc_macro_attribute]
pub fn interrupt(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return parse::Error::new(
            Span::call_site(),
            "#[interrupt] attribute accepts no arguments",
        )
        .to_compile_error()
        .into();
    }

    let f = parse_macro_input!(input as ItemFn);

    if f.sig.inputs.len() != 0 {
        return parse::Error::new(
            f.sig.inputs.span(),
            "`#[interrupt]` function should not include any parameter",
        )
        .to_compile_error()
        .into();
    }

    let valid_signature = f.sig.constness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.inputs.is_empty()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            ReturnType::Default => true,
            ReturnType::Type(_, ref ty) => match **ty {
                Type::Tuple(ref tuple) => tuple.elems.is_empty(),
                Type::Never(..) => true,
                _ => false,
            },
        };

    if !valid_signature {
        return parse::Error::new(
            f.sig.span(),
            "`#[interrupt]` handlers must have signature `[unsafe] fn() [-> !]`",
        )
        .to_compile_error()
        .into();
    }

    let attrs = f.attrs;
    let unsafety = f.sig.unsafety;
    let stmts = f.block.stmts;
    let ident = f.sig.ident;
    let output = f.sig.output;

    quote!(
        #(#attrs)*
        #[no_mangle]
        pub #unsafety extern "C" fn #ident() #output {
            #(#stmts)*
        }
    )
    .into()
}
