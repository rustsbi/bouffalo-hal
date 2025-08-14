use super::{Error, Numbered, pads::IntoPads, register::*};
use crate::{
    glb::{self, v2::I2cClockSource},
    gpio::FlexPad,
};
use core::marker::PhantomData;
use embedded_time::rate::Hertz;

/// Managed Inter-Integrated Circuit peripheral.
pub struct I2c<'a> {
    i2c: &'a super::RegisterBlock,
    _pads: PhantomData<FlexPad<'a>>,
}

impl<'a> I2c<'a> {
    /// Create a new Inter-Integrated Circuit instance.
    #[inline]
    pub fn new<const I: usize>(
        i2c: impl Numbered<'a, I>,
        pads: impl IntoPads<'a, I>,
        frequency: Hertz,
        glb: &glb::v2::RegisterBlock,
    ) -> Self {
        // TODO: use a fixed peripheral clock for now, should be configurable later
        const I2C_PCLK_HZ: u32 = 38_000_000;

        let i2c = i2c.register_block();
        unsafe {
            // Enable I2C clock and select clock source
            glb.i2c_config.modify(|config| {
                config
                    .enable_clock()
                    .set_clock_source(I2cClockSource::Xclk)
                    .set_clock_divide(0)
            });
            glb.clock_config_1.modify(|config| config.enable_i2c());

            // Basic setup: 7-bit address, enable SCL sync, disable sub-address, enable deglitch
            i2c.config.write(
                Config(0)
                    .disable_ten_bit_address()
                    .enable_scl_sync()
                    .disable_sub_address()
                    .enable_deglitch(),
            );

            let freq_hz = frequency.0 as u32;
            let mut phase = (I2C_PCLK_HZ + freq_hz / 2) / freq_hz;
            if phase == 0 {
                phase = 1;
            }

            // Duty cycle: 50% when <= 100 kHz; ~33% otherwise
            let (mut ph0, mut ph1, mut ph2, mut ph3) = if freq_hz <= 100_000 {
                let p0 = (phase + 2) / 4;
                let p1 = p0;
                let p2 = phase / 2 - p0;
                let p3 = phase - p0 - p1 - p2;
                (p0 as i32, p1 as i32, p2 as i32, p3 as i32)
            } else {
                let p0 = (phase + 2) / 3;
                let p1 = (phase + 3) / 6;
                let p2 = (phase + 1) / 3 - p1;
                let p3 = phase - p0 - p1 - p2;
                (p0 as i32, p1 as i32, p2 as i32, p3 as i32)
            };

            // Compute bias from deglitch and scl sync
            let mut bias = i2c.config.read().get_deglitch_cycle_count() as i32;
            if i2c.config.read().is_scl_sync_enabled() {
                bias += 3;
            }

            // Clamp to [1, 256]
            let clamp_1_256 = |x: i32| -> i32 {
                if x <= 0 {
                    1
                } else if x >= 256 {
                    256
                } else {
                    x
                }
            };
            ph0 = clamp_1_256(ph0);
            ph1 = clamp_1_256(ph1.max(bias + 1)); // data phase1 must be > bias
            ph2 = clamp_1_256(ph2);
            ph3 = clamp_1_256(ph3);

            // Data phases are "minus one" encoded; d1 must be >= 1 after bias
            let d0 = clamp_1_256(ph0 - 1);
            let d1 = clamp_1_256((ph1 - bias) - 1).max(1);
            let d2 = clamp_1_256(ph2 - 1);
            let d3 = clamp_1_256(ph3 - 1);

            // Start phases
            let s0 = clamp_1_256(ph0 - 1);
            let s1 = (ph0 + ph3 - 1).min(255);
            let s2 = (ph1 + ph2 - 1).min(255);
            let s3 = clamp_1_256(ph3 - 1);

            // Stop phases
            let p0w = clamp_1_256(ph0 - 1);
            let p1w = (ph1 + ph2 - 1).min(255);
            let p2w = clamp_1_256(ph0 - 1);
            let p3w = clamp_1_256(ph3 - 1);

            // Write timing registers
            i2c.period_data.write(
                PeriodData(0)
                    .set_phase(0, d0 as u8)
                    .set_phase(1, d1 as u8)
                    .set_phase(2, d2 as u8)
                    .set_phase(3, d3 as u8),
            );
            i2c.period_start.write(
                PeriodStart(0)
                    .set_phase(0, s0 as u8)
                    .set_phase(1, s1 as u8)
                    .set_phase(2, s2 as u8)
                    .set_phase(3, s3 as u8),
            );
            i2c.period_stop.write(
                PeriodStop(0)
                    .set_phase(0, p0w as u8)
                    .set_phase(1, p1w as u8)
                    .set_phase(2, p2w as u8)
                    .set_phase(3, p3w as u8),
            );
        }

        let pads = pads.into_i2c_pads();
        core::mem::forget(pads);
        Self {
            i2c,
            _pads: PhantomData,
        }
    }

    /// Release the I2C instance and disable the I2C clock.
    #[inline]
    pub fn free(self, glb: &glb::v2::RegisterBlock) {
        unsafe {
            glb.i2c_config.modify(|config| config.disable_clock());
            glb.clock_config_1.modify(|config| config.disable_i2c());
        }
    }

