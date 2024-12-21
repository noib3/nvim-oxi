//! Functions called by the code generated by `#[nvim_oxi::test].`

use std::any::Any;
use std::env;
use std::fmt::{Debug, Display};
use std::panic::{self, Location, UnwindSafe};
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::sync::{Arc, OnceLock};
use std::thread;

use crate::IntoResult;

/// The body of the `#[nvim_oxi::plugin]` generated by the `#[nvim_oxi::test]`
/// macro.
pub fn plugin_body<F, R>(test_body: F)
where
    F: FnOnce() -> R + UnwindSafe,
    R: IntoResult<()>,
    R::Error: Display,
{
    let panic_info: Arc<OnceLock<PanicInfo>> = Arc::default();

    {
        let panic_info = panic_info.clone();

        panic::set_hook(Box::new(move |info| {
            let _ = panic_info.set(info.into());
        }));
    }

    let result = match panic::catch_unwind(|| test_body().into_result()) {
        Ok(Ok(())) => Ok(()),
        Ok(Err(err)) => Err(Failure::Error(err.to_string())),
        Err(_) => Err(Failure::Panic(panic_info.get().unwrap().clone())),
    };

    exit(result);
}

/// The body of the `#[nvim_oxi::plugin]` generated by the `#[nvim_oxi::test]`
/// macro when the `test-terminator` feature is enabled and the test function
/// takes a `TestTerminator` argument.
#[cfg(feature = "test-terminator")]
pub fn plugin_body_with_terminator<F>(test_body: F)
where
    F: FnOnce(super::terminator::TestTerminator),
{
    let lock = Arc::new(OnceLock::<Result<(), Failure>>::new());

    let handle = {
        let lock = lock.clone();

        crate::libuv::AsyncHandle::new(move || {
            let result = lock.get().unwrap().clone();
            crate::schedule(move |()| exit(result));
            Ok::<_, std::convert::Infallible>(())
        })
    }
    .unwrap();

    test_body(super::terminator::TestTerminator { lock, handle });
}

/// The body of the `#[test]` generated by the `#[nvim_oxi::test]` macro.
pub fn test_body(
    crate_name: &str,
    manifest_path: &str,
    plugin_name: &str,
    extra_cmd: Option<&str>,
) -> Result<(), String> {
    panic::set_hook(Box::new(move |info| {
        let mut info = info
            .payload()
            .downcast_ref::<PanicInfo>()
            .cloned()
            .unwrap_or_else(|| info.into());

        if let Some(thread) = thread::current().name() {
            if !thread.is_empty() {
                info.thread = thread.to_owned();
            }
        }

        eprintln!("{}", info);
    }));

    let output =
        run_nvim_command(crate_name, manifest_path, plugin_name, extra_cmd)?
            .output()
            .map_err(|err| err.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stdout = stdout.trim();

    if !stdout.is_empty() {
        println!("{}", stdout)
    }

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);

    if stderr.is_empty() {
        let msg = output
            .status
            .code()
            .map(|i| format!("Neovim exited with non-zero exit code: {i}"))
            .unwrap_or_else(|| String::from("Neovim segfaulted"));

        return Err(msg);
    }

    let Ok(failure) = Failure::from_str(&stderr) else {
        return Err(stderr.into_owned());
    };

    match failure {
        Failure::Error(err) => Err(err),
        Failure::Panic(info) => panic::panic_any(info),
    }
}

fn exit(result: Result<(), Failure>) {
    #[cfg(all(feature = "neovim-0-9", not(feature = "neovim-0-10")))]
    let exec = |cmd: &str| crate::api::exec(cmd, false).unwrap();

    #[cfg(feature = "neovim-0-10")]
    let exec = |cmd: &str| {
        let opts = crate::api::opts::ExecOpts::builder().output(false).build();
        crate::api::exec2(cmd, &opts).unwrap();
    };

    if let Err(failure) = result {
        eprintln!("{failure}");
        exec("cquit 1");
    } else {
        exec("qall!");
    }
}

