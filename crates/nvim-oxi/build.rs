fn main() {
    #[cfg(not(any(
        feature = "neovim-0-7",
        feature = "neovim-0-8",
        feature = "neovim-nightly"
    )))]
    compile_error!(
        "You must enable one of the features: neovim-0-7, neovim-0-8, \
         neovim-nightly"
    );

    #[cfg(all(
        feature = "neovim-0-7",
        any(feature = "neovim-0-8", feature = "neovim-nightly")
    ))]
    compile_error!(
        "You can only enable one of the features: neovim-0-7, neovim-0-8, \
         neovim-nightly"
    );

    #[cfg(all(feature = "neovim-0-8", feature = "neovim-nightly"))]
    compile_error!(
        "You can only enable one of the features: neovim-0-7, neovim-0-8, \
         neovim-nightly"
    );

    println!("cargo:rerun-if-changed=build");
}
