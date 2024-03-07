#[cfg(not(any(
    feature = "neovim-0-8",
    feature = "neovim-0-9",
    feature = "neovim-nightly"
)))]
compile_error!(
    "You must enable one of the features: neovim-0-8, neovim-0-9, \
     neovim-nightly"
);

#[cfg(all(
    feature = "neovim-0-8",
    any(feature = "neovim-0-9", feature = "neovim-nightly")
))]
compile_error!(
    "You can only enable one of the features: neovim-0-8, neovim-0-9, \
     neovim-nightly"
);

#[cfg(all(feature = "neovim-0-9", feature = "neovim-nightly"))]
compile_error!(
    "You can only enable one of the features: neovim-0-8, neovim-0-9, \
     neovim-nightly"
);

fn main() {
    println!("cargo:rerun-if-changed=build");

    if cfg!(target_env = "msvc") {
        println!("cargo:rustc-link-search=native=lib");
        println!("cargo:rustc-link-lib=dylib=nvim");
        println!("cargo:rustc-link-lib=dylib=lua51");
    }
}