fn run_nvim_command(
    crate_name: &str,
    manifest_path: &str,
    plugin_name: &str,
    extra_cmd: Option<&str>,
) -> Result<Command, String> {
    let manifest = super::build::CargoManifest::from_path(manifest_path)
        .map_err(|err| err.to_string())?;

    let target_dir: PathBuf = manifest.target_dir().into().into();

    let profile =
        env::var(manifest.profile_env()).map_err(|err| err.to_string())?;

    let library_name = format!(
        "{prefix}{crate_name}{suffix}",
        prefix = env::consts::DLL_PREFIX,
        suffix = env::consts::DLL_SUFFIX,
    );

    let library_path = target_dir.join(profile).join(library_name);

    if !library_path.exists() {
        return Err(format!(
            "couldn't find library at '{}'. Did you forget to use the build \
             script?",
            library_path.display()
        ));
    }

    let load_library = format!(
        "lua local f = package.loadlib([[{}]], 'luaopen_{}'); f()",
        library_path.display(),
        plugin_name,
    );

    let mut command = Command::new("nvim");

    command
        .args(["-u", "NONE", "--headless"])
        .args(["-i", "NONE"])
        .args(["-c", "set noswapfile"])
        .args(extra_cmd.map(|cmd| ["-c", cmd]).unwrap_or_default())
        .args(["-c", &load_library]);

    Ok(command)
}

#[derive(Clone)]
pub(super) struct PanicInfo {
    msg: String,
    thread: String,
    file: Option<String>,
    line: Option<u32>,
    column: Option<u32>,
}

impl Debug for PanicInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "panic:{}", self.msg)?;

        write!(f, "\nthread:{}", self.thread)?;

        if let Some(file) = &self.file {
            write!(f, "\nfile:{file}")?;
        }

        if let Some(line) = self.line {
            write!(f, "\nline:{line}")?;
        }

        if let Some(column) = self.column {
            write!(f, "\ncolumn:{column}")?;
        }

        Ok(())
    }
}

impl Display for PanicInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "thread '{}' panicked", self.thread)?;

        if let Some(file) = &self.file {
            write!(f, " at {file}")?;

            if let (Some(line), Some(col)) = (self.line, self.column) {
                write!(f, ":{line}:{col}")?;
            }
        }

        write!(f, ":\n{}", self.msg)?;

        Ok(())
    }
}

impl FromStr for PanicInfo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut info = PanicInfo {
            msg: String::new(),
            thread: String::new(),
            file: None,
            line: None,
            column: None,
        };

        let (_, s) = s.split_once("panic:").ok_or(())?;

        let (msg, s) = s.split_once("thread:").ok_or(())?;
        info.msg = msg.trim().to_owned();

        let (thread, s) = s.split_once("file:").ok_or(())?;
        info.thread = thread.trim().to_owned();

        let (file, s) = s.split_once("line:").ok_or(())?;
        info.file = Some(file.trim().to_owned());

        let (line, s) = s.split_once("column:").ok_or(())?;
        info.line = Some(line.trim().parse().map_err(|_| ())?);

        let column = s.trim().parse().map_err(|_| ())?;
        info.column = Some(column);

        Ok(info)
    }
}

impl From<&panic::PanicHookInfo<'_>> for PanicInfo {
    fn from(info: &panic::PanicHookInfo) -> Self {
        let payload = info.payload();

        let msg = downcast_display::<&str>(payload)
            .or_else(|| downcast_display::<String>(payload))
            .or_else(|| downcast_display::<&String>(payload))
            .map(ToString::to_string)
            .unwrap_or_default();

        let current_thread = thread::current();

        let thread = match current_thread.name() {
            Some(name) if !name.is_empty() => name,
            _ => "<unnamed>",
        };

        Self {
            msg,
            thread: thread.to_owned(),
            file: info.location().map(|l| l.file().to_owned()),
            line: info.location().map(Location::line),
            column: info.location().map(Location::column),
        }
    }
}

#[derive(Clone)]
pub(super) enum Failure {
    Error(String),
    Panic(PanicInfo),
}

impl Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Failure::Error(err) => write!(f, "error:{err}"),
            Failure::Panic(info) => write!(f, "{info:?}"),
        }
    }
}

impl FromStr for Failure {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once("error:") {
            Some((_, msg)) => Ok(Failure::Error(msg.trim().to_owned())),
            None => PanicInfo::from_str(s).map(Self::Panic),
        }
    }
}

#[cfg(feature = "test-terminator")]
impl<E: Display> From<super::terminator::TestFailure<'_, E>> for Failure {
    fn from(err: super::terminator::TestFailure<'_, E>) -> Self {
        match err {
            super::terminator::TestFailure::Error(err) => {
                Self::Error(err.to_string())
            },
            super::terminator::TestFailure::Panic(info) => {
                Self::Panic(info.into())
            },
        }
    }
}

fn downcast_display<T: Any + Display>(
    value: &dyn Any,
) -> Option<&dyn Display> {
    value.downcast_ref::<T>().map(|msg| msg as &dyn Display)
}
