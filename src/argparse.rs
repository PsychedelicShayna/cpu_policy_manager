use anyhow as ah;
use std::env::{self};

use crate::{
    frequency::{Frequency},
    policies::PolicyDir,
    CPU_FREQ_PATH,
};

// For frequencies
// ----------------------------------------------------------------------------------------------------
//
// Ok this is the hard part, coming up with a intuitive and easy way for the
// user to interact with the program.. and then implementing it.

// cpm set <policy_n> <attribute> <value>
// format of value determined by attribute
//
// The policy_n can be replaced with * or "all" to perform the operation
// across every policy file
//
// : used as delim for min/max, if no value present on one side
// only set the value of the other
//
// cpm set 0 freq :2.5
// cpm set 0 freq 1.5:2.5
//
// If a floating point value is given, GHz is assumed
// g,k,m,h = last char present used to determine format manually
// cpm set 0 freq 1.5g:4,500,000k
//
// cpm set 0 freq 1,500,000k:4500m (1.5 GHz to 4.5 GHz)
// identical to below beause , is ignored
// cpm set 0 freq 1500000k:4,500m (1.5 GHz to 4.5 GHz)
//
//
// For frequencies, a format can be provided (ghz, mhz, etc)
// but if none is, then the format is inferred.
//
// if comas are present, they will be stripped
// and format will default to khz.
//
// If a format is provided that is incompatible
// with the value, e.g. 2.5 khz, count as error.
//
// To get values
//
// cpm get 0 freq
// If no value is provided, default to current
// cpm get 0 freq current
//
// Scaling min/max
// cpm get 0 freq smin
// cpm get 0 freq smax
//
// Rated min/max
// cpm get 0 freq rmin
// cpm get 0 freq rmax
//

//----------------------------------------------------------------------------------------------------
// For governors
// cpm set 0 gov performance
//
// Aliases allowedd
// cpm get 0 gov available, avail
// cpm get 0 gov current, curr
//
// Providing none defaults to: current
// cpm get 0 gov
//
//
//----------------------------------------------------------------------------------------------------
// For performance profiles
//
// cpm set 0 perf profile_name
// cpm get 0 perf available, avail
// cpm get 0 perf current, curr

// Providing none defaults to: current
// cpm get 0 perf
//
// ----------------------------------------------------------------------------------------------------
// For printing statistics
//
//  What do we want to list?
//
//  summary of all active policy values for a specific policy
//  summary of all active policy values for all policies
//
//  cpm list 0
//  cpm list all
//
//  Output should be nicely formatted as a table.

// Now how do we parse the input and direct it to appropriate functions..

pub fn parse_policy_ident(_ident: &str) {

    // could be 0:4
    // could be 4,2,6
    // could be 6
    // could be * or 'all'
}

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

        num_str => policy_dir_numbers.push(num_str.parse::<u32>()?),

        _ => ah::bail!("Invalid policy identifier provided."),
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
                let mut govs = "Policy {} available governors...\n\n".to_string();

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
                let mut perfs = "Policy {} available performance profiles...\n\n".to_string();

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

    Ok(())
}

pub fn parse_arguments() -> ah::Result<()> {
    let arguments: Vec<String> = env::args().collect();
    let mut arg_iter: std::vec::IntoIter<String> = arguments.into_iter();
    let policy_dirs = PolicyDir::collect_from_dir(CPU_FREQ_PATH)?;

    let first = arg_iter
        .next()
        .ok_or(ah::anyhow!("No arguments provided."))?;

    match first.as_str() {
        "set" => op_set(policy_dirs, &mut arg_iter)?,
        "get" => op_get(policy_dirs, &mut arg_iter)?,
        _ => (),
    };

    Ok(())
}
//
// let parts: Vec<String> = arg_iter.take(2).collect();
//
// let action = parts.first().ok_or(ah::anyhow!(
//     "Index 0 of operation parts was empty; missing an action."
// ))?;
//
// let policy = parts.get(1).ok_or(ah::anyhow!(
//     "Index 1 of operation parts was empty; missing a target policy."
// ))?;
//
// match action.to_lowercase().as_str() {
//     pat @ ("set" | "get") => {
//         let target = parts.get(3).ok_or(ah::anyhow!(
//             "Missing third argument specifying the attribute."
//         ))?;
//
//         match target.as_str() {
//             "freq" if pat == "set" => {
//                 let desired = parts.get(4).ok_or(ah::anyhow!(
//                     "Missing fourth argument specifying the desired value."
//                 ))?;
//
//
//
//                 // set_freq_operation(PolicyDir::from(&policy)?, desired.to_string());
//             }
//             "gov" => (),
//             "perf" => (),
//             _ => (),
//         }
//     }
//
//     "get" => {
//         let target = parts.get(3).ok_or(ah::anyhow!(
//             "Missing third argument indicating what to get."
//         ))?;
//
//         match target.as_str() {
//             "freq" => (),
//             "gov" => (),
//             "perf" => (),
//             _ => (),
//         }
//     }
//
//     _ => {}
// }
//
