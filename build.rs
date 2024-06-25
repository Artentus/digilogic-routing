fn main() {
    let bindings = cbindgen::generate(".").unwrap();
    bindings.write_to_file("routing.h");
}
