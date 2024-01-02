pub fn is_btrfs_subvol(path: &std::path::Path) -> bool {
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
