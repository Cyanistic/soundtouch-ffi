#![allow(unused_imports)]
use std::path::PathBuf;

// use bindgen::NonCopyUnionStyle;

const SOUNDTOUCH_DIR: &str = "soundtouch-2_3_2";

// #[cfg(unix)]
// fn build() {
//     let bootstrap = std::path::Path::new(SOUNDTOUCH_DIR).join("bootstrap").canonicalize().expect("to canonicalize");
//     let _ = std::process::Command::new(bootstrap)
//         .current_dir(SOUNDTOUCH_DIR)
//         .output()
//         .expect("To run bootstrap");
//     let mut config = autotools::Config::new(SOUNDTOUCH_DIR);

//     let host = std::env::var("HOST").expect("To have env:HOST");
//     let target = std::env::var("TARGET").expect("To have env:TARGET");
//     if let Ok(override_host) = std::env::var("SOUNDTOUCH_SYS_OVERRIDE_HOST") {
//         config.config_option("host", Some(override_host.as_str()));
//     } else if host != target {
//         #[cfg(not(feature = "target_host"))]
//         {
//             println!("cargo:warning=Cross-compilation may not be supported");
//         }
//         #[cfg(feature = "target_host")]
//         {
//             config.config_option("host", Some(target.as_str()));
//         }
//     }

//     let res = config.enable_shared()
//         .enable_static()
//         .insource(true)
//         .fast_build(true)
//         .build();

//     //libraries are installed in <out>/lib
//     println!("cargo:rustc-link-lib=dylib=stdc++");
//     println!("cargo:rustc-link-search=native={}/lib", res.display());
//     println!("cargo:rustc-link-lib=static=SoundTouch");
// }

fn build() {

    // let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR is not set by cargo");
    let soundtouch_dir = std::path::Path::new(SOUNDTOUCH_DIR);
    let source_dir = soundtouch_dir.join("source").join("SoundTouch");

    let mut cc = cc::Build::new();
    cc.warnings(false)
        .cpp(true)
        .extra_warnings(false)
        .file(source_dir.join("AAFilter.cpp"))
        .file(source_dir.join("BPMDetect.cpp"))
        .file(source_dir.join("FIFOSampleBuffer.cpp"))
        .file(source_dir.join("FIRFilter.cpp"))
        .file(source_dir.join("InterpolateCubic.cpp"))
        .file(source_dir.join("InterpolateLinear.cpp"))
        .file(source_dir.join("InterpolateShannon.cpp"))
        .file(source_dir.join("PeakFinder.cpp"))
        .file(source_dir.join("RateTransposer.cpp"))
        .file(source_dir.join("SoundTouch.cpp"))
        .file(source_dir.join("TDStretch.cpp"))
        .file(source_dir.join("cpu_detect_x86.cpp"))
        .file(source_dir.join("mmx_optimized.cpp"))
        .file(source_dir.join("sse_optimized.cpp"))
        .include(soundtouch_dir.join("include"))
        .include(soundtouch_dir.join("source/SoundTouch"))
        .shared_flag(false)
        .pic(false)
        .warnings(false);

    if let Ok(compiler) = std::env::var("CC") {
        let compiler = std::path::Path::new(&compiler);
        let compiler = compiler.file_stem().expect("To have file name in CC").to_str().unwrap();
        if compiler == "clang-cl"{
            cc.flag("/W0");
        }
    }

    cc.compile("SoundTouch")

}

fn main() {
    if std::env::var("DOCS_RS").map(|docs| docs == "1").unwrap_or(false) {
        //skip docs.rs build
        return;
    }
    const PREPEND_LIB: &'static str = "
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use root::{soundtouch::*, TDStretch, uint};
";

    let mut out = PathBuf::new();
    out.push("src");
    out.push("lib.rs");
    let header = PathBuf::from("wrapper.hpp");

    let bindings = bindgen::Builder::default().header(header.display().to_string())
                                              .raw_line(PREPEND_LIB)
                                              .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
                                              .generate_comments(false)
                                              .layout_tests(false)
                                              .ctypes_prefix("libc")
                                              .constified_enum_module("*")
                                              .allowlist_type("soundtouch::SoundTouch")
                                              .allowlist_type("soundtouch::SAMPLETYPE")
                                              .allowlist_type("soundtouch::BPMDetect")
                                              .allowlist_function("soundtouch::SoundTouch::putSamples")
                                              .allowlist_type("soundtouch::TDStretch")
                                              .allowlist_type("soundtouch::RateTransposer")
                                              .opaque_type("std::.*")
                                              .manually_drop_union(".*")
                                              .default_non_copy_union_style(bindgen::NonCopyUnionStyle::ManuallyDrop)
                                              .use_core()
                                              .enable_cxx_namespaces()
                                              .trust_clang_mangling(true)
                                              .clang_arg("-x").clang_arg("c++")
                                              .generate()
                                              .expect("Unable to generate bindings");

    bindings.write_to_file(out).expect("Couldn't write bindings!");

    build();
}
