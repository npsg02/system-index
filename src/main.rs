use clap::{Parser, Subcommand};
use system_index::{models::SystemInfo, tui::App};

/// A CLI and TUI tool for displaying system information
#[derive(Parser)]
#[command(name = "system-index")]
#[command(about = "Display comprehensive system information")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the interactive TUI
    Tui,
    /// Display system overview
    Overview,
    /// Display CPU information
    Cpu,
    /// Display memory information
    Memory,
    /// Display disk information
    Disks,
    /// Display network information
    Network,
    /// Display all system information
    All,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Tui) | None => {
            // Default to TUI mode
            let mut app = App::new();
            app.run()?;
        }
        Some(Commands::Overview) => {
            print_overview();
        }
        Some(Commands::Cpu) => {
            print_cpu_info();
        }
        Some(Commands::Memory) => {
            print_memory_info();
        }
        Some(Commands::Disks) => {
            print_disk_info();
        }
        Some(Commands::Network) => {
            print_network_info();
        }
        Some(Commands::All) => {
            print_all_info();
        }
    }

    Ok(())
}

fn print_overview() {
    let info = SystemInfo::collect();

    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║              SYSTEM OVERVIEW                          ║");
    println!("╚═══════════════════════════════════════════════════════╝");
    println!();
    println!("🖥️  Hostname:        {}", info.hostname);
    println!("💻 Operating System: {} {}", info.os_name, info.os_version);
    println!("🔧 Kernel Version:   {}", info.kernel_version);
    println!(
        "⏰ System Uptime:    {}",
        SystemInfo::format_uptime(info.uptime)
    );
    println!();
    println!("⚙️  CPU:             {}", info.cpu_brand);
    println!("📊 CPU Cores:        {}", info.cpu_count);
    println!();
    println!(
        "💾 Total Memory:     {}",
        SystemInfo::format_bytes(info.total_memory)
    );
    println!(
        "📈 Used Memory:      {}",
        SystemInfo::format_bytes(info.used_memory)
    );
    println!();
    println!("💿 Mounted Disks:    {}", info.disks.len());
    println!("🌐 Network Interfaces: {}", info.networks.len());
    if let Some(local_ip) = &info.network_details.local_ip {
        println!("🏠 Local IP:         {}", local_ip);
    }
    if let Some(public_ip) = &info.network_details.public_ip {
        println!("🌍 Public IP:        {}", public_ip);
    }
    println!("📋 Running Processes: {}", info.processes_count);
}

fn print_cpu_info() {
    let info = SystemInfo::collect();

    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║              CPU INFORMATION                          ║");
    println!("╚═══════════════════════════════════════════════════════╝");
    println!();
    println!("⚙️  CPU Brand:       {}", info.cpu_brand);
    println!("📊 Number of Cores:  {}", info.cpu_count);
}

fn print_memory_info() {
    let info = SystemInfo::collect();

    let total_mem = info.total_memory;
    let used_mem = info.used_memory;
    let free_mem = total_mem - used_mem;
    let mem_usage_percent = if total_mem > 0 {
        used_mem as f64 / total_mem as f64 * 100.0
    } else {
        0.0
    };

    let total_swap = info.total_swap;
    let used_swap = info.used_swap;
    let free_swap = total_swap.saturating_sub(used_swap);
    let swap_usage_percent = if total_swap > 0 {
        used_swap as f64 / total_swap as f64 * 100.0
    } else {
        0.0
    };

    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║              MEMORY INFORMATION                       ║");
    println!("╚═══════════════════════════════════════════════════════╝");
    println!();
    println!("═══ RAM MEMORY ═══");
    println!("Total Memory:    {}", SystemInfo::format_bytes(total_mem));
    println!(
        "Used Memory:     {} ({:.2}%)",
        SystemInfo::format_bytes(used_mem),
        mem_usage_percent
    );
    println!("Free Memory:     {}", SystemInfo::format_bytes(free_mem));
    println!();
    println!("═══ SWAP MEMORY ═══");
    println!("Total Swap:      {}", SystemInfo::format_bytes(total_swap));
    println!(
        "Used Swap:       {} ({:.2}%)",
        SystemInfo::format_bytes(used_swap),
        swap_usage_percent
    );
    println!("Free Swap:       {}", SystemInfo::format_bytes(free_swap));
}

fn print_disk_info() {
    let info = SystemInfo::collect();

    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║              DISK INFORMATION                         ║");
    println!("╚═══════════════════════════════════════════════════════╝");
    println!();

    if info.disks.is_empty() {
        println!("No disk information available.");
        return;
    }

    for (idx, disk) in info.disks.iter().enumerate() {
        let used_space = disk.total_space - disk.available_space;
        let usage_percent = if disk.total_space > 0 {
            used_space as f64 / disk.total_space as f64 * 100.0
        } else {
            0.0
        };

        println!("═══ Disk {} ═══", idx + 1);
        println!("Name:           {}", disk.name);
        println!("Mount Point:    {}", disk.mount_point);
        println!("File System:    {}", disk.file_system);
        println!(
            "Total Space:    {}",
            SystemInfo::format_bytes(disk.total_space)
        );
        println!(
            "Used Space:     {} ({:.2}%)",
            SystemInfo::format_bytes(used_space),
            usage_percent
        );
        println!(
            "Available Space: {}",
            SystemInfo::format_bytes(disk.available_space)
        );
        println!();
    }
}

fn print_network_info() {
    let info = SystemInfo::collect();

    println!("╔═══════════════════════════════════════════════════════╗");
    println!("║              NETWORK INFORMATION                      ║");
    println!("╚═══════════════════════════════════════════════════════╝");
    println!();

    // Display network details (IP and bandwidth)
    println!("═══ NETWORK DETAILS ═══");
    if let Some(local_ip) = &info.network_details.local_ip {
        println!("🏠 Local IP:        {}", local_ip);
    } else {
        println!("🏠 Local IP:        Not available");
    }

    if let Some(public_ip) = &info.network_details.public_ip {
        println!("🌍 Public IP:       {}", public_ip);
    } else {
        println!("🌍 Public IP:       Not available");
    }

    if let Some(bandwidth) = info.network_details.bandwidth_mbps {
        println!("⚡ Bandwidth:       {:.2} Mbps", bandwidth);
    } else {
        println!("⚡ Bandwidth:       Not available");
    }
    println!();

    // Display network interfaces
    if info.networks.is_empty() {
        println!("No network interfaces available.");
        return;
    }

    println!("═══ NETWORK INTERFACES ═══");
    for (idx, network) in info.networks.iter().enumerate() {
        println!("Interface {}: {}", idx + 1, network.interface_name);
        println!(
            "  Received:       {}",
            SystemInfo::format_bytes(network.received_bytes)
        );
        println!(
            "  Transmitted:    {}",
            SystemInfo::format_bytes(network.transmitted_bytes)
        );
        println!(
            "  Total:          {}",
            SystemInfo::format_bytes(network.received_bytes + network.transmitted_bytes)
        );
        println!();
    }
}

fn print_all_info() {
    print_overview();
    println!();
    print_cpu_info();
    println!();
    print_memory_info();
    println!();
    print_disk_info();
    println!();
    print_network_info();
}
