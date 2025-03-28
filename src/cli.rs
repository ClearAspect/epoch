use clap::{value_parser, Arg, ArgAction, Command};

pub fn create_cli() -> Command {
    let xxd_rs = Command::new("xxd-rs")
        .author("Roan Mason, roanmason@live.ca")
        .version("0.0.1")
        .about("make a hex dump or do the reverse.")
        //.arg(
        //    Arg::new("cols")
        //        .short('c')
        //        .long("cols")
        //        .value_parser(value_parser!(usize))
        //        .required(false)
        //        .help("Format <cols> octets per line. Default 16 (-i: 12, -ps: 30, -b: 6). Max 256.  No maximum for -ps. With -ps, 0 results in one long line of output."),
        //)
    ;

    return xxd_rs;
}
