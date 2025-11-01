use serde::{Deserialize, Serialize};
use sysinfo::{Disks, Networks, System};

/// Bytes per kilobyte/megabyte/etc unit
const BYTES_PER_UNIT: f64 = 1024.0;

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
    pub network_details: NetworkDetails,
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
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDetails {
    pub local_ip: Option<String>,
    pub public_ip: Option<String>,
    pub bandwidth_mbps: Option<f64>,
}

impl SystemInfo {
    /// Collect current system information
    pub fn collect() -> Self {
        let mut sys = System::new_all();

        // Refresh system information
        sys.refresh_all();

        let disks = Disks::new_with_refreshed_list();
        let networks = Networks::new_with_refreshed_list();

        let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
        let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
        let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        let hostname = System::host_name().unwrap_or_else(|| "Unknown".to_string());

        let cpu_count = sys.cpus().len();
        let cpu_brand = sys
            .cpus()
            .first()
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
                ip_address: None, // Interface-specific IPs not provided by sysinfo crate
            })
            .collect();

        let processes_count = sys.processes().len();
        let uptime = System::uptime();

        let network_details = NetworkDetails {
            local_ip: Self::get_local_ip(),
            public_ip: Self::get_public_ip(),
            bandwidth_mbps: Self::benchmark_bandwidth(),
        };

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
            network_details,
            processes_count,
            uptime,
        }
    }

    /// Format memory size in human-readable format
    pub fn format_bytes(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= BYTES_PER_UNIT && unit_index < UNITS.len() - 1 {
            size /= BYTES_PER_UNIT;
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

    /// Get local IP address
    pub fn get_local_ip() -> Option<String> {
        local_ip_address::local_ip().ok().map(|ip| ip.to_string())
    }

    /// Get public IP address
    pub fn get_public_ip() -> Option<String> {
        // Try multiple services for reliability
        let services = [
            "https://api.ipify.org",
            "https://ifconfig.me/ip",
            "https://icanhazip.com",
        ];

        for service in &services {
            if let Ok(response) = reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(5))
                .build()
                .and_then(|client| client.get(*service).send())
            {
                if let Ok(ip) = response.text() {
                    let ip = ip.trim().to_string();
                    if !ip.is_empty() {
                        return Some(ip);
                    }
                }
            }
        }
        None
    }

    /// Benchmark network bandwidth
    pub fn benchmark_bandwidth() -> Option<f64> {
        // Download a test file from a fast CDN to measure bandwidth
        let test_urls = [
            "https://speed.cloudflare.com/__down?bytes=1000000", // ~976 KB test
        ];

        for url in &test_urls {
            let start = std::time::Instant::now();

            match reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .and_then(|client| client.get(*url).send())
            {
                Ok(response) => {
                    if let Ok(bytes) = response.bytes() {
                        let duration = start.elapsed();
                        let duration_secs = duration.as_secs_f64();

                        if duration_secs > 0.0 {
                            let bytes_count = bytes.len() as f64;
                            // Calculate Mbps (megabits per second)
                            let mbps = (bytes_count * 8.0) / (duration_secs * 1_000_000.0);
                            return Some(mbps);
                        }
                    }
                }
                Err(_) => continue,
            }
        }
        None
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

    #[test]
    fn test_get_local_ip() {
        // Test that the function doesn't panic
        let result = SystemInfo::get_local_ip();
        // Result can be Some or None depending on environment
        let _ = result;
    }

    #[test]
    fn test_network_details_struct() {
        // Test that we can create a NetworkDetails struct
        let details = NetworkDetails {
            local_ip: Some("192.168.1.1".to_string()),
            public_ip: Some("1.2.3.4".to_string()),
            bandwidth_mbps: Some(100.0),
        };

        assert_eq!(details.local_ip, Some("192.168.1.1".to_string()));
        assert_eq!(details.public_ip, Some("1.2.3.4".to_string()));
        assert_eq!(details.bandwidth_mbps, Some(100.0));
    }
}
