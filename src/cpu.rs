use sysinfo::System;

pub struct CpuMonitor {
    system: System,
    usages: Vec<f32>,
    avg: f32,
    memory_used: u64,
    memory_total: u64,
}

impl CpuMonitor {
    pub fn new() -> Self {
        let system = System::new_all();
        let mut monitor = Self {
            system,
            usages: Vec::new(),
            avg: 0.0,
            memory_used: 0,
            memory_total: 0,
        };
        monitor.refresh();
        monitor
    }

    pub fn refresh(&mut self) {
        self.system.refresh_cpu();
        self.system.refresh_memory();
        self.usages = self
            .system
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .collect();
        self.avg = if self.usages.is_empty() {
            0.0
        } else {
            self.usages.iter().copied().sum::<f32>() / self.usages.len() as f32
        };

        self.memory_total = self.system.total_memory();
        self.memory_used = self.system.used_memory();
    }

    pub fn avg(&self) -> f32 {
        self.avg
    }

    pub fn usages(&self) -> &[f32] {
        &self.usages
    }

    pub fn thread_count(&self) -> usize {
        self.usages.len()
    }

    pub fn memory_usage(&self) -> (u64, u64) {
        (self.memory_used, self.memory_total)
    }

    pub fn memory_ratio(&self) -> f64 {
        if self.memory_total == 0 {
            0.0
        } else {
            self.memory_used as f64 / self.memory_total as f64
        }
    }
}
