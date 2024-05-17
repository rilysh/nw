fn main() {
    println!("cargo::rerun-if-changed=src/lib/lib.c");

    cc::Build::new()
        .file("src/lib/lib.c")
        .compile("lib");
}
