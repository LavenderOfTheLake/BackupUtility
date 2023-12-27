pub mod types;

use std::path::Path;
use std::{fs::DirBuilder, path::PathBuf};
use types::{Snapshot, Vault, Volume};

fn snap<'a>(vault: &'a Vault) -> Result<Snapshot<'a>, String> {
    //Stage 1 - verify that all relevant paths are btrfs subvols
    if !is_btrfs_subvol(vault.path.as_path()) {
        return Err(format!(
            "Vault Path {:?} is not a valid btrfs subvolume",
            vault.path
        ));
    }

    for vol in vault.volumes.clone() {
        if !is_btrfs_subvol(vol.path.as_path()) {
            return Err(format!(
                "Subvolume Path {:?} is not a valid btrfs subvolume",
                vol.path
            ));
        }
    }

    //Stage 2 - create the btrfs snapshot folder
    let snap_time = chrono::Local::now();
    let snap_folder: PathBuf = snap_time //relative path
        .format("%F %I:%M%p")
        .to_string()
        .into();
    let snap_folder = vault
        .path //absolute path
        .join(snap_folder);

    println!("creating snapshot dir {:?}", snap_folder);

    match { DirBuilder::new().recursive(true).create(&snap_folder) } {
        Err(_) => {
            return Err("Couldn't create Snapshot folder".into());
        }
        Ok(_) => (),
    };

    //stage 3 - create snapshots
    for vol in vault.volumes.clone() {
        let source = vol.path.as_path();
        let destination = &snap_folder.join(&vol.name);

        let result = std::process::Command::new("btrfs")
            .arg("subvolume")
            .arg("snapshot")
            .arg(&source)
            .arg(&destination)
            .output();

        match result {
            Err(err) => return Err(err.to_string()),
            Ok(_) => (),
        }
    }

    Ok(Snapshot {
        vault,
        time: snap_time.naive_local(),
    })
}

fn main() {
    let v = Volume {
        name: "@home".into(),
        path: "/home".into(),
    };

    snap(&Vault {
        path: "/mnt/@snapshots".into(),
        retain_number: 0,
        snapshots: Vec::new(),
        volumes: vec![&v],
    })
    .unwrap();
}

fn is_btrfs_subvol(path: &Path) -> bool {
    match std::process::Command::new("btrfs")
        .arg("subvolume")
        .arg("show")
        .arg(path)
        .output()
    {
        Ok(_) => true,
        Err(_) => false,
    }
}
