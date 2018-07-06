use std::ffi::OsStr;

pub fn to_wstring(s: &str) -> Vec<u16> {
    OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect()
}