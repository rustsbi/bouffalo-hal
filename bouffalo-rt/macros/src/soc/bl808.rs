#[rustfmt::skip]
#[cfg(feature = "bl808-dsp")]
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

// TODO const BL808_MCU_INTERRUPTS

// TODO const BL808_LP_INTERRUPTS
