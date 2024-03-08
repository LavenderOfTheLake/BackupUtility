use regex;
use serde::Deserialize;
use serde_json;
use std::path::Path;

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

pub fn get_btrfs_mounts() -> Result<Vec<BtrfsMount>, &'static str> {
    let mounts: Vec<u8> = std::process::Command::new("findmnt")
        .arg("-J")
        .arg("-t btrfs")
        .output()
        .map_err(|_| "couln't run `findmnt -J -t btrfs`")?
        .stdout;

    let mounts: &str = &String::from_utf8(mounts)
        .map_err(|_| "Couldn't parse findmnt output as valid utf-8 string")?;

    let mounts: FindMntOutput =
        // serde_json::from_str(mounts).map_err(|_| "Couldn't parse findmnt output as valid: json")?;

    return Err("Unimplemented"); //mounts.filesystems.iter().map().collect();
                                 //SERDE STRUCTS

    #[derive(Debug, Deserialize)]
    struct FindMntOutput {
        filesystems: Vec<FindMntMountPoint>,
    }

    #[derive(Debug, Deserialize)]
    struct FindMntMountPoint {
        target: String,
        source: String,
        fstype: String,
        options: String,
    }

    impl BtrfsMount {
        fn from_serde(obj: FindMntMountPoint) -> Option<BtrfsMount> {
            // if obj.source.ends_with("]") {}

            // BtrfsMount {
            //     options: obj.options,
            //     target: obj.target,
            // }
            todo!();
        }
    }
}

pub struct BtrfsMount {
    target: Box<Path>,
    source_device: Box<Path>,
    source_subvol: Option<Box<Path>>,
    options: Vec<String>, //might change
}

pub fn get_mounted_btrfs_partitions() -> Result<Vec<String>, &'static str> {
    let output = std::process::Command::new("btrfs")
        .arg("filesystem")
        .arg("show")
        .arg("--mounted") //
        .arg("--all-devices")
        .arg("--raw")
        .output()
        .map_err(|_| "couldn't run `btrfs filesystem show`")?;

    if !std::process::ExitStatus::success(&output.status) {
        return Err("`btrfs fs show exited with errors (are you root?)");
    }

    dbg!(regex::Regex::new(r"^\tdev.+path (.+)$").unwrap());

    todo!();
}

//We can get the start of the subvol from options.find(",subvol=/")

// $ findmnt -Jt btrfs
// """"
// {
//    "filesystems": [
//       {
//          "target": "/home",
//          "source": "/dev/mapper/vg0-home[/@home]",
//          "fstype": "btrfs",
//          "options": "rw,relatime,seclabel,ssd,space_cache=v2,subvolid=256,subvol=/@home"
//       },{
//          "target": "/mnt",
//          "source": "/dev/mapper/vg0-home",
//          "fstype": "btrfs",
//          "options": "rw,relatime,seclabel,ssd,space_cache=v2,subvolid=5,subvol=/"
//       },{
//          "target": "/media/ ",
//          "source": "/dev/mapper/vg0-home",
//          "fstype": "btrfs",
//          "options": "rw,relatime,seclabel,ssd,space_cache=v2,subvolid=5,subvol=/"
//       }
//    ]
// }
// """
