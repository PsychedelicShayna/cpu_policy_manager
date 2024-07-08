use anyhow as ah;
use std::env::{self};

use crate::{frequency::Frequency, policies::PolicyDir, CPU_FREQ_PATH};

pub fn parse_freq_value(freq_str: &str) -> ah::Result<(Option<Frequency>, Option<Frequency>)> {
    let freq_str = freq_str.replace(',', "");
    let parts: Vec<&str> = freq_str.split(':').collect();

    if parts.len() < 2 {
        ah::bail!("Invalid frequency value provided. Must be in the format of <min>:<max>, :<max>, or <min>:");
    }

    let min_freq = parts[0].to_string();
    let mut min_freq_val: Option<Frequency> = None;

    if !min_freq.is_empty() {
        let mut suffix: Option<char> = min_freq.chars().last();

        if let Some(s) = suffix {
            if !s.is_alphabetic() {
                suffix = None;
            }
        }

        match suffix {
            Some('g') => min_freq_val = Some(Frequency::GHz(min_freq.parse::<f64>()?)),
            Some('m') => min_freq_val = Some(Frequency::MHz(min_freq.parse::<u64>()?)),
            Some('k') => min_freq_val = Some(Frequency::KHz(min_freq.parse::<u64>()?)),
            Some('h') => min_freq_val = Some(Frequency::Hz(min_freq.parse::<u64>()?)),
            Some(_) => ah::bail!(
                "Invalid suffix provided for frequency value.\nValid suffixes are: g, m, k, h"
            ),
            None if min_freq.contains('.') => {
                min_freq_val = Some(Frequency::GHz(min_freq.parse::<f64>()?))
            }
            None => min_freq_val = Some(Frequency::KHz(min_freq.parse::<u64>()?)),
        }
    }

    let max_freq = parts[1].to_string();
    let mut max_freq_val: Option<Frequency> = None;

    if !max_freq.is_empty() {
        let mut suffix: Option<char> = max_freq.chars().last();

        if let Some(s) = suffix {
            if !s.is_alphabetic() {
                suffix = None;
            }
        }

        match suffix {
            Some('g') => max_freq_val = Some(Frequency::GHz(max_freq.parse::<f64>()?)),
            Some('m') => max_freq_val = Some(Frequency::MHz(max_freq.parse::<u64>()?)),
            Some('k') => max_freq_val = Some(Frequency::KHz(max_freq.parse::<u64>()?)),
            Some('h') => max_freq_val = Some(Frequency::Hz(max_freq.parse::<u64>()?)),
            Some(_) => ah::bail!(
                "Invalid suffix provided for frequency value.\nValid suffixes are: g, m, k, h"
            ),
            None if max_freq.contains('.') => {
                max_freq_val = Some(Frequency::GHz(max_freq.parse::<f64>()?))
            }
            None => max_freq_val = Some(Frequency::KHz(max_freq.parse::<u64>()?)),
        }
    }

    Ok((min_freq_val, max_freq_val))
}

pub fn op_set(
    policy_dirs: Vec<PolicyDir>,
    args: &mut std::vec::IntoIter<String>,
) -> ah::Result<()> {
    let target_policy = args
        .next()
        .ok_or(ah::anyhow!("No policy specified to set."))?;

    let target_attrib = args
        .next()
        .ok_or(ah::anyhow!("No target specified to set."))?;

    let target_value = args
        .next()
        .ok_or(ah::anyhow!("No value specified to set."))?;

    let mut policy_dir_numbers: Vec<u32> = Vec::new();

    // Collect the policy directory numbers identifiers that we're targeting.
    match target_policy.as_str() {
        "*" | "all" => policy_dir_numbers = policy_dirs.iter().map(|pd| pd.policy_number).collect(),

        num_str if num_str.contains(':') => {
            let parts: Vec<&str> = num_str.split(':').collect();

            if parts.len() < 2 {
                ah::bail!("Invalid policy identifier provided.");
            }

            let start = parts[0].parse::<u32>()?;
            let end = parts[1].parse::<u32>()?;

            if end > start {
                ah::bail!("The end policy number must be greater than the start policy number.");
            }

            policy_dir_numbers = (start..=end).collect();
        }

        num_str if num_str.contains(',') => {
            let parts: Vec<&str> = num_str.split(',').collect();

            for part in parts {
                let num = part.parse::<u32>()?;
                policy_dir_numbers.push(num);
            }
        }

        num_str => policy_dir_numbers.push(
            num_str
                .parse::<u32>()
                .map_err(|_| ah::anyhow!("Invalid policy identifier provided."))?,
        ),
    };

    let target_policy_dirs: Vec<PolicyDir> = policy_dirs
        .into_iter()
        .filter(|pd| policy_dir_numbers.contains(&pd.policy_number))
        .collect();

    for policy_dir in target_policy_dirs {
        match target_attrib.to_lowercase().as_str() {
            "freq" => {
                let frequency = parse_freq_value(&target_value)?;
                let (min, max) = frequency;

                if let (None, None) = (min, max) {
                    ah::bail!("No frequency values provided.");
                }

                if let Some(min) = min {
                    policy_dir.set_scaling_min_freq(&min)?;
                }

                if let Some(max) = max {
                    policy_dir.set_scaling_max_freq(&max)?;
                }
            }
            "gov" => {
                let available_govs = policy_dir.read_available_governors()?;
                let target_gov = target_value.to_lowercase();

                if !available_govs.contains(&target_gov) {
                    ah::bail!(
                        "The governor '{}' is not available for policy {}.",
                        target_gov,
                        policy_dir.policy_number
                    );
                }

                policy_dir.set_governor(&target_gov)?;
            }
            "perf" => {
                let available_profiles = policy_dir.read_available_perf_profiles()?;
                let target_profile = target_value.to_lowercase();

                if !available_profiles.contains(&target_profile) {
                    ah::bail!(
                        "The performance profile '{}' is not available for policy {}.",
                        target_profile,
                        policy_dir.policy_number
                    );
                }

                policy_dir.set_perf_profile(&target_profile)?;
            }
            _ => (),
        }
    }

    Ok(())
}

