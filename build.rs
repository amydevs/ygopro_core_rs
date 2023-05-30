use std::env;
use std::path::{PathBuf, Path};

fn main() {
    let base_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let src_folder = Path::new(&base_dir).join("ygopro-core");

    println!("cargo:rerun-if-changed={}", src_folder.display());
    println!("cargo:rustc-link-search={}", src_folder.display());

    let src_files = [
        "card.cpp",
        "duel.cpp",
        "effect.cpp",
        "field.cpp",
        "group.cpp",
        "interpreter.cpp",
        "libcard.cpp",
        "libdebug.cpp",
        "libduel.cpp",
        "libeffect.cpp",
        "libgroup.cpp",
        "ocgapi.cpp",
        "operations.cpp",
        "playerop.cpp",
        "processor.cpp",
        "scriptlib.cpp",
    ];

    cc::Build::new()
        .cpp(true)
        .files(src_files.map(|f| src_folder.join(f)))
        .includes(["./include"])
        .compile("ocgcore.lib");

    let bindings = bindgen::Builder::default()
        .header("ygopro-core/ocgapi.h")
        .clang_arg("-I./ygopro-core")
        .clang_arg("-I./include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}