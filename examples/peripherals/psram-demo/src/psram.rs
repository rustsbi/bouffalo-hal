//! psram

use crate::*;

const GLB_BASE: u32 = 0x20000000;
const PSRAM_UHS_BASE: u32 = 0x3000f000;
const PDS_BASE: u32 = 0x2000e000;
const PLL_BASE_ADDRESS: u32 = GLB_BASE + 0x7D0;

pub(crate) fn uhs_psram_init() {
    glb_config_uhs_pll();
    psram_uhs_x16_init();
}

// -------------glb_config_uhs_pll

fn glb_config_uhs_pll() {
    glb_power_off_pll();
    glb_clock_select_pll();
    glb_power_on_pll();
}

fn glb_power_off_pll() {
    /* cfg0 : pu_aupll=0 */
    let mut tmp_val = read_memory(PLL_BASE_ADDRESS + 4 * 0);
    tmp_val = set_bits(tmp_val, 10, 1, 0);
    write_memory(PLL_BASE_ADDRESS + 4 * 0, tmp_val);
    /* cfg0 : pu_aupll_sfreg=0 */
    let mut tmp_val = read_memory(PLL_BASE_ADDRESS + 4 * 0);
    tmp_val = set_bits(tmp_val, 9, 1, 0);
    write_memory(PLL_BASE_ADDRESS + 4 * 0, tmp_val);
}

fn glb_power_on_pll() {
    /* cfg1:Set aupll_refclk_sel and aupll_refdiv_ratio */
    let mut tmp_val = read_memory(PLL_BASE_ADDRESS + 4 * 1);
    tmp_val = set_bits(tmp_val, 8, 4, 2);
    write_memory(PLL_BASE_ADDRESS + 4 * 1, tmp_val);

    /* cfg4:Set aupll_sel_sample_clk */
    let mut tmp_val = read_memory(PLL_BASE_ADDRESS + 4 * 4);
    tmp_val = set_bits(tmp_val, 0, 2, 2);
    write_memory(PLL_BASE_ADDRESS + 4 * 4, tmp_val);

    /* cfg5:Set aupll_vco_speed */
    let mut tmp_val = read_memory(PLL_BASE_ADDRESS + 4 * 5);
    tmp_val = set_bits(tmp_val, 0, 3, 7);
    write_memory(PLL_BASE_ADDRESS + 4 * 5, tmp_val);

    /* cfg1: uhspll_even_div_en and uhspll_even_div_ratio */
    let mut tmp_val = read_memory(PLL_BASE_ADDRESS + 4 * 1);
    tmp_val = set_bits(tmp_val, 7, 1, 1);
    tmp_val = set_bits(tmp_val, 0, 7, 2000 / 50);
    write_memory(PLL_BASE_ADDRESS + 4 * 1, tmp_val);

    /* cfg6:Set aupll_sdm_bypass,aupll_sdmin */
    let mut tmp_val = read_memory(PLL_BASE_ADDRESS + 4 * 6);
    tmp_val = set_bits(tmp_val, 0, 19, 0x32000);
    write_memory(PLL_BASE_ADDRESS + 4 * 6, tmp_val);

    /* Step2:config pu */
    /* cfg0 : pu_aupll_sfreg=1 */
    let mut tmp_val = read_memory(PLL_BASE_ADDRESS + 4 * 0);
    tmp_val = set_bits(tmp_val, 0, 19, 1);
    write_memory(PLL_BASE_ADDRESS + 4 * 0, tmp_val);

    sleep_us(3);

    /* cfg0 : pu_wifipll=1 */
    let mut tmp_val = read_memory(PLL_BASE_ADDRESS + 4 * 0);
    tmp_val = set_bits(tmp_val, 10, 1, 1);
    write_memory(PLL_BASE_ADDRESS + 4 * 0, tmp_val);

    sleep_us(3);

    /* cfg0 : aupll_sdm_reset */
    let mut tmp_val = read_memory(PLL_BASE_ADDRESS + 4 * 0);
    tmp_val = set_bits(tmp_val, 0, 1, 1);
    write_memory(PLL_BASE_ADDRESS + 4 * 0, tmp_val);
    sleep_us(2);
    let mut tmp_val = read_memory(PLL_BASE_ADDRESS + 4 * 0);
    tmp_val = set_bits(tmp_val, 0, 1, 0);
    write_memory(PLL_BASE_ADDRESS + 4 * 0, tmp_val);
    sleep_us(2);
    let mut tmp_val = read_memory(PLL_BASE_ADDRESS + 4 * 0);
    tmp_val = set_bits(tmp_val, 0, 1, 1);
    write_memory(PLL_BASE_ADDRESS + 4 * 0, tmp_val);

    /* Step3:reset pll */
    let mut tmp_val = read_memory(PLL_BASE_ADDRESS + 4 * 0);
    tmp_val = set_bits(tmp_val, 2, 1, 1);
    write_memory(PLL_BASE_ADDRESS + 4 * 0, tmp_val);
    sleep_us(2);
    let mut tmp_val = read_memory(PLL_BASE_ADDRESS + 4 * 0);
    tmp_val = set_bits(tmp_val, 2, 1, 0);
    write_memory(PLL_BASE_ADDRESS + 4 * 0, tmp_val);
    sleep_us(2);
    let mut tmp_val = read_memory(PLL_BASE_ADDRESS + 4 * 0);
    tmp_val = set_bits(tmp_val, 2, 1, 1);
    write_memory(PLL_BASE_ADDRESS + 4 * 0, tmp_val);

    sleep_us(45);
}

