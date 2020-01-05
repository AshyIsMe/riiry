with import <nixpkgs> {};
pkgs.mkShell {
  buildInputs = [

    gtkd
    atk
    cairo
    gdk_pixbuf
    glib
    gnome2.pango

    gnome3.glade
    cargo
    rustc
    rustfmt
    rustup
    pkgconfig
  ];
}
