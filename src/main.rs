use anyhow as ah;

use std::collections::HashMap;
use std::env;
use std::fs::{self, DirEntry, OpenOptions};
use std::io::{BufWriter, Read, Write};
use std::path::Path;

pub mod argparse;
pub mod frequency;
pub mod macros;
pub mod policies;

pub mod globals;
use globals::*;

const HELP_TEXT: &str = "
\n

Setting Values:
cpm set <policies> <attribute> <value>

<policies> format:
    0     - A single policy number.
    0:4   - A range of policy numbers.
    0,2,5 - Specific policy number.

    (all | *) - Every policy.

<attribute> format:
    freq - CPU Frequency
    gov  - CPU Governor
    perf - CPU Performance Profile

<value> format:
    freq: <min>:<max>
        If no suffix is provided, and a period is present, defualts to GHz
        2.5:3.5 - Sets scaling min/max to 2.5 and 3.5 GHz

        If no suffix is provided, and no periods are present, defaults to KHz
        2500000:3500000 - Sets scaling min/max to 2.5/3.5 GHz

        To set only min, or only max, provide no value on the left or right.
        The inclusion of a : is mandatory however.

        :3.5 - Sets scaling max to 3.5 GHz
        2.5: - Sets scaling min to 2.5 Ghz

        Available suffixes: g, m, k, h

        :3.5g            - Sets scaling max to 3.5 GHz
        :3,500m          - Sets scaling max to 3,500 MHz        (3.5GHz)
        :3,500,000k      - Sets scaling max to 3,500,000 KHz    (3.5GHz)
        :3,500,000,000h  - Sets scaling max to 3,500,000,000 Hz (3.5GHz)

        Comas are ignored, so feel free to use them for readability.

        Suffixes can also be mixed

        2,500m:3,500,000k - Sets scaling min/max to 2.5/3.5Ghz

    gov: <governor>
        Must be a valid CPU governor. You can check available governors using
            cpm get 0 gov avail

        Then set it like this
            cpm set all gov powersave

    perf: <profile>
        Must be a valid performance profile. Get available profiles with
            cpm get 0 perf avail

        Then set it like this
            cpm set all perf balance_performance
        

Getting Values:
    cpm set <policies> <attribute> <value> 

    <policies> format:
        0     - A single policy number.
        0:4   - A range of policy numbers.
        0,2,5 - Specific policy number.

        (all | *)   - Every policy.

    <attribute> format:
        freq - CPU Frequency
        gov  - CPU Governor
        perf - CPU Performance Profile

    <value> format:
        freq: min, max, (curr | current)
        gov:  (curr | current), (avail | available)
        perf: (curr | current), (avail | available)
\n
";

fn main() {
    if let Err(e) = argparse::parse_arguments() {
        eprintln!("{}\n\n{}", e, HELP_TEXT);
    }
}
