extern crate gcc;

fn main() {
    gcc::compile_library("libstb_image_impl.a", &["src/stb_image_impl.c"]);
}
