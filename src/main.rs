use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::thread;

use clap::Parser;
use crossterm::{
    cursor, execute, queue, terminal,
    style::{self, Color, SetForegroundColor, ResetColor, Attribute, SetAttribute},
    event::{self, Event, KeyCode, KeyEvent},
};

#[derive(Parser)]
#[command(name = "countdown-timer", about = "A colorful ASCII countdown timer")]
struct Args {
    /// Number of minutes for the countdown
    minutes: f64,
}

const DIGITS: [[&str; 7]; 10] = [
    // 0
    [
        " ██████ ",
        "██    ██",
        "██    ██",
        "██    ██",
        "██    ██",
        "██    ██",
        " ██████ ",
    ],
    // 1
    [
        "   ██   ",
        " ████   ",
        "   ██   ",
        "   ██   ",
        "   ██   ",
        "   ██   ",
        " ██████ ",
    ],
    // 2
    [
        " ██████ ",
        "██    ██",
        "      ██",
        " ██████ ",
        "██      ",
        "██      ",
        "████████",
    ],
    // 3
    [
        " ██████ ",
        "██    ██",
        "      ██",
        "  █████ ",
        "      ██",
        "██    ██",
        " ██████ ",
    ],
    // 4
    [
        "██    ██",
        "██    ██",
        "██    ██",
        "████████",
        "      ██",
        "      ██",
        "      ██",
    ],
    // 5
    [
        "████████",
        "██      ",
        "██      ",
        "███████ ",
        "      ██",
        "██    ██",
        " ██████ ",
    ],
    // 6
    [
        " ██████ ",
        "██      ",
        "██      ",
        "███████ ",
        "██    ██",
        "██    ██",
        " ██████ ",
    ],
    // 7
    [
        "████████",
        "      ██",
        "     ██ ",
        "    ██  ",
        "   ██   ",
        "   ██   ",
        "   ██   ",
    ],
    // 8
    [
        " ██████ ",
        "██    ██",
        "██    ██",
        " ██████ ",
        "██    ██",
        "██    ██",
        " ██████ ",
    ],
    // 9
    [
        " ██████ ",
        "██    ██",
        "██    ██",
        " ███████",
        "      ██",
        "      ██",
        " ██████ ",
    ],
];

const COLON: [&str; 7] = [
    "  ",
    "██",
    "██",
    "  ",
    "██",
    "██",
    "  ",
];

fn get_color_for_remaining(remaining_secs: u64, total_secs: u64, flash_on: bool) -> Color {
    if remaining_secs <= 120 {
        if flash_on {
            Color::Red
        } else {
            Color::DarkRed
        }
    } else if remaining_secs <= total_secs / 4 {
        Color::Yellow
    } else if remaining_secs <= total_secs / 2 {
        Color::Cyan
    } else {
        Color::Green
    }
}

