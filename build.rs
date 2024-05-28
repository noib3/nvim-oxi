#[cfg(not(any(
    feature = "neovim-0-9",
    feature = "neovim-0-10",
    feature = "neovim-nightly"
)))]
compile_error!(
    "You must enable one of the features: neovim-0-9, neovim-0-10, \
     neovim-nightly"
);

fn main() {
    println!("cargo:rerun-if-changed=build");
}