fn glb_clock_select_pll() {
    /* xxxpll_refclk_sel */
    let mut tmp_val = read_memory(PLL_BASE_ADDRESS + 4 * 1);
    tmp_val = set_bits(tmp_val, 16, 2, 0);
    write_memory(PLL_BASE_ADDRESS + 4 * 1, tmp_val);
}

// -------------psram_uhs_x16_init

fn psram_uhs_x16_init() {
    psram_uhs_init();
    uhs_phy_init();
}

fn psram_uhs_init() {
    write_memory(PSRAM_UHS_BASE + 0x30, 0x1a03000f);

    psram_analog_init();

    sleep_us(150);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0xC);
    tmp_val &= 0x00ffffff;
    tmp_val |= 0x04000000;
    write_memory(PSRAM_UHS_BASE + 0xC, tmp_val);

    write_memory(PSRAM_UHS_BASE + 0x10, 0x16e360);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x14);
    tmp_val = set_bits(tmp_val, 0, 16, 370);
    write_memory(PSRAM_UHS_BASE + 0x14, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x1C);
    tmp_val = set_bits(tmp_val, 0, 7, 5);
    write_memory(PSRAM_UHS_BASE + 0x1C, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x0);
    tmp_val = set_bits(tmp_val, 16, 8, 0x3f);
    tmp_val = set_bits(tmp_val, 28, 4, 0x0B);
    write_memory(PSRAM_UHS_BASE + 0x0, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x0);
    tmp_val = set_bits(tmp_val, 0, 1, 1);
    write_memory(PSRAM_UHS_BASE + 0x0, tmp_val);
}

