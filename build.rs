#[cfg(not(any(
    feature = "neovim-0-11",
    feature = "neovim-0-12",
    feature = "neovim-nightly"
)))]
compile_error!(
    "You must enable one of the features: neovim-0-11, neovim-0-12, \
     neovim-nightly"
);
#[cfg(all(feature = "mlua", feature = "oximlua"))]
compile_error!("You must enable only one of the features: mlua, oximlua");

fn main() {
    println!("cargo:rerun-if-changed=build");
}
