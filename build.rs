// #[cfg(not(any(
//     feature = "neovim-0-8",
//     feature = "neovim-0-9",
//     feature = "neovim-nightly"
// )))]
// compile_error!(
//     "You must enable one of the features: neovim-0-8, neovim-0-9, \
//      neovim-nightly"
// );

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

use std::{env, io};
use std::path::PathBuf;

fn locate_nvim() -> PathBuf {
    if let Ok(path) = which::which("nvim")  {
        return path.parent().unwrap().to_path_buf();
    }

    PathBuf::from(env::var("NVIM_DIR").expect("Could not locate NeoVim."))
}

fn main() {
    println!("cargo:rerun-if-changed=build");
    println!("cargo:rustc-link-search=native={}", locate_nvim().display());
}
