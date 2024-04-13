use glob::glob;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("clib/osal.c")
        .file("clib/osal_fs.c")
        .file("clib/osal_socket.c")
        .file("clib/hal_uart.c")
        .file("clib/hal_network.c")
        .define("_GNU_SOURCE", None)
        .include("clib")
        .include("psdk")
        .compile("osal");

    let prefix = env::var("CONDA_PREFIX").unwrap_or("/usr".to_owned());
    let prefix_lib = PathBuf::from(prefix).join("lib");
    env::set_var("LIBCLANG_PATH", &prefix_lib);
    let gcc_inc = prefix_lib.join("gcc/*/*/include");
    let gcc_inc = glob(gcc_inc.to_str().unwrap())
        .unwrap()
        .next()
        .unwrap()
        .unwrap();

    let p = match build_target::target_arch().unwrap().as_str() {
        "x86_64" => "x86_64-linux-gnu-gcc",
        x => panic!("target arch {} not supported", x),
    };

    println!("cargo:rustc-link-search=psdk/lib/{}", p);
    println!("cargo:rustc-link-lib=payloadsdk");
    println!("cargo:rerun-if-changed=clib");
    println!("cargo:rustc-link-lib=osal");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-Ipsdk")
        .clang_arg("-Iclib")
        .clang_arg(&format!("-I{}", gcc_inc.to_str().unwrap()))
        .prepend_enum_name(false)
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(&out_path)
        .expect("Couldn't write bindings!");

    fs::create_dir_all("tmp").unwrap();
    fs::copy(&out_path, "tmp/bindings.rs").unwrap();
}
