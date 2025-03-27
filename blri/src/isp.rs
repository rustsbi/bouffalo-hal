const GET_BOOT_INFO: u8 = 0x10;
const ERASE_FLASH: u8 = 0x30;
const WRITE_FLASH: u8 = 0x31;

#[derive(thiserror::Error, Debug)]
pub enum IspError {
    #[error("Wrong response length: {wrong_length}")]
    ResponseLength { wrong_length: usize },
}

pub trait IspCommand {
    type Response;
    const COMMAND: u8;
    const RESPONSE_PAYLOAD: bool;
    fn data_size(&self) -> usize;
    fn write_packet_data(&self, buf: &mut [u8]);
    fn parse_response(bytes: &[u8]) -> Result<Self::Response, IspError>;
}

// Ref: https://github.com/pine64/blisp/blob/e45941c45e2418b2bb7e3dab49468a8f4d132439/include/blisp.h#L26
#[repr(C)]
pub struct BootInfo {
    pub boot_rom_version: [u8; 4],
    _reserved1: [u8; 4],
    pub flash_info_from_boot: u32,
    pub chip_id: [u8; 6],
    _reserved2: [u8; 6],
}

impl BootInfo {
    pub fn flash_pin(&self) -> u32 {
        (self.flash_info_from_boot >> 14) & 0x1f
    }
}

pub struct GetBootInfo;

// Ref: https://github.com/pine64/blisp/blob/e45941c45e2418b2bb7e3dab49468a8f4d132439/lib/blisp.c#L234
impl IspCommand for GetBootInfo {
    type Response = BootInfo;
    const COMMAND: u8 = GET_BOOT_INFO;
    const RESPONSE_PAYLOAD: bool = true;
    fn data_size(&self) -> usize {
        0
    }
    fn write_packet_data(&self, buf: &mut [u8]) {
        assert!(buf.len() == 0);
        // nothing to write
    }
    fn parse_response(bytes: &[u8]) -> Result<Self::Response, IspError> {
        if bytes.len() != 24 {
            return Err(IspError::ResponseLength {
                wrong_length: bytes.len(),
            });
        }
        Ok(BootInfo {
            boot_rom_version: bytes[0..4].try_into().unwrap(),
            _reserved1: bytes[4..8].try_into().unwrap(),
            flash_info_from_boot: u32::from_le_bytes(bytes[8..12].try_into().unwrap()),
            chip_id: bytes[12..18].try_into().unwrap(),
            _reserved2: bytes[18..24].try_into().unwrap(),
        })
    }
}

#[repr(C)]
pub struct EraseFlash {
    start: [u8; 4],
    end: [u8; 4],
}

impl EraseFlash {
    pub fn new(start_addr: u32, end_addr: u32) -> Self {
        Self {
            start: start_addr.to_le_bytes(),
            end: end_addr.to_le_bytes(),
        }
    }
}

// Ref: https://github.com/pine64/blisp/blob/e45941c45e2418b2bb7e3dab49468a8f4d132439/lib/blisp.c#L355
impl IspCommand for EraseFlash {
    type Response = ();
    const COMMAND: u8 = ERASE_FLASH;
    const RESPONSE_PAYLOAD: bool = false;
    fn data_size(&self) -> usize {
        8
    }
    fn write_packet_data(&self, buf: &mut [u8]) {
        assert!(buf.len() == 8);
        buf[0..4].clone_from_slice(&self.start);
        buf[4..8].clone_from_slice(&self.end);
    }
    fn parse_response(bytes: &[u8]) -> Result<Self::Response, IspError> {
        assert!(bytes.len() == 0);
        Ok(())
    }
}

#[repr(C)]
pub struct WriteFlash<'a> {
    start: [u8; 4],
    payload: &'a [u8],
}

impl<'a> WriteFlash<'a> {
    pub fn new(start_addr: u32, payload: &'a [u8]) -> Self {
        Self {
            start: start_addr.to_le_bytes(),
            payload,
        }
    }
}

// Ref: https://github.com/pine64/blisp/blob/e45941c45e2418b2bb7e3dab49468a8f4d132439/lib/blisp.c#L372
impl<'a> IspCommand for WriteFlash<'a> {
    type Response = ();
    const COMMAND: u8 = WRITE_FLASH;
    const RESPONSE_PAYLOAD: bool = false;
    fn data_size(&self) -> usize {
        4 + self.payload.len()
    }
    fn write_packet_data(&self, buf: &mut [u8]) {
        buf[0..4].clone_from_slice(&self.start);
        buf[4..].clone_from_slice(&self.payload);
    }
    fn parse_response(bytes: &[u8]) -> Result<Self::Response, IspError> {
        assert!(bytes.len() == 0);
        Ok(())
    }
}
