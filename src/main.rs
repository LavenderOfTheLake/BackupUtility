use crate::types::RetentionTier;
use std::path::Path;
use types::Vault;

mod btrfs;
mod clap_factory;
mod config;
mod types;

fn main() {
    let args = clap_factory::clap_factory().get_matches();

    let cfg_file_path: &String = args.get_one::<String>("match").unwrap();
    let cfg_dir = Path::new(cfg_file_path);
    let config_file = config::read_config_from_file(&cfg_dir).expect("Couldn't read config");

    match args.subcommand() {
        Some(("snap", x)) => snap(&config_file, x),
        Some(("check", x)) => {
            println!("Checking config file at {cfg_file_path}");
            check(&config_file, x);
        }
        Some(("list", x)) => list(&config_file, x),
        None => (),
        Some((_, _)) => (),
    }
}

fn snap(
    config_file: &Vec<Vault>,
    args: &clap::ArgMatches,
) {
}
fn check(
    config_file: &Vec<Vault>,
    args: &clap::ArgMatches,
) -> bool {
    for (i, vault) in config_file.iter().enumerate() {
        if !btrfs::is_btrfs_subvol(&vault.path) {
            println!("Vault #{} is not a valid btrfs fs", i + 1);
        }

        //for_duration should be strictly increasing
        //keep_every should be strictly increasing
        let mut most_permissive: &RetentionTier = &vault.retention_policy[0];

        for i in 1..vault.retention_policy.len() {
            let retention_tier = &vault.retention_policy[i];
            if retention_tier.max_snapshots == None {
                if (&retention_tier).keep_every < most_permissive.keep_every {
                    //error, current `retention_tier` is shadowed by `most_permissive`
                }
                if let Some(a) = (&retention_tier).for_duration {
                } else {
                    //error, it doesn't have a for_duration or a max_snapshots
                }

                continue; //We can only check that previous rules don't shadow this one
            };
        }
    }

    true
}
fn list(
    config_file: &Vec<Vault>,
    args: &clap::ArgMatches,
) {
}
