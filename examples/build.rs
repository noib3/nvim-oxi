fn main() -> Result<(), Box<dyn std::error::Error>> {
    let info = get_nvim_info("nvim").or_else(|_| get_nvim_info("vim"))?;
    let (v, is_nightly) = nvim_version(&info)?;
    println!(
        "cargo:rustc-env=NVIM={v}\ncargo:rustc-env=NVIM_NIGHTLY={is_nightly}"
    );
    Ok(())
}

const ERR: &str = "NVIM is NOT found.";

// `nvim --version` or `vim --version`
fn get_nvim_info(program: &str) -> std::io::Result<String> {
    std::process::Command::new(program)
        .arg("--version")
        .output()
        .map(|o| unsafe { String::from_utf8_unchecked(o.stdout) })
}

// get version number and is_nightly
fn nvim_version(info: &str) -> Result<(&str, bool), &'static str> {
    let version = info.split_once('\n').unwrap_or_default().0;
    if !version.starts_with("NVIM") {
        return Err(ERR);
    }
    let pos1 = version.find('v').unwrap_or_default();
    let pos2 = version.find("-dev").unwrap_or(version.len());
    let v = &version[pos1..pos2];
    Ok((v, v.starts_with("v0.8")))
}
