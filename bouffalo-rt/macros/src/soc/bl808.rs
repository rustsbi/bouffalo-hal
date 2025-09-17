#[rustfmt::skip]
#[cfg(feature = "bl808-dsp")]
#[allow(dead_code)]
pub(crate) const BL808_DSP_INTERRUPTS: [&'static str; 67] = [
    "bmx_dsp_bus_err",	"dsp_reserved1",	"dsp_reserved2",	"dsp_reserved3",	"uart3",
    "i2c2",	            "i2c3",	            "spi1",	            "dsp_reserved4",	"dsp_reserved5",
    "seof_int0",	    "seof_int1",	    "seof_int2",	    "dvp2_bus_int0",	"dvp2_bus_int1",
    "dvp2_bus_int2",	"dvp2_bus_int3",	"h264_bs",	        "h264_frame",	    "h264_seq_done",
    "mjpeg",	        "h264_s_bs",	    "h264_s_frame",	    "h264_s_seq_done",	"dma2_int0",
    "dma2_int1",	    "dma2_int2",	    "dma2_int3",	    "dma2_int4",	    "dma2_int5",
    "dma2_int6",	    "dma2_int7",	    "dsp_reserved6",	"dsp_reserved7",	"dsp_reserved8",
    "dsp_reserved9",	"dsp_reserved10",	"mipi_csi",	        "ipc_d0",	        "dsp_reserved11",
    "mjdec",	        "dvp2_bus_int4",	"dvp2_bus_int5",	"dvp2_bus_int6",	"dvp2_bus_int7",
    "dma2_d_int0",	    "dma2_d_int1",	    "display",	        "pwm",	            "seof_int3",
    "dsp_reserved12",	"dsp_reserved13",	"osd",	            "dbi",	            "dsp_reserved14",
    "osda_bus_drain",	"osdb_bus_drain",	"osd_pb",	        "dsp_reserved15",	"mipi_dsi",
    "dsp_reserved16",	"timer0",	        "timer1",	        "wdt",	            "audio",
    "wl_all",	        "pds",
];

// MCU and LP cores share the same interrupt sources (MCU is M0, LP is Low Power)
#[rustfmt::skip]
#[cfg(any(feature = "bl808-mcu", feature = "bl808-lp"))]
#[allow(dead_code)]
pub(crate) const BL808_MCU_LP_INTERRUPTS: [&'static str; 64] = [
    "bmx_mcu_bus_err",      "bmx_mcu_to",           "m0_reserved2",         "ipc_m0",
    "audio",                "rf_top_int0",          "rf_top_int1",          "lz4d",
    "gauge_itf",            "sec_eng_id1_sha_aes_trng_pka_gmac", "sec_eng_id0_sha_aes_trng_pka_gmac", "sec_eng_id1_cdet",
    "sec_eng_id0_cdet",     "sf_ctrl_id1",          "sf_ctrl_id0",          "dma0_all",
    "dma1_all",             "sdh",                  "mm_all",               "irtx",
    "irrx",                 "usb",                  "aupdm_touch",          "m0_reserved23",
    "emac",                 "gpadc_dma",            "efuse",                "spi0",
    "uart0",                "uart1",                "uart2",                "gpio_dma",
    "i2c0",                 "pwm",                  "ipc_rsvd",             "ipc_lp",
    "timer0_ch0",           "timer0_ch1",           "timer0_wdt",           "i2c1",
    "i2s",                  "ana_ocp_out_to_cpu_0", "ana_ocp_out_to_cpu_1", "ana_ocp_out_to_cpu_2",
    "gpio_int0",            "dm",                   "bt",                   "m154_req_ack",
    "m154_int",             "m154_aes",             "pds_wakeup",           "hbn_out0",
    "hbn_out1",             "bor",                  "wifi",                 "bz_phy_int",
    "ble",                  "mac_txrx_timer",       "mac_txrx_misc",        "mac_rx_trg",
    "mac_tx_trg",           "mac_gen",              "mac_port_trg",         "wifi_ipc_public",
];
