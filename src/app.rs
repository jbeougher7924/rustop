use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use crate::{cpu::CpuMonitor, gpu::GpuMonitor, ui};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut cpu_monitor = CpuMonitor::new();
    let mut gpu_monitor = GpuMonitor::new();
    let res = run_app(&mut terminal, &mut cpu_monitor, &mut gpu_monitor);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res.map_err(|err| err.into())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    cpu_monitor: &mut CpuMonitor,
    gpu_monitor: &mut GpuMonitor,
) -> io::Result<()> {
    let tick_rate = Duration::from_millis(500);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| ui::draw(frame, cpu_monitor, gpu_monitor))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_millis(0));

        if event::poll(timeout)? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::F(10) | KeyCode::Char('q') | KeyCode::Char('Q') => return Ok(()),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            cpu_monitor.refresh();
            gpu_monitor.refresh();
            last_tick = Instant::now();
        }
    }
}
