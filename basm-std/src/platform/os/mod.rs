#[cfg(not(any(target_arch = "wasm32", target_arch = "aarch64")))]
pub mod windows;
#[cfg(not(any(target_arch = "wasm32", target_arch = "aarch64")))]
pub mod linux;
#[cfg(target_arch = "aarch64")]
pub mod macos;
#[cfg(target_arch = "wasm32")]
pub mod wasm32;
pub mod unknown;