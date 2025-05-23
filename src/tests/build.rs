use core::error::Error;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
use std::{env, io};

use cargo_metadata::camino::Utf8PathBuf;

/// Builds the library required to run integration tests within Neovim.
///
/// This function is designed to be used in the build script (`build.rs`) of a
/// crate containing integration tests for Neovim plugins. It is tightly
/// coupled with the [`test`](crate::test) macro, which is used to annotate the
/// functions to test.
///
/// Together, they enable a workflow where the build script compiles the test
/// crate into a dynamic library, and the macro generates functions that load
/// this library into Neovim and execute the tests within it.
///
/// # Usage
///
/// Add the following to your test crate's `build.rs`:
///
/// ```ignore
/// fn main() -> Result<(), nvim_oxi::tests::BuildError> {
///     nvim_oxi::tests::build()
/// }
/// ```
///
/// # Notes
///
/// Like the plugin crate, the test crate must also be configured to be built
/// as a dynamic library by including the following in its Cargo.toml:
///
/// ```toml
/// [lib]
/// crate-type = ["cdylib"]
/// ```
pub fn build() -> Result<(), BuildError> {
    let Some(_g) = BuildGuard::<EnvVarGuard>::new()? else { return Ok(()) };
    let compilation_opts = CompilationOpts::from_env()?;
    let manifest_path = EnvVar::get("CARGO_MANIFEST_PATH")?;
    let manifest = CargoManifest::from_path(manifest_path.as_str())?;
    let features = EnabledFeatures::from_env(&manifest)?;
    BuildCommand::new(&manifest, &compilation_opts, &features).exec()?;
    println!(
        "cargo:rustc-env={}={}",
        manifest.profile_env(),
        compilation_opts.profile.as_str()
    );
    // Rerun the build script if the compiled library is removed/changed.
    println!(
        "cargo:rerun-if-changed=\"{}\"",
        manifest.library_path(compilation_opts.profile.as_str()),
    );
    Ok(())
}

/// An opaque error returned when [`build`]ing a test crate fails.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct BuildError {
    #[from]
    kind: BuildErrorKind,
}

pub(super) struct CargoManifest {
    metadata: cargo_metadata::Metadata,
}

struct BuildGuard<G: Guard> {
    guard: Option<G>,
}

struct EnvVarGuard;

struct CompilationOpts {
    profile: Profile,
}

enum Profile {
    Debug,
    Release,
    Other(EnvVar),
}

struct EnabledFeatures {
    features: Vec<String>,
}

struct BuildCommand {
    command: Command,
}

impl EnvVarGuard {
    const NAME: &'static str = "NVIM_OXI_BUILDING_TESTS";
}

struct EnvVar(String);

#[derive(Debug, thiserror::Error)]
enum BuildErrorKind {
    #[error("couldn't build tests: {0}")]
    Build(io::Error),

    #[error("couldn't acquire guard: {0}")]
    CouldntAcquireGuard(Box<dyn Error>),

    #[error("couldn't read manifest: {0}")]
    CouldntReadManifest(cargo_metadata::Error),

    #[error("nvim_oxi::tests::build() can only be used inside a build script")]
    NotInBuildScript,

    #[error("couldn't get the root package")]
    NoRootPackage,
}

impl<G: Guard> BuildGuard<G> {
    fn new() -> Result<Option<Self>, BuildError> {
        match G::acquire() {
            Ok(guard) => Ok(Some(Self { guard: Some(guard) })),
            Err(Ok(_busy)) => Ok(None),
            Err(Err(acquire_err)) => {
                Err(BuildErrorKind::CouldntAcquireGuard(Box::new(acquire_err))
                    .into())
            },
        }
    }
}

impl CompilationOpts {
    fn from_env() -> Result<Self, BuildError> {
        Ok(Self { profile: Profile::from_env_var(EnvVar::get("PROFILE")?) })
    }
}

impl Profile {
    fn as_args(&self) -> Vec<impl AsRef<OsStr> + '_> {
        enum Arg<'a> {
            Str(&'a str),
            EnvVar(&'a EnvVar),
        }

        impl AsRef<OsStr> for Arg<'_> {
            fn as_ref(&self) -> &OsStr {
                match self {
                    Arg::Str(s) => s.as_ref(),
                    Arg::EnvVar(s) => s.as_str().as_ref(),
                }
            }
        }

        match self {
            Profile::Debug => vec![],
            Profile::Release => vec![Arg::Str("--release")],
            Profile::Other(other) => {
                vec![Arg::Str("--profile"), Arg::EnvVar(other)]
            },
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Profile::Debug => "debug",
            Profile::Release => "release",
            Profile::Other(other) => other.as_str(),
        }
    }

    fn from_env_var(profile: EnvVar) -> Self {
        match profile.as_str() {
            "debug" => Self::Debug,
            "release" => Self::Release,
            _ => Self::Other(profile),
        }
    }
}

