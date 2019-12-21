#![allow( non_upper_case_globals
        , non_camel_case_types
        , non_snake_case
        , dead_code)]
#![warn(unused_variables)]

extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::process::Command;
use os_info;

// WINDOWS
fn link_libs_windows(){
  let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  // The compiled libraries have been copied out of PJPROJECT to pjproject-sys/pjlibs/  
  println!("cargo:rustc-link-search={}/pjlibs/windows", project_dir);
  println!("cargo:rustc-link-lib=static=libpjproject-x86_64-x64-vc14-Release");
  println!("cargo:rustc-link-lib=static=Ole32");
}

// UNIX 
fn link_libs_unix(){
  let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

  // The compiled libraries have been copied out of PJPROJECT to pjproject-sys/pjlibs/
  println!("cargo:rustc-link-search={}/pjlibs/linux", project_dir);
  println!("cargo:rustc-link-search={}/pjlibs/linux/extras", project_dir);

  //Libraries inside PJPROEJCT
  println!("cargo:rustc-link-lib=pjsua-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=pjsip-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=pj-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=pjsip-simple-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=pjsua2-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=pjsip-ua-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=pjmedia-codec-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=pjsdp-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=pjmedia-videodev-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=pjmedia-audiodev-x86_64-unknown-linux-gnu");
                              
  println!("cargo:rustc-link-lib=pjmedia-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=pjnath-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=pjlib-util-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=yuv-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=ilbccodec-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=g7221codec-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=gsmcodec-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=resample-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=srtp-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=webrtc-x86_64-unknown-linux-gnu");
  println!("cargo:rustc-link-lib=speex-x86_64-unknown-linux-gnu");
  
  println!("cargo:rustc-link-lib=ssl");
  println!("cargo:rustc-link-lib=crypto");
  println!("cargo:rustc-link-lib=z");
  println!("cargo:rustc-link-lib=asound");

}

// MAC_OS
fn link_libs_macos(){
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // Need to include this so I can call core functions and security functions on mac
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=Security");
    // fuck me im clever for figuring this shit out

    println!("cargo:rustc-link-search={}/pjlibs/mac_os", project_dir);

    println!("cargo:rustc-link-lib=pjsua-x86_64-apple-darwin17.7.0");
    println!("cargo:rustc-link-lib=pjsip-x86_64-apple-darwin17.7.0");
    println!("cargo:rustc-link-lib=pj-x86_64-apple-darwin17.7.0");
    println!("cargo:rustc-link-lib=pjsip-simple-x86_64-apple-darwin17.7.0");
    println!("cargo:rustc-link-lib=pjsua2-x86_64-apple-darwin17.7.0");
    println!("cargo:rustc-link-lib=pjsip-ua-x86_64-apple-darwin17.7.0");
    println!("cargo:rustc-link-lib=pjmedia-codec-x86_64-apple-darwin17.7.0");
    println!("cargo:rustc-link-lib=pjsdp-x86_64-apple-darwin17.7.0");
    println!("cargo:rustc-link-lib=pjmedia-videodev-x86_64-apple-darwin17.7.0");	
    println!("cargo:rustc-link-lib=pjmedia-audiodev-x86_64-apple-darwin17.7.0");	
    println!("cargo:rustc-link-lib=pjmedia-x86_64-apple-darwin17.7.0");
    println!("cargo:rustc-link-lib=pjnath-x86_64-apple-darwin17.7.0");
    println!("cargo:rustc-link-lib=pjlib-util-x86_64-apple-darwin17.7.0");	
    println!("cargo:rustc-link-lib=yuv-x86_64-apple-darwin17.7.0");
    println!("cargo:rustc-link-lib=ilbccodec-x86_64-apple-darwin17.7.0");	
    println!("cargo:rustc-link-lib=g7221codec-x86_64-apple-darwin17.7.0");	
    println!("cargo:rustc-link-lib=gsmcodec-x86_64-apple-darwin17.7.0");
    println!("cargo:rustc-link-lib=resample-x86_64-apple-darwin17.7.0");
    println!("cargo:rustc-link-lib=srtp-x86_64-apple-darwin17.7.0");
    println!("cargo:rustc-link-lib=webrtc-x86_64-apple-darwin17.7.0");
    println!("cargo:rustc-link-lib=speex-x86_64-apple-darwin17.7.0");   

}

fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .clang_arg("-I./pjproject/pjlib/include")
        .clang_arg("-I./pjproject/pjsip/include")
        .clang_arg("-I./pjproject/pjlib-util/include")
        .clang_arg("-I./pjproject/pjmedia/include")
        .clang_arg("-I./pjproject/pjnath/include")
        .blacklist_item("FP_NAN")
        .blacklist_item("FP_INFINITE")
        .blacklist_item("FP_ZERO")
        .blacklist_item("FP_SUBNORMAL")
        .blacklist_item("FP_NORMAL")
        .derive_debug(true)
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings"); 

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

pub fn main() {

    //1. Fetch pjproject 
    let curr_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    Command::new("git").arg("submodule").arg("update").arg("--init").current_dir(&curr_dir).status().unwrap();

    //2. Create config_site.h (so bindgen doesnt complain about missing header files)  
    let file = File::create("pjproject/pjlib/include/pj/config_site.h");
    match file { 
      Ok(_x) => println!("config_site.h created"),
      Err(_x) => {
        println!("config_site.h not created, Error!");
        panic!("config_site.h not created, Error!");
      },
    };
    
    //3. Determine OS and Link Libraries Accordingly      
    let info = os_info::get();
    if info.os_type() == os_info::Type::Windows {
        link_libs_windows();
    }
    else if (info.os_type() == os_info::Type::Linux) || (info.os_type() == os_info::Type::Ubuntu) {
        link_libs_unix();
    }
    else if info.os_type() == os_info::Type::Macos {
        link_libs_macos();
    }

    //4. Produce bindings.rs file
    generate_bindings();

}
