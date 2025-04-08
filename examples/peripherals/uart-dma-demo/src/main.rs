#![no_std]
#![no_main]

use bouffalo_hal::{dma::*, prelude::*, uart::Config};
use bouffalo_rt::{Clocks, Peripherals, entry};
use embedded_time::rate::*;
use panic_halt as _;

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    let tx = p.gpio.io14.into_uart();
    let rx = p.gpio.io15.into_uart();
    let sig2 = p.uart_muxes.sig2.into_transmit::<0>();
    let sig3 = p.uart_muxes.sig3.into_receive::<0>();
    let pads = ((tx, sig2), (rx, sig3));
    let mut led = p.gpio.io8.into_floating_output();

    let config = Config::default().set_baudrate(2000000.Bd());
    let mut serial = p.uart0.freerun(config, pads, &c).unwrap().enable_tx_dma();
    let tx_config = DmaChannelConfig {
        direction: DmaMode::Mem2Periph,
        src_req: DmaPeriphReq::None,
        dst_req: DmaPeriphReq::Dma01(Periph4Dma01::Uart0Tx),
        src_addr_inc: true,
        dst_addr_inc: false,
        src_burst_size: BurstSize::INCR1,
        dst_burst_size: BurstSize::INCR1,
        src_transfer_width: TransferWidth::Byte,
        dst_transfer_width: TransferWidth::Byte,
    };
    let dma0_ch0 = Dma::new(&p.dma0, Dma0Channel0, tx_config, &p.glb);
    let tx_lli_pool = &mut [LliPool::new(); 1];
    let hello = b"Welcome to Universal Asynchronous Receiver/Transmitter with Direct Memory Access demo!\r\nHello world!";
    let hello_ptr = hello.as_ptr();
    let hello_len = hello.len();
    let tx_transfer = &mut [LliTransfer {
        src_addr: hello_ptr as u32,
        dst_addr: DmaAddr::Uart0Tx as u32,
        nbytes: hello_len as u32,
    }];
    if dma0_ch0.lli_reload(tx_lli_pool, 1, tx_transfer, 1) == -12 {
        writeln!(serial, "Out of memory.").unwrap();
        loop {
            riscv::asm::delay(20_000);
            led.set_high().ok();
            riscv::asm::delay(20_000);
            led.set_low().ok();
        }
    }
    dma0_ch0.start();
    // TODO: use interrupt to know when DMA transfer is done.
    // Wait for transfer to complete.
    while dma0_ch0.is_busy() {
        core::hint::spin_loop();
    }

    dma0_ch0.stop();

    loop {
        led.set_low().ok();
        riscv::asm::delay(100_000);
        led.set_high().ok();
        riscv::asm::delay(100_000);
    }
}
