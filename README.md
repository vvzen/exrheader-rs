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

# Notice

This project is still heavily WIP and doesn't produce identical output to the one produced by `exrheader`. Some details will differ since I tried to optimize for legibility instead of keeping 100% compatibility on the generated output.

For some images, the results should already be pretty similar.
For example:

```
$ exrheader openexr-images/Tiles/Spirals.exr 2>/dev/null

file openexr-images/Tiles/Spirals.exr:

file format version: 2, flags 0x200
capDate (type string): "2004:01:19 10:25:14"
channels (type chlist):
    A, 16-bit floating-point, sampling 1 1
    B, 16-bit floating-point, sampling 1 1
    G, 16-bit floating-point, sampling 1 1
    R, 16-bit floating-point, sampling 1 1
    Z, 32-bit floating-point, sampling 1 1
chromaticities (type chromaticities):
    red   (0.62955 0.341)
    green (0.2867 0.6108)
    blue  (0.1489 0.07125)
    white (0.3155 0.33165)
compression (type compression): pxr24: lossy 24-bit float compression, in blocks of 16 scan lines.
dataWindow (type box2i): (-20 -20) - (1019 1019)
displayWindow (type box2i): (0 0) - (999 999)
lineOrder (type lineOrder): increasing y
owner (type string): "Copyright 2004 Industrial Light & Magic"
pixelAspectRatio (type float): 1
preview (type preview): 100 by 100 pixels
screenWindowCenter (type v2f): (0 0)
screenWindowWidth (type float): 1
tiles (type tiledesc):
    single level
    tile size 287 by 126 pixels
type (type string): "tiledimage"
utcOffset (type float): 28800
whiteLuminance (type float): 90
```

vs

```
$ exrheader-rs openexr-images/Tiles/Spirals.exr 2>/dev/null
File 'openexr-images/Tiles/Spirals.exr'

File format version: 2
Flags:
	deep: false
	multiple layers: false
	long names: false
	single layer and tiled: true
capDate: "2004:01:19 10:25:14"
channels:
	A, 16-bit floating-point, sampling 1 1
	B, 16-bit floating-point, sampling 1 1
	G, 16-bit floating-point, sampling 1 1
	R, 16-bit floating-point, sampling 1 1
	Z, 32-bit floating-point, sampling 1 1
chromaticies:
	red: (0.62955 0.341)
	green: (0.2867 0.6108)
	blue: (0.1489 0.07125)
	white: (0.3155 0.33165)
chunkCount: 36
compression: pxr24
dataWindow: (-20 -20) - (1019 1019)
displayWindow: (0 0) - (999 999)
lineOrder: increasing
owner: "Copyright 2004 Industrial Light & Magic"
pixelAspectRatio: 1
preview: 100 by 100 pixels
screenWindowCenter: (0 0)
screenWindowWidth: 1
tiles:
	single level
	tile size: 287 by 126 pixels
type: "tiledimage"
utcOffset: 28800
whiteLuminance: 90
```


# Installation

## From source

If you have the Rust toolchain setup locally, you can install `exrheader-rs` via `cargo`:

``` bash
cargo install --git ssh://git@github.com/vvzen/exrheader-rs.git --bin exrheader-rs
```

## From prebuilt binaries

This project uses [cargo-dist](https://axodotdev.github.io/cargo-dist/) to create releases.
