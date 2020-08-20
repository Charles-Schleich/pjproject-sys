# pjproject-sys

A rust project that creates rust bindings to pjproject.
For a minimal coverage sip phone library in Rust please visit https://github.com/Charles-Schleich/sip-phone-rs . 


## Disclaimer
This project contains prebuilt libraries of pjproject in `pjlibs/` as the build process for windows  requires visual studio and  is not super easily integrated into the cargo build pipeline. 
The libraries built here were built on `Ubuntu 18.04`, `Windows 10`, `MacOs High Sierra`. 
I would strongly encourage you to compile pjproject and use your compiled library files instead, but atleast my precompiled library files are another option.

At some point in the future i may Integrate a proper build pipeline of pjproject into the build pipeline of pjproject-sys, but this would have to work for all three platforms (Linux, MacOS and Windows).
I will happily accept pull requests if anyone wants to give this a shot (specifically for windows), Mac and Linux shouldnt be too difficult.

## Dependencies 
- `openssl-1.0.2s`
- `zlib-1.2.11`
- `cargo =< 1.37.0` (will probably work on older versions)
- `rustc =< 1.37.0`
- For rust's bindgen: pjproject source tree to be in the root folder of this project, this should be included as a git submodule (https://github.com/pjsip/pjproject) 

## Building
`cargo build`

### TODO
- Restructure the build pipeline to potentially include pjproject compilation (large endeavor speficifally for windows)
