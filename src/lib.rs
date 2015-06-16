#[link(name = "stb_image_impl", kind="static")]
extern {
    fn stbi_hdr_to_ldr_gamma(gamma: f32);
}

#[test]
fn it_works() {
    hdr_to_ldr_gamma(0.5f32);
}

pub fn hdr_to_ldr_gamma(gamma: f32) {
    unsafe {
        stbi_hdr_to_ldr_gamma(gamma)
    }
}
