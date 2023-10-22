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

    #[cfg(feature = "bl808-dsp")]
    if !BL808_DSP_INTERRUPTS.contains(&format!("{}", f.sig.ident).as_str()) {
        return parse::Error::new(
            f.sig.ident.span(),
            format!(
                "invalid `#[interrupt]` source. Must be one of: {}.",
                BL808_DSP_INTERRUPTS.join(", ")
            ),
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

#[cfg(feature = "bl808-dsp")]
const BL808_DSP_INTERRUPTS: [&'static str; 67] = [
    "bmx_dsp_bus_err",
    "dsp_reserved1",
    "dsp_reserved2",
    "dsp_reserved3",
    "uart3",
    "i2c2",
    "i2c3",
    "spi1",
    "dsp_reserved4",
    "dsp_reserved5",
    "seof_int0",
    "seof_int1",
    "seof_int2",
    "dvp2_bus_int0",
    "dvp2_bus_int1",
    "dvp2_bus_int2",
    "dvp2_bus_int3",
    "h264_bs",
    "h264_frame",
    "h264_seq_done",
    "mjpeg",
    "h264_s_bs",
    "h264_s_frame",
    "h264_s_seq_done",
    "dma2_int0",
    "dma2_int1",
    "dma2_int2",
    "dma2_int3",
    "dma2_int4",
    "dma2_int5",
    "dma2_int6",
    "dma2_int7",
    "dsp_reserved6",
    "dsp_reserved7",
    "dsp_reserved8",
    "dsp_reserved9",
    "dsp_reserved10",
    "mipi_csi",
    "ipc_d0",
    "dsp_reserved11",
    "mjdec",
    "dvp2_bus_int4",
    "dvp2_bus_int5",
    "dvp2_bus_int6",
    "dvp2_bus_int7",
    "dma2_d_int0",
    "dma2_d_int1",
    "display",
    "pwm",
    "seof_int3",
    "dsp_reserved12",
    "dsp_reserved13",
    "osd",
    "dbi",
    "dsp_reserved14",
    "osda_bus_drain",
    "osdb_bus_drain",
    "osd_pb",
    "dsp_reserved15",
    "mipi_dsi",
    "dsp_reserved16",
    "timer0",
    "timer1",
    "wdt",
    "audio",
    "wl_all",
    "pds",
];
