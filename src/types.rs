use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq)]
pub struct Vault {
    pub path: PathBuf,
    pub volumes: Vec<Volume>,
    pub timestamp_format: String,
    pub retention_policy: Vec<RetentionTier>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Volume {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RetentionTier {
    pub keep_every: u64, //time in seconds
    pub for_duration: Option<u64>,
    pub name: Option<String>,
    pub max_snapshots: Option<i32>,
}
