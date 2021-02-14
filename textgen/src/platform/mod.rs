#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod platform_impl;

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod platform_impl;

pub fn generate_windows() -> Vec<String> {
    platform_impl::generate_windows()
}