impl CargoManifest {
    pub(super) fn from_path(
        path: impl AsRef<Path>,
    ) -> Result<Self, BuildError> {
        let metadata = cargo_metadata::MetadataCommand::new()
            .manifest_path(path.as_ref())
            .exec()
            .map_err(BuildErrorKind::CouldntReadManifest)?;

        if metadata.root_package().is_none() {
            return Err(BuildErrorKind::NoRootPackage.into());
        }

        Ok(Self { metadata })
    }

    /// The name of the environment variable representing the profile the test
    /// crate was compiled for.
    pub(super) fn profile_env(&self) -> String {
        format!(
            "NVIM_OXI_TEST_BUILD_PROFILE_{}",
            self.root_package().name.to_ascii_uppercase().replace('-', "_")
        )
    }

    /// The path to the target directory containing the compiled test library
    /// for the crate represented by this [`CargoManifest`].
    pub(super) fn target_dir(&self) -> Utf8PathBuf {
        self.metadata
            .target_directory
            // We have to use a different target directory to avoid a deadlock
            // caused by invoking `cargo build` in a build script.
            //
            // See https://github.com/rust-lang/cargo/issues/6412 for more.
            .join("nvim_oxi_tests")
            // Namespace by the package name to allow for multiple test crates
            // in the same workspace.
            .join(&self.root_package().name)
    }

    pub(super) fn library_path(&self, profile_name: &str) -> Utf8PathBuf {
        let library_name = format!(
            "{prefix}{crate_name}{suffix}",
            prefix = env::consts::DLL_PREFIX,
            suffix = env::consts::DLL_SUFFIX,
            crate_name = self.root_package().name.replace('-', "_"),
        );
        self.target_dir().join(profile_name).join(library_name)
    }

    fn root_package(&self) -> &cargo_metadata::Package {
        self.metadata.root_package().expect("checked in `from_path()`")
    }
}

impl EnabledFeatures {
    fn from_env(manifest: &CargoManifest) -> Result<Self, BuildError> {
        let mut features = Vec::new();

        for feature in manifest.root_package().features.keys() {
            let env = format!(
                "CARGO_FEATURE_{}",
                feature.to_ascii_uppercase().replace('-', "_")
            );
            if EnvVar::get(&env).is_ok() {
                features.push(feature.clone());
            }
        }

        Ok(Self { features })
    }
}

impl BuildCommand {
    fn exec(mut self) -> Result<(), BuildError> {
        self.command
            .status()
            .map(|_| ())
            .map_err(|io_err| BuildErrorKind::Build(io_err).into())
    }

    fn new(
        manifest: &CargoManifest,
        compilation_opts: &CompilationOpts,
        enabled_features: &EnabledFeatures,
    ) -> Self {
        let mut command = Command::new("cargo");
        command
            .arg("build")
            .args(compilation_opts.profile.as_args())
            .args(["--target-dir", manifest.target_dir().as_str()])
            .arg("--no-default-features")
            .arg("--features")
            .arg(enabled_features.features.join(","));
        Self { command }
    }
}

impl EnvVar {
    fn as_str(&self) -> &str {
        &self.0
    }

    fn get(env: &str) -> Result<Self, BuildError> {
        match env::var(env) {
            Ok(value) => Ok(Self(value)),
            Err(_) => Err(BuildErrorKind::NotInBuildScript.into()),
        }
    }
}

impl Guard for EnvVarGuard {
    type Error = env::VarError;

    fn acquire() -> Result<Self, Result<GuardBusy, Self::Error>> {
        match env::var(Self::NAME) {
            Ok(_) => Err(Ok(GuardBusy)),
            Err(env::VarError::NotPresent) => unsafe {
                env::set_var(Self::NAME, "1");
                Ok(Self)
            },
            Err(var_error) => Err(Err(var_error)),
        }
    }

    fn release(self) -> Result<(), Self::Error> {
        // Env variables are process-local.
        Ok(())
    }
}

impl<G: Guard> Drop for BuildGuard<G> {
    fn drop(&mut self) {
        if let Err(err) = self.guard.take().unwrap().release() {
            panic!("couldn't release guard: {err}");
        }
    }
}

trait Guard: Sized {
    type Error: Error + 'static;
    fn acquire() -> Result<Self, Result<GuardBusy, Self::Error>>;
    fn release(self) -> Result<(), Self::Error>;
}

/// A sentinel value returned by [`Guard::acquire()`] indicating that the guard
/// has already been acquired by another build process.
struct GuardBusy;
