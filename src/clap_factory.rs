use clap::{self, ValueHint};

pub fn clap_factory() -> clap::Command {
    let arg_vaults = clap::Arg::new("vaults")
        .short('v')
        .required(false)
        .value_hint(clap::ValueHint::DirPath)
        .value_name("vaults")
        .num_args(1..)
        .action(clap::ArgAction::Set);

    let arg_config = clap::Arg::new("config")
        .short('c')
        .default_value("$XDG_CONFIG_DIR/rusty-daemon.yaml")
        .help("path to config file")
        .value_hint(ValueHint::FilePath)
        .action(clap::ArgAction::Set)
        .num_args(1)
        .required(false);

    return clap::Command::new("Rusty Daemon")
        .author("Anabelle")
        .arg_required_else_help(true)
        .arg(arg_config)
        .subcommand(
            clap::Command::new("snap")
                .alias("backup")
                .alias("snapshot")
                .about("Takes a snapshot")
                .arg(arg_vaults.clone().help("Only backup the specified vaults")),
        )
        .subcommand(
            clap::Command::new("check")
                .about("parses the config to reveal syntax and shadowing errors"),
        )
        .subcommand(
            clap::Command::new("list").about("Lists snapshots").arg(
                arg_vaults
                    .clone()
                    .help("Only list snapshots from the specified vaults"),
            ),
        );
}
