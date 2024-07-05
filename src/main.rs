use anyhow as ah;



use std::collections::HashMap;
use std::fs::{self, DirEntry, OpenOptions};
use std::io::{BufWriter, Read, Write};
use std::path::Path;

mod macros;

const CPU_FREQ_PATH: &str = "/sys/devices/system/cpu/cpufreq/";

fn collect_policy_paths() -> ah::Result<Vec<String>> {
    Ok(fs::read_dir(CPU_FREQ_PATH)?
        .filter_map(|de| {
            de.ok()
                .and_then(|de| de.path().is_dir().then_some(de))
                .and_then(|de| de.file_name().to_str().map(|ds| ds.to_string()))
                .and_then(|ds| ds.starts_with("policy").then_some(ds))
                .map(|de| format!("{}{}", CPU_FREQ_PATH, de))
        })
        .collect())
}

#[derive(Default)]
struct PolicyAttibutes {
    affected_cpus: String,
    base_frequency: String,
    cpuinfo_max_freq: String,
    cpuinfo_min_freq: String,
    cpuinfo_transition_latency: String,
    energy_performance_available_preferences: String,
    energy_performance_preference: String,
    related_cpus: String,
    scaling_available_governors: String,
    scaling_cur_freq: String,
    scaling_driver: String,
    scaling_governor: String,
    scaling_max_freq: String,
    scaling_min_freq: String,
    scaling_setspeed: String,
}

struct FreqPolicy {
    attributes: PolicyAttibutes,
    directory: String,
}

impl FreqPolicy {
    fn from_policy_path(path: &String) -> ah::Result<FreqPolicy> {
        let path_buf = Path::new(path);

        if !path_buf.exists() {
            ah::bail!("The provided path doesn't exist.");
        }

        if !path_buf.is_dir() {
            ah::bail!("The provided path isn't a directory.");
        }

        let mut policy_attributes = PolicyAttibutes::default();

        let mut policy_map = map!(&str, &mut String {
            "affected_cpus" => &mut policy_attributes.affected_cpus;
            "base_frequency" => &mut policy_attributes.base_frequency;
            "cpuinfo_max_freq" => &mut policy_attributes.cpuinfo_max_freq;
            "cpuinfo_min_freq" => &mut policy_attributes.cpuinfo_min_freq;
            "cpuinfo_transition_latency" => &mut policy_attributes.cpuinfo_transition_latency;
            "energy_performance_available_preferences" => &mut policy_attributes.energy_performance_available_preferences;
            "energy_performance_preference" => &mut policy_attributes.energy_performance_preference;
            "related_cpus" => &mut policy_attributes.related_cpus;
            "scaling_available_governors" => &mut policy_attributes.scaling_available_governors;
            "scaling_cur_freq" => &mut policy_attributes.scaling_cur_freq;
            "scaling_driver" => &mut policy_attributes.scaling_driver;
            "scaling_governor" => &mut policy_attributes.scaling_governor;
            "scaling_max_freq" => &mut policy_attributes.scaling_max_freq;
            "scaling_min_freq" => &mut policy_attributes.scaling_min_freq;
            "scaling_setspeed" => &mut policy_attributes.scaling_setspeed;
        });

        let policy_entries: Vec<DirEntry> = fs::read_dir(path)?
            .filter_map(|de| de.ok().and_then(|de| de.path().is_file().then_some(de)))
            .collect::<Vec<DirEntry>>();

        for &entry in policy_map.keys() {
            if !policy_entries
                .iter()
                .any(|de| de.file_name().to_str().unwrap() == entry)
            {
                ah::bail!(
                    "The file {} is missing from the policy directory {}",
                    entry,
                    path
                );
            }
        }

        let mut buffer = String::new();

        for entry in policy_entries {
            let file_name = entry.file_name();

            let file_name = match file_name.to_str() {
                Some(fname) => fname,
                None => {
                    println!("The file name is not a valid UTF-8 string. {:?}", entry);
                    continue;
                }
            };
            let file_path = entry.path();

            let file_path = match file_path.to_str() {
                Some(fpath) => fpath,
                None => {
                    println!("The file path is not a valid UTF-8 string. {:?}", entry);
                    continue;
                }
            };

            let mut file = OpenOptions::new().read(true).open(entry.path())?;
            file.read_to_string(&mut buffer)?;

            let destination: &mut String = match policy_map.get_mut(file_name) {
                Some(destination) => destination,
                None => {
                    eprintln!(
                        "The file {} is not a valid policy attribute, skipping.",
                        file_path
                    );
                    continue;
                }
            };

            *destination = buffer.clone();
            buffer.clear();
        }

        Ok(FreqPolicy {
            attributes: policy_attributes,
            directory: path.clone(),
        })
    }
}

