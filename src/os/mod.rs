#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
pub(crate) use self::macos::*;
#[cfg(target_os = "windows")]
pub(crate) use self::windows::*;