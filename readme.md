Set of utilities I made for my kindle. 

I recommend using `zig-build` to compile. Don't need to worry about getting the required toolchain that way. 

Unfortunately, from 2022> Rust raised the minimum glib version to 2.17>. 
This is an issue because the kindle uses 2.12, and I am not able to use `musl` instead of `gnu` since I am linking to a shared library on the kindle. 
Thus, my options were:  
1. Downgrade the rust compiler
2. Dynamically load the shared libraries in code. 

I went for 2.  

Luckily, `bindgen` already plays nicely with `libloading` - simply use `--dynamic-loading LIPC` to generate in "dynamic mode". 
Unfortunately not documented well, but read more here:
- [Github issue](https://github.com/rust-lang/rust-bindgen/issues/1541)
- [Source code](https://docs.rs/bindgen/latest/src/bindgen/codegen/dyngen.rs.html)