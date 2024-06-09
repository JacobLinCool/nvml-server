pub mod stats;

pub async fn root() -> String {
    let version = std::env!("CARGO_PKG_VERSION");
    format!("NVML Server v{}\n- /stats for GPU stats", version)
}
