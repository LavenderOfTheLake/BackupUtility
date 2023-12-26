pub mod types; 
use std::{fs::DirBuilder, path::PathBuf};
use std::path::Path;

use types::{Snapshot, Vault, Volume};


/// # Take_Snapshot
/// ## Preconditions
///     vol is valid 
///     user is root or has perms
/// 
/// 
/// ## Example executed command
///     `sudo btrfs subvol snap /home/anabelle/ . /mnt/@snapshots/1`
/// ## TODO
///     make it use the local timezone
fn snap_volume<'a,'b>(
    vol: &'a Volume, 
    vault: &'a Vault
) -> Result<Snapshot<'a>,String> {

    if ! is_btrfs_subvol(vault.path.as_path()) {
        let err_text = format!("Vault Path {:?} is not a valid btrfs subvolume",vault.path);
        return Err(err_text);
    }

    // for vol in vault.volumes.clone() {
        if ! is_btrfs_subvol(vol.path.as_path()) {
            return Err(format!("Subvolume Path {:?} is not a valid btrfs subvolume",vol.path))
        }
    // }

    let snap_time = chrono::Local::now();

    let snap_time_folder: PathBuf = snap_time
        .format("%F %I:%M%p")
        .to_string()
        .into();
        
    let snap_time_folder = vault.path
        .join(snap_time_folder);

    println!("creating snapshot dir {:?}",snap_time_folder);
    DirBuilder::new()
        .recursive(true)
        .create(&snap_time_folder)
        .expect("Dirbuilder should never panic when recursive=true");

    let source = vol.path.as_path();

    let destination = &snap_time_folder
        .join(&vol.name);
    
    let _proc = std::process::Command::new("btrfs")
        .arg("subvolume")
        .arg("snapshot")
        .arg(source)
        .arg(destination)
        .output();

    println!("running subvolume snapshot {:?} {:?}",&source,&destination);

    match  _proc {
        Err(_) => Err("btrfs subvol snapshot failed".into()),
        Ok (_) => Ok(Snapshot{time:snap_time.naive_local(),vault}),
    }

}

fn main() {
    let v = Volume{
        name: "@home".into(),
        path: "/home".into()
    };

    snap_volume(
        &v, &Vault {
            path: "/mnt/@snapshots".into(),
            retain_number: 0,
            snapshots: Vec::new(),
            volumes: vec![&v]
        }
    ).unwrap();
}

fn is_btrfs_subvol(path: &Path) -> bool{
    match std::process::Command::new("btrfs")
        .arg("subvolume")
        .arg("show")
        .arg(path)
        .output()
    {
        Ok(_)  => true,
        Err(_) => false,
    }
}