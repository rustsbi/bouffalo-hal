use super::{Error, SclPin, SdaPin, register::*};
use crate::glb::{self, v2::I2cClockSource};
use core::ops::Deref;

/// Managed Inter-Integrated Circuit peripheral.
pub struct I2c<I2C, PADS> {
    i2c: I2C,
    pads: PADS,
}

impl<I2C: Deref<Target = RegisterBlock>, SCL, SDA> I2c<I2C, (SCL, SDA)> {
    /// Create a new Inter-Integrated Circuit instance.
    #[inline]
    pub fn new<const I: usize>(i2c: I2C, pads: (SCL, SDA), glb: &glb::v2::RegisterBlock) -> Self
    where
        SCL: SclPin<I>,
        SDA: SdaPin<I>,
    {
        // TODO: support custom clock and frequency
        // Enable clock
        unsafe {
            glb.i2c_config.modify(|config| {
                config
                    .enable_clock()
                    .set_clock_source(I2cClockSource::Xclk)
                    .set_clock_divide(0xff)
            });
            glb.clock_config_1.modify(|config| config.enable_i2c());
            i2c.period_start.write(
                PeriodStart(0)
                    .set_phase(0, 0xff)
                    .set_phase(1, 0xff)
                    .set_phase(2, 0xff)
                    .set_phase(3, 0xff),
            );
            i2c.period_stop.write(
                PeriodStop(0)
                    .set_phase(0, 0xff)
                    .set_phase(1, 0xff)
                    .set_phase(2, 0xff)
                    .set_phase(3, 0xff),
            );
            i2c.period_data.write(
                PeriodData(0)
                    .set_phase(0, 0xff)
                    .set_phase(1, 0xff)
                    .set_phase(2, 0xff)
                    .set_phase(3, 0xff),
            );
            i2c.config.write(
                Config(0)
                    .disable_ten_bit_address()
                    .disable_scl_sync()
                    .disable_sub_address(),
            );
        }

        Self { i2c, pads }
    }

    /// Release the I2C instance and return the pads.
    #[inline]
    pub fn free(self, glb: &glb::v2::RegisterBlock) -> (I2C, (SCL, SDA)) {
        unsafe {
            glb.i2c_config.modify(|config| config.disable_clock());
            glb.clock_config_1.modify(|config| config.disable_i2c());
        }
        (self.i2c, self.pads)
    }

    /// Enable sub-address.
    #[inline]
    pub fn enable_sub_address(&mut self, sub_address: u8) {
        // TODO: support sub-address with more than one byte
        unsafe {
            self.i2c.config.modify(|config| {
                config
                    .enable_sub_address()
                    .set_sub_address_byte_count(SubAddressByteCount::One)
            });
            self.i2c.sub_address.write(sub_address as u32);
        }
    }

    /// Disable sub-address.
    #[inline]
    pub fn disable_sub_address(&mut self) {
        unsafe {
            self.i2c
                .config
                .modify(|config| config.disable_sub_address());
        }
    }
}

impl<I2C: Deref<Target = RegisterBlock>, PADS> embedded_hal::i2c::ErrorType for I2c<I2C, PADS> {
    type Error = Error;
}

impl<I2C: Deref<Target = RegisterBlock>, PADS> embedded_hal::i2c::I2c for I2c<I2C, PADS> {
    #[inline]
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        for op in operations {
            match op {
                embedded_hal::i2c::Operation::Write(_bytes) => {
                    todo!()
                }
                embedded_hal::i2c::Operation::Read(bytes) => {
                    let len = bytes.len() as u8;
                    unsafe {
                        self.i2c.config.modify(|config| {
                            config
                                .set_read_direction()
                                .set_slave_address(address as u16)
                                .set_packet_length(len - 1)
                                .enable_master()
                        })
                    };

                    let mut i = 0;
                    let max_retry = len * 100;
                    let mut retry = 0;
                    while i < len {
                        while self.i2c.fifo_config_1.read().receive_available_bytes() == 0 {
                            retry += 1;
                            if retry >= max_retry {
                                unsafe { self.i2c.config.modify(|config| config.disable_master()) };
                                return Err(Error::Other);
                            }
                        }
                        let word = self.i2c.fifo_read.read();
                        let bytes_to_read = core::cmp::min(len - i, 4);
                        for j in 0..bytes_to_read {
                            bytes[i as usize] = (word >> (j * 8)) as u8;
                            i += 1;
                        }
                    }

                    unsafe { self.i2c.config.modify(|config| config.disable_master()) };
                }
            }
        }
        Ok(())
    }
}
