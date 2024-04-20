use std::any::Any;
use std::env;
use std::fmt::{Debug, Display};
use std::panic::{self, Location, UnwindSafe};
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::str::FromStr;
use std::sync::{Arc, OnceLock};

use miniserde::json;

/// Returns the `target` directory in which cargo will place the compiled
/// artifacts for the crate whose manifest is located at `manifest_dir`.
pub fn target_dir(manifest_dir: &Path) -> PathBuf {
    let output = Command::new(
        env::var("CARGO").ok().unwrap_or_else(|| "cargo".to_owned()),
    )
    .arg("metadata")
    .arg("--format-version=1")
    .arg("--no-deps")
    .current_dir(manifest_dir)
    .output()
    .unwrap();

    let object: json::Object =
        json::from_str(&String::from_utf8(output.stdout).unwrap()).unwrap();

    let target_dir = match object.get("target_directory").unwrap() {
        json::Value::String(s) => s,
        _ => panic!("must be string value"),
    };

    target_dir.into()
}

/// TODO: docs
pub fn plugin_body<F, R>(test_body: F)
where
    F: FnOnce() -> R + UnwindSafe,
    R: IntoResult,
{
    let panic_info = Arc::new(OnceLock::new());

    {
        let panic_info = panic_info.clone();

        panic::set_hook(Box::new(move |info| {
            let payload = info.payload();

            let msg = downcast_display::<&str>(payload)
                .or_else(|| downcast_display::<String>(payload))
                .or_else(|| downcast_display::<&String>(payload))
                .map(ToString::to_string)
                .unwrap_or_default();

            let info = PanicInfo {
                msg,
                file: info.location().map(|l| l.file().to_owned()),
                line: info.location().map(Location::line),
                column: info.location().map(Location::column),
            };

            let _ = panic_info.set(info);
        }));
    }

    let result = match panic::catch_unwind(|| test_body().into_result()) {
        Ok(Ok(())) => Ok(()),
        Ok(Err(err)) => Err(Failure::Error(err.to_string())),
        Err(_) => Err(Failure::Panic(panic_info.get().unwrap().clone())),
    };

    if let Err(failure) = &result {
        eprintln!("{failure}");
    }

    exit(result.is_err().into());
}

/// TODO: docs
pub fn test_body(
    plugin_name: &str,
    extra_cmd: Option<&str>,
) -> Result<(), String> {
    let panic_info = Arc::new(OnceLock::new());

    {
        let panic_info = panic_info.clone();
        panic::set_hook(Box::new(move |_| {
            println!("{}", panic_info.get().unwrap());
        }));
    }

    let output = run_nvim_command(plugin_name, extra_cmd)
        .output()
        .map_err(|err| err.to_string())?;

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
        Failure::Error(err) => return Err(err),
        Failure::Panic(info) => {
            panic_info.set(info).unwrap();
            panic!()
        },
    }
}

/// TODO: docs
fn run_nvim_command(plugin_name: &str, extra_cmd: Option<&str>) -> Command {
    let library_name = {
        let mut s = ::std::string::String::new();
        s.push_str(::std::env::consts::DLL_PREFIX);
        s.push_str(env!("CARGO_CRATE_NAME"));
        s.push_str(::std::env::consts::DLL_SUFFIX);
        s
    };

    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    // The full path to the compiled library.
    let library_path =
        target_dir(manifest_dir.as_ref()).join("debug").join(library_name);

    if !library_path.exists() {
        panic!(
            "Compiled library not found in '{}'. Please run `cargo build` \
             before running the tests.",
            library_path.display()
        )
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
        .args(["-c", &load_library])
        .args(["+quit"]);

    command
}

#[derive(Clone)]
struct PanicInfo {
    msg: String,
    file: Option<String>,
    line: Option<u32>,
    column: Option<u32>,
}

impl Debug for PanicInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "panic:{}", self.msg)?;

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
        // thread 'tests::it_works' panicked at src/lib.rs:15:9:
        // AA

        write!(f, "thread panicked")?;

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
            msg: s.to_owned(),
            file: None,
            line: None,
            column: None,
        };

        let mut lines = s.lines();

        let first = lines.next().ok_or(())?;
        let (_, msg) = first.split_once("panic:").ok_or(())?;
        info.msg = msg.trim().to_owned();

        let second = lines.next().ok_or(())?;
        let (_, file) = second.split_once("file:").ok_or(())?;
        info.file = Some(file.trim().to_owned());

        let third = lines.next().ok_or(())?;
        let (_, line) = third.split_once("line:").ok_or(())?;
        info.line = Some(line.trim().parse().map_err(|_| ())?);

        let fourth = lines.next().ok_or(())?;
        let (_, column) = fourth.split_once("column:").ok_or(())?;
        info.column = Some(column.trim().parse().map_err(|_| ())?);

        Ok(info)
    }
}

enum Failure {
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

pub trait IntoResult {
    type Error: Display;

    fn into_result(self) -> Result<(), Self::Error>;
}

impl IntoResult for () {
    type Error = std::convert::Infallible;

    fn into_result(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<E: Display> IntoResult for Result<(), E> {
    type Error = E;

    fn into_result(self) -> Result<(), E> {
        self
    }
}

fn downcast_display<T: Any + Display>(
    value: &dyn Any,
) -> Option<&dyn Display> {
    value.downcast_ref::<T>().map(|msg| msg as &dyn Display)
}