fn psram_analog_init() {
    /* power on ldo12 power_up_ldo12uhs */
    let mut tmp_val = read_memory(GLB_BASE + 0x6D0);
    tmp_val = set_bits(tmp_val, 0, 1, 1);
    write_memory(GLB_BASE + 0x6D0, tmp_val);
    sleep_us(300);
    let mut tmp_val = read_memory(GLB_BASE + 0x6D0);
    tmp_val = set_bits(tmp_val, 20, 4, 6);
    write_memory(GLB_BASE + 0x6D0, tmp_val);
    sleep_us(1);

    /* set cen ck ckn set_cen_ck_ckn */
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x150);
    tmp_val = set_bits(tmp_val, 12, 3, 0);
    tmp_val = set_bits(tmp_val, 8, 3, 0);
    write_memory(PSRAM_UHS_BASE + 0x150, tmp_val);
    sleep_us(1);
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x140);
    tmp_val &= 0xFFFCFFFF;
    tmp_val = set_bits(tmp_val, 8, 8, 1);
    write_memory(PSRAM_UHS_BASE + 0x140, tmp_val);
    sleep_us(1);

    write_memory(PSRAM_UHS_BASE + 0x100 + 0x0, 0x802b0200);
    write_memory(PSRAM_UHS_BASE + 0x100 + 0x4, 0x60206020);
    write_memory(PSRAM_UHS_BASE + 0x100 + 0x8, 0x70027002);
    write_memory(PSRAM_UHS_BASE + 0x100 + 0xC, 0x70027002);
    write_memory(PSRAM_UHS_BASE + 0x100 + 0x10, 0x70027002);
    write_memory(PSRAM_UHS_BASE + 0x100 + 0x14, 0x70027002);
    write_memory(PSRAM_UHS_BASE + 0x100 + 0x18, 0x70027002);
    write_memory(PSRAM_UHS_BASE + 0x100 + 0x1C, 0x70027002);
    write_memory(PSRAM_UHS_BASE + 0x100 + 0x20, 0x70027002);
    write_memory(PSRAM_UHS_BASE + 0x100 + 0x24, 0x70027002);
    write_memory(PSRAM_UHS_BASE + 0x100 + 0x28, 0x26000000);
    write_memory(PSRAM_UHS_BASE + 0x100 + 0x2C, 0x26000006);
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x130);
    tmp_val &= 0x08ffffff;
    tmp_val |= 0x07000000;
    write_memory(PSRAM_UHS_BASE + 0x130, tmp_val);
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x148);
    tmp_val &= 0xfffffcff;
    tmp_val |= 0x00000200;
    write_memory(PSRAM_UHS_BASE + 0x148, tmp_val);
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x14C);
    tmp_val &= 0xffe0ffff;
    write_memory(PSRAM_UHS_BASE + 0x14C, tmp_val);
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x150);
    tmp_val &= 0xff88ff88;
    tmp_val |= 0x00330033;
    write_memory(PSRAM_UHS_BASE + 0x150, tmp_val);

    sleep_us(1);

    /* switch to ldo12 switch_to_ldo12uhs */
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x140);
    tmp_val &= 0xFFCFFFFF;
    write_memory(PSRAM_UHS_BASE + 0x140, tmp_val);
    sleep_us(1);

    /* release cen ck release_cen_ck_ckn */
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x140);
    tmp_val &= 0xFFFCFFFF;
    tmp_val |= 0x30000;
    write_memory(PSRAM_UHS_BASE + 0x140, tmp_val);
    sleep_us(1);
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x150);
    tmp_val = set_bits(tmp_val, 12, 3, 3);
    tmp_val = set_bits(tmp_val, 8, 3, 3);
    write_memory(PSRAM_UHS_BASE + 0x150, tmp_val);
    sleep_us(1);

    /* config phy paramater config_uhs_phy */
    write_memory(PSRAM_UHS_BASE + 0x130, 0x1a03000f);
    write_memory(PSRAM_UHS_BASE + 0x134, 0x0b030404);
    write_memory(PSRAM_UHS_BASE + 0x138, 0x050e0419);
    write_memory(PSRAM_UHS_BASE + 0x13C, 0x0a6a1c1c);
    write_memory(PSRAM_UHS_BASE + 0x144, 0x0711070e);
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x150);
    tmp_val = set_bits(tmp_val, 24, 3, 3);
    write_memory(PSRAM_UHS_BASE + 0x150, tmp_val);
}

// -------------uhs_phy_init

