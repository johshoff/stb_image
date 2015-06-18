use std::ffi::CString;
use std::slice;

#[link(name = "stb_image_impl", kind="static")]
extern {
    fn stbi_hdr_to_ldr_gamma(gamma: f32);

    // TODO: should use libc::c_char instead of i8 and libc::c_int instead of u64, but
    //       using `extern crate libc` is broken at the moment. Possibly fixed by
    //       https://github.com/rust-lang/rust/issues/26043
    fn stbi_load(filename: *const i8, x: *mut u64, y: *mut u64, comp: *mut u64, req_comp: u64) -> *mut u8;
    fn stbi_image_free(retval_from_stbi_load: *mut u8);
}

pub fn hdr_to_ldr_gamma(gamma: f32) {
    unsafe {
        stbi_hdr_to_ldr_gamma(gamma)
    }
}

pub fn load(filename: &str, components: u32) -> (u64, u64, u64, Vec<u8>) {
    let mut x : u64 = 0;
    let mut y : u64 = 0;
    let mut c : u64 = 0;

    // TODO: This should use the filesystem encoding, which might be different from utf8
    let c_to_print = CString::new(filename).unwrap();

    let raw_data = unsafe { stbi_load(c_to_print.as_ptr(), &mut x, &mut y, &mut c, components as u64) };

    let num_bytes = (x * y * c) as usize;

    let mut data : Vec<u8> = Vec::with_capacity(num_bytes);

    // TODO: Don't copy the returned data, but use the buffer returned.
    let s = unsafe { slice::from_raw_parts(raw_data, num_bytes) };

    for i in s {
        data.push(*i);
    }

    unsafe { stbi_image_free(raw_data) };

    (x, y, c, data)
}

#[test]
fn test_gamma() {
    hdr_to_ldr_gamma(0.5f32);
}

#[test]
fn test_load() {
    let (width, height, channels, data) = load("test.png", 4);

    assert!(width == 20);
    assert!(height == 32);
    assert!(channels == 4);

    assert!(data[0] == 255);
    assert!(data[1] == 255);
    assert!(data[2] == 255);
    assert!(data[3] == 255);

    let last_pixel = data.len() - 4;
    assert!(data[last_pixel + 0] == 0);
    assert!(data[last_pixel + 1] == 0);
    assert!(data[last_pixel + 2] == 0);
    assert!(data[last_pixel + 3] == 255);
}

