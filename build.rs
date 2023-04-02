use std::env;
use std::env::var;
use std::path::PathBuf;
use std::process::Command;

const REPO_URL: &str = "https://gitlab.com/akihe/radamsa/";
const REPO_REV: &str = "ba1b18bbf8f89c9f727030845c7a4e859a0069d8";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    if env::var("DOCS_RS").is_ok() {
        // no clone for building docs
        return;
    }

    if env::var("CARGO_CFG_TARGET_OS").unwrap() != "linux" {
        eprintln!("Only Linux is supported.");
        return;
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let radamsa_path = out_path.join("radamsa");

    if !radamsa_path.is_dir() {
        Command::new("git")
            .current_dir(&out_path)
            .arg("clone")
            .arg(REPO_URL)
            .status()
            .unwrap();
        Command::new("git")
            .current_dir(&out_path)
            .arg("checkout")
            .arg(REPO_REV)
            .status()
            .unwrap();
    }

    let out_lib = out_path.join("radamsa/lib/libradamsa.a");

    if !out_lib.is_file() {
        println!("building libradamsa.a");
        let jobs = var("CARGO_BUILD_JOBS").unwrap_or(String::from("1"));
        Command::new("make")
            .arg("-j")
            .arg(&jobs)
            .current_dir(&radamsa_path)
            .output()
            .expect("make in radamsa failed");
        Command::new("make")
            .arg("-j")
            .arg(&jobs)
            .arg("lib/libradamsa.a")
            .current_dir(&radamsa_path)
            .output()
            .expect("make in radamsa failed");
    }

    let header = radamsa_path.join("c/radamsa.h");
    let bindings = bindgen::Builder::default()
        .header(format!("{}", header.display()))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search={}/lib/", radamsa_path.display());
    println!("cargo:rustc-link-lib=static=radamsa");
}
