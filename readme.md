Set of utilities I made for my kindle. 

I recommend using `zig-build` to compile. Don't need to worry about getting the required toolchain that way. 

Unfortunately, from 2022> Rust raised the minimum glib version to 2.17>. 
This is an issue because the kindle uses 2.12, and I am not able to use `musl` instead of `gnu` since I am linking to a shared library on the kindle. 

Unfortunately, for `musl` dynamic loading requires `ld-musl-arm.so.1` to be present in `/lib`.  

I got the `.so` from [here](https://packages.debian.org/sid/armel/musl/download) and copied it into `/lib` (require `mntroot rw`)  
However, now I could simply do 1, and everything worked!  

# Todo
- Make a Kual package to do something similar to `usbnet` - store the required `.so` file in its local dir and use the compile directive to point to the right file

