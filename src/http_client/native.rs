#[cfg(all(feature = "curl-client", not(target_arch = "wasm32")))]
pub(crate) use super::chttp::ChttpClient as NativeClient;

#[cfg(all(feature = "wasm-client", target_arch = "wasm32"))]
pub(crate) use super::wasm::WasmClient as NativeClient;
