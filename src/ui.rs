//! Rendering layer for the `rustop` TUI.

use crate::{cpu::CpuMonitor, gpu::GpuMonitor, utilities};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Gauge, Paragraph, Row, Table, Wrap},
    Frame,
};

/// Draw the entire UI frame.
pub fn draw(frame: &mut Frame<'_>, cpu: &CpuMonitor, gpu: &GpuMonitor) {
    let banner_text = utilities::banner_text();
    let banner_height = banner_text.lines().count() as u16;
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(banner_height.saturating_add(2)),
            Constraint::Length(6),
            Constraint::Min(5),
            Constraint::Length(2),
        ])
        .split(frame.size());

    let banner = Paragraph::new(banner_text)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(banner, vertical[0]);

    let stats_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(34),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(vertical[1]);

    render_cpu_gauge(frame, stats_chunks[0], cpu);
    render_memory_gauge(frame, stats_chunks[1], cpu);
    render_info_panel(frame, stats_chunks[2], cpu, gpu);

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(67), Constraint::Percentage(33)])
        .split(vertical[2]);

    render_cpu_table(frame, body_chunks[0], cpu);
    render_gpu_panel(frame, body_chunks[1], gpu);

    let instructions = Paragraph::new(utilities::QUIT_INSTRUCTIONS)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    frame.render_widget(instructions, vertical[3]);
}

fn render_cpu_gauge(frame: &mut Frame<'_>, area: Rect, cpu: &CpuMonitor) {
    let avg_usage = cpu.avg().clamp(0.0, 100.0);
    let gauge_color = usage_color(avg_usage);
    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(Span::styled(
                    "CPU Average",
                    Style::default().fg(Color::Blue),
                ))
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(gauge_color))
        .ratio(f64::from(avg_usage) / 100.0)
        .label(format!("{:5.1}%", avg_usage));
    frame.render_widget(gauge, area);
}

fn render_memory_gauge(frame: &mut Frame<'_>, area: Rect, cpu: &CpuMonitor) {
    let (used_kib, total_kib) = cpu.memory_usage();
    let ratio = cpu.memory_ratio().clamp(0.0, 1.0);
    let percent = (ratio * 100.0) as f32;
    let gauge_color = usage_color(percent);
    let label = if total_kib == 0 {
        "N/A".to_string()
    } else {
        format!(
            "{} / {}",
            format_bytes_from_kib(used_kib),
            format_bytes_from_kib(total_kib)
        )
    };

    let gauge = Gauge::default()
        .block(
            Block::default()
                .title(Span::styled("RAM Usage", Style::default().fg(Color::Blue)))
                .borders(Borders::ALL),
        )
        .gauge_style(Style::default().fg(gauge_color))
        .ratio(ratio)
        .label(label);
    frame.render_widget(gauge, area);
}

fn render_info_panel(frame: &mut Frame<'_>, area: Rect, cpu: &CpuMonitor, gpu: &GpuMonitor) {
    let info = Paragraph::new(format!(
        "Threads: {}\nGPUs: {}\nRefresh: 500ms",
        cpu.thread_count(),
        gpu.stats().len()
    ))
    .block(
        Block::default()
            .title(Span::styled("Info", Style::default().fg(Color::Blue)))
            .borders(Borders::ALL),
    );
    frame.render_widget(info, area);
}

fn render_cpu_table(frame: &mut Frame<'_>, area: Rect, cpu: &CpuMonitor) {
    let thread_count = cpu.thread_count();
    if thread_count == 0 {
        let empty = Paragraph::new("No CPU data available").block(
            Block::default()
                .title(Span::styled(
                    "Per-thread usage",
                    Style::default().fg(Color::Blue),
                ))
                .borders(Borders::ALL),
        );
        frame.render_widget(empty, area);
        return;
    }

    let min_column_width = 30_u16;
    let mut columns = ((area.width / min_column_width) as usize).max(1);
    columns = columns.min(thread_count.max(1));
    if columns == 0 {
        columns = 1;
    }
    let rows = (thread_count + columns - 1) / columns;

    let mut column_constraints = Vec::with_capacity(columns);
    let percentage = (100 / columns as u16).max(1);
    for _ in 0..columns {
        column_constraints.push(Constraint::Percentage(percentage));
    }

    let cpu_rows = (0..rows).map(|row_idx| {
        let cells = (0..columns).map(|col_idx| {
            let idx = row_idx * columns + col_idx;
            if let Some(usage) = cpu.usages().get(idx) {
                let color = usage_color(*usage);
                let line = Line::from(vec![
                    Span::raw(format!("CPU {:02}: {:5.1}% ", idx, usage)),
                    Span::styled(make_bar_no_pct(*usage), Style::default().fg(color)),
                ]);
                Cell::from(line)
            } else {
                Cell::from(String::new())
            }
        });
        Row::new(cells)
    });

    let cpu_table = Table::new(cpu_rows, column_constraints)
        .block(
            Block::default()
                .title(Span::styled(
                    "Per-thread usage",
                    Style::default().fg(Color::Blue),
                ))
                .borders(Borders::ALL),
        )
        .column_spacing(1);

    frame.render_widget(cpu_table, area);
}

