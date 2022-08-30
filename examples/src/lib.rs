pub const NVIM: &str = env!("NVIM");
pub const NVIM_NIGHTLY: &str = env!("NVIM_NIGHTLY");

// Empty vars
const EMPTY: &[(&str, &str)] = &[];
// nightly nvim-oxi
const NIGHTLY: &[(&str, &str)] = &[("features", "-F nvim-oxi/nightly")];

pub fn nvim_nightly() -> (bool, &'static [(&'static str, &'static str)]) {
    let is_nightly =
        NVIM_NIGHTLY.parse().expect("There must be a NVIM locally.");
    let nightly = if is_nightly { NIGHTLY } else { EMPTY };
    (is_nightly, nightly)
}

#[cfg(test)]
mod test;
