// build-pass
// ignore-sgx

// pretty-expanded FIXME #23616

#[cfg(target_family = "windows")]
pub fn main() {
}

#[cfg(target_family = "unix")]
pub fn main() {
}

#[cfg(target_family="wasm")]
pub fn main() {
}