    /// Enable sub-address mode and set sub-address value (supports up to 4 bytes).
    #[inline]
    pub fn enable_sub_address(&mut self, sub_address: &[u8]) {
        let count = sub_address.len();
        if count == 0 {
            unsafe {
                self.i2c
                    .config
                    .modify(|config| config.disable_sub_address());
            }
            return;
        }
        unsafe {
            self.i2c.config.modify(|config| {
                config
                    .enable_sub_address()
                    .set_sub_address_byte_count(match count {
                        1 => SubAddressByteCount::One,
                        2 => SubAddressByteCount::Two,
                        3 => SubAddressByteCount::Three,
                        _ => SubAddressByteCount::Four,
                    })
            });
            // Pack sub-address bytes into u32
            let mut val = 0u32;
            for (i, b) in sub_address.iter().enumerate().take(4) {
                val |= (*b as u32) << (8 * i);
            }
            self.i2c.sub_address.write(val);
        }
    }

    /// Disable sub-address mode.
    #[inline]
    fn disable_sub_address(&mut self) {
        unsafe {
            self.i2c
                .config
                .modify(|config| config.disable_sub_address());
        }
    }
}

impl<'a> embedded_hal::i2c::ErrorType for I2c<'a> {
    type Error = Error;
}

impl<'a> embedded_hal::i2c::I2c for I2c<'a> {
    #[inline]
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        let mut op_iter = operations.iter_mut().peekable();
        while let Some(op) = op_iter.next() {
            match op {
                // Regular write operation
                embedded_hal::i2c::Operation::Write(bytes) => {
                    // If this is a write_read scenario (next op is Read and Write length <= 4), treat Write as sub-address
                    let is_sub_addr =
                        if let Some(embedded_hal::i2c::Operation::Read(_)) = op_iter.peek() {
                            bytes.len() <= 4
                        } else {
                            false
                        };

                    if is_sub_addr {
                        self.enable_sub_address(bytes);
                    } else {
                        self.disable_sub_address();
                    }

                    while self.i2c.bus_busy.read().is_bus_busy() {
                        core::hint::spin_loop();
                    }

                    let len = bytes.len() as u8;
                    unsafe {
                        self.i2c.config.modify(|config| {
                            config
                                .set_write_direction()
                                .set_slave_address(address as u16)
                                .set_packet_length(len - 1)
                                .enable_master()
                        });
                    }

                    let mut i = 0;
                    while i < len {
                        let mut word = 0u32;
                        let bytes_to_write = core::cmp::min(len - i, 4);
                        for j in 0..bytes_to_write {
                            word |= (bytes[i as usize] as u32) << (j * 8);
                            i += 1;
                        }
                        unsafe {
                            self.i2c.fifo_write.write(word);
                        }
                    }

                    while self.i2c.bus_busy.read().is_bus_busy() {
                        core::hint::spin_loop();
                    }
                    unsafe { self.i2c.config.modify(|config| config.disable_master()) };

                    // If just set sub-address for write_read, keep sub-address mode enabled for next Read
                    if !is_sub_addr {
                        self.disable_sub_address();
                    }
                }
                embedded_hal::i2c::Operation::Read(bytes) => {
                    // Read exactly 'bytes' length
                    let len = bytes.len();
                    if len == 0 {
                        continue;
                    }
                    if len > 256 {
                        // Hardware limit: 256 bytes per transfer
                        return Err(Error::Other);
                    }

                    // Wait until bus is idle
                    while self.i2c.bus_busy.read().is_bus_busy() {
                        core::hint::spin_loop();
                    }

                    // Configure read direction and length (register expects len-1)
                    unsafe {
                        self.i2c.config.modify(|config| {
                            config
                                .set_read_direction()
                                .set_slave_address(address as u16)
                                .set_packet_length((len as u16 - 1) as u8)
                                .enable_master()
                        });
                    }

                    // Pull RX FIFO in 32-bit words and split into bytes
                    let mut done = 0usize;
                    let mut retry: u32 = 0;
                    const RETRY_MAX: u32 = 5_000_00;
                    while done < len {
                        // Wait until RX FIFO has data
                        while self.i2c.fifo_config_1.read().receive_available_bytes() == 0 {
                            retry += 1;
                            if retry >= RETRY_MAX {
                                unsafe { self.i2c.config.modify(|c| c.disable_master()) };
                                self.disable_sub_address();
                                return Err(Error::Other);
                            }
                            core::hint::spin_loop();
                        }
                        let word = self.i2c.fifo_read.read();
                        let chunk = core::cmp::min(4, len - done);
                        for j in 0..chunk {
                            bytes[done + j] = ((word >> (8 * j as u32)) & 0xFF) as u8;
                        }
                        done += chunk;
                    }

                    // Wait for completion and finalize
                    while self.i2c.bus_busy.read().is_bus_busy() {
                        core::hint::spin_loop();
                    }
                    unsafe { self.i2c.config.modify(|config| config.disable_master()) };
                    // If previous op enabled sub-address for write-read, disable it now
                    self.disable_sub_address();
                }
            }
        }
        Ok(())
    }
}