impl FreqPolicy {
    fn set_energy_performance_preference(&self, value: &str) -> ah::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .open(format!("{}/energy_performance_preference", self.directory))
            .unwrap();

        let mut writer = BufWriter::new(file);

        if let Err(e) = writer.write_all(value.as_bytes()) {
            eprintln!("Error writing to file: {:?}", e);
        }
        Ok(())
    }

    fn set_scaling_governor(&self, value: &str) -> ah::Result<()> {
        let mut available_governors = self.attributes.scaling_available_governors.split_whitespace();

        if !available_governors.any(|g| g == value) {
            ah::bail!("The governor requested ({}) is not available for this policy.\nAvailable governors: {}", value, self.attributes.scaling_available_governors);
        }

        let file = OpenOptions::new()
            .write(true)
            .open(format!("{}/scaling_governor", self.directory))
            .unwrap();

        let mut writer = BufWriter::new(file);

        if let Err(e) = writer.write_all(value.as_bytes()) {
            eprintln!("Error writing to file: {:?}", e);
        }

        Ok(())
    }

    fn set_scaling_max_freq(&self, value: &str) -> ah::Result<()> {
        let maximum = self.attributes.cpuinfo_max_freq.parse::<u64>().unwrap();
        let value_u64 = value.parse::<u64>().unwrap();

        if value_u64 > maximum {
            ah::bail!("The frequency requested ({}) exceeds the rated maximum ({}) of your CPU.\nDo you want your computer to melt? Not under this program's watch.", value, maximum);
        }

        let file = OpenOptions::new()
            .write(true)
            .open(format!("{}/scaling_max_freq", self.directory))
            .unwrap();

        let mut writer = BufWriter::new(file);

        if let Err(e) = writer.write_all(value.as_bytes()) {
            eprintln!("Error writing to file: {:?}", e);
        }
        Ok(())
    }

    fn set_scaling_min_freq(&self, value: &str) -> ah::Result<()> {
        let minimum = self.attributes.cpuinfo_min_freq.parse::<u64>().unwrap();
        let value_u64 = value.parse::<u64>().unwrap();

        if value_u64 < minimum {
            ah::bail!("The frequency requested ({}) is below the minimum required ({}) for your CPU to operate.\nDo you really think your CPU's gonna run at that low of a frequency? Not happening, dork.", value, minimum);
        }

        let file = OpenOptions::new()
            .write(true)
            .open(format!("{}/scaling_min_freq", self.directory))
            .unwrap();

        let mut writer = BufWriter::new(file);

        if let Err(e) = writer.write_all(value.as_bytes()) {
            eprintln!("Error writing to file: {:?}", e);
        }
        Ok(())
    }

    fn set_scaling_setspeed(&self, value: &str) -> ah::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .open(format!("{}/scaling_setspeed", self.directory))
            .unwrap();

        let mut writer = BufWriter::new(file);

        if let Err(e) = writer.write_all(value.as_bytes()) {
            eprintln!("Error writing to file: {:?}", e);
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Argument {
    name: String,
    value: Option<Vec<String>>,
}

fn collect_arguments(args: &Vec<String>, recognized: &Vec<String>) -> Vec<Argument> {
    let mut arguments: Vec<Argument> = Vec::with_capacity(args.len());
    let argument_iterator = args.iter().peekable();

    let mut latest_argument: Option<Argument> = None;

    for argument in argument_iterator {
        let is_recognized = recognized.contains(argument);

        if is_recognized {
            if let Some(previous) = latest_argument {
                arguments.push(previous);
            }

            latest_argument = Some(Argument {
                name: argument.clone(),
                value: None,
            });

            continue;
        } 

        let latest_argument: &mut Argument = {
            match &mut latest_argument {
                Some(v) => v,
                None => continue,
            }
        };

        if let Some(value) = &mut latest_argument.value {
            value.push(argument.clone());
        } else {
            latest_argument.value = Some(vec![argument.clone()]);
        }
    }

    if let Some(previous) = latest_argument {
        arguments.push(previous);
    }

    arguments
}


fn main() {
    let policy_dirs = collect_policy_paths().expect("Couldn't collect policies. (Try running as root?)");

    let policy_managers = policy_dirs
        .iter()
        .map(|path| FreqPolicy::from_policy_path(path).unwrap())
        .collect::<Vec<FreqPolicy>>();

    let RECOGNIZED_ARGUMENTS: Vec<String> = ["--set-scmax",
        "--set-scmin",
        "--set-governor",
        "--get-governor",
        "--list-governors",
        "--get-scmax",
        "--get-scmin",
        "--get-rmax",
        "--get-rmin"]

    .iter()
    .map(|s| s.to_string())
    .collect();

    let arguments = std::env::args().collect::<Vec<String>>();
    // println!("Arguments Env: {:?}", arguments);

    let arguments = collect_arguments(&arguments, &RECOGNIZED_ARGUMENTS);
    // println!("Arguments: {:?}", arguments);
    

    for (_index, argument) in arguments.iter().enumerate() {
        if let Some(values) = &argument.value {
            let argument_name = argument.name.as_ref();


            let _first_value = {
                match values.first() {
                    Some(value) => value,
                    None => continue,
                }
            };

            match argument_name {
                "--set-scmax" => {
                    let value = values.first().expect("Couldn't get value for --set-scmax");

                    for policy_manager in policy_managers.iter() {
                        policy_manager.set_scaling_max_freq(value).unwrap();
                    }
                }

                "--set-scmin" => {
                    let value = values.first().expect("Couldn't get value for --set-scmin");

                    for policy_manager in policy_managers.iter() {
                        policy_manager.set_scaling_min_freq(value).unwrap();
                    }
                }

                "--set-governor" => {
                    let value = values.first().expect("Couldn't get value for --set-governor");

                    for policy_manager in policy_managers.iter() {
                        policy_manager.set_scaling_governor(value).unwrap();
                    }
                }

                "--get-governor" => {
                    for policy_manager in policy_managers.iter() {
                        println!("{} - {}", policy_manager.directory, policy_manager.attributes.scaling_governor);
                    }
                }
                
                "--get-scmax" => {
                    for policy_manager in policy_managers.iter() {
                        println!("{} - {}", policy_manager.directory, policy_manager.attributes.scaling_max_freq);
                    }
                }

                "--get-scmin" => {
                    for policy_manager in policy_managers.iter() {
                        println!("{} - {}", policy_manager.directory, policy_manager.attributes.scaling_min_freq);
                    }
                }

                "--get-rmax" => {
                    for policy_manager in policy_managers.iter() {
                        println!("{} - {}", policy_manager.directory, policy_manager.attributes.cpuinfo_max_freq);
                    }
                }

                "--get-rmin" => {
                    for policy_manager in policy_managers.iter() {
                        println!("{} - {}", policy_manager.directory, policy_manager.attributes.cpuinfo_min_freq);
                    }
                }

                _ => { 
                    println!("Unrecognized argument: {}", argument_name);
                    std::process::exit(1);
                }
            }
        }
    }
}
