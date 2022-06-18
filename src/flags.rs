use super::filters::{isolate::*, Filters};

use std::collections::{VecDeque, HashMap};
use std::vec::Vec;

pub type Flags = HashMap<String, Vec<String>>;


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


pub fn retrieve_filters(flags: Flags) -> Vec<Filters> {
    let mut filters = Vec::new();
    
    for (flag, _parameters) in flags {
        match flag.as_str() {
            "-isor" => {
                filters.push(
                    Filters::IsolateRed( IsolateRed{} )
                );
            },

            "-isog" => {
                filters.push(
                    Filters::IsolateGreen( IsolateGreen{} )
                );
            },

            "-isob" => {
                filters.push(
                    Filters::IsolateBlue( IsolateBlue{} )
                );
            },
            
            _ => {  }
        }
    }

    return filters;
}