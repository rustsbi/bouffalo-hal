mod bl808;

use proc_macro2::Ident;
use syn::parse::Error;

pub fn check_interrupt_name(ident: &Ident) -> Option<Error> {
    #[cfg(feature = "bl808-dsp")]
    if !bl808::BL808_DSP_INTERRUPTS.contains(&format!("{}", ident).as_str()) {
        return Some(Error::new(
            ident.span(),
            format!(
                "invalid `#[interrupt]` source. Must be one of: {}.",
                bl808::BL808_DSP_INTERRUPTS.join(", ")
            ),
        ));
    }
    // TODO: support for other chips and contexts
    #[cfg(not(feature = "bl808-dsp"))]
    let _ = ident;
    None
}