fn uhs_phy_init() {
    power_up_mm();
    power_up_uhspll();

    power_up_ldo12uhs();
    set_cen_ck_ckn();

    set_or_uhs();
    switch_to_ldo12uhs();
    release_cen_ck_ckn();

    let mut tmp_val = read_memory(GLB_BASE + 0x7F4);
    tmp_val = set_bits(tmp_val, 0, 1, 0);
    write_memory(GLB_BASE + 0x7F4, tmp_val);

    set_uhs_phy_init();
    set_uhs_latency_w();
    set_uhs_latency_r();
    psram_init();

    sleep_ms(1);
}

fn power_up_mm() {
    let mut tmp_val = read_memory(PDS_BASE + 0x10);
    tmp_val = set_bits(tmp_val, 1, 1, 0);
    write_memory(PDS_BASE + 0x10, tmp_val);
    sleep_us(150);
    let mut tmp_val = read_memory(PDS_BASE + 0x10);
    tmp_val = set_bits(tmp_val, 5, 1, 0);
    write_memory(PDS_BASE + 0x10, tmp_val);
    sleep_us(10);
    let mut tmp_val = read_memory(PDS_BASE + 0x10);
    tmp_val = set_bits(tmp_val, 17, 1, 0);
    write_memory(PDS_BASE + 0x10, tmp_val);
    sleep_us(10);
    let mut tmp_val = read_memory(PDS_BASE + 0x10);
    tmp_val = set_bits(tmp_val, 13, 1, 0);
    write_memory(PDS_BASE + 0x10, tmp_val);
    sleep_us(10);
    let mut tmp_val = read_memory(PDS_BASE + 0x10);
    tmp_val = set_bits(tmp_val, 9, 1, 0);
    write_memory(PDS_BASE + 0x10, tmp_val);
    sleep_us(150);
}

fn power_up_uhspll() {
    let mut tmp_val = read_memory(GLB_BASE + 0x7D0);
    tmp_val = set_bits(tmp_val, 9, 1, 1);
    tmp_val = set_bits(tmp_val, 10, 1, 1);
    write_memory(GLB_BASE + 0x7D0, tmp_val);
    sleep_us(10);
    let mut tmp_val = read_memory(GLB_BASE + 0x7D0);
    tmp_val = set_bits(tmp_val, 0, 1, 0);
    write_memory(GLB_BASE + 0x7D0, tmp_val);
    sleep_us(50);
    let mut tmp_val = read_memory(GLB_BASE + 0x7D0);
    tmp_val = set_bits(tmp_val, 0, 1, 1);
    write_memory(GLB_BASE + 0x7D0, tmp_val);
    sleep_us(50);
    let mut tmp_val = read_memory(GLB_BASE + 0x7D0);
    tmp_val = set_bits(tmp_val, 2, 1, 0);
    write_memory(GLB_BASE + 0x7D0, tmp_val);
    sleep_us(50);
    let mut tmp_val = read_memory(GLB_BASE + 0x7D0);
    tmp_val = set_bits(tmp_val, 2, 1, 1);
    write_memory(GLB_BASE + 0x7D0, tmp_val);
    sleep_us(50);
}

fn power_up_ldo12uhs() {
    let mut tmp_val = read_memory(GLB_BASE + 0x6D0);
    tmp_val = set_bits(tmp_val, 0, 1, 1);
    tmp_val = set_bits(tmp_val, 20, 4, 5);
    write_memory(GLB_BASE + 0x6D0, tmp_val);
    sleep_us(200);
}

fn set_cen_ck_ckn() {
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x150);
    tmp_val = set_bits(tmp_val, 12, 3, 0);
    tmp_val = set_bits(tmp_val, 8, 3, 0);
    write_memory(PSRAM_UHS_BASE + 0x150, tmp_val);
    sleep_us(1);
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x140);
    tmp_val &= 0xFFFCFFFF;
    tmp_val = set_bits(tmp_val, 16, 8, 0xfc);
    tmp_val = set_bits(tmp_val, 8, 8, 0x1);
    write_memory(PSRAM_UHS_BASE + 0x140, tmp_val);
    sleep_us(1);
}

