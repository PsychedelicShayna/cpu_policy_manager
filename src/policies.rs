use crate::{frequency::Frequency, CPU_FREQ_PATH};

use strum_macros::AsRefStr;

use anyhow as ah;
use std::fs::{self, read_dir, DirEntry};

use std::path::{Path, PathBuf};

/// Enum representing the policy files present in a policy directory, e.g.
/// policy0/scaling_max_freq, or policy1/scaling_governor. These are used by
/// the PolicyDir struct to specify what policy file to read or alter.

// The #[allow(non_camel_case_types)] attribute is used to suppress the warning
// that the enum variants are not camel case. This is because the variants are
// named after the policy files, and we're converting them to strings / paths.

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, AsRefStr)]
pub enum PolicyFile {
    affected_cpus,
    base_frequency,
    cpuinfo_max_freq,
    cpuinfo_min_freq,
    cpuinfo_transition_latency,
    energy_performance_available_preferences,
    energy_performance_preference,
    related_cpus,
    scaling_available_governors,
    scaling_cur_freq,
    scaling_driver,
    scaling_governor,
    scaling_max_freq,
    scaling_min_freq,
    scaling_setspeed,
}

pub struct PolicyDir {
    pub full_path: PathBuf,
    pub dir_name: String,
    pub policy_number: u32,
}

/// Generates methods for reading frequency values in KHz from PolicyFiles,
/// and converts them to Frequency enum variants.

macro_rules! generate_frequency_readers {
    ($($method_name:ident, $file:ident)+) => {
        $(
            pub fn $method_name(&self) -> ah::Result<Frequency> {
                let content = self.read(PolicyFile::$file)?;
                let parsed_num = content.trim().parse::<u64>()?;
                let frequency = Frequency::KHz(parsed_num);
                Ok(frequency)
            }
        )+
    };
}

impl PolicyDir {
    /// Creates a new PolicyDir struct from a path pointing to a policy directory.
    /// The policy directory name is expected to be in the format "policyN", where
    /// N is the number associated with the CPU core the policy governs. This
    /// function makes no garantuees about the presence of the expected policy
    /// files. Just because the function returns a PolicyDir struct doesn't mean
    /// the policy directory contains the expected policy files files.

    pub fn from(path: &str) -> ah::Result<Self> {
        let path_str = path;
        let path = Path::new(&path_str);

        if !path.exists() {
            ah::bail!("The provided path '{}' doesn't exist.", path.display());
        }

        if !path.is_dir() {
            ah::bail!("The provided path '{}' wasn't a directory.", path.display());
        }

        let full_path = path.to_path_buf();

        let dir_name = path
            .file_name()
            .ok_or(ah::anyhow!(
                "The directory name of path '{}' could not be retrieved.",
                path.display()
            ))?
            .to_string_lossy()
            .to_string();

        let name_trimmed = dir_name.trim_start_matches("policy");

        let policy_number = name_trimmed
            .chars()
            .all(|c| c.is_digit(10))
            .then(|| name_trimmed.parse::<u32>())
            .ok_or(ah::anyhow!(
                "The policy number '{:?}' couldn't be parsed as a u32.",
                name_trimmed
            ))??;

        Ok(Self {
            full_path,
            dir_name,
            policy_number,
        })
    }

    /// Collects all policy directories from the provided path.
    pub fn collect_from_dir(path: &str) -> ah::Result<Vec<Self>> {
        let path = Path::new(path);

        if !path.exists() {
            ah::bail!("The provided path '{}' doesn't exist.", path.display());
        }

        if !path.is_dir() {
            ah::bail!("The provided path '{}' wasn't a directory.", path.display());
        }

        let entries = read_dir(path)?.collect::<Result<Vec<DirEntry>, std::io::Error>>()?;

        let policy_dir_paths: Vec<String> = entries
            .into_iter()
            .filter_map(|entry| {
                let entry_path = entry.path();
                let entry_name = entry_path.file_name()?.to_string_lossy().to_string();

                if !entry_name.starts_with("policy") {
                    return None;
                }

                if !entry_name
                    .trim_start_matches("policy")
                    .chars()
                    .all(|c| c.is_digit(10))
                {
                    return None;
                }

                Some(entry_path.to_string_lossy().to_string())
            })
            .collect();

        policy_dir_paths
            .into_iter()
            .map(|path| Self::from(&path))
            .collect::<Result<Vec<Self>, _>>()
    }

