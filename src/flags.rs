use super::filters::{Filters, isolate::*, grey::*};

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
            "-iso_r" => {
                filters.push(
                    Filters::IsolateRed( IsolateRed{} )
                );
            },

            "-iso_g" => {
                filters.push(
                    Filters::IsolateGreen( IsolateGreen{} )
                );
            },

            "-iso_b" => {
                filters.push(
                    Filters::IsolateBlue( IsolateBlue{} )
                );
            },

            "-grey" => {
                filters.push(
                    Filters::Greyscale( Greyscale{} )
                )
            },

            "-grey_avg" => {
                filters.push(
                    Filters::GreyscaleAvg( GreyscaleAvg{} )
                )
            },

            "-grey_r" => {
                filters.push(
                    Filters::GreyscaleR( GreyscaleR{} )
                )
            },

            "-grey_g" => {
                filters.push(
                    Filters::GreyscaleG( GreyscaleG{} )
                )
            },

            "-grey_b" => {
                filters.push(
                    Filters::GreyscaleB( GreyscaleB{} )
                )
            },
            
            _ => {  }
        }
    }

    return filters;
}