fn set_or_uhs() {
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x148);
    tmp_val = set_bits(tmp_val, 8, 2, 2);
    write_memory(PSRAM_UHS_BASE + 0x148, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x14C);
    tmp_val = set_bits(tmp_val, 20, 1, 0);
    write_memory(PSRAM_UHS_BASE + 0x14C, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x130);
    tmp_val = set_bits(tmp_val, 28, 4, 0x0);
    tmp_val = set_bits(tmp_val, 26, 1, 1);
    write_memory(PSRAM_UHS_BASE + 0x130, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x100);
    tmp_val = set_bits(tmp_val, 28, 4, 8);
    tmp_val = set_bits(tmp_val, 16, 4, 0xb);
    write_memory(PSRAM_UHS_BASE + 0x100, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x104);
    tmp_val = set_bits(tmp_val, 28, 4, 7);
    tmp_val = set_bits(tmp_val, 12, 4, 7);
    write_memory(PSRAM_UHS_BASE + 0x104, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x124);
    tmp_val = set_bits(tmp_val, 28, 4, 7);
    tmp_val = set_bits(tmp_val, 12, 4, 7);
    write_memory(PSRAM_UHS_BASE + 0x124, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x120);
    tmp_val = set_bits(tmp_val, 28, 4, 7);
    tmp_val = set_bits(tmp_val, 12, 4, 7);
    write_memory(PSRAM_UHS_BASE + 0x120, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x11C);
    tmp_val = set_bits(tmp_val, 28, 4, 7);
    tmp_val = set_bits(tmp_val, 12, 4, 7);
    write_memory(PSRAM_UHS_BASE + 0x11C, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x118);
    tmp_val = set_bits(tmp_val, 28, 4, 7);
    tmp_val = set_bits(tmp_val, 12, 4, 7);
    write_memory(PSRAM_UHS_BASE + 0x118, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x114);
    tmp_val = set_bits(tmp_val, 28, 4, 7);
    tmp_val = set_bits(tmp_val, 12, 4, 7);
    write_memory(PSRAM_UHS_BASE + 0x114, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x110);
    tmp_val = set_bits(tmp_val, 28, 4, 7);
    tmp_val = set_bits(tmp_val, 12, 4, 7);
    write_memory(PSRAM_UHS_BASE + 0x110, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x10C);
    tmp_val = set_bits(tmp_val, 28, 4, 7);
    tmp_val = set_bits(tmp_val, 12, 4, 7);
    write_memory(PSRAM_UHS_BASE + 0x10C, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x108);
    tmp_val = set_bits(tmp_val, 28, 4, 7);
    tmp_val = set_bits(tmp_val, 12, 4, 7);
    write_memory(PSRAM_UHS_BASE + 0x108, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x128);
    tmp_val = set_bits(tmp_val, 24, 4, 6);
    write_memory(PSRAM_UHS_BASE + 0x128, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x12C);
    tmp_val = set_bits(tmp_val, 24, 4, 6);
    write_memory(PSRAM_UHS_BASE + 0x12C, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x130);
    tmp_val = set_bits(tmp_val, 24, 2, 3);
    write_memory(PSRAM_UHS_BASE + 0x130, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x100);
    tmp_val = set_bits(tmp_val, 20, 2, 2);
    tmp_val = set_bits(tmp_val, 8, 2, 2);
    write_memory(PSRAM_UHS_BASE + 0x100, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x104);
    tmp_val = set_bits(tmp_val, 20, 2, 2);
    tmp_val = set_bits(tmp_val, 4, 2, 2);
    write_memory(PSRAM_UHS_BASE + 0x104, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x124);
    tmp_val = set_bits(tmp_val, 0, 2, 2);
    tmp_val = set_bits(tmp_val, 16, 2, 2);
    write_memory(PSRAM_UHS_BASE + 0x124, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x120);
    tmp_val = set_bits(tmp_val, 0, 2, 2);
    tmp_val = set_bits(tmp_val, 16, 2, 2);
    write_memory(PSRAM_UHS_BASE + 0x120, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x11C);
    tmp_val = set_bits(tmp_val, 0, 2, 2);
    tmp_val = set_bits(tmp_val, 16, 2, 2);
    write_memory(PSRAM_UHS_BASE + 0x11C, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x118);
    tmp_val = set_bits(tmp_val, 0, 2, 2);
    tmp_val = set_bits(tmp_val, 16, 2, 2);
    write_memory(PSRAM_UHS_BASE + 0x118, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x114);
    tmp_val = set_bits(tmp_val, 0, 2, 2);
    tmp_val = set_bits(tmp_val, 16, 2, 2);
    write_memory(PSRAM_UHS_BASE + 0x114, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x110);
    tmp_val = set_bits(tmp_val, 0, 2, 2);
    tmp_val = set_bits(tmp_val, 16, 2, 2);
    write_memory(PSRAM_UHS_BASE + 0x110, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x10C);
    tmp_val = set_bits(tmp_val, 0, 2, 2);
    tmp_val = set_bits(tmp_val, 16, 2, 2);
    write_memory(PSRAM_UHS_BASE + 0x10C, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x108);
    tmp_val = set_bits(tmp_val, 0, 2, 2);
    tmp_val = set_bits(tmp_val, 16, 2, 2);
    write_memory(PSRAM_UHS_BASE + 0x108, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x150);
    tmp_val = set_bits(tmp_val, 16, 3, 3);
    tmp_val = set_bits(tmp_val, 20, 3, 3);
    tmp_val = set_bits(tmp_val, 0, 3, 3);
    tmp_val = set_bits(tmp_val, 4, 3, 3);
    write_memory(PSRAM_UHS_BASE + 0x150, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x124);
    tmp_val = set_bits(tmp_val, 24, 4, 0);
    tmp_val = set_bits(tmp_val, 8, 4, 0);
    write_memory(PSRAM_UHS_BASE + 0x124, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x120);
    tmp_val = set_bits(tmp_val, 24, 4, 0);
    tmp_val = set_bits(tmp_val, 8, 4, 0);
    write_memory(PSRAM_UHS_BASE + 0x120, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x11c);
    tmp_val = set_bits(tmp_val, 24, 4, 0);
    tmp_val = set_bits(tmp_val, 8, 4, 0);
    write_memory(PSRAM_UHS_BASE + 0x11c, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x118);
    tmp_val = set_bits(tmp_val, 24, 4, 0);
    tmp_val = set_bits(tmp_val, 8, 4, 0);
    write_memory(PSRAM_UHS_BASE + 0x118, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x114);
    tmp_val = set_bits(tmp_val, 24, 4, 0);
    tmp_val = set_bits(tmp_val, 8, 4, 0);
    write_memory(PSRAM_UHS_BASE + 0x114, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x110);
    tmp_val = set_bits(tmp_val, 24, 4, 0);
    tmp_val = set_bits(tmp_val, 8, 4, 0);
    write_memory(PSRAM_UHS_BASE + 0x110, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x10c);
    tmp_val = set_bits(tmp_val, 24, 4, 0);
    tmp_val = set_bits(tmp_val, 8, 4, 0);
    write_memory(PSRAM_UHS_BASE + 0x10c, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x108);
    tmp_val = set_bits(tmp_val, 24, 4, 0);
    tmp_val = set_bits(tmp_val, 8, 4, 0);
    write_memory(PSRAM_UHS_BASE + 0x108, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x128);
    tmp_val = set_bits(tmp_val, 24, 4, 0);
    tmp_val = set_bits(tmp_val, 8, 4, 0);
    tmp_val = set_bits(tmp_val, 28, 4, 3);
    write_memory(PSRAM_UHS_BASE + 0x128, tmp_val);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x12C);
    tmp_val = set_bits(tmp_val, 24, 4, 0);
    tmp_val = set_bits(tmp_val, 8, 4, 0);
    tmp_val = set_bits(tmp_val, 28, 4, 3);
    write_memory(PSRAM_UHS_BASE + 0x12C, tmp_val);

    sleep_us(200);
}

