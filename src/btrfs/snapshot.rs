use crate::types::Vault;

pub fn snap(vault: &Vault) -> Result<(), String> {
    //Stage 1 - verify that all relevant paths are btrfs subvols
    if !btrfs::is_btrfs_subvol(vault.path.as_path()) {
        return Err(format!(
            "Vault Path {:?} is not a valid btrfs subvolume",
            vault.path
        ));
    }

    for vol in vault.volumes.clone() {
        if !btrfs::is_btrfs_subvol(vol.path.as_path()) {
            return Err(format!(
                "Subvolume Path {:?} is not a valid btrfs subvolume",
                vol.path
            ));
        }
    }

    //Stage 2 - create the btrfs snapshot folder
    let snap_time = chrono::Local::now();
    let snap_folder: std::path::PathBuf = snap_time //relative path
        .format("%F %I:%M%p")
        .to_string()
        .into();
    let snap_folder = vault
        .path //absolute path
        .join(snap_folder);

    println!("creating snapshot dir {:?}", snap_folder);

    match {
        std::fs::DirBuilder::new()
            .recursive(true)
            .create(&snap_folder)
    } {
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

    Ok(())
}
