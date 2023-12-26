use std::path::PathBuf;

pub struct Vault<'a> {
    pub path: PathBuf,
    pub volumes: Vec<&'a Volume>,
    pub snapshots: Vec<&'a Snapshot<'a>>,
    pub retain_number: usize,
}

pub struct Volume {
    pub name: String,
    pub path: PathBuf,
}

pub struct Snapshot<'a> {
    pub vault: &'a Vault<'a>,
    pub time: chrono::NaiveDateTime,
}