fn main() {
    if time::now().tm_wday == 2 {
        println!("cargo:rustc-cfg=tuesday");
    }
}