    /// Returns the available governors from the scaling_available_governors file.
    pub fn read_available_governors(&self) -> ah::Result<Vec<String>> {
        let content = self.read(PolicyFile::scaling_available_governors)?;
        let governors: Vec<String> = content.split_whitespace().map(|s| s.to_string()).collect();

        if governors.is_empty() {
            ah::bail!(
                "The list of governors was empty for policy: {}",
                self.full_path.display()
            );
        }

        Ok(governors)
    }

    /// Sets the scaling_governor to the desired governor, if the governor is
    /// in the list of available governors given by scaling_available_governors.
    pub fn set_governor(&self, governor: &str) -> ah::Result<()> {
        let available = self.read_available_governors()?;

        if !available.contains(&governor.to_string()) {
            ah::bail!(
                "The governor '{}' isn't supported.\nSupported governors: {:?}'",
                governor,
                available
            );
        }

        self.write(PolicyFile::scaling_governor, governor)
    }

    /// Returns the current governor from the scaling_governor file.
    pub fn read_current_governor(&self) -> ah::Result<String> {
        self.read(PolicyFile::scaling_governor)
    }

    /// Returns the available governors from the scaling_available_governors file.
    pub fn read_available_perf_profiles(&self) -> ah::Result<Vec<String>> {
        let content = self.read(PolicyFile::energy_performance_available_preferences)?;

        let perf_profiles: Vec<String> =
            content.split_whitespace().map(|s| s.to_string()).collect();

        if perf_profiles.is_empty() {
            ah::bail!(
                "The list of performance profiles was empty for policy: {}",
                self.full_path.display()
            );
        }

        Ok(perf_profiles)
    }

    /// Sets the performance profile to the desired profile, if the profile is
    /// in the list of available power profiles given by read_available_perf_profiles.
    pub fn set_perf_profile(&self, profile: &str) -> ah::Result<()> {
        let available = self.read_available_perf_profiles()?;

        if !available.contains(&profile.to_string()) {
            ah::bail!(
                "The performance profile '{}' can't be found.\nAvailable profiles: {:?}",
                profile,
                available
            );
        }

        self.write(PolicyFile::energy_performance_preference, profile)
    }

    /// Returns the current performance profile from the energy_performance_preference file.
    pub fn read_current_perf_profile(&self) -> ah::Result<String> {
        self.read(PolicyFile::energy_performance_preference)
    }

    // Boilerplate reduction; the parsing and conversion logic is identical.
    generate_frequency_readers!(
        read_base_frequency,   base_frequency
        read_rated_max_freq,   cpuinfo_max_freq
        read_rated_min_freq,   cpuinfo_min_freq
        read_current_freq,     scaling_cur_freq
        read_scaling_max_freq, scaling_max_freq
        read_scaling_min_freq, scaling_min_freq
    );

    /// Sets the scaling_max_freq to the desired frequency, if the desired
    /// frequency falls within the rated min and max rated frequencies.
    pub fn set_scaling_max_freq(&self, frequency: &Frequency) -> ah::Result<()> {
        let desired_scaling_max = frequency.to_khz();

        // Desired scaling_max frequency cannot be lower than the scaling_min.
        let scaling_min = self.read_scaling_min_freq()?.to_khz();

        if u64::from(desired_scaling_max) < u64::from(scaling_min) {
            ah::bail!(
                "The desired scaling_max frequency '{}' is lower than the current scaling_min '{}'",
                desired_scaling_max.to_ghz(),
                scaling_min.to_ghz()
            );
        }

        // Desired scaling_max frequency cannot be higher than the rated max.
        let max_rated = self.read_rated_max_freq()?.to_khz();

        if u64::from(desired_scaling_max) > u64::from(max_rated) {
            ah::bail!(
                "The desired scaling_max frequency '{}' is higher than the maximum rated frequency '{}'",
                desired_scaling_max.to_ghz(),
                max_rated.to_ghz()
            );
        }

        // Desired scaling_max frequency cannot be lower than the rated min.
        let min_rated = self.read_rated_min_freq()?.to_khz();

        if u64::from(desired_scaling_max) < u64::from(min_rated) {
            ah::bail!(
                "The desired scaling_max frequency '{}' is lower than the minimum rated frequency '{}'",
                desired_scaling_max.to_ghz(),
                min_rated.to_ghz()
            );
        }

        self.write(
            PolicyFile::scaling_max_freq,
            &desired_scaling_max.to_khz().to_string_u64(),
        )?;

        Ok(())
    }

