//! Process statistics collection module
//! 
//! This module handles reading process information from /proc
//! and system statistics.

use sysinfo::{System, Process, Pid};

/// Represents statistics for a single process
#[derive(Debug, Clone)]
pub struct ProcessStats {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub mem_usage: u64,      // in bytes
    pub mem_percent: f32,
    pub disk_read: u64,      // bytes read
    pub disk_write: u64,     // bytes written
}

impl ProcessStats {
    /// Create ProcessStats from sysinfo::Process
    pub fn from_sysinfo(pid: &Pid, process: &Process, total_memory: u64) -> Self {
        let mem_bytes = process.memory();
        let mem_percent = if total_memory > 0 {
            (mem_bytes as f32 / total_memory as f32) * 100.0
        } else {
            0.0
        };

        Self {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().to_string(),
            cpu_usage: process.cpu_usage(),
            mem_usage: mem_bytes,
            mem_percent,
            disk_read: process.disk_usage().read_bytes,
            disk_write: process.disk_usage().written_bytes,
        }
    }
}

/// Collector for process statistics
pub struct ProcessCollector {
    system: System,
}

impl ProcessCollector {
    /// Create a new ProcessCollector
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self { system }
    }

    /// Refresh all process data
    pub fn refresh(&mut self) {
        self.system.refresh_all();
    }

    /// Get all current process statistics
    pub fn get_all_processes(&self) -> Vec<ProcessStats> {
        let total_memory = self.system.total_memory();
        
        self.system
            .processes()
            .iter()
            .map(|(pid, process)| {
                ProcessStats::from_sysinfo(pid, process, total_memory)
            })
            .collect()
    }

    /// Get processes sorted by CPU usage
    pub fn get_top_by_cpu(&self, limit: usize) -> Vec<ProcessStats> {
        let mut processes = self.get_all_processes();
        processes.sort_by(|a, b| {
            b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap()
        });
        processes.truncate(limit);
        processes
    }

    /// Get processes sorted by memory usage
    pub fn get_top_by_memory(&self, limit: usize) -> Vec<ProcessStats> {
        let mut processes = self.get_all_processes();
        processes.sort_by(|a, b| b.mem_usage.cmp(&a.mem_usage));
        processes.truncate(limit);
        processes
    }
}

impl Default for ProcessCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collector_creation() {
        let collector = ProcessCollector::new();
        let processes = collector.get_all_processes();
        assert!(!processes.is_empty(), "Should find at least one process");
    }
}
