mod soc;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{ItemFn, ReturnType, Type, Visibility, parse, parse_macro_input};

/// ROM runtime function entry.
#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "#[entry] attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    let f = parse_macro_input!(input as ItemFn);

    if f.sig.inputs.len() != 2 {
        return parse::Error::new(
            f.sig.inputs.span(),
            "`#[entry]` function should include exactly two parameters",
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
    let inputs = f.sig.inputs;

    quote!(
        #[unsafe(no_mangle)]
        pub extern "C" fn main() -> ! {
            let (p, c) = bouffalo_rt::__rom_init_params(40_000_000);
            unsafe { __bouffalo_rt_macros__main(p, c) }
        }
        #[allow(non_snake_case)]
        #[inline(always)]
        #(#attrs)*
        #unsafety fn __bouffalo_rt_macros__main(#inputs) -> ! {
            #(#stmts)*
        }
    )
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

    if let Some(syntax_err) = soc::check_interrupt_name(&f.sig.ident) {
        return syntax_err.to_compile_error().into();
    }

    let attrs = f.attrs;
    let unsafety = f.sig.unsafety;
    let stmts = f.block.stmts;
    let ident = f.sig.ident;
    let output = f.sig.output;

    quote!(
        #(#attrs)*
        #[unsafe(no_mangle)]
        pub #unsafety extern "C" fn #ident() #output {
            #(#stmts)*
        }
    )
    .into()
}

/// Exception handler function.
#[proc_macro_attribute]
pub fn exception(args: TokenStream, input: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return parse::Error::new(
            Span::call_site(),
            "#[exception] attribute accepts no arguments",
        )
        .to_compile_error()
        .into();
    }

    let f = parse_macro_input!(input as ItemFn);

    if f.sig.inputs.len() != 1 {
        return parse::Error::new(
            f.sig.inputs.span(),
            "`#[exception]` function should include exactly one parameter",
        )
        .to_compile_error()
        .into();
    }

    let valid_signature = f.sig.constness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
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
            "`#[exception]` handlers must have signature `[unsafe] fn(&mut TrapFrame) [-> !]`",
        )
        .to_compile_error()
        .into();
    }

    let attrs = f.attrs;
    let unsafety = f.sig.unsafety;
    let stmts = f.block.stmts;
    let ident = f.sig.ident;
    let output = f.sig.output;
    let inputs = f.sig.inputs;

    // FIXME: check input type of arguments

    quote!(
        #(#attrs)*
        #[unsafe(export_name = "exceptions")]
        pub #unsafety extern "C" fn #ident(#inputs) #output {
            #(#stmts)*
        }
    )
    .into()
}
