use crate::{glb, hbn};

pub fn delay(tim: u32) {
    unsafe {
        for _ in 0..tim * 5 {
            core::arch::asm!("nop");
        }
    }
}

/// Set glb clock.
// #[cfg(any(feature = "bl702", feature = "bl602"))]
pub fn set_glb_clock(clock: u32) {
    unsafe {
        core::ptr::write_volatile(0x4000F108 as *mut u32, clock);
    }
}

/// Get glb clock.
// #[cfg(any(feature = "bl702", feature = "bl602"))]
pub fn get_glb_clock() -> u32 {
    unsafe { core::ptr::read_volatile(0x4000F108 as *const u32) }
}

/// Glb clock set system clock divide.
// #[cfg(any(feature = "bl702", feature = "bl602"))]
pub fn glb_clock_set_system_clock(bdiv: u8, hdiv: u8, glb: &glb::v1::RegisterBlock) {
    unsafe {
        glb.clock_config0
            .modify(|v| v.set_bclk_div(bdiv).set_hclk_div(hdiv));
        glb.clock_config0.modify(|v| v.enable_bclk());
        // Only bl702 and bl602 use this function, and some of their address is the same,
        // so writing directly to this address is acceptable.
        core::ptr::write_volatile(0x40000FFC as *mut u32, 0x1);
        core::ptr::write_volatile(0x40000FFC as *mut u32, 0x0);

        set_glb_clock(get_glb_clock() / (hdiv as u32 + 1));

        for _ in 0..8 {
            core::arch::asm!("nop");
        }

        glb.clock_config0.modify(|v| v.enable_hclk().enable_bclk());

        for _ in 0..8 {
            core::arch::asm!("nop");
        }
    }
}

/// Efuse read write switch cpu clock save
pub fn efuse_switch_cpu_clock_save(
    glb: &glb::v1::RegisterBlock,
    hbn: &hbn::RegisterBlock,
) -> (u8, u8, glb::v1::GlbRootClk, hbn::RootClockSource) {
    let glb_cfg = glb.clock_config0.read();
    let hbn_cfg = hbn.global.read();
    let (bdiv, hdiv, rt_clk, xclk) = (
        glb_cfg.bclk_div(),
        glb_cfg.hclk_div(),
        glb_cfg.hbn_root_clk_sel(),
        hbn_cfg.root_clock(),
    );

    unsafe {
        hbn.global
            .modify(|v| v.set_root_clock(hbn::RootClockSource::RC32M));
        glb_clock_set_system_clock(0, 0, glb);
    }
    (bdiv, hdiv, rt_clk, xclk)
}

///  Efuse read write switch cpu clock restore.
pub fn efuse_switch_cpu_clock_restore(
    bdiv: u8,
    hdiv: u8,
    rt_clk: glb::v1::GlbRootClk,
    xclk: hbn::RootClockSource,
    glb: &glb::v1::RegisterBlock,
    hbn: &hbn::RegisterBlock,
) {
    unsafe {
        glb_clock_set_system_clock(bdiv, hdiv, glb);
        glb.clock_config0.modify(|v| v.set_hbn_root_clk_sel(rt_clk));
        hbn.global.modify(|v| v.set_root_clock(xclk));
    }
}
