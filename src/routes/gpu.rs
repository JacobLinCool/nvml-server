use crate::error::AppError;
use axum::Json;
use nvml_wrapper::{
    struct_wrappers::device::{MemoryInfo, ProcessInfo, Utilization},
    Nvml,
};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

static NVML: OnceCell<Nvml> = OnceCell::new();

pub async fn gpu() -> Result<Json<Res>, AppError> {
    let nvml = NVML.get_or_try_init(Nvml::init)?;

    let device_count = nvml.device_count()?;

    let mut gpus = Vec::with_capacity(device_count as usize);
    let mut processes = Vec::new();

    for i in 0..device_count {
        let device = nvml.device_by_index(i)?;

        let uuid = device.uuid()?;
        let name = device.name()?;
        let power = device.power_usage()?;
        let memory = device.memory_info()?;
        let utilization = device.utilization_rates()?;

        gpus.push(Gpu {
            uuid,
            name,
            power,
            memory,
            utilization,
        });

        let running_processes = device.running_compute_processes()?;
        for process in running_processes {
            processes.push(process);
        }
    }

    Ok(Json(Res { gpus, processes }))
}

#[derive(Serialize, Deserialize)]
pub struct Gpu {
    pub uuid: String,
    pub name: String,
    pub power: u32,
    pub memory: MemoryInfo,
    pub utilization: Utilization,
}

#[derive(Serialize, Deserialize)]
pub struct Res {
    pub gpus: Vec<Gpu>,
    pub processes: Vec<ProcessInfo>,
}
