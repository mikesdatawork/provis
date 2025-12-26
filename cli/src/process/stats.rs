use sysinfo::{System, ProcessesToUpdate};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ProcessStats {
    pub pid: i32,
    pub name: String,
    pub cpu_usage: f32,
    pub mem_usage: u64,
    pub mem_percent: f32,
    pub io_read_bytes: u64,
    pub io_write_bytes: u64,
}

pub struct ProcessCollector {
    system: System,
}

impl ProcessCollector {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_processes(ProcessesToUpdate::All);
    }

    pub fn get_all_processes(&self) -> Vec<ProcessStats> {
        let total_memory = self.system.total_memory();
        
        // Group by process name to aggregate threads
        let mut process_map: HashMap<String, ProcessStats> = HashMap::new();

        for (pid, process) in self.system.processes() {
            let pid_num = pid.as_u32() as i32;
            let name = process.name().to_string_lossy().to_string();
            let cpu = process.cpu_usage();
            let mem = process.memory();
            let mem_pct = ((mem as f64 / total_memory as f64) * 100.0) as f32;
            
            // Get disk usage (read + write bytes)
            let disk_usage = process.disk_usage();
            let read_bytes = disk_usage.total_read_bytes;
            let write_bytes = disk_usage.total_written_bytes;

            // Aggregate by process name (combines all threads)
            process_map
                .entry(name.clone())
                .and_modify(|stats| {
                    stats.cpu_usage += cpu;
                    stats.mem_usage += mem;
                    stats.mem_percent += mem_pct;
                    stats.io_read_bytes += read_bytes;
                    stats.io_write_bytes += write_bytes;
                })
                .or_insert(ProcessStats {
                    pid: pid_num,
                    name,
                    cpu_usage: cpu,
                    mem_usage: mem,
                    mem_percent: mem_pct,
                    io_read_bytes: read_bytes,
                    io_write_bytes: write_bytes,
                });
        }

        process_map.into_values().collect()
    }

    pub fn get_top_by_cpu(&self, limit: usize) -> Vec<ProcessStats> {
        let mut processes = self.get_all_processes();
        
        // Filter out provis itself
        processes.retain(|p| p.name != "provis");
        
        processes.sort_by(|a, b| {
            b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap()
        });
        processes.into_iter().take(limit).collect()
    }

    pub fn get_top_by_memory(&self, limit: usize) -> Vec<ProcessStats> {
        let mut processes = self.get_all_processes();
        
        // Filter out provis itself
        processes.retain(|p| p.name != "provis");
        
        processes.sort_by(|a, b| {
            b.mem_usage.cmp(&a.mem_usage)
        });
        processes.into_iter().take(limit).collect()
    }
}
