use ah::bail;
use anyhow as ah;
use std::env::{self, args, Args};

use crate::frequency::Frequency;
use crate::policies::{PolicyDir, PolicyFile};

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

pub fn set_freq_operation(policy: PolicyDir, desired: String) {}

pub fn parse_policy_ident(ident: &str) {

    // could be 0:4
    // could be 4,2,6
    // could be 6
    // could be * or 'all'


}

pub fn parse_arguments() -> ah::Result<()> {
    let arguments: Vec<String> = env::args().collect();
    let arg_iter = arguments.into_iter();

    let parts: Vec<String> = arg_iter.take(2).collect();

    let action = parts.get(0).ok_or(ah::anyhow!(
        "Index 0 of operation parts was empty; missing an action."
    ))?;

    let policy = parts.get(1).ok_or(ah::anyhow!(
        "Index 1 of operation parts was empty; missing a target policy."
    ))?;

    match action.to_lowercase().as_str() {
        "set" => {

        }

        "get" => {

        }

        _ => {
        }

    }

    Ok(())
}