fn render_gpu_panel(frame: &mut Frame<'_>, area: Rect, gpu: &GpuMonitor) {
    let panel = Block::default()
        .title(Span::styled("GPU Usage", Style::default().fg(Color::Blue)))
        .borders(Borders::ALL);
    frame.render_widget(&panel, area);
    let inner = panel.inner(area);

    let stats = gpu.stats();
    if stats.is_empty() {
        let message = if gpu.nvml_available() {
            "No NVIDIA GPUs detected"
        } else {
            "NVML unavailable - GPU stats disabled"
        };
        let block = Paragraph::new(message);
        frame.render_widget(block, inner);
        return;
    }

    let mut constraints: Vec<Constraint> = Vec::with_capacity(stats.len());
    constraints.extend(std::iter::repeat(Constraint::Length(7)).take(stats.len()));
    let cards = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(inner);

    for (chunk, gpu_stat) in cards.iter().zip(stats.iter()) {
        render_gpu_card(frame, *chunk, gpu_stat);
    }
}

fn render_gpu_card(frame: &mut Frame<'_>, area: Rect, gpu: &crate::gpu::GpuStats) {
    let load_pct = gpu.utilization.clamp(0.0, 100.0);
    let load_line = Line::from(vec![
        Span::raw(format!("Use: {:3.0}% ", load_pct)),
        Span::styled(
            make_bar_no_pct(load_pct),
            Style::default().fg(usage_color(load_pct)),
        ),
    ]);

    let vram_line = if gpu.memory_total == 0 {
        Line::from("Ram: N/A")
    } else {
        let ratio = gpu.memory_used as f64 / gpu.memory_total as f64;
        let pct = (ratio * 100.0) as f32;
        let bar = make_bar_no_pct(pct);
        let used = format_bytes(gpu.memory_used);
        let total = format_bytes(gpu.memory_total);
        Line::from(vec![
            Span::raw(format!("Ram: {:3.0}% ", pct)),
            Span::styled(bar, Style::default().fg(usage_color(pct))),
            Span::raw(format!(" {} / {}", used, total)),
        ])
    };

    let temp_line = Line::from(format!(
        "Temp: {}",
        gpu.temperature
            .map(|t| format!("{t}°C"))
            .unwrap_or_else(|| "N/A".to_string())
    ));

    let lines = vec![
        Line::from(format!("GPU {}: {}", gpu.index, gpu.name)),
        load_line,
        vram_line,
        temp_line,
    ];

    let card = Paragraph::new(lines).block(Block::default().borders(Borders::ALL));
    frame.render_widget(card, area);
}

fn usage_color(value: f32) -> Color {
    match value {
        v if v < 40.0 => Color::Green,
        v if v < 75.0 => Color::Yellow,
        v if v < 90.0 => Color::Rgb(255, 165, 0),
        _ => Color::Red,
    }
}

fn format_bytes_from_kib(kib: u64) -> String {
    let bytes = kib.saturating_mul(1024);
    format_bytes(bytes)
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut unit = 0;
    let mut value = bytes as f64;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{:.0} {}", value, UNITS[unit])
    } else {
        format!("{:.1} {}", value, UNITS[unit])
    }
}

fn make_bar_no_pct(pct: f32) -> String {
    let ratio = pct / 100.0;
    let total_blocks = 20usize;
    let filled = (ratio * total_blocks as f32).round() as usize;
    let empty = total_blocks.saturating_sub(filled);
    format!("[{}{}]", "█".repeat(filled), "-".repeat(empty))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usage_color_thresholds() {
        assert_eq!(usage_color(0.0), Color::Green);
        assert_eq!(usage_color(39.9), Color::Green);
        assert_eq!(usage_color(40.0), Color::Yellow);
        assert_eq!(usage_color(74.9), Color::Yellow);
        assert_eq!(usage_color(75.0), Color::Rgb(255, 165, 0));
        assert_eq!(usage_color(89.9), Color::Rgb(255, 165, 0));
        assert_eq!(usage_color(90.0), Color::Red);
        assert_eq!(usage_color(150.0), Color::Red);
    }

    #[test]
    fn make_bar_no_pct_shapes_bar() {
        assert_eq!(make_bar_no_pct(0.0), "[--------------------]");
        assert_eq!(make_bar_no_pct(50.0), "[██████████----------]");
        assert_eq!(make_bar_no_pct(100.0), "[████████████████████]");
    }

    #[test]
    fn format_bytes_handles_units() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
    }

    #[test]
    fn format_bytes_from_kib_converts() {
        assert_eq!(format_bytes_from_kib(0), "0 B");
        assert_eq!(format_bytes_from_kib(1), "1.0 KB");
        assert_eq!(format_bytes_from_kib(1024), "1.0 MB");
    }
}
