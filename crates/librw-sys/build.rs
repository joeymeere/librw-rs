use std::env;
use std::path::{Path, PathBuf};

const ENV_VARS: &[&str] = &[
    "LIBRW_DIR",
    "LIBRW_INCLUDE_DIR",
    "LIBRW_LIB_DIR",
    "LIBRW_LIB_NAME",
    "LIBRW_LINK_KIND",
    "LIBRW_CXXFLAGS",
    "LIBRW_PLATFORM",
    "LIBRW_SYS_NO_SHIM",
    "SDL2_DIR",
    "SDL2_INCLUDE_DIR",
    "SDL2_LIB_DIR",
];

struct NativeCfg {
    src: PathBuf,
    ext_lib: Option<PathBuf>,
    vendored: bool,
    platform: String,
    gl3: bool,
}

impl NativeCfg {
    fn from_env() -> Self {
        let ext_root = nonempty_path("LIBRW_DIR");
        let ext_inc = nonempty_path("LIBRW_INCLUDE_DIR");
        let ext_lib = nonempty_path("LIBRW_LIB_DIR");

        let vendored = ext_root.is_none() && ext_inc.is_none() && ext_lib.is_none();

        let src = ext_inc.or_else(|| ext_root.clone()).unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../vendor/librw")
        });

        let feat_gl3 = env::var_os("CARGO_FEATURE_GL3").is_some();
        let req_platform = env::var("LIBRW_PLATFORM").ok();
        let platform =
            req_platform.unwrap_or_else(|| if feat_gl3 { "GL3" } else { "NULL" }.to_owned());

        let gl3 = feat_gl3 || platform.eq_ignore_ascii_case("GL3");

        if feat_gl3 && !platform.eq_ignore_ascii_case("GL3") {
            panic!("the gl3 feature requires LIBRW_PLATFORM=GL3");
        }

        Self {
            src,
            ext_lib,
            vendored,
            platform,
            gl3,
        }
    }
}

// todo @joey inline pls
fn nonempty_path(name: &str) -> Option<PathBuf> {
    env::var_os(name)
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

#[cfg(feature = "vendor")]
fn build_vendor(src: &Path, platform: &str, gl3: bool) -> PathBuf {
    if !src.join("CMakeLists.txt").is_file() {
        panic!("librw is missing at {:?}; run make vendor-init", src);
    }

    let profile = match env::var("PROFILE").as_deref() {
        Ok("release") => "Release",
        _ => "Debug",
    };
    let mut cmake = cmake::Config::new(src);
    cmake
        .profile(profile)
        .define("LIBRW_PLATFORM", platform)
        .define("LIBRW_TOOLS", "OFF")
        .define("LIBRW_EXAMPLES", "OFF")
        .define("LIBRW_INSTALL", "ON")
        .define("BUILD_SHARED_LIBS", "OFF");

    let target = env::var("TARGET").unwrap_or_default();
    if target.ends_with("apple-darwin") {
        cmake.define("CMAKE_OSX_DEPLOYMENT_TARGET", "11.0");
        if target.starts_with("aarch64") {
            cmake.define("CMAKE_OSX_ARCHITECTURES", "arm64");
        } else if target.starts_with("x86_64") {
            cmake.define("CMAKE_OSX_ARCHITECTURES", "x86_64");
        }
    } else if target.ends_with("pc-windows-gnullvm") {
        cmake
            .define("CMAKE_SYSTEM_NAME", "Windows")
            .define("CMAKE_TRY_COMPILE_TARGET_TYPE", "STATIC_LIBRARY");
    }

    if let Ok(flags) = env::var("LIBRW_CXXFLAGS") {
        cmake.define("CMAKE_CXX_FLAGS", flags);
    }

    if gl3 {
        cmake.define("LIBRW_GL3_GFXLIB", "SDL2");
        configure_cmake_sdl2(&mut cmake);
    }

    cmake.build().join("lib")
}

#[cfg(not(feature = "vendor"))]
fn build_vendor(src: &Path, _platform: &str, _gl3: bool) -> PathBuf {
    panic!(
        "no librw configured and the vendor feature disabled (searched {:?})",
        src
    );
}

#[cfg(feature = "vendor")]
fn configure_cmake_sdl2(cmake: &mut cmake::Config) {
    if let Some(path) = nonempty_path("SDL2_DIR") {
        cmake.define("SDL2_DIR", path);
    }
    if let Some(path) = nonempty_path("SDL2_INCLUDE_DIR") {
        cmake.define("SDL2_INCLUDE_DIR", path);
    }
    if let Some(path) = nonempty_path("SDL2_LIB_DIR") {
        cmake.define("CMAKE_LIBRARY_PATH", path);
    }
}

fn build_shim(cfg: &NativeCfg) {
    if !cfg.src.join("rw.h").is_file() {
        panic!(
            "{:?} does not contain rw.h; LIBRW_INCLUDE_DIR must name librw's repo or install include directory",
            cfg.src
        );
    }

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++11")
        .include(&cfg.src)
        .include("include")
        .file("src/librw_sys.cpp")
        .warnings(false)
        .define(&format!("RW_{}", cfg.platform), None);

    if cfg.gl3 {
        build.define("LIBRW_SDL2", None);
        configure_shim_sdl2(&mut build);
    }

    if let Ok(flags) = env::var("LIBRW_CXXFLAGS") {
        for flag in flags.split_whitespace() {
            build.flag(flag);
        }
    }

    // cc selects the targets c++ stdlib and honors CXXSTDLIB
    build.compile("librw_rs_shim");
}

fn configure_shim_sdl2(build: &mut cc::Build) {
    if let Some(include) = nonempty_path("SDL2_INCLUDE_DIR") {
        build.include(include);
    } else if let Some(root) = nonempty_path("SDL2_DIR") {
        build.include(root.join("include"));
        build.include(root.join("include/SDL2"));
    } else {
        let library = pkg_config::Config::new()
            .cargo_metadata(true)
            .probe("sdl2")
            .expect("GL3 requires SDL2; set SDL2_DIR or SDL2_INCLUDE_DIR/SDL2_LIB_DIR");
        for include in library.include_paths {
            build.include(include);
        }
    }

    if let Some(lib_dir) = nonempty_path("SDL2_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        println!("cargo:rustc-link-lib=SDL2");
    }
}

fn main() {
    for variable in ENV_VARS {
        println!("cargo:rerun-if-env-changed={variable}");
    }

    println!("cargo:rerun-if-changed=include/librw_sys.h");
    println!("cargo:rerun-if-changed=src/librw_sys.cpp");

    let config = NativeCfg::from_env();
    let vendored_lib = config
        .vendored
        .then(|| build_vendor(&config.src, &config.platform, config.gl3));

    if env::var_os("LIBRW_SYS_NO_SHIM").is_none() {
        build_shim(&config);
    }

    if let Some(lib_dir) = config.ext_lib.or(vendored_lib) {
        println!("cargo:rustc-link-search=native={}", lib_dir.display());
        let name = env::var("LIBRW_LIB_NAME").unwrap_or_else(|_| "rw".to_owned());
        let kind = env::var("LIBRW_LINK_KIND").unwrap_or_else(|_| "static".to_owned());
        println!("cargo:rustc-link-lib={kind}={name}");
    }
}
