use std::path::PathBuf;

// const LIB_DIR: &str = "c/lib";
// const LIBS: [&str; 4] = ["glib-2.0", "c", "dbus-1", "gthread-2.0"];

fn main() {
    // let lib_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(LIB_DIR);

    // println!("cargo:rustc-link-search={}", lib_dir.display()); // add inclide dir

    // Force linking against older glibc symbols
    // println!("cargo:rustc-link-arg=-Wl,--version-script=glibc_compat.lds");
    // println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());

    // // Try to force older symbol versions
    // println!("cargo:rustc-link-arg=-Wl,--wrap=memcpy");
    // println!("cargo:rustc-link-arg=-Wl,--defsym=__wrap_memcpy=memcpy");

    // println!("cargo:rustc-link-arg=-Wl,--version-script=glibc_compat.lds");

    // println!("cargo:rustc-link-arg=-Wl,--dynamic-linker=/lib/ld-linux.so.3");

    // link libraries
    // println!("cargo:rustc-link-lib=dylib=lipc"); // Note: no "lib" prefix - it's a dynamic library

    // for lib in LIBS {
    //     println!("cargo:rustc-link-lib=dylib={lib}");
    // }
}
