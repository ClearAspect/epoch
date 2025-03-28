use chrono::{DateTime, Local};
use crossterm::{
    cursor,
    event::{poll, read, Event},
    execute,
    style::Print,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io::stdout, thread::sleep, time::Duration};

fn get_time() -> String {
    let local: DateTime<Local> = Local::now();
    let time_format = local.format("%g:%m:%d:%H:%M:%S:%3f").to_string();
    return time_format[..time_format.len() - 1].to_string();
}

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();

    // Enter the alternate screen and hide the cursor.
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    terminal::enable_raw_mode()?;

    loop {
        // Exit on any key press.
        if poll(Duration::from_millis(10))? {
            if let Event::Key(_event) = read()? {
                break;
            }
        }

        let time_str = get_time();
        let (cols, rows) = terminal::size()?;
        let x = (cols.saturating_sub(time_str.len() as u16)) / 2;
        let y = rows / 2;

        // Clear the screen and print the time in the center.
        execute!(
            stdout,
            Clear(ClearType::All),
            cursor::MoveTo(x, y),
            Print(&time_str)
        )?;

        sleep(Duration::from_millis(1));
    }

    // Restore the terminal to its original state.
    terminal::disable_raw_mode()?;
    execute!(stdout, cursor::Show, LeaveAlternateScreen)?;

    Ok(())
}
