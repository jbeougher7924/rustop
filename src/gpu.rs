use nvml_wrapper::{enum_wrappers::device::TemperatureSensor, error::NvmlError, Nvml};

#[derive(Debug, Clone)]
pub struct GpuStats {
    pub index: u32,
    pub name: String,
    pub utilization: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub temperature: Option<u32>,
}

pub struct GpuMonitor {
    nvml: Option<Nvml>,
    stats: Vec<GpuStats>,
}

impl GpuMonitor {
    pub fn new() -> Self {
        let nvml = Nvml::init().ok();
        let mut monitor = Self {
            nvml,
            stats: Vec::new(),
        };
        monitor.refresh();
        monitor
    }

    pub fn refresh(&mut self) {
        if self.nvml.is_none() {
            match Nvml::init() {
                Ok(nvml) => self.nvml = Some(nvml),
                Err(_) => {
                    self.stats.clear();
                    return;
                }
            }
        }

        let Some(nvml) = self.nvml.as_ref() else {
            self.stats.clear();
            return;
        };

        let device_count = match nvml.device_count() {
            Ok(count) => count,
            Err(_) => {
                self.stats.clear();
                return;
            }
        };

        let mut stats = Vec::with_capacity(device_count as usize);
        for i in 0..device_count {
            match nvml.device_by_index(i) {
                Ok(device) => {
                    let name = device.name().unwrap_or_else(|_| format!("GPU {}", i));
                    let utilization = device
                        .utilization_rates()
                        .map(|u| u.gpu as f32)
                        .unwrap_or(0.0);
                    let (memory_used, memory_total) = device
                        .memory_info()
                        .map(|m| (m.used, m.total))
                        .unwrap_or((0, 0));
                    let temperature = device.temperature(TemperatureSensor::Gpu).ok();

                    stats.push(GpuStats {
                        index: i,
                        name,
                        utilization,
                        memory_used,
                        memory_total,
                        temperature,
                    });
                }
                Err(NvmlError::NotSupported) => {
                    // Skip devices that do not support the requested metrics
                    continue;
                }
                Err(_) => continue,
            }
        }

        self.stats = stats;
    }

    pub fn stats(&self) -> &[GpuStats] {
        &self.stats
    }

    pub fn nvml_available(&self) -> bool {
        self.nvml.is_some()
    }
}
