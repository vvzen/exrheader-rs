# Why

If you just want to look at the metadata of an OpenEXR file, `exrheader` does a fantastic job at that.
Unfortunately, the way it's built and packaged by most package managers requires you to have quite a few shared libraries installed as well, which is not ideal when working in more constrained environments.

For example, on my Arch installation (exrheader v3.4.5):
```
$ ldd $(which exrheader)
        linux-vdso.so.1 (0x00007f6569052000)
        libOpenEXR-3_4.so.33 => /usr/lib/libOpenEXR-3_4.so.33 (0x00007f6568ed2000)
        libstdc++.so.6 => /usr/lib/libstdc++.so.6 (0x00007f6568c00000)
        libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x00007f6568ea5000)
        libc.so.6 => /usr/lib/libc.so.6 (0x00007f6568a0f000)
        libIlmThread-3_4.so.33 => /usr/lib/libIlmThread-3_4.so.33 (0x00007f6568e9b000)
        libOpenEXRCore-3_4.so.33 => /usr/lib/libOpenEXRCore-3_4.so.33 (0x00007f6568938000)
        libIex-3_4.so.33 => /usr/lib/libIex-3_4.so.33 (0x00007f65688b2000)
        libImath-3_2.so.30 => /usr/lib/libImath-3_2.so.30 (0x00007f6568860000)
        libm.so.6 => /usr/lib/libm.so.6 (0x00007f6568742000)
        /lib64/ld-linux-x86-64.so.2 => /usr/lib64/ld-linux-x86-64.so.2 (0x00007f6569054000)
        libdeflate.so.0 => /usr/lib/libdeflate.so.0 (0x00007f656872a000)
        libopenjph.so.0.24 => /usr/lib/libopenjph.so.0.24 (0x00007f656869f000)
```

On the other hand, this rewrite in Rust leverages the [excellent work](https://github.com/johannesvollmer/exrs) by Johannes Vollmer, allowing this CLI to only link against glibc and a few other essentials libs, making it a lot more portable:

```
$ ldd ./target/release/exrheader-rs
        linux-vdso.so.1 (0x00007fa5ef221000)
        libc.so.6 => /usr/lib/libc.so.6 (0x00007fa5eec0f000)
        /lib64/ld-linux-x86-64.so.2 => /usr/lib64/ld-linux-x86-64.so.2 (0x00007fa5ef223000)
        libgcc_s.so.1 => /usr/lib/libgcc_s.so.1 (0x00007fa5ef1ab000)
```

So this CLI trades off disk space for ease of use.

```bash
$ du -sh $(which exrheader-rs)
2.6M    /home/vv/.cargo/bin/exrheader-rs

$ du -sh $(which exrheader)
32K     /usr/bin/exrheader
```

Altough, as always, reality is more complex:
```bash
$ ldd $(which exrheader) | awk '{print $3}' | xargs -I {} du -Lsh {}
1.2M    /usr/lib/libOpenEXR-3_4.so.33
2.6M    /usr/lib/libstdc++.so.6
176K    /usr/lib/libgcc_s.so.1
2.0M    /usr/lib/libc.so.6
40K     /usr/lib/libIlmThread-3_4.so.33
348K    /usr/lib/libOpenEXRCore-3_4.so.33
532K    /usr/lib/libIex-3_4.so.33
324K    /usr/lib/libImath-3_2.so.30
1.2M    /usr/lib/libm.so.6
232K    /usr/lib64/ld-linux-x86-64.so.2
92K     /usr/lib/libdeflate.so.0
512K    /usr/lib/libopenjph.so.0.24
```

# Installation

## From source

If you have the Rust toolchain setup locally, you can install `exrheader-rs` via `cargo`:

``` bash
cargo install --git ssh://git@github.com/vvzen/exrheader-rs.git --bin exrheader-rs
```

## From prebuilt binaries

I need to setup CI for this repo, so currently I don't ship any prebuilt binaries.
Once I figure out to how build binaries here via Github Actions, I suppose I'll make releases for Linux and macOS!
