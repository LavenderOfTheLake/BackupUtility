use crate::types::{RetentionTier, Vault, Volume};
use std::{
    path::{self, PathBuf},
    str::FromStr,
};
use yaml_rust::yaml::{Yaml, YamlLoader};

pub fn read_config_from_file(path: &std::path::Path) -> Option<Vec<Vault>> {
    let text = std::fs::read_to_string(path).ok()?;
    return read_config(&text);
}

pub fn read_config(text: &str) -> Option<Vec<Vault>> {
    let key_retention_policy: &Yaml = &Yaml::String(String::from("Retention Policy"));
    let key_volumes = &Yaml::String(String::from("Volumes"));
    let key_name: &Yaml = &Yaml::String(String::from("name"));
    let key_path: &Yaml = &Yaml::String(String::from("path"));
    let key_timestamp_format: &Yaml = &Yaml::String(String::from("Timestamp Format"));

    let y = YamlLoader::load_from_str(text).ok()?;
    let y = y.get(0)?.as_hash()?;

    let mut vaults = Vec::new();

    for (vault_name, vault_config) in y {
        //setup - each vault should be a hash
        let vault_config = vault_config.as_hash()?;

        //1 - get the vault path
        let vault_name = vault_name.as_str()?;
        let vault_name = PathBuf::from_str(vault_name).ok()?;

        //2 - parse vault retention policy
        let retention_policy = match vault_config.get(key_retention_policy) {
            Some(retention_policy) => parse_retention_policy(retention_policy)?,
            None => Vec::new(),
        };

        //3 - parse volumes
        let volumes_yaml = vault_config.get(key_volumes)?.as_vec()?;
        let mut volumes = Vec::new();
        for volume in volumes_yaml {
            let volume = volume.as_hash()?;
            volumes.push(Volume {
                name: volume.get(key_name)?.as_str()?.to_owned(),
                path: volume.get(key_path)?.as_str()?.into(),
            });
        }

        //4 - parse timestamp format
        let timestamp_format = match vault_config.get(key_timestamp_format) {
            Some(x) => x.as_str()?.to_owned(),
            None => "%F".to_owned(),
        };

        vaults.push(Vault {
            path: vault_name,
            volumes,
            timestamp_format,
            retention_policy,
        })
    }

    return Some(vaults);
}

fn parse_retention_policy(retention_policy: &Yaml) -> Option<Vec<RetentionTier>> {
    let retention_policy_yaml = retention_policy.as_vec()?;
    let mut retention_policy = Vec::new();

    for retention_tier in retention_policy_yaml {
        retention_policy.push(parse_retention_tier(retention_tier)?);
    }

    Some(retention_policy)
}

fn parse_retention_tier(retention_tier: &Yaml) -> Option<RetentionTier> {
    let key_for: &Yaml = &Yaml::String(String::from("for"));
    let key_keep_every: &Yaml = &Yaml::String(String::from("keep every"));
    let key_name: &Yaml = &Yaml::String(String::from("name"));
    let key_max_snapshots: &Yaml = &Yaml::String(String::from("max snapshots"));

    let retention_tier = retention_tier.as_hash()?;

    let for_duration = match retention_tier.get(key_for) {
        Some(duration) => Some(parse_duration::parse(duration.as_str()?).ok()?.as_secs()),
        None => None,
    };

    let keep_every = match retention_tier.get(key_keep_every) {
        Some(duration) => Some(parse_duration::parse(duration.as_str()?).ok()?.as_secs()),
        None => None,
    };

    let max_snapshots = match retention_tier.get(key_max_snapshots) {
        Some(quantity) => Some(quantity.as_i64()? as i32),
        None => None,
    };
    let name = match retention_tier.get(key_name) {
        Some(name) => Some(name.as_str()?.to_owned()),
        None => None,
    };

    Some(RetentionTier {
        keep_every,
        for_duration,
        max_snapshots,
        name,
    })
}

mod tests {
    use yaml_rust::YamlLoader;

    #[test]
    fn test() {
        let y = super::read_config_from_file(std::path::Path::new("test_config.yaml"))
            .expect("Can't read the file");
    }
}
