fn main() {
    cc::Build::new()
        .file("../../vendor/dag-cbrrr/src/cbrrr/_cbrrr.c")
        // lol
        .include("/opt/homebrew/Cellar/python@3.11/3.11.6/Frameworks/Python.framework/Versions/3.11/include/python3.11/")
        .compile("cbrrr");

    // lmao
    println!("cargo:rustc-link-search=/opt/homebrew/Cellar/python@3.11/3.11.6/Frameworks/Python.framework/Versions/3.11/lib/");
    println!("cargo:rustc-link-lib=python3.11")
}