    /// Sets the scaling_min_freq to the desired frequency, if the desired
    /// frequency falls within the rated min and max rated frequencies.
    pub fn set_scaling_min_freq(&self, frequency: &Frequency) -> ah::Result<()> {
        let desired_scaling_min = frequency.to_khz();

        // Desired scaling_min frequency cannot be higher than the scaling_max.
        let scaling_max = self.read_scaling_max_freq()?.to_khz();

        if u64::from(desired_scaling_min) > u64::from(scaling_max) {
            ah::bail!(
                "The desired scaling_min frequency '{}' is higher than the current scaling_max '{}'",
                desired_scaling_min.to_ghz(),
                scaling_max.to_ghz()
            );
        }

        // Desired scaling_min frequency cannot be lower than the rated min.
        let min_rated = self.read_rated_min_freq()?.to_khz();

        if u64::from(desired_scaling_min) < u64::from(min_rated) {
            ah::bail!(
                "The desired scaling_min frequency '{}' is higher than the minimum rated frequency '{}'",
                desired_scaling_min.to_ghz(),
                min_rated.to_ghz()
            );
        }

        // Desired scaling_min frequency cannot be higher than the rated max.
        let max_rated = self.read_rated_max_freq()?.to_khz();

        if u64::from(desired_scaling_min) > u64::from(max_rated) {
            ah::bail!(
                "The desired scaling_min frequency '{}' is higher than the maximum rated frequency '{}'",
                desired_scaling_min.to_ghz(),
                max_rated.to_ghz()
            );
        }

        self.write(
            PolicyFile::scaling_min_freq,
            &desired_scaling_min.to_khz().to_string_u64(),
        )?;

        Ok(())
    }

    /// Generic method for reading from a policy file.
    pub fn read(&self, policy_file: PolicyFile) -> ah::Result<String> {
        let file_name = policy_file.as_ref();
        let path = self.full_path.join(file_name);

        if !path.is_file() {
            return Err(ah::anyhow!(
                "The policy file '{}' doesn't exist. Couldn't read from it.",
                path.display()
            ));
        }

        std::fs::read_to_string(&path).map_err(|e| {
            ah::anyhow!(
                "Coudln't read policy file '{}' due to error '{}'",
                path.display(),
                e
            )
        })
    }

    /// Generic method for writing to a policy file.
    pub fn write(&self, policy_file: PolicyFile, contents: &str) -> ah::Result<()> {
        let file_name = policy_file.as_ref();
        let path = self.full_path.join(file_name);

        if !path.is_file() {
            return Err(ah::anyhow!(
                "The policy file '{}' doesn't exist. Couldn't write to it.",
                path.display()
            ));
        }

        fs::write(&path, contents).map_err(|e| {
            ah::anyhow!(
                "Couldn't write to policy file '{}' due to error '{}'",
                path.display(),
                e
            )
        })
    }

    /// Return paths to each policy0, policy1, policy2.. directory in CPU_FREQ_PATH.
    pub fn get_policy_dirs() -> ah::Result<Vec<(String, String)>> {
        let mut policy_dirs = Vec::<(String, String)>::new();

        for entry in read_dir(CPU_FREQ_PATH)? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            let dir_name = match path.file_name() {
                Some(name) => name.to_string_lossy().to_string(),
                None => continue,
            };

            if !dir_name.starts_with("policy") {
                continue;
            }

            policy_dirs.push((dir_name, path.to_string_lossy().to_string()));
        }

        if policy_dirs.is_empty() {
            return Err(ah::anyhow!("No policy folders found!"));
        }

        Ok(policy_dirs)
    }

    pub fn get_policy_files(policy_dir: &str) -> ah::Result<Vec<(String, String)>> {
        let mut policy_files = Vec::<(String, String)>::new();

        for entry in read_dir(policy_dir)? {
            let entry = entry?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            let file_name = match path.file_name() {
                Some(name) => name.to_string_lossy().to_string(),
                None => continue,
            };

            policy_files.push((file_name, path.to_string_lossy().to_string()));
        }

        if policy_files.is_empty() {
            return Err(ah::anyhow!("No policy files found!"));
        }

        Ok(policy_files)
    }
}
