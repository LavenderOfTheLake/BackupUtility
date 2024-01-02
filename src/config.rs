use crate::types::{RetentionTier, Vault, Volume};
use std::{path::PathBuf, str::FromStr};
use yaml_rust::yaml::Yaml;

//todo:
//write functions to see if RetentionTier rules are shadowed.

pub fn read_config_from_file(path: &std::path::Path) -> Result<Vec<Vault>, &'static str> {
    let text = std::fs::read_to_string(path).map_err(|_| "Couldn't read file")?;
    return read_config(&text);
}

pub fn read_config(text: &str) -> Result<Vec<Vault>, &'static str> {
    let key_retention_policy: &Yaml = &Yaml::String(String::from("Retention Policy"));
    let key_volumes = &Yaml::String(String::from("Volumes"));
    let key_name: &Yaml = &Yaml::String(String::from("name"));
    let key_path: &Yaml = &Yaml::String(String::from("path"));
    let key_timestamp_format: &Yaml = &Yaml::String(String::from("Timestamp Format"));

    let y = yaml_rust::yaml::YamlLoader::load_from_str(text)
        .map_err(|_| "YamlLoader failed to load from string")?;
    let y = y
        .get(0)
        .ok_or("YamlLoader failed to load from string")?
        .as_hash()
        .ok_or("The root of the config file must be a hash")?;

    let mut vaults = Vec::new();

    for (vault_name, vault_config) in y {
        //setup - each vault should be a hash
        let vault_config = vault_config.as_hash().ok_or("Vaults must be hashes")?;

        //1 - get the vault path
        let vault_name = vault_name.as_str().ok_or("Vault names must be paths")?;
        let vault_name = PathBuf::from_str(vault_name)
            .map_err(|_| "PathBuf::fromstr failed to parse vault path {vaultname}")?;

        //2 - parse vault retention policy
        let retention_policy = match vault_config.get(key_retention_policy) {
            Some(retention_policy) => parse_retention_policy(&retention_policy)?,
            None => Vec::new(),
        }
        .to_vec();

        //3 - parse volumes
        let volumes_yaml = vault_config
            .get(key_volumes)
            .ok_or("Couln't get `Volumes` key for vault {vault_name}")?
            .as_vec()
            .ok_or("`Volumes` must be a list")?;
        let mut volumes = Vec::new();
        for volume in volumes_yaml {
            let volume = volume.as_hash().ok_or("Each Volume must be a hash")?;
            volumes.push(Volume {
                name: volume
                    .get(key_name)
                    .ok_or("Each Volume must have a `name`")?
                    .as_str()
                    .ok_or("Volume names must be strings")?
                    .to_owned(),
                path: volume
                    .get(key_path)
                    .ok_or("Each Volume must have a `path`")?
                    .as_str()
                    .ok_or("Volume names must be strings")?
                    .into(),
            });
        }

        //4 - parse timestamp format
        let timestamp_format = match vault_config.get(key_timestamp_format) {
            Some(x) => x
                .as_str()
                .ok_or("`Timestamp Format` must be a string")?
                .to_owned(),
            None => "%F".to_owned(),
        };

        vaults.push(Vault {
            path: vault_name,
            volumes,
            timestamp_format,
            retention_policy,
        })
    }

    return Ok(vaults);
}

fn parse_retention_policy(retention_policy: &Yaml) -> Result<Vec<RetentionTier>, &'static str> {
    let retention_policy_yaml = retention_policy
        .as_vec()
        .ok_or("If a Retention Policy is defined, it must be a *list* of Retention Tiers.")?;
    let mut retention_policy = Vec::new();

    for retention_tier in retention_policy_yaml {
        retention_policy.push(parse_retention_tier(retention_tier)?);
    }

    Ok(retention_policy.clone())
}

fn parse_retention_tier(retention_tier: &Yaml) -> Result<RetentionTier, &'static str> {
    let key_for: &Yaml = &Yaml::String(String::from("for"));
    let key_keep_every: &Yaml = &Yaml::String(String::from("keep every"));
    let key_name: &Yaml = &Yaml::String(String::from("name"));
    let key_max_snapshots: &Yaml = &Yaml::String(String::from("max snapshots"));

    let retention_tier = retention_tier
        .as_hash()
        .ok_or("Retention Tiers must be hashes")?;

    let for_duration = match retention_tier.get(key_for) {
        Some(duration) => Some(
            parse_duration::parse(
                duration
                    .as_str()
                    .ok_or("`for:` must contain a duration, in the form of a string.")?,
            )
            .map_err(|_| "Invalid Duration: {duration.as_str().unwrap()}")?
            .as_secs(),
        ),
        None => None,
    };

    let keep_every = match retention_tier.get(key_keep_every) {
        Some(duration) => Some(
            parse_duration::parse(
                duration
                    .as_str()
                    .ok_or("`keep every:` must contain a duration, in the form of a string.")?,
            )
            .map_err(|_| "Invalid Duration: {duration.as_str().unwrap()}")?
            .as_secs(),
        ),
        None => None,
    };

    let max_snapshots = match retention_tier.get(key_max_snapshots) {
        Some(quantity) => Some(
            quantity
                .as_i64()
                .ok_or("`max_snapshots:` must contain a positive integer, without quotes.")?
                as i32,
        ),
        None => None,
    };
    let name = match retention_tier.get(key_name) {
        Some(name) => Some(name.as_str().ok_or("`name:` must be a string.")?.to_owned()),
        None => None,
    };

    Ok(RetentionTier {
        keep_every,
        for_duration,
        max_snapshots,
        name,
    })
}

mod tests {

    #[test]
    fn test() {
        use crate::types::Vault;
        use crate::types::{RetentionTier, Volume};

        let y = super::read_config_from_file(std::path::Path::new("test_config.yaml"))
            .expect("Can't read the file");

        assert!(y.len() == 1);

        assert_eq!(
            y[0],
            Vault {
                path: "/media/backup/snapshots".into(),
                timestamp_format: "%F".into(),
                retention_policy: vec![
                    RetentionTier {
                        name: Some("Hourly".into()),
                        keep_every: Some(3600),
                        max_snapshots: Some(30),
                        for_duration: None,
                    },
                    RetentionTier {
                        name: Some("Daily".into()),
                        keep_every: Some(86400),
                        max_snapshots: None,
                        for_duration: Some(2629746),
                    }
                ],
                volumes: vec![
                    Volume {
                        name: "@home".into(),
                        path: "/home".into()
                    },
                    Volume {
                        name: "@root".into(),
                        path: "/".into()
                    },
                ]
            }
        )
    }
}
