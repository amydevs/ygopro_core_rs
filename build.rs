extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    let deps_folder = "deps";

    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let target_os = env::var("CARGO_CFG_TARGET_OS");
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY");
    
    // compile lua
    let lua_dir = PathBuf::from(deps_folder).join("lua53");

    let mut lua_cc_config = cc::Build::new();
    lua_cc_config.warnings(false);

    if target_os == Ok("linux".to_string()) {
        lua_cc_config.define("LUA_USE_LINUX", None);
    } else if target_os == Ok("macos".to_string()) {
        lua_cc_config.define("LUA_USE_MACOSX", None);
    } else if target_family == Ok("unix".to_string()) {
        lua_cc_config.define("LUA_USE_POSIX", None);
    } else if target_family == Ok("windows".to_string()) {
        lua_cc_config.define("LUA_USE_WINDOWS", None);
    }

    let mut lua_cc_config_build = lua_cc_config.include(&lua_dir);

    lua_cc_config_build = lua_cc_config_build
        .file(lua_dir.join("lapi.c"))
        .file(lua_dir.join("lauxlib.c"))
        .file(lua_dir.join("lbaselib.c"))
        .file(lua_dir.join("lbitlib.c"))
        .file(lua_dir.join("lcode.c"))
        .file(lua_dir.join("lcorolib.c"))
        .file(lua_dir.join("lctype.c"))
        .file(lua_dir.join("ldblib.c"))
        .file(lua_dir.join("ldebug.c"))
        .file(lua_dir.join("ldo.c"))
        .file(lua_dir.join("ldump.c"))
        .file(lua_dir.join("lfunc.c"))
        .file(lua_dir.join("lgc.c"))
        .file(lua_dir.join("liolib.c"))
        .file(lua_dir.join("llex.c"))
        .file(lua_dir.join("lmathlib.c"))
        .file(lua_dir.join("lmem.c"))
        .file(lua_dir.join("loadlib.c"))
        .file(lua_dir.join("lobject.c"))
        .file(lua_dir.join("lopcodes.c"))
        .file(lua_dir.join("lparser.c"))
        .file(lua_dir.join("lstate.c"))
        .file(lua_dir.join("lstring.c"))
        .file(lua_dir.join("lstrlib.c"))
        .file(lua_dir.join("ltable.c"))
        .file(lua_dir.join("ltablib.c"))
        .file(lua_dir.join("ltm.c"))
        .file(lua_dir.join("lundump.c"))
        .file(lua_dir.join("lutf8lib.c"))
        .file(lua_dir.join("lvm.c"))
        .file(lua_dir.join("lzio.c"));

    if !cfg!(feature = "lua-no-oslib") {
        lua_cc_config_build = lua_cc_config_build
            .file(lua_dir.join("loslib.c"))
            .file(lua_dir.join("linit.c"));
    }

    lua_cc_config_build
        .out_dir(dst.join("lib"))
        .compile("liblua5.3.a");

    // compile the ygopro
    let ygopro_dir = PathBuf::from(deps_folder).join("ygopro-core");
    let mut ygopro_cc_config = cc::Build::new();
    ygopro_cc_config.cpp(true);

    println!("cargo:rustc-link-search={}", dst.join("lib").to_str().unwrap());
    println!("cargo:rustc-link-lib=lua5.3");

    let mut ygopro_cc_config_build = ygopro_cc_config.include(&ygopro_dir).include(&lua_dir);
    ygopro_cc_config_build = ygopro_cc_config_build
        .file(ygopro_dir.join("card.cpp"))
        .file(ygopro_dir.join("duel.cpp"))
        .file(ygopro_dir.join("effect.cpp"))
        .file(ygopro_dir.join("field.cpp"))
        .file(ygopro_dir.join("group.cpp"))
        .file(ygopro_dir.join("interpreter.cpp"))
        .file(ygopro_dir.join("libcard.cpp"))
        .file(ygopro_dir.join("libdebug.cpp"))
        .file(ygopro_dir.join("libduel.cpp"))
        .file(ygopro_dir.join("libeffect.cpp"))
        .file(ygopro_dir.join("libgroup.cpp"))
        .file(ygopro_dir.join("ocgapi.cpp"))
        .file(ygopro_dir.join("operations.cpp"))
        .file(ygopro_dir.join("playerop.cpp"))
        .file(ygopro_dir.join("processor.cpp"))
        .file(ygopro_dir.join("scriptlib.cpp"));

    ygopro_cc_config_build
        .out_dir(dst.join("lib"))
        .compile("ocgcore");

    // generate the bindings
    let bindings = bindgen::Builder::default()
        .header(ygopro_dir.join("ocgapi.h").to_str().unwrap())
        .clang_arg(format!("-I{}", ygopro_dir.to_str().unwrap()))
        .clang_arg(format!("-I{}", lua_dir.to_str().unwrap()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(dst.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}