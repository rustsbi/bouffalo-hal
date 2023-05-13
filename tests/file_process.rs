use blri::Error;
use std::io::{Seek, SeekFrom, Write};

const CORRECT_IMAGE: &[u8; 4256] = include_bytes!("blinky-bl808.bin");

#[test]
fn error_magic_number() {
    let mut f = tempfile::tempfile().expect("create tempfile for test");
    f.write_all(CORRECT_IMAGE).expect("prepare correct image");
    f.seek(SeekFrom::Start(0x0)).expect("seek to magic number");
    f.write_all(&[0x11, 0x22, 0x33, 0x44])
        .expect("prepare wrong magic number");
    let ans = blri::process(&mut f);
    if let Err(Error::MagicNumber { wrong_magic }) = ans {
        assert_eq!(wrong_magic, 0x11223344);
    } else {
        panic!("this test case should raise error")
    }
}
