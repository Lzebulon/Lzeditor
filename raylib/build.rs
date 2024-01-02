use std::{env, path::PathBuf};

pub fn main() {
    //track file
    println!("cargo:rerun-if-changed=src/wrapper.h");

    //generate bindings
    generate_bindings();
    compile_lib_linux();

    link_lib_linux();
}

pub fn get_blocked_enum_names() -> Vec<String> {
    vec![
        "BlendMode",
        "CameraMode",
        "CameraProjection",
        "ConfigFlags",
        "CubemapLayout",
        "FontType",
        "GamepadAxis",
        "GamepadButton",
        "Gesture",
        "KeyboardKey",
        "MaterialMapIndex",
        "MouseButton",
        "MouseCursor",
        "NPatchLayout",
        "PixelFormat",
        "ShaderAttributeDataType",
        "ShaderLocationIndex",
        "ShaderUniformDataType",
        "TextureFilter",
        "TextureWrap",
        "TraceLogLevel",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}

fn generate_bindings() {
    let header_path = "src/wrapper.h";

    let builder = bindgen::Builder::default()
        .header(header_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // .blocklist_item("DE2GRAD")
        // .blocklist_item("PI")
        // .blocklist_item("RAD2DEG")
        .blocklist_item("__gnuc_va_list")
        .blocklist_item("__bool_true_false_are_defined")
        .blocklist_item("false_")
        .blocklist_item("true_");

    //for enum_name in get_blocked_enum_names(){
    //    builder = builder.blocklist_type(format!("{}.*",enum_name))
    //}

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings !");
}

fn compile_lib_linux() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    if !std::process::Command::new("make")
        .current_dir("raylib/src")
        .arg("clean")
        .output()
        .expect("could not spawn `make`")
        .status
        .success()
    {
        panic!("error in make");
    }

    #[cfg(feature = "wayland")]
    let use_wayland = "USE_WAYLAND_DISPLAY=TRUE";
    #[cfg(not(feature = "wayland"))]
    let use_wayland = "";

    let make = std::process::Command::new("make")
        .current_dir("raylib/src")
        .arg("PLATFORM=PLATFORM_DESKTOP")
        .arg(format!("RAYLIB_RELEASE_PATH={}", out_path.display()))
        .arg(use_wayland)
        .status()
        .expect("could not spawn `make`");
    if !make.success() {
        panic!("error in make {}", make);
    }
    if !std::process::Command::new("make")
        .current_dir("raylib/src")
        .arg("clean")
        .output()
        .expect("could not spawn `make`")
        .status
        .success()
    {
        panic!("error in make");
    }
}

fn link_lib_linux() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search=native={}", out_path.display());

    println!("cargo:rustc-link-lib=static=raylib");
}
