use clap;

pub fn cli() -> clap::Command {
    clap::Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            clap::Arg::new("day-number")
                .index(1)
                .help("Type the day you want to run.")

        )
        .arg(
            clap::Arg::new("run-all")
                .short('a')
                .long("all")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("day-number")
        )
}