fn switch_to_ldo12uhs() {
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x140);
    tmp_val = set_bits(tmp_val, 16, 8, 0xcc);
    write_memory(PSRAM_UHS_BASE + 0x140, tmp_val);
    sleep_us(200);
}

fn release_cen_ck_ckn() {
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x140);
    tmp_val = set_bits(tmp_val, 16, 8, 0xcf);
    tmp_val = set_bits(tmp_val, 8, 8, 0x0);
    write_memory(PSRAM_UHS_BASE + 0x140, tmp_val);
    sleep_us(10);
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x150);
    tmp_val = set_bits(tmp_val, 8, 3, 3);
    tmp_val = set_bits(tmp_val, 12, 3, 3);
    write_memory(PSRAM_UHS_BASE + 0x150, tmp_val);
    sleep_us(10);
}

fn set_uhs_phy_init() {
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x130);
    tmp_val = set_bits(tmp_val, 28, 4, 0x0);
    tmp_val = set_bits(tmp_val, 27, 1, 1);
    tmp_val = set_bits(tmp_val, 26, 1, 1);
    tmp_val = set_bits(tmp_val, 24, 2, 3);
    write_memory(PSRAM_UHS_BASE + 0x130, tmp_val);

    write_memory(PSRAM_UHS_BASE + 0x134, 0x09020303);
    write_memory(PSRAM_UHS_BASE + 0x138, 0x040c0313);
    write_memory(PSRAM_UHS_BASE + 0x13C, 0x07d11515);
    write_memory(PSRAM_UHS_BASE + 0x144, 0x060f050c);

    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x150);
    tmp_val = set_bits(tmp_val, 24, 3, 1);
    write_memory(PSRAM_UHS_BASE + 0x150, tmp_val);

    sleep_us(100);
}

