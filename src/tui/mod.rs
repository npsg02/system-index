use crate::models::SystemInfo;
use crate::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};

/// Width of progress bars in characters
const PROGRESS_BAR_WIDTH: usize = 50;

/// Application state
pub struct App {
    system_info: SystemInfo,
    last_refresh: Instant,
    status_message: String,
    current_tab: Tab,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Overview,
    Memory,
    Disks,
    Network,
}

impl App {
    pub fn new() -> Self {
        Self {
            system_info: SystemInfo::collect(),
            last_refresh: Instant::now(),
            status_message: "Welcome to System Index! Press 'h' for help, 'q' to quit.".to_string(),
            current_tab: Tab::Overview,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_app(&mut terminal);

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        result
    }

    fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            // Auto-refresh every 2 seconds
            if self.last_refresh.elapsed() > Duration::from_secs(2) {
                self.refresh();
            }

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press && self.handle_input(key.code)? {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_input(&mut self, key: KeyCode) -> Result<bool> {
        match key {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('h') => {
                self.status_message =
                    "Keys: q=quit, r=refresh, 1=overview, 2=memory, 3=disks, 4=network".to_string();
            }
            KeyCode::Char('r') => {
                self.refresh();
                self.status_message = "System information refreshed!".to_string();
            }
            KeyCode::Char('1') => {
                self.current_tab = Tab::Overview;
                self.status_message = "Showing: Overview".to_string();
            }
            KeyCode::Char('2') => {
                self.current_tab = Tab::Memory;
                self.status_message = "Showing: Memory".to_string();
            }
            KeyCode::Char('3') => {
                self.current_tab = Tab::Disks;
                self.status_message = "Showing: Disks".to_string();
            }
            KeyCode::Char('4') => {
                self.current_tab = Tab::Network;
                self.status_message = "Showing: Network".to_string();
            }
            _ => {}
        }
        Ok(false)
    }

    fn refresh(&mut self) {
        self.system_info = SystemInfo::collect();
        self.last_refresh = Instant::now();
    }

    fn ui(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(f.size());

        // Title with tabs
        let tab_titles = [
            ("1: Overview", self.current_tab == Tab::Overview),
            ("2: Memory", self.current_tab == Tab::Memory),
            ("3: Disks", self.current_tab == Tab::Disks),
            ("4: Network", self.current_tab == Tab::Network),
        ];

        let tabs_text: Vec<String> = tab_titles
            .iter()
            .map(|(title, selected)| {
                if *selected {
                    format!("[{}]", title)
                } else {
                    title.to_string()
                }
            })
            .collect();

        let title = Paragraph::new(format!("🖥️  System Index - {}", tabs_text.join(" | ")))
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Content based on current tab
        match self.current_tab {
            Tab::Overview => self.render_overview(f, chunks[1]),
            Tab::Memory => self.render_memory(f, chunks[1]),
            Tab::Disks => self.render_disks(f, chunks[1]),
            Tab::Network => self.render_network(f, chunks[1]),
        }

        // Status bar
        let status = Paragraph::new(self.status_message.clone())
            .style(Style::default())
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Status"));
        f.render_widget(status, chunks[2]);
    }

    fn render_overview(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let info = &self.system_info;
        let items = vec![
            format!("🖥️  Hostname: {}", info.hostname),
            format!("💻 OS: {} {}", info.os_name, info.os_version),
            format!("🔧 Kernel: {}", info.kernel_version),
            format!("⏰ Uptime: {}", SystemInfo::format_uptime(info.uptime)),
            String::new(),
            format!("⚙️  CPU: {}", info.cpu_brand),
            format!("📊 CPU Cores: {}", info.cpu_count),
            String::new(),
            format!(
                "💾 Total Memory: {}",
                SystemInfo::format_bytes(info.total_memory)
            ),
            format!(
                "📈 Used Memory: {}",
                SystemInfo::format_bytes(info.used_memory)
            ),
            format!(
                "📉 Free Memory: {}",
                SystemInfo::format_bytes(info.total_memory - info.used_memory)
            ),
            String::new(),
            format!(
                "🔄 Total Swap: {}",
                SystemInfo::format_bytes(info.total_swap)
            ),
            format!("📊 Used Swap: {}", SystemInfo::format_bytes(info.used_swap)),
            String::new(),
            format!("💿 Disks: {}", info.disks.len()),
            format!("🌐 Network Interfaces: {}", info.networks.len()),
            format!("📋 Running Processes: {}", info.processes_count),
        ];

        let list_items: Vec<ListItem> = items
            .iter()
            .map(|item| ListItem::new(item.as_str()))
            .collect();

        let list = List::new(list_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("System Overview"),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(list, area);
    }

    fn render_memory(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let info = &self.system_info;

        let total_mem = info.total_memory;
        let used_mem = info.used_memory;
        let free_mem = total_mem - used_mem;
        let mem_usage_percent = if total_mem > 0 {
            (used_mem as f64 / total_mem as f64 * 100.0) as u32
        } else {
            0
        };

        let total_swap = info.total_swap;
        let used_swap = info.used_swap;
        let free_swap = total_swap.saturating_sub(used_swap);
        let swap_usage_percent = if total_swap > 0 {
            (used_swap as f64 / total_swap as f64 * 100.0) as u32
        } else {
            0
        };

        let items = vec![
            "═══ RAM MEMORY ═══".to_string(),
            format!("Total:     {}", SystemInfo::format_bytes(total_mem)),
            format!(
                "Used:      {} ({}%)",
                SystemInfo::format_bytes(used_mem),
                mem_usage_percent
            ),
            format!("Free:      {}", SystemInfo::format_bytes(free_mem)),
            format!(
                "Usage Bar: [{}]",
                Self::create_progress_bar(mem_usage_percent)
            ),
            String::new(),
            "═══ SWAP MEMORY ═══".to_string(),
            format!("Total:     {}", SystemInfo::format_bytes(total_swap)),
            format!(
                "Used:      {} ({}%)",
                SystemInfo::format_bytes(used_swap),
                swap_usage_percent
            ),
            format!("Free:      {}", SystemInfo::format_bytes(free_swap)),
            format!(
                "Usage Bar: [{}]",
                Self::create_progress_bar(swap_usage_percent)
            ),
        ];

        let list_items: Vec<ListItem> = items
            .iter()
            .map(|item| {
                if item.starts_with("═══") {
                    ListItem::new(item.as_str()).style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    ListItem::new(item.as_str())
                }
            })
            .collect();

        let list = List::new(list_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Memory Details"),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(list, area);
    }

    /// Create a progress bar string for the given percentage
    fn create_progress_bar(percent: u32) -> String {
        let filled = (percent / 2) as usize;
        let empty = PROGRESS_BAR_WIDTH - filled;
        format!("{}{}", "█".repeat(filled), "░".repeat(empty))
    }

    fn render_disks(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let info = &self.system_info;

        let mut items = vec!["Mounted Disks:".to_string(), String::new()];

        for (idx, disk) in info.disks.iter().enumerate() {
            let used_space = disk.total_space - disk.available_space;
            let usage_percent = if disk.total_space > 0 {
                (used_space as f64 / disk.total_space as f64 * 100.0) as u32
            } else {
                0
            };

            items.push(format!("═══ Disk {} ═══", idx + 1));
            items.push(format!("Name:       {}", disk.name));
            items.push(format!("Mount:      {}", disk.mount_point));
            items.push(format!("Filesystem: {}", disk.file_system));
            items.push(format!(
                "Total:      {}",
                SystemInfo::format_bytes(disk.total_space)
            ));
            items.push(format!(
                "Used:       {} ({}%)",
                SystemInfo::format_bytes(used_space),
                usage_percent
            ));
            items.push(format!(
                "Available:  {}",
                SystemInfo::format_bytes(disk.available_space)
            ));
            items.push(format!(
                "Usage Bar:  [{}]",
                Self::create_progress_bar(usage_percent)
            ));
            items.push(String::new());
        }

        if info.disks.is_empty() {
            items.push("No disks found.".to_string());
        }

        let list_items: Vec<ListItem> = items
            .iter()
            .map(|item| {
                if item.starts_with("═══") {
                    ListItem::new(item.as_str()).style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    ListItem::new(item.as_str())
                }
            })
            .collect();

        let list = List::new(list_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Disk Information"),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(list, area);
    }

    fn render_network(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let info = &self.system_info;

        let mut items = vec!["Network Interfaces:".to_string(), String::new()];

        for (idx, network) in info.networks.iter().enumerate() {
            items.push(format!("═══ Interface {} ═══", idx + 1));
            items.push(format!("Name:       {}", network.interface_name));
            items.push(format!(
                "Received:   {}",
                SystemInfo::format_bytes(network.received_bytes)
            ));
            items.push(format!(
                "Transmitted: {}",
                SystemInfo::format_bytes(network.transmitted_bytes)
            ));
            items.push(format!(
                "Total:      {}",
                SystemInfo::format_bytes(network.received_bytes + network.transmitted_bytes)
            ));
            items.push(String::new());
        }

        if info.networks.is_empty() {
            items.push("No network interfaces found.".to_string());
        }

        let list_items: Vec<ListItem> = items
            .iter()
            .map(|item| {
                if item.starts_with("═══") {
                    ListItem::new(item.as_str()).style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                } else {
                    ListItem::new(item.as_str())
                }
            })
            .collect();

        let list = List::new(list_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Network Information"),
            )
            .style(Style::default().fg(Color::White));

        f.render_widget(list, area);
    }
}
