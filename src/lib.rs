use std::ffi::CString;
use std::slice;
use std::mem;

// TODO:
// - Figure out what happens to pointers from C. Do we need to do a `mem::forget(raw_data);`

#[link(name = "stb_image_impl", kind="static")]
extern {
    fn stbi_hdr_to_ldr_gamma(gamma: f32);

    // TODO: should use libc::c_char instead of i8 and libc::c_int instead of u64, but
    //       using `extern crate libc` is broken at the moment. Possibly fixed by
    //       https://github.com/rust-lang/rust/issues/26043
    fn stbi_load(filename: *const i8, x: *mut u64, y: *mut u64, comp: *mut u64, req_comp: u64) -> *mut u8;
    fn stbi_load_from_memory(buffer: *const i8, len: u64, x: *mut u64, y: *mut u64, comp: *mut u64, req_comp: u64) -> *mut u8;
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

pub fn load_from_memory(buffer: &[u8], components: u32) -> (u64, u64, u64, Vec<u8>) {
    let mut x : u64 = 0;
    let mut y : u64 = 0;
    let mut c : u64 = 0;

    // The C library expects an array of i8, but files in rust are loaded as u8.
    // It's also the author's opinion that that opaque bytes are better
    // represented as u8. Therefore, the function takes a u8 argument and converts
    // it to i8, rather than leaving that to the caller.
    let buffer : &[i8] = unsafe { mem::transmute(buffer) };
    let raw_data = unsafe { stbi_load_from_memory(buffer.as_ptr(), buffer.len() as u64, &mut x, &mut y, &mut c, components as u64) };

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


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use super::hdr_to_ldr_gamma;
    use super::load;
    use super::load_from_memory;

    #[test]
    fn test_gamma() {
        hdr_to_ldr_gamma(0.5f32);
    }

    #[test]
    fn test_load() {
        let (width, height, channels, data) = load("test.png", 4);

        check_image(width, height, channels, data);
    }

    #[test]
    fn test_load_from_memory() {
        let mut file = File::open("test.png").unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        let (width, height, channels, data) = load_from_memory(&bytes, 4);

        check_image(width, height, channels, data);
    }

    // helper function for to check if image 'test.png' is loaded correctly
    fn check_image(width: u64, height: u64, channels: u64, data: Vec<u8>) {
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
}
