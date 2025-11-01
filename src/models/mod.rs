use serde::{Deserialize, Serialize};
use sysinfo::{Components, Disks, Networks, System};

/// System information model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub hostname: String,
    pub cpu_count: usize,
    pub cpu_brand: String,
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub disks: Vec<DiskInfo>,
    pub networks: Vec<NetworkInfo>,
    pub processes_count: usize,
    pub uptime: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub file_system: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub interface_name: String,
    pub received_bytes: u64,
    pub transmitted_bytes: u64,
}

impl SystemInfo {
    /// Collect current system information
    pub fn collect() -> Self {
        let mut sys = System::new_all();
        
        // Refresh system information
        sys.refresh_all();
        
        let _components = Components::new_with_refreshed_list();
        let disks = Disks::new_with_refreshed_list();
        let networks = Networks::new_with_refreshed_list();

        let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
        let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
        let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());

        let cpu_count = sys.cpus().len();
        let cpu_brand = sys.cpus().first()
            .map(|cpu| cpu.brand().to_string())
            .unwrap_or_else(|| "Unknown".to_string());

        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let total_swap = sys.total_swap();
        let used_swap = sys.used_swap();

        let disk_info: Vec<DiskInfo> = disks
            .iter()
            .map(|disk| DiskInfo {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total_space: disk.total_space(),
                available_space: disk.available_space(),
                file_system: disk.file_system().to_string_lossy().to_string(),
            })
            .collect();

        let network_info: Vec<NetworkInfo> = networks
            .iter()
            .map(|(interface_name, data)| NetworkInfo {
                interface_name: interface_name.clone(),
                received_bytes: data.received(),
                transmitted_bytes: data.transmitted(),
            })
            .collect();

        let processes_count = sys.processes().len();
        let uptime = System::uptime();

        Self {
            os_name,
            os_version,
            kernel_version,
            hostname,
            cpu_count,
            cpu_brand,
            total_memory,
            used_memory,
            total_swap,
            used_swap,
            disks: disk_info,
            networks: network_info,
            processes_count,
            uptime,
        }
    }

    /// Format memory size in human-readable format
    pub fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        format!("{:.2} {}", size, UNITS[unit_index])
    }

    /// Format uptime in human-readable format
    pub fn format_uptime(seconds: u64) -> String {
        let days = seconds / 86400;
        let hours = (seconds % 86400) / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;

        if days > 0 {
            format!("{}d {}h {}m {}s", days, hours, minutes, secs)
        } else if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, secs)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, secs)
        } else {
            format!("{}s", secs)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_system_info() {
        let info = SystemInfo::collect();
        
        // Basic assertions to ensure data is collected
        assert!(!info.os_name.is_empty());
        assert!(!info.hostname.is_empty());
        assert!(info.cpu_count > 0);
        assert!(info.total_memory > 0);
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(SystemInfo::format_bytes(512), "512.00 B");
        assert_eq!(SystemInfo::format_bytes(1024), "1.00 KB");
        assert_eq!(SystemInfo::format_bytes(1048576), "1.00 MB");
        assert_eq!(SystemInfo::format_bytes(1073741824), "1.00 GB");
    }

    #[test]
    fn test_format_uptime() {
        assert_eq!(SystemInfo::format_uptime(30), "30s");
        assert_eq!(SystemInfo::format_uptime(90), "1m 30s");
        assert_eq!(SystemInfo::format_uptime(3661), "1h 1m 1s");
        assert_eq!(SystemInfo::format_uptime(90061), "1d 1h 1m 1s");
    }
}
