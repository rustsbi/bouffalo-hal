use std::path::PathBuf;

use blri::{ImageToFuse, fuse_image_header};
use bouffalo_rt::soc::bl808::HalBootheader;

#[test]
fn test_fuse_image_header() {
    let m0_image = ImageToFuse {
        path: PathBuf::from("tests/image_fusion/multicore-demo-mcu.bin"),
        addr: 0x58001000,
    };
    let d0_image = ImageToFuse {
        path: PathBuf::from("tests/image_fusion/multicore-demo-dsp.bin"),
        addr: 0x58000000,
    };
    let _fused_image = fuse_image_header(Some(m0_image.clone()), Some(d0_image), None);

    let _blcube_fused_image = include_bytes!("./image_fusion/multicore-demo-fused.bin");
    let _blcube_fused_image = HalBootheader::from_bytes(_blcube_fused_image).unwrap();
    let a = _blcube_fused_image;
    let _b = a;
    println!("{}", m0_image.path.display());
}
