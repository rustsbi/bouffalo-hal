use blri::Error;
use std::io::{Read, Seek, SeekFrom, Write};

const CORRECT_IMAGE: &[u8; 4256] = include_bytes!("blinky-bl808.bin");

#[test]
fn error_magic_number() {
    let mut f = tempfile::tempfile().expect("create tempfile for test");
    f.write_all(CORRECT_IMAGE).expect("prepare correct image");
    f.seek(SeekFrom::Start(0x00)).expect("seek to magic number");
    f.write_all(&[0x11, 0x22, 0x33, 0x44])
        .expect("prepare wrong magic number");
    let res = blri::check(&mut f);
    if let Err(Error::MagicNumber { wrong_magic }) = res {
        assert_eq!(wrong_magic, 0x11223344);
    } else {
        panic!("this test case should raise error")
    }
}

#[test]
fn error_head_length() {
    let mut f = tempfile::tempfile().expect("create tempfile for test");
    f.write_all(CORRECT_IMAGE).expect("prepare correct image");
    f.set_len(0x123).expect("truncate file to 0x123");
    let res = blri::check(&mut f);
    if let Err(Error::HeadLength { wrong_length }) = res {
        assert_eq!(wrong_length, 0x123)
    } else {
        panic!("this test case should raise HeadLength error")
    }
}

#[test]
fn error_flash_config_magic() {
    let mut f = tempfile::tempfile().expect("create tempfile for test");
    f.write_all(CORRECT_IMAGE).expect("prepare correct image");
    f.seek(SeekFrom::Start(0x08)).expect("seek to flash magic");
    f.write_all(&[0x55, 0x66, 0x77, 0x88])
        .expect("prepare wrong flash magic number");
    let res = blri::check(&mut f);
    if let Err(Error::FlashConfigMagic { wrong_magic }) = res {
        assert_eq!(wrong_magic, 0x55667788)
    } else {
        panic!("this test case should raise FlashConfigMagic error")
    }
}

#[test]
fn error_clock_config_magic() {
    let mut f = tempfile::tempfile().expect("create tempfile for test");
    f.write_all(CORRECT_IMAGE).expect("prepare correct image");
    f.seek(SeekFrom::Start(0x64)).expect("seek to clock magic");
    f.write_all(&[0x22, 0x33, 0x10, 0x37])
        .expect("prepare wrong clock magic number");
    let res = blri::check(&mut f);
    if let Err(Error::ClockConfigMagic { wrong_magic }) = res {
        assert_eq!(wrong_magic, 0x22331037)
    } else {
        panic!("this test case should raise ClockConfigMagic error")
    }
}

#[test]
fn error_image_offset_overflow() {
    let mut f = tempfile::tempfile().expect("create tempfile for test");
    f.write_all(CORRECT_IMAGE).expect("prepare correct image");
    f.set_len(0x1037).expect("truncate file to 0x1037");
    let res = blri::check(&mut f);
    if let Err(Error::ImageOffsetOverflow {
        file_length,
        wrong_image_offset,
        wrong_image_length,
    }) = res
    {
        assert_eq!(file_length, 0x1037);
        assert_eq!(wrong_image_offset, 0x1000);
        assert_eq!(wrong_image_length, 0xa0);
    } else {
        panic!("this test case should raise ImageOffsetOverflow error")
    }
}

#[test]
fn error_sha256_checksum() {
    let mut f = tempfile::tempfile().expect("create tempfile for test");
    f.write_all(CORRECT_IMAGE).expect("prepare correct image");
    f.seek(SeekFrom::Start(0x90))
        .expect("seek to checksum offset before read");
    let mut buf = [0u8; 32];
    f.read_exact(&mut buf).expect("read sha256 sum");
    let old_checksum = buf.clone();
    buf[0] >>= 1;
    buf[0] = buf[0].wrapping_add(1);
    f.seek(SeekFrom::Start(0x90))
        .expect("seek to checksum offset after read");
    f.write_all(&buf).expect("prepare wrong sha256 sum");
    let res = blri::check(&mut f);
    if let Err(Error::Sha256Checksum { wrong_checksum }) = res {
        assert_eq!(wrong_checksum, buf);
        assert_ne!(wrong_checksum.as_slice(), old_checksum);
    } else {
        panic!("this test case should raise Sha256Sum error")
    }
}
