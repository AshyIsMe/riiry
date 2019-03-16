# riiry launcher

Rewrite it in Rust they said...

rofi launcher is pretty sweet (especially those app icons amirite?) but why isn't it cross platform?
Also, haven't they heard Rust is what computers crave?

## Build

Rust has pretty nice tooling, just build and run with cargo:


```
cargo build

cargo run
```


### NixOS

`default.nix` is ready to be used with `nix-shell` and should be as simple as:

```
git clone https://github.com/AshyIsMe/riiry
cd riiry

nix-shell

cargo run

```

(Note: default.nix is based on what is required for neovim-gtk and probably has some extras still)

### Other linux distros

Just use nix (untested lol...)

https://nixos.org/nix/download.html

### OSX

Just use nix (untested lol...)

https://nixos.org/nix/download.html

### Windows

Should be possible but not tested yet.

gtk-rs supports windows: http://gtk-rs.org/docs/requirements.html#windows

fd supports windows: https://github.com/sharkdp/fd#on-windows


### Screenshot

Behold:

![alt text](https://github.com/ashyisme/riiry/raw/master/screenshots/screenshot1.png "Wow")
