use system_index::models::SystemInfo;

#[test]
fn test_system_info_collection() {
    let info = SystemInfo::collect();

    // Basic assertions to ensure data is collected
    assert!(!info.os_name.is_empty(), "OS name should not be empty");
    assert!(!info.hostname.is_empty(), "Hostname should not be empty");
    assert!(info.cpu_count > 0, "CPU count should be greater than 0");
    assert!(info.total_memory > 0, "Total memory should be greater than 0");
}

#[test]
fn test_format_bytes() {
    assert_eq!(SystemInfo::format_bytes(0), "0.00 B");
    assert_eq!(SystemInfo::format_bytes(512), "512.00 B");
    assert_eq!(SystemInfo::format_bytes(1024), "1.00 KB");
    assert_eq!(SystemInfo::format_bytes(1048576), "1.00 MB");
    assert_eq!(SystemInfo::format_bytes(1073741824), "1.00 GB");
    assert_eq!(SystemInfo::format_bytes(1099511627776), "1.00 TB");
}

#[test]
fn test_format_uptime() {
    assert_eq!(SystemInfo::format_uptime(0), "0s");
    assert_eq!(SystemInfo::format_uptime(30), "30s");
    assert_eq!(SystemInfo::format_uptime(60), "1m 0s");
    assert_eq!(SystemInfo::format_uptime(90), "1m 30s");
    assert_eq!(SystemInfo::format_uptime(3600), "1h 0m 0s");
    assert_eq!(SystemInfo::format_uptime(3661), "1h 1m 1s");
    assert_eq!(SystemInfo::format_uptime(86400), "1d 0h 0m 0s");
    assert_eq!(SystemInfo::format_uptime(90061), "1d 1h 1m 1s");
}

#[test]
fn test_system_info_fields() {
    let info = SystemInfo::collect();

    // Verify all fields are present and have reasonable values
    assert!(!info.os_version.is_empty(), "OS version should not be empty");
    assert!(!info.kernel_version.is_empty(), "Kernel version should not be empty");
    assert!(!info.cpu_brand.is_empty(), "CPU brand should not be empty");
    
    // Memory values should be reasonable
    assert!(info.used_memory <= info.total_memory, "Used memory should not exceed total memory");
    assert!(info.used_swap <= info.total_swap, "Used swap should not exceed total swap");
    
    // Process count should be reasonable
    assert!(info.processes_count > 0, "Should have at least one process running");
}
