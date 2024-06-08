pub mod gpu;

pub async fn root() -> String {
    let version = std::env!("CARGO_PKG_VERSION");
    format!("NVML Server v{}", version)
}
