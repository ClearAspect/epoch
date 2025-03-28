mod cli;

use chrono::{Local, NaiveDateTime};
use clap::ArgMatches;
use crossterm::{
    cursor,
    event::{poll, read, Event},
    execute,
    style::Print,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io::stdout, thread::sleep, time::Duration};

fn get_time_str(time: NaiveDateTime, args: &ArgMatches) -> String {
    let mut clock = String::new();

    // Create the format string
    if args.get_flag("year") {
        clock.push_str(&time.format("%g:").to_string());
    }
    if args.get_flag("month") {
        clock.push_str(&time.format("%m:").to_string());
    }
    if args.get_flag("day") {
        clock.push_str(&time.format("%d:").to_string());
    }
    clock.push_str(&time.format("%H:%M").to_string());
    if args.get_flag("second") {
        clock.push_str(&time.format(":%S").to_string());
    }
    if args.get_flag("millisecond") {
        clock.push_str(&time.format(":%3f").to_string()[..3].to_string());
    }

    // Format is yy:mm:dd:hh:mm:ss:ms
    return clock;
}

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    let args = cli::create_cli().get_matches();

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

        let time_str = get_time_str(Local::now().naive_local(), &args);
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

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveTime};

    use super::*;

    #[test]
    fn test_get_time_str() {
        let date = NaiveDate::from_ymd_opt(2025, 3, 28).unwrap();

        let time = NaiveTime::from_hms_milli_opt(10, 59, 40, 10).unwrap();

        let date_time = NaiveDateTime::new(date, time);

        let args = cli::create_cli().get_matches_from(vec![
            "chrono",
            "--year",
            "--month",
            "--day",
            "--second",
            "--millisecond",
        ]);
        assert_eq!(get_time_str(date_time, &args), "25:03:28:10:59:40:01");
    }
}
