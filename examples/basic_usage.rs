use system_index::models::SystemInfo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("System Information Example");
    println!("==========================\n");

    // Collect system information
    let info = SystemInfo::collect();

    // Display basic information
    println!("Hostname: {}", info.hostname);
    println!("OS: {} {}", info.os_name, info.os_version);
    println!("Kernel: {}", info.kernel_version);
    println!("Uptime: {}", SystemInfo::format_uptime(info.uptime));
    println!();

    // Display CPU information
    println!("CPU Information:");
    println!("  Brand: {}", info.cpu_brand);
    println!("  Cores: {}", info.cpu_count);
    println!();

    // Display memory information
    println!("Memory Information:");
    println!(
        "  Total RAM: {}",
        SystemInfo::format_bytes(info.total_memory)
    );
    println!("  Used RAM: {}", SystemInfo::format_bytes(info.used_memory));
    println!(
        "  Free RAM: {}",
        SystemInfo::format_bytes(info.total_memory - info.used_memory)
    );
    println!();

    // Display disk information
    println!("Disk Information:");
    for (idx, disk) in info.disks.iter().enumerate() {
        println!("  Disk {}:", idx + 1);
        println!("    Name: {}", disk.name);
        println!("    Mount: {}", disk.mount_point);
        println!("    Total: {}", SystemInfo::format_bytes(disk.total_space));
        println!(
            "    Available: {}",
            SystemInfo::format_bytes(disk.available_space)
        );
    }
    println!();

    // Display network information
    println!("Network Information:");
    for (idx, network) in info.networks.iter().enumerate() {
        println!("  Interface {}:", idx + 1);
        println!("    Name: {}", network.interface_name);
        println!(
            "    Received: {}",
            SystemInfo::format_bytes(network.received_bytes)
        );
        println!(
            "    Transmitted: {}",
            SystemInfo::format_bytes(network.transmitted_bytes)
        );
    }

    Ok(())
}