fn render_timer(
    stdout: &mut io::Stdout,
    remaining_secs: u64,
    total_secs: u64,
    flash_on: bool,
) -> io::Result<()> {
    let hours = remaining_secs / 3600;
    let mins = (remaining_secs % 3600) / 60;
    let secs = remaining_secs % 60;

    let digit_indices: Vec<usize> = if hours > 0 {
        vec![
            (hours / 10) as usize,
            (hours % 10) as usize,
            10, // colon
            (mins / 10) as usize,
            (mins % 10) as usize,
            10, // colon
            (secs / 10) as usize,
            (secs % 10) as usize,
        ]
    } else {
        vec![
            (mins / 10) as usize,
            (mins % 10) as usize,
            10, // colon
            (secs / 10) as usize,
            (secs % 10) as usize,
        ]
    };

    let color = get_color_for_remaining(remaining_secs, total_secs, flash_on);
    let (term_width, term_height) = terminal::size()?;

    // Calculate total width of the ASCII art
    let total_width: usize = digit_indices.iter().map(|&i| {
        if i == 10 { 2 } else { 8 }
    }).sum::<usize>() + (digit_indices.len() - 1); // spacing between chars

    let start_col = if (term_width as usize) > total_width {
        ((term_width as usize) - total_width) / 2
    } else {
        0
    };
    let start_row = if (term_height as usize) > 12 {
        ((term_height as usize) - 10) / 2
    } else {
        1
    };

    // Flash effect: hide digits briefly when flashing
    let visible = remaining_secs > 120 || flash_on;

    queue!(stdout, SetForegroundColor(color))?;

    if remaining_secs <= 120 && flash_on {
        queue!(stdout, SetAttribute(Attribute::Bold))?;
    }

    for row in 0..7 {
        queue!(stdout, cursor::MoveTo(start_col as u16, (start_row + row) as u16))?;
        if visible {
            for (i, &idx) in digit_indices.iter().enumerate() {
                if i > 0 {
                    queue!(stdout, style::Print(" "))?;
                }
                let line = if idx == 10 {
                    COLON[row]
                } else {
                    DIGITS[idx][row]
                };
                queue!(stdout, style::Print(line))?;
            }
        }
    }

    // Progress bar
    let bar_width = if (term_width as usize) > 4 { (term_width as usize) - 4 } else { 20 };
    let progress = if total_secs > 0 {
        remaining_secs as f64 / total_secs as f64
    } else {
        0.0
    };
    let filled = (bar_width as f64 * progress) as usize;
    let empty = bar_width - filled;

    let bar_row = start_row + 9;
    queue!(
        stdout,
        cursor::MoveTo(2, bar_row as u16),
        SetForegroundColor(color),
        style::Print("▐"),
        style::Print("█".repeat(filled)),
        SetForegroundColor(Color::DarkGrey),
        style::Print("░".repeat(empty)),
        SetForegroundColor(color),
        style::Print("▌"),
    )?;

    // Status text
    let status_row = bar_row + 2;
    let status = if remaining_secs == 0 {
        "⏰  TIME'S UP!  ⏰".to_string()
    } else if remaining_secs <= 120 {
        format!("⚠  {} seconds remaining!", remaining_secs)
    } else {
        let mins_left = (remaining_secs as f64 / 60.0).ceil() as u64;
        format!("⏳  {} minute{} remaining", mins_left, if mins_left == 1 { "" } else { "s" })
    };

    let status_col = if (term_width as usize) > status.chars().count() {
        ((term_width as usize) - status.chars().count()) / 2
    } else {
        0
    };

    queue!(
        stdout,
        cursor::MoveTo(status_col as u16, status_row as u16),
        SetForegroundColor(if remaining_secs == 0 { Color::Red } else { color }),
        style::Print(&status),
    )?;

    queue!(
        stdout,
        SetAttribute(Attribute::Reset),
        ResetColor,
    )?;

    stdout.flush()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.minutes <= 0.0 {
        eprintln!("Error: minutes must be a positive number");
        std::process::exit(1);
    }

    let total_secs = (args.minutes * 60.0).round() as u64;
    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All),
    )?;

    let start = Instant::now();
    let mut flash_toggle = true;
    let flash_interval = Duration::from_millis(500);
    let mut last_flash = Instant::now();

    loop {
        let elapsed = start.elapsed().as_secs();
        let remaining = total_secs.saturating_sub(elapsed);

        // Flash toggle for the last 2 minutes
        if remaining <= 120 && last_flash.elapsed() >= flash_interval {
            flash_toggle = !flash_toggle;
            last_flash = Instant::now();
        }

        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
        render_timer(&mut stdout, remaining, total_secs, flash_toggle)?;

        if remaining == 0 {
            // Flash "TIME'S UP" a few times then wait for keypress
            for _ in 0..10 {
                thread::sleep(Duration::from_millis(300));
                execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
                render_timer(&mut stdout, 0, total_secs, false)?;
                thread::sleep(Duration::from_millis(300));
                execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
                render_timer(&mut stdout, 0, total_secs, true)?;
            }
            // Wait for any key to exit
            loop {
                if event::poll(Duration::from_millis(100))? {
                    if let Event::Key(_) = event::read()? {
                        break;
                    }
                }
            }
            break;
        }

        // Check for 'q' or Escape to quit early
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => break,
                    _ => {}
                }
            }
        } else {
            thread::sleep(Duration::from_millis(50));
        }
    }

    execute!(
        stdout,
        cursor::Show,
        terminal::LeaveAlternateScreen,
    )?;
    terminal::disable_raw_mode()?;

    Ok(())
}
