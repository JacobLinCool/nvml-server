use std::ops::Sub;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::error::AppError;
use axum::Json;
use nvml_wrapper::{
    enums::device::UsedGpuMemory,
    struct_wrappers::device::{MemoryInfo, Utilization},
    Nvml,
};
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
use sysinfo::{Pid, System, Users};

static NVML: OnceCell<Nvml> = OnceCell::new();
static SYSTEM: Lazy<Mutex<System>> = Lazy::new(|| Mutex::new(System::new_all()));
static LAST_UPDATE: Lazy<Mutex<Instant>> =
    Lazy::new(|| Mutex::new(Instant::now().sub(Duration::from_secs(1))));
static STATS_CACHE: Lazy<Mutex<Option<Stats>>> = Lazy::new(|| Mutex::new(None));

pub async fn stats() -> Result<Json<Stats>, AppError> {
    let now = Instant::now();
    let last_update = LAST_UPDATE.lock().unwrap();
    if now.duration_since(*last_update) < Duration::from_secs(1) {
        if let Some(stats) = STATS_CACHE.lock().unwrap().as_ref() {
            return Ok(Json(stats.clone()));
        }
    }

    let nvml = NVML.get_or_try_init(Nvml::init)?;
    let mut system = SYSTEM.lock().unwrap();
    system.refresh_all();
    let users = Users::new_with_refreshed_list();

    let device_count = nvml.device_count()?;

    let mut gpus = Vec::with_capacity(device_count as usize);
    let mut processes = Vec::<Process>::new();

    for i in 0..device_count {
        let device = nvml.device_by_index(i)?;

        let uuid = device.uuid()?;
        let name = device.name()?;
        let power = device.power_usage()?;
        let memory = device.memory_info()?;
        let utilization = device.utilization_rates()?;

        gpus.push(Gpu {
            uuid: uuid.clone(),
            name,
            power,
            memory,
            utilization,
        });

        let running_processes = device.running_compute_processes()?;
        for process in running_processes {
            if let Some(p) = processes.iter_mut().find(|p| p.pid == process.pid) {
                p.gpus.push(ProcessGpu {
                    uuid: uuid.clone(),
                    memory: match process.used_gpu_memory {
                        UsedGpuMemory::Unavailable => 0,
                        UsedGpuMemory::Used(x) => x,
                    },
                });
            } else {
                let mut name = String::new();
                let mut user = String::new();
                let mut run_time = 0;
                let mut cpu_usage = 0.0;

                if let Some(p) = system.process(Pid::from_u32(process.pid)) {
                    name = p.name().to_string();

                    if let Some(uid) = p.user_id() {
                        if let Some(u) = users.get_user_by_id(uid) {
                            user = u.name().to_string();
                        }
                    }

                    run_time = p.run_time();
                    cpu_usage = p.cpu_usage();
                }

                processes.push(Process {
                    pid: process.pid,
                    name,
                    user,
                    run_time,
                    cpu_usage,
                    gpus: vec![ProcessGpu {
                        uuid: uuid.clone(),
                        memory: match process.used_gpu_memory {
                            UsedGpuMemory::Unavailable => 0,
                            UsedGpuMemory::Used(x) => x,
                        },
                    }],
                });
            }
        }
    }

    let stats = Stats { gpus, processes };
    *LAST_UPDATE.lock().unwrap() = now;
    *STATS_CACHE.lock().unwrap() = Some(stats.clone());
    Ok(Json(stats))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Gpu {
    pub uuid: String,
    pub name: String,
    pub power: u32,
    pub memory: MemoryInfo,
    pub utilization: Utilization,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Process {
    pub pid: u32,
    pub name: String,
    pub user: String,
    pub run_time: u64,
    pub cpu_usage: f32,
    pub gpus: Vec<ProcessGpu>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProcessGpu {
    pub uuid: String,
    pub memory: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Stats {
    pub gpus: Vec<Gpu>,
    pub processes: Vec<Process>,
}