fn set_uhs_latency_w() {
    let uhs_latency = 9;
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x130);
    tmp_val = set_bits(tmp_val, 12, 3, uhs_latency % 4);
    tmp_val = set_bits(tmp_val, 8, 3, uhs_latency / 4);
    tmp_val = set_bits(tmp_val, 4, 3, (uhs_latency + 1) % 4);
    tmp_val = set_bits(tmp_val, 0, 3, (uhs_latency + 1) / 4);
    write_memory(PSRAM_UHS_BASE + 0x130, tmp_val);
    sleep_us(50);
}

fn set_uhs_latency_r() {
    let uhs_latency = 30;
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x130);
    tmp_val = set_bits(tmp_val, 20, 3, uhs_latency % 4);
    tmp_val = set_bits(tmp_val, 16, 4, uhs_latency / 4);
    write_memory(PSRAM_UHS_BASE + 0x130, tmp_val);
    sleep_us(50);
}

fn psram_init() {
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x0);
    tmp_val = set_bits(tmp_val, 2, 1, 1);
    write_memory(PSRAM_UHS_BASE + 0x0, tmp_val);
    sleep_us(10);
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x4);
    tmp_val = set_bits(tmp_val, 0, 1, 1);
    write_memory(PSRAM_UHS_BASE + 0x4, tmp_val);
    sleep_us(100);
    let mut tmp_val = read_memory(PSRAM_UHS_BASE + 0x0);
    tmp_val = set_bits(tmp_val, 2, 1, 0);
    write_memory(PSRAM_UHS_BASE + 0x0, tmp_val);
    sleep_us(100);
}
