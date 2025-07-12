use super::config::*;
use super::error::Error;
use heapless::String;
use serde::Serialize;
use serde_device_tree::{
    Dtb, DtbPtr,
    buildin::{Node, StrSeq},
    error::{Error as DtbError, ErrorType},
    from_raw_mut,
    ser::{patch::Patch, serializer::ValueType, to_dtb},
};

#[derive(Serialize)]
struct Chosen<'a> {
    pub bootargs: &'a String<128>,
}

/// Set the bootargs of the dtb.
pub fn set_bootargs(new_bootargs: &String<128>) -> Result<(), Error<()>> {
    match DtbPtr::from_raw(OPAQUE_ADDRESS as *mut u8) {
        Ok(ptr) => {
            // Get the aligned size of the dtb.
            let size = ptr.align();
            let dtb = Dtb::from(ptr).share();
            let root: Node = from_raw_mut(&dtb).map_err(|_| Error::InvalidDTB)?;
            let chosen = Chosen {
                bootargs: new_bootargs,
            };
            // Ensure the chosen node exists.
            let patch = if root.chosen().is_none() {
                Patch::new("/chosen", &chosen, ValueType::Node)
            } else {
                Patch::new("/chosen/bootargs", new_bootargs, ValueType::Prop)
            };
            let patches = [patch];
            let mut temp_buffer =
                unsafe { core::slice::from_raw_parts_mut(FIRMWARE_ADDRESS as *mut u8, size * 2) };
            to_dtb(&root, &patches, &mut temp_buffer).map_err(|_| Error::InvalidDTB)?;
            let target =
                unsafe { core::slice::from_raw_parts_mut(OPAQUE_ADDRESS as *mut u8, size) };
            target.copy_from_slice(&temp_buffer[..size]);

            Ok(())
        }
        Err(DtbError::Typed {
            error_type: ErrorType::InvalidMagic { wrong_magic },
            ..
        }) => Err(Error::InvalideMagic(wrong_magic)),
        Err(_) => Err(Error::InvalidDTB),
    }
}

/// Get the bootargs of the dtb.
pub fn get_bootargs() -> Result<&'static str, ()> {
    match DtbPtr::from_raw(OPAQUE_ADDRESS as *mut u8) {
        Ok(ptr) => {
            let dtb = Dtb::from(ptr).share();
            let root: Node = from_raw_mut(&dtb).map_err(|_| ())?;
            let result = root
                .chosen()
                .ok_or(())?
                .get_prop("bootargs")
                .ok_or(())?
                .deserialize::<StrSeq>()
                .iter()
                .next()
                .ok_or(())?;
            if let Some(pos) = result.find(':') {
                return Ok(result.split_at(pos).0);
            } else {
                return Err(());
            }
        }
        Err(_) => Err(()),
    }
}
