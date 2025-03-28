use clap::{Arg, ArgAction, Command};

// yy:mm:dd:hh:mm:ss:ms
pub fn create_cli() -> Command {
    let xxd_rs = Command::new("xxd-rs")
        .author("Roan Mason, roanmason@live.ca")
        .version("0.0.1")
        .about("A simple clock screen saver.")
        .arg(
            Arg::new("year")
                .short('y')
                .long("year")
                .action(ArgAction::SetTrue)
                .required(false)
                .help("Display the year."),
        )
        .arg(
            Arg::new("month")
                .short('m')
                .long("month")
                .action(ArgAction::SetTrue)
                .required(false)
                .help("Display the month."),
        )
        .arg(
            Arg::new("day")
                .short('d')
                .long("day")
                .action(ArgAction::SetTrue)
                .required(false)
                .help("Display the day."),
        )
        .arg(
            Arg::new("second")
                .short('s')
                .long("second")
                .action(ArgAction::SetTrue)
                .required(false)
                .help("Display the seconds."),
        )
        .arg(
            Arg::new("millisecond")
                .short('f')
                .long("millisecond")
                .action(ArgAction::SetTrue)
                .required(false)
                .help("Display the milliseconds."),
        );

    return xxd_rs;
}