pub fn op_get(
    policy_dirs: Vec<PolicyDir>,
    args: &mut std::vec::IntoIter<String>,
) -> ah::Result<()> {
    let target_policy = args.next().ok_or(ah::anyhow!("No policies specified."))?;

    let target_attrib = args.next().ok_or(ah::anyhow!("No attribute specified."))?;

    let target_value = args.next().ok_or(ah::anyhow!("No value specified."))?;

    let mut policy_dir_numbers: Vec<u32> = Vec::new();

    // Collect the policy directory numbers identifiers that we're targeting.
    match target_policy.as_str() {
        "*" | "all" => policy_dir_numbers = policy_dirs.iter().map(|pd| pd.policy_number).collect(),

        num_str if num_str.contains(':') => {
            let parts: Vec<&str> = num_str.split(':').collect();

            if parts.len() < 2 {
                ah::bail!("Invalid policy identifier provided.");
            }

            let start = parts[0].parse::<u32>()?;
            let end = parts[1].parse::<u32>()?;

            if end > start {
                ah::bail!("The end policy number must be greater than the start policy number.");
            }

            policy_dir_numbers = (start..=end).collect();
        }

        num_str if num_str.contains(',') => {
            let parts: Vec<&str> = num_str.split(',').collect();

            for part in parts {
                let num = part.parse::<u32>()?;
                policy_dir_numbers.push(num);
            }
        }

        num_str => policy_dir_numbers.push(
            num_str
                .parse::<u32>()
                .map_err(|_| ah::anyhow!("Invalid policy identifier provided."))?,
        ),
    };

    let target_policy_dirs: Vec<PolicyDir> = policy_dirs
        .into_iter()
        .filter(|pd| policy_dir_numbers.contains(&pd.policy_number))
        .collect();

    let mut output: Vec<String> = Vec::new();

    for (i, policy_dir) in target_policy_dirs.iter().enumerate() {
        if i > 0 {
            output.push("--------------------".to_string());
        }

        match (
            target_attrib.to_lowercase().as_str(),
            target_value.to_lowercase().as_str(),
        ) {
            ("freq", "min") => {
                let min_freq = policy_dir.read_scaling_min_freq()?;

                output.push(format!(
                    "Policy {} scaling min frequency: {}",
                    policy_dir.policy_number, min_freq
                ));
            }

            ("freq", "max") => {
                let max_freq = policy_dir.read_scaling_max_freq()?;

                output.push(format!(
                    "Policy {} scaling max frequency: {}",
                    policy_dir.policy_number, max_freq
                ));
            }

            ("freq", "current" | "curr") => {
                let current_freq = policy_dir.read_current_freq()?;

                output.push(format!(
                    "Policy {} current frequency: {}",
                    policy_dir.policy_number, current_freq
                ));
            }

            ("gov", "avail" | "available") => {
                let available_govs = policy_dir.read_available_governors()?;
                let mut govs = format!("Policy {} available governors...\n\n", policy_dir.policy_number);

                for (i, gov) in available_govs.iter().enumerate() {
                    govs += &format!("{}.) - {}\n", i, gov);
                }

                output.push(govs);
            }

            ("gov", "curr" | "current") => {
                let current_gov = policy_dir.read_current_governor()?;
                output.push(format!(
                    "Policy {} current governor: {}",
                    policy_dir.policy_number, current_gov
                ));
            }

            ("perf", "avail" | "available") => {
                let available_profiles = policy_dir.read_available_perf_profiles()?;
                let mut perfs = format!("Policy {} available performance profiles...\n\n", policy_dir.policy_number);

                for (i, perf) in available_profiles.iter().enumerate() {
                    perfs += &format!("{}.) - {}\n", i, perf);
                }

                output.push(perfs);
            }

            ("perf", "curr" | "current") => {
                let current_perf = policy_dir.read_current_perf_profile()?;
                output.push(format!(
                    "Policy {} current performance profile: {}",
                    policy_dir.policy_number, current_perf
                ));
            }
            _ => (),
        }
    }

    for line in output {
        println!("{}", line);
    }

    Ok(())
}

pub fn parse_arguments() -> ah::Result<()> {
    let arguments: Vec<String> = env::args().collect();
    let mut arg_iter: std::vec::IntoIter<String> = arguments.into_iter();
    let policy_dirs = PolicyDir::collect_from_dir(CPU_FREQ_PATH)?;

    // Ignore the first argument, since it's the path to the binary.
    arg_iter.next();

    let first = arg_iter
        .next()
        .ok_or(ah::anyhow!("No arguments provided."))?;

    match first.as_str() {
        "set" => op_set(policy_dirs, &mut arg_iter)?,
        "get" => op_get(policy_dirs, &mut arg_iter)?,
        a => println!("Unrecognized: {}", a)
    };

    Ok(())
}
