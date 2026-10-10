use std::env;
use std::path::{Path, PathBuf};

// TODO @joey revist this file & cleanup (prob need a "common" mod)
fn main() {
    for variable in [
        "LIBRW_DIR",
        "LIBRW_INCLUDE_DIR",
        "LIBRW_LIB_DIR",
        "LIBRW_LIB_NAME",
        "LIBRW_LINK_KIND",
        "LIBRW_CXXFLAGS",
        "LIBRW_PLATFORM",
        "LIBRW_SYS_NO_SHIM",
    ] {
        println!("cargo:rerun-if-env-changed={variable}");
    }
    println!("cargo:rerun-if-changed=include/librw_sys.h");
    println!("cargo:rerun-if-changed=src/librw_sys.cpp");

    let ext_root = env::var_os("LIBRW_DIR")
        .filter(|value| !value.is_empty())
        .map(PathBuf::from);
    let ext_inc = env::var_os("LIBRW_INCLUDE_DIR")
        .filter(|value| !value.is_empty())
        .map(PathBuf::from);
    let ext_lib = env::var_os("LIBRW_LIB_DIR")
        .filter(|value| !value.is_empty())
        .map(PathBuf::from);

    let vendored = ext_root.is_none() && ext_inc.is_none() && ext_lib.is_none();
    let src = ext_inc
        .or_else(|| ext_root.clone())
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../vendor/librw"));

    let req_platform = env::var("LIBRW_PLATFORM").ok();
    let platform = req_platform.as_deref().unwrap_or("NULL");

    let vendored_lib = if vendored {
        Some(build_vendor(&src, platform))
    } else {
        None
    };

    if env::var_os("LIBRW_SYS_NO_SHIM").is_none() {
        let shim_platform = (vendored || req_platform.is_some()).then_some(platform);
        process_shim(&src, shim_platform);
    }

    if let Some(lib_dir) = ext_lib.or(vendored_lib) {
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        let name = env::var("LIBRW_LIB_NAME").unwrap_or_else(|_| "rw".to_owned());
        let kind = env::var("LIBRW_LINK_KIND").unwrap_or_else(|_| "static".to_owned());
        println!("cargo:rustc-link-lib={kind}={name}");
    }

    // shim is cpp regardless of librw's build system
    if cfg!(target_env = "msvc") {
        println!("cargo:rustc-link-lib=dylib=msvcp140");
    } else if cfg!(target_vendor = "apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}

#[cfg(feature = "vendor")]
fn build_vendor(src: &Path, platform: &str) -> PathBuf {
    if !src.join("CMakeLists.txt").is_file() {
        panic!("librw is missing at {:?} - pull submodule", src.display());
    }

    let profile = match env::var("PROFILE").as_deref() {
        Ok("release") => "Release",
        _ => "Debug",
    };
    let dest = cmake::Config::new(src)
        .profile(profile)
        .define("LIBRW_PLATFORM", platform)
        .define("LIBRW_TOOLS", "OFF")
        .define("LIBRW_EXAMPLES", "OFF")
        .define("LIBRW_INSTALL", "ON")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();
    dest.join("lib")
}

#[cfg(not(feature = "vendor"))]
fn build_vendor(src: &Path, _platform: &str) -> PathBuf {
    // nothing specified, wasted
    panic!(
        "no librw configured and `vendor` ft is disabled (searched {:?})",
        src.display()
    );
}

fn process_shim(inc: &Path, platform: Option<&str>) {
    if !inc.join("rw.h").is_file() {
        panic!(
            "{:?} doesnt contain rw.h - LIBRW_INCLUDE_DIR must name librw's repo or install include dir",
            inc.display()
        );
    }

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++11")
        .include(inc)
        .include("include")
        .file("src/librw_sys.cpp")
        .warnings(false);

    if let Some(platform) = platform {
        build.define(&format!("RW_{platform}"), None);
    }

    if let Ok(flags) = env::var("LIBRW_CXXFLAGS") {
        for flag in flags.split_whitespace() {
            build.flag(flag);
        }
    }
    build.compile("librw_rs_shim");
}
