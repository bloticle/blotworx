fn main() {
    println!("cargo:rerun-if-changed=third_party/mojoshader");
    println!("cargo:rerun-if-changed=src/blotworx/d3d9_translate_bridge.cpp");
    println!("cargo:rerun-if-changed=3ds/c3d_bridge.c");
    println!("cargo:rerun-if-changed=3ds/vshader.pica");
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let android = target_os == "android";
    let horizon = target_os == "horizon";
    let windows = target_os == "windows";
    if android {
        println!("cargo:rustc-cdylib-link-arg=-Wl,-z,max-page-size=16384");
        cc::Build::new().cpp(true).std("c++17").opt_level(2)
            .define("MOJOSHADER_NO_VERSION_INCLUDE", None)
            .include("third_party/mojoshader").file("src/blotworx/d3d9_translate_bridge.cpp")
            .compile("blotworx_shader_bridge");
    }
    if !horizon {
        let mut shaders = cc::Build::new();
        if android {
            shaders.std("c99");
        } else if windows {
            shaders
                .cpp(true)
                .std("c++17")
                .file("src/blotworx/d3d9_translate_bridge.cpp");
        } else {
            // Compiling the C sources fails on clang
            cc::Build::new()
                .cpp(true)
                .std("c++17")
                .opt_level(2)
                .define("MOJOSHADER_NO_VERSION_INCLUDE", None)
                .include("third_party/mojoshader")
                .file("src/blotworx/d3d9_translate_bridge.cpp")
                .compile("blotworx_shader_bridge");
            shaders.std("c99");
        }
        shaders
            .opt_level(2)
            .warnings(false)
            .include("third_party/mojoshader")
            .define("MOJOSHADER_NO_VERSION_INCLUDE", None)
            .define("_CRT_SECURE_NO_WARNINGS", None)
            .file("third_party/mojoshader/mojoshader.c")
            .file("third_party/mojoshader/mojoshader_common.c")
            .file("third_party/mojoshader/profiles/mojoshader_profile_common.c")
            .file("third_party/mojoshader/profiles/mojoshader_profile_glsl.c");
        for profile in [
            "D3D", "BYTECODE", "HLSL", "ARB1", "ARB1_NV", "METAL", "SPIRV", "GLSPIRV",
        ] {
            shaders.define(&format!("SUPPORT_PROFILE_{profile}"), Some("0"));
        }
        shaders.compile("blotworx_mojoshader");
    }
    if horizon {
        let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
        let dkp = std::path::PathBuf::from(
            std::env::var("DEVKITPRO").expect("DEVKITPRO must be set for 3DS builds"),
        );
        let picasso = dkp.join("tools").join("bin").join(if cfg!(windows) { "picasso.exe" } else { "picasso" });
        let shbin = out_dir.join("vshader.shbin");
        let status = std::process::Command::new(&picasso)
            .args(["-o", shbin.to_str().unwrap(), "3ds/vshader.pica"])
            .status()
            .expect("Failed to run picasso for 3ds/vshader.pica");
        assert!(status.success(), "picasso failed compiling 3ds/vshader.pica");

        let shbin_bytes = std::fs::read(&shbin).expect("Read vshader.shbin");
        let mut hdr = format!(
            "#pragma once\n#include <stdint.h>\nstatic const uint32_t vshader_shbin_size = {};\nstatic const uint8_t vshader_shbin[] = {{",
            shbin_bytes.len()
        );
        for (i, b) in shbin_bytes.iter().enumerate() {
            if i % 16 == 0 { hdr.push('\n'); }
            hdr.push_str(&format!("0x{b:02x},"));
        }
        hdr.push_str("\n};\n");
        std::fs::write(out_dir.join("vshader_shbin.h"), hdr).expect("Write vshader_shbin.h");

        cc::Build::new()
            .std("c11")
            .opt_level(2)
            .warnings(false)
            .include(&out_dir)
            .include(dkp.join("libctru").join("include"))
            .define("__3DS__", None)
            .file("3ds/c3d_bridge.c")
            .compile("blotworx_c3d_bridge");
    }
    println!("cargo:rerun-if-changed=third_party/box3d");
    println!("cargo:rerun-if-changed=src/blotworx/box3d_bridge.c");
    let mut build = cc::Build::new();
    build
        .include("third_party/box3d/include")
        .include("third_party/box3d/src")
        .std("c17")
        .opt_level(2)
        .define("NDEBUG", None)
        .define("BOX3D_DISABLE_AVX2", None)
        .warnings(false)
        .file("src/blotworx/box3d_bridge.c");
    let mut sources: Vec<_> = std::fs::read_dir("third_party/box3d/src")
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|s| s == "c"))
        .collect();
    sources.sort();
    for source in sources {
        build.file(source);
    }
    if build.get_compiler().is_like_msvc() {
        build.flag("/fp:precise");
    } else {
        build.flag_if_supported("-ffp-contract=off");
        if horizon {
            build.flag_if_supported("-Wno-incompatible-pointer-types");
        }
    }
    build.compile("blotworx_box3d");
    if std::env::var("CARGO_CFG_UNIX").is_ok() || horizon {
        println!("cargo:rustc-link-lib=m");
    }
}
