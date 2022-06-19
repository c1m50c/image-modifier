use std::collections::{VecDeque, HashMap};
use std::vec::Vec;

use crate::filters::{Filter, FILTERS};

pub type Flags = HashMap<String, Vec<String>>;


// TODO: This returns the flags in improper order, fix
pub fn get_flags(mut args: VecDeque<String>) -> Flags {
    let mut flags = HashMap::new();

    while args.len() != 0 {
        let current = args.pop_front().unwrap();

        if current.chars().nth(0) == Some('-') {
            flags.insert(current.clone(), Vec::new());

            while args.len() != 0 {
                if args[0].chars().nth(0) == Some('-') {
                    break;
                }

                flags.get_mut(&current).unwrap()
                    .push(args.pop_front().unwrap());
            }
        }
    }

    return flags;
}


pub fn get_filters_from_flags(flags: Flags) -> Vec<Filter> {
    let mut filters = Vec::new();

    unsafe {
        for (name, func) in &FILTERS {
            if let Some(_) = flags.get(name) {
                filters.push(func.clone());
            }
        }
    }

    return filters